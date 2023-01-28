use retry::{retry, OperationResult};
use std::{net::TcpStream, path::Path};

fn main() {
    let a:Vec<&str> = "/atm-cloud/task-list/luotao9-task/luotao9-test-subtask/PF2CAV75_20SNZ5GCUS/logs/CT-Build22000-64afterpbr".split("/").collect();
    println!("a：{:?}",a);

    let mut collection = vec![1, 2, 3].into_iter();

    // let result = retry(retry::delay::Fixed::from_millis(100).take(10), || {
    //     let n = collection.next();
    //     println!("next:{:?}", &n);
    //     match n {
    //         Some(n) if n == 3 => OperationResult::Ok("n is 3!"),
    //         Some(_) => OperationResult::Retry("n must be 3!"),
    //         None => OperationResult::Err("n was never 3!"),
    //     }
    // });

    // println!("{:?}", result);
    // let tcp  = TcpStream::connect("10.176.19.101:22").unwrap();
    // let mut sess = ssh2::Session::new().unwrap();
    // sess.set_tcp_stream(tcp);

    // sess.handshake().unwrap();
    // sess.userauth_password("ftpuser", "1qasw@").unwrap();

    // // let mut agent = sess.agent().unwrap();
    // // agent.connect().unwrap();
    // // agent.list_identities().unwrap();
    // // {
    // //     let identity = &agent.identities().unwrap()[0];
    // //     agent.userauth(&user, &identity).unwrap();
    // // }
    // let sftp = sess.sftp().unwrap();
    // let paths = sftp.readdir(Path::new("/"));
    // println!("paths:{:?}", paths);

    println!("Hello, world!");
}
