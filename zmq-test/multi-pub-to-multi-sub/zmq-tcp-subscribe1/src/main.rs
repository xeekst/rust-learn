fn main() {
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SocketType::SUB).unwrap();
    let addr = "tcp://localhost:5557";
    subscriber.connect(addr).unwrap();  
    let topic = "__TOPIC__SYS_EVENT".to_owned().into_bytes();
    subscriber.set_subscribe(&topic).unwrap();

    loop {
        let topic = subscriber.recv_msg(0).unwrap();
        let data = subscriber.recv_msg(0).unwrap();
        //assert_eq!(&topic[..], &subscription[..]);
        println!(
            "topic:{}, data:{}",
            std::str::from_utf8(&topic).unwrap(),
            std::str::from_utf8(&data).unwrap()
        );
    }
}
