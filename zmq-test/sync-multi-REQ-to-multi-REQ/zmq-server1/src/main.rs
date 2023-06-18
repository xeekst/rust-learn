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
        println!("Server1 Received request:{}", string);
        thread::sleep(Duration::from_millis(750));
        responder.send("sever 1111111 response World", 0).unwrap();
    }
}
