use std::{env, process, str, thread, time::Duration};

use paho_mqtt as mqtt;

const BROKER_URI: &str = "tcp://localhost:1883";
const CLIENT_ID: &str = "rust_subscribe";
const SUB_TOPIC: &str = "in/#";
const PUB_TOPIC: &str = "out/pub";
const QOS: i32 = 1;

fn process_message(msg: &[u8]) -> f64 {
    str::from_utf8(msg)
        .unwrap()
        .parse::<f64>()
        .unwrap_or_default()
        * 2.0
}

fn init_mqtt_client() -> mqtt::Client {
    let host = env::args().nth(1).unwrap_or_else(|| BROKER_URI.to_string());

    // Define the set of options for the create.
    // Use an ID for a persistent session.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(CLIENT_ID.to_string())
        .finalize();

    // Create a client.
    mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    })
}

// Initial connection to broker
fn mqtt_connect(cli: &mqtt::Client) -> bool {
    // Define the set of options for the connection.
    let lwt = mqtt::MessageBuilder::new()
        .topic("test")
        .payload("Consumer lost connection")
        .finalize();
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .will_message(lwt)
        .finalize();

    println!("Connecting to broker");
    loop {
        if cli.connect(conn_opts.clone()).is_ok() {
            println!("Successfully connected");
            subscribe_topics(&cli);
            return true;
        }
        println!("Connection failed. Waiting to retry connection");
        thread::sleep(Duration::from_millis(5000));
        println!("Retrying");
    }
}

fn mqtt_reconnect(cli: &mqtt::Client) -> bool {
    loop {
        thread::sleep(Duration::from_millis(5000));
        println!("Reconnecting");
        if cli.reconnect().is_ok() {
            println!("Successfully reconnected");
            subscribe_topics(&cli);
            return true;
        }
    }
}

// Subscribes to multiple topics.
fn subscribe_topics(cli: &mqtt::Client) {
    if let Err(e) = cli.subscribe(SUB_TOPIC, QOS) {
        println!("Error while subscribing to topic: {:?}", e);
        process::exit(1);
    }
}

fn main() {
    use tokio_modbus::prelude::*;

    // let socket_addr = "127.0.0.1:502".parse().unwrap();
    // let mut modbus_client = client::sync::tcp::connect(socket_addr).unwrap();
    // let data = client.read_input_registers(0x1000, 7);
    // println!("Response is '{:?}'", data);

    let tty_path = "/dev/ttyUSB0";
    let slave = Slave(0x17);

    let builder = tokio_serial::new(tty_path, 9600);

    let mut ctx = sync::rtu::connect_slave(&builder, slave).unwrap();
    println!("Reading a sensor value");
    let rsp = ctx.read_holding_registers(0x082B, 2).unwrap();
    println!("Sensor value is: {:?}", rsp);

    let mut client = init_mqtt_client();

    // Initialize the consumer before connecting.
    let rx = client.start_consuming();

    // Connect to broker
    mqtt_connect(&client);

    println!("Processing requests...");
    for msg in rx.iter() {
        if let Some(msg) = msg {
            println!("topic: {:?}; msg: {:?}", msg.topic(), msg.payload_str());
            let msg_out = mqtt::Message::new(
                PUB_TOPIC,
                process_message(msg.payload()).to_string().as_bytes(),
                QOS
            );
            client.publish(msg_out).unwrap();
        } else if !client.is_connected() {
            println!("Connection lost. Waiting to retry connection");
            mqtt_reconnect(&client);
        }
    }

    // If still connected, then disconnect now.
    if client.is_connected() {
        println!("Disconnecting");
        client.unsubscribe(SUB_TOPIC).unwrap();
        client.disconnect(None).unwrap();
    }
    println!("Exiting");
}
