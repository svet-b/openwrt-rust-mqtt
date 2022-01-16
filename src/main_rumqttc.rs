use rumqttc::{Client, MqttOptions, QoS};
use std::str;
use std::thread;
use std::time::Duration;

const BROKER_HOST: &str = "localhost";
const BROKER_PORT: u16 = 1883;
const CLIENT_ID: &str = "rust_subscribe";
const SUB_TOPIC: &str = "hello/#";
const PUB_TOPIC: &str = "hello/output";
// const RECONNECT_TIME: u64 = 5;
const KEEPALIVE: u64 = 20;
const CAPACITY: usize = 5;

fn main() {
    let mut mqttoptions = MqttOptions::new(CLIENT_ID, BROKER_HOST, BROKER_PORT);
    mqttoptions.set_keep_alive(Duration::from_secs(KEEPALIVE));

    let (mut client, mut connection) = Client::new(mqttoptions, CAPACITY);
    client.subscribe(SUB_TOPIC, QoS::AtMostOnce).unwrap();

    thread::spawn(move || {
        for _ in 0..10 {
            client
                .publish(PUB_TOPIC, QoS::AtLeastOnce, false, "hello")
                .unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    // Iterate to poll the eventloop for connection progress
    for (_, notification) in connection.iter().enumerate() {
        println!("Notification = {:?}", notification.unwrap());
    }
}
