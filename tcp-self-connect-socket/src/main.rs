use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

//https://developer.jboss.org/thread/268935
//https://blog.csdn.net/haoel/article/details/2224055
//https://blog.csdn.net/haoel/article/details/2224069

// 起因是端口占用，然后看见很多tcp 自己连自己, 好奇是怎么造成的，发现了原来是 java nio 为了唤醒线程使用了 向线程发送socket消息的机制，导致端口占用
// > netstat -anot | findstr 206708
//   TCP    127.0.0.1:49890        127.0.0.1:49891        ESTABLISHED     206708   InHost
//   TCP    127.0.0.1:49891        127.0.0.1:49890        ESTABLISHED     206708   InHost
//   TCP    127.0.0.1:49893        127.0.0.1:49894        ESTABLISHED     206708   InHost
//   TCP    127.0.0.1:49894        127.0.0.1:49893        ESTABLISHED     206708   InHost
//   TCP    127.0.0.1:49895        127.0.0.1:49896        ESTABLISHED     206708   InHost
//   TCP    127.0.0.1:49896        127.0.0.1:49895        ESTABLISHED     206708   InHost

//以下是复现的方式
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:18080")?; // 绑定一个端口

    // 接受连接并创建新的套接字
    let (mut socket1, _) = listener.accept()?;
    let mut socket2 = TcpStream::connect("127.0.0.1:18080")?;

    // 向第一个套接字写入数据
    socket1.write_all(b"Hello, self!")?;

    // 从第二个套接字读取数据
    let mut buffer = [0; 12];
    socket2.read_exact(&mut buffer)?;
    println!("Received: {:?}", String::from_utf8_lossy(&buffer));

    Ok(())
}
