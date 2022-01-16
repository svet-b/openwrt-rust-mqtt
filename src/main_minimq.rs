use core::str;
use std::thread::sleep;
use std::time::Duration;
use minimq::{Minimq, QoS, Retain};

use std_embedded_nal;
use std_embedded_time;

const BROKER_HOST: &str = "127.0.0.1";
// const BROKER_PORT: u16 = 1883;
const CLIENT_ID: &str = "rust_subscribe";
const SUB_TOPIC: &str = "topic/#";
const PUB_TOPIC: &str = "hello/output";
const RECONNECT_TIME: u64 = 1;
const KEEPALIVE: u16 = 20;

fn process_message(msg: &[u8]) -> f64 {
    str::from_utf8(msg)
    .unwrap()
    .parse::<f64>()
    .unwrap_or_default() * 2.0
}

fn main() {
    // Construct an MQTT client with a maximum packet size of 256 bytes.
    // Connect to a broker at localhost - Use a client ID of "test".
    let mut mqtt: Minimq<_, _, 256, 16> = Minimq::new(
        BROKER_HOST.parse().unwrap(),
        CLIENT_ID,
        std_embedded_nal::Stack::default(),
        std_embedded_time::StandardClock::default(),
    )
    .unwrap();

    mqtt.client.set_keepalive_interval(KEEPALIVE).unwrap();
    let mut subscribed = false;

    loop {
        if mqtt.client.is_connected() && !subscribed {
            mqtt.client.subscribe(SUB_TOPIC, &[]).unwrap_or_else(|err| {
                println!("Error {:?}", err);
                subscribed = false;
            });
            subscribed = true;
        }

        mqtt.poll(|client, topic, msg, _properties| {
            match topic {
                "topic/abc" => {
                    println!("Message: {}", str::from_utf8(msg).unwrap_or_else(|_err| {
                        "Cannot parse as UTF-8"
                    }));
                    client
                        .publish(
                            PUB_TOPIC,
                            process_message(msg).to_string().as_bytes(),
                            QoS::AtLeastOnce,
                            Retain::NotRetained,
                            &[],
                        ).unwrap();           
                }
                topic => println!("Unknown topic: {}", topic),
            };
        })
        .unwrap_or_else(|err| {
            println!("Error while polling for messages: {:?}", err);
            subscribed = false;
            sleep(Duration::from_secs(RECONNECT_TIME));
        });
    }
}
