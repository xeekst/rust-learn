use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    responder
        .connect("tcp://localhost:5560")
        .expect("failed connecting responder");

    loop {
        let string = responder.recv_string(0).unwrap().unwrap();
        println!("server2 Received request:{}", string);
        thread::sleep(Duration::from_millis(3330));
        responder.send("sever 222222 response World", 0).unwrap();
    }
}
