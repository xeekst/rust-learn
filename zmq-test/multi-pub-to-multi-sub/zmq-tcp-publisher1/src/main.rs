use std::time::Duration;

use rand::RngCore;

fn main() {
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .connect("tcp://localhost:5556")
        .expect("could not bind publisher socket");
    let topic = "__TOPIC__SYS_EVENT".to_owned().into_bytes();
    let mut rng = rand::thread_rng();
    loop {
        std::thread::sleep(Duration::from_millis(1000));
        publisher.send(&topic, zmq::SNDMORE).unwrap();
        let data = rng.next_u64().to_string() + "Off with his head!";
        publisher.send(&data, 0).unwrap();
        println!("send :{data}");
    }
}
