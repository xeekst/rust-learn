use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let xpub_sub = context.socket(zmq::XPUB).unwrap();
    let pub_xsub = context.socket(zmq::XSUB).unwrap();
    pub_xsub.bind("tcp://*:5556").unwrap();
    xpub_sub.bind("tcp://*:5557").unwrap();

    zmq::proxy(&pub_xsub, &xpub_sub).unwrap();

    loop {
        std::thread::sleep(Duration::from_millis(1000));
        println!("proxy success");
    }
}
