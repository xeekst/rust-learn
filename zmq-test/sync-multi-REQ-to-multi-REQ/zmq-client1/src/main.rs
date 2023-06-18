
fn main() {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    requester
        .connect("tcp://localhost:5559")
        .expect("failed to connect requester");
    for request_nbr in 0..100 {
        requester.send(&format!("idx:{request_nbr},11111111 Client Send REQ"), 0).unwrap();
        let message = requester.recv_msg(0).unwrap();
        println!(
            "Received reply {} {}",
            request_nbr,
            message.as_str().unwrap()
        );
    }
}
