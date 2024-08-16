use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, thread, time::Duration};
use tokio::{
    self,
    io::{self, Interest},
    net::{TcpSocket, TcpStream},
    runtime::Runtime,
    sync::{self, broadcast, oneshot, watch, Barrier, Notify, RwLock, Semaphore},
    task::{self, JoinError},
    time,
};
use tokio_stream::{self as stream, StreamExt as TokioStreamExt};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn test_drop() {
    let rt = Runtime::new().unwrap();

    // 进入runtime，但不阻塞当前线程
    let guard1 = rt.enter();

    // 生成的异步任务将放入当前的runtime上下文中执行
    tokio::spawn(async {
        time::sleep(time::Duration::from_secs(5)).await;
        println!("task1 sleep over: {}", now());
    });

    // 释放runtime上下文，这并不会删除runtime
    drop(guard1);

    // 可以再次进入runtime
    let guard2 = rt.enter();
    tokio::spawn(async {
        time::sleep(time::Duration::from_secs(2)).await;
        println!("task2 sleep over: {}", now());
    });

    drop(guard2);

    // 阻塞当前线程，等待异步任务的完成
    thread::sleep(std::time::Duration::from_secs(10));
}

fn task_spawn() {
    let rt = Runtime::new().unwrap();
    let _guard = rt.enter();
    task::spawn(async {
        time::sleep(time::Duration::from_secs(3)).await;
        println!("task over:{}", now());
    });

    thread::sleep(time::Duration::from_secs(4));
}

async fn task_spawn_blocking() {
    let join = task::spawn_blocking(|| {
        // do some compute-heavy work or call synchronous code
        "blocking completed"
    });

    let result = join.await.unwrap();
}

fn task_block_in_place() {
    // run in current task worker thread
    task::block_in_place(move || {
        // do some compute-heavy work or call synchronous code
    });
}

async fn task_yield_now() {
    // 放弃当前CPU，该任务放入就绪队列等待轮询调度
    task::spawn(async {
        // ...
        println!("spawned task done!")
    });

    // Yield, allowing the newly-spawned task to execute first.
    task::yield_now().await;
    println!("main task done!");
}

async fn cancel_token() {
    // Step 1: Create a new CancellationToken
    let token = CancellationToken::new();

    // Step 2: Clone the token for use in another task
    let cloned_token = token.clone();

    // Task 1 - Wait for token cancellation or a long time
    let task1_handle = tokio::spawn(async move {
        tokio::select! {
            // Step 3: Using cloned token to listen to cancellation requests
            _ = cloned_token.cancelled() => {
                println!("cloned_token.cancelled()");
                // The token was cancelled, task can shut down
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(9999)) => {
                println!("sleep 9999");
                // Long work has completed
            }
        }
    });

    // Task 2 - Cancel the original token after a small delay
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        // Step 4: Cancel the original or cloned token to notify other tasks about shutting down gracefully
        token.cancel();
    });

    // Wait for tasks to complete
    task1_handle.await.unwrap()
}

fn cancel_task() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let task = tokio::task::spawn(async {
            time::sleep(time::Duration::from_secs(1)).await;
        });

        time::sleep(time::Duration::from_millis(3000)).await;
        task.abort(); //cancel task
                      //
        let abort_err: JoinError = task.await.unwrap_err();
        println!("abort_err.is_cancelled {}", abort_err.is_cancelled());
    })
}

async fn do_one() {
    println!("doing one: {}", now());
    time::sleep(time::Duration::from_secs(4)).await;
    println!("do one done: {}", now());
}

async fn do_two() {
    println!("doing two: {}", now());
    time::sleep(time::Duration::from_secs(1)).await;
    println!("do two done: {}", now());
}

fn await_all_finish() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        tokio::join!(do_one(), do_two()); // 等待两个任务均完成，才继续向下执行代码
        println!("all done: {}", now());
    });
}

async fn do_stuff_async() -> Result<(), &'static str> {
    // async work
    do_one().await;

    Ok(())
}

async fn more_async_work() -> Result<(), &'static str> {
    do_two().await;

    Err("quick err")
}

fn await_try_all_finish() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let res = tokio::try_join!(do_stuff_async(), more_async_work());

        match res {
            Ok((first, second)) => {
                // do something with the values
            }
            Err(err) => {
                println!("processing failed; error = {}", err);
            }
        }
    });
}

async fn sleep(n: u64) -> u64 {
    time::sleep(Duration::from_secs(n)).await;
    n
}

fn await_any_finish_select() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        tokio::select! {
          v = sleep(5) => println!("sleep 5 secs, branch 1 done: {}", v),
          v = sleep(3) => println!("sleep 3 secs, branch 2 done: {}", v),
        };

        println!("select! done");
    });
}

async fn test_else() {
    let (mut tx1, mut rx1) = tokio::sync::mpsc::channel::<Option<i32>>(128);
    let (mut tx2, mut rx2) = tokio::sync::mpsc::channel::<Option<i32>>(128);

    tokio::spawn(async move {
        tx1.send(None).await;
        // Do something w/ `tx1` and `tx2`
    });

    tokio::select! {
        Some(v) = rx1.recv() => {
            println!("Got {:?} from rx1", v);
        }
        Some(v) = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        }
        else => {
            println!("Both channels closed");
        }
    }
}

fn await_any_finish_select_biased() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut count = 0u8;
        loop {
            tokio::select! {
                //biased;
                _ = async {}, if count < 1 => { count += 1; println!("no.1:{}",count); }
                _ = async {}, if count < 2 => { count += 1; println!("no.2:{}",count); }
                _ = async {}, if count < 3 => { count += 1; println!("no.3:{}",count); }
                _ = async {}, if count < 4 => { count += 1; println!("no.4:{}",count); }
                else => { break; }
            }
        }
    })
}

//同步机制与通信机制
//通信机制：
// 1-1 oneshot : 单次使用
// n-1 mpsc : 有界通道（通道有容量限制，即通道中最多可以存放指定数量(至少为1)的消息，通过mpsc::channel()创建）/无界通道 （无界通道，通道中可以无限存放消息，直到内存耗尽，通过mpsc::unbounded_channel()创建）
// 1-n watch
// n-n broadcast
fn await_oneshot() {
    let (tx, rx) = oneshot::channel::<&str>();
    if let Err(_) = tx.send("send data") {
        println!("receiver closed");
    }

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match rx.await {
            Ok(v) => println!("got = {:?}", v),
            Err(_) => println!("the sender dropped"),
            // Err(e: RecvError) => xxx,
        }
    });
}

fn await_mpsc() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let (tx, mut rx) = sync::mpsc::channel::<i32>(10);

        tokio::spawn(async move {
            for i in 1..=10 {
                // if let Err(_) = tx.send(i).await {}
                if tx.send(i).await.is_err() {
                    println!("receiver closed");
                    return;
                }
            }
        });
        //sleep(1).await;
        while let Some(i) = rx.recv().await {
            println!("received: {}", i);
        }
    });
}

fn await_mpsc_broadcase() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let (tx, mut rx1) = broadcast::channel(16);

        // Sender的subscribe()方法可生成新的Receiver
        let mut rx2 = tx.subscribe();
        tokio::spawn(async move {
            println!("{}", rx1.recv().await.unwrap());
            println!("{}", rx1.recv().await.unwrap());
        });

        tokio::spawn(async move {
            println!("{}", rx2.recv().await.unwrap());
            println!("{}", rx2.recv().await.unwrap());
        });

        tx.send(10).unwrap();
        tx.send(20).unwrap();
    })
}

// 内部使用读写锁
fn await_watch() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // 创建watch通道时，需指定一个初始值存放在通道中
        let (tx, mut rx) = watch::channel("hello");

        // Recevier端，通过changed()来等待通道的数据发生变化
        // 通过borrow()引用通道中的数据
        tokio::spawn(async move {
            while rx.changed().await.is_ok() {
                println!("received = {:?}", *rx.borrow());
            }
        });

        // 向通道中发送数据，实际上是修改通道中的那个数据
        tx.send("world").unwrap();
    })
}

// 同步机制：
// Mutex: 互斥锁 => 任务要执行某些操作时，必须先申请锁，只有申请到锁之后才能执行操作，否则就等待
// RwLock: 读写锁 => 类似于互斥锁，但粒度更细，区分读操作和写操作，可以同时存在多个读操作，但写操作必须独占锁资源
// Notify: 任务通知 => 用于唤醒正在等待的任务，使其进入就绪态等待调度
// Barrier: 屏障, 多个任务同步点 => 多个任务在某个屏障处互相等待，只有这些任务都达到了那个屏障点，这些任务才都继续向下执行
// Semaphore: 信号量(信号灯) => 限制同时执行的任务数量，例如限制最多只有20个线程(或tokio的异步任务)同时执行

fn await_mutex() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mutex = Arc::new(sync::Mutex::new(0));
        for i in 0..10 {
            let lock = Arc::clone(&mutex);
            tokio::spawn(async move {
                let mut data = lock.lock().await;
                *data += 1;
                println!("task: {}, data: {}", i, data);
            });
        }

        time::sleep(Duration::from_secs(1)).await;
    });
}

fn await_rwlock() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let lock = RwLock::new(5);

        // 多个读锁共存
        {
            // read()返回RwLockReadGuard
            let r1 = lock.read().await;
            let r2 = lock.read().await;
            assert_eq!(*r1, 5); // 对Guard解引用，即可得到其内部的值
            assert_eq!(*r2, 5);
        } // 读锁(r1, r2)在此释放

        // 只允许一个写锁存在
        {
            // write()返回RwLockWriteGuard
            let mut w = lock.write().await;
            *w += 1;
            assert_eq!(*w, 6);
        } // 写锁(w)被释放
    });
}

fn await_notify() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let notify = Arc::new(Notify::new());
        let notify2 = notify.clone();

        tokio::spawn(async move {
            notify2.notified().await;
            println!("received notification");
        });

        println!("sending notification");
        notify.notify_one();
    });
}

fn await_barrier() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut handles = Vec::with_capacity(10);
        // 参数10表示屏障宽度为10，只等待10个任务达到屏障点就放行这一批任务
        // 也就是说，某时刻已经有9个任务在等待，当第10个任务调用wait的时候，屏障将放行这一批
        let barrier = Arc::new(Barrier::new(10));
        for _ in 0..10 {
            let c = barrier.clone();
            handles.push(tokio::spawn(async move {
                println!("before wait");

                // 在此设置屏障，保证10个任务都已输出before wait才继续向下执行
                let wait_result = c.wait().await;
                println!("after wait");
                wait_result
            }));
        }

        let mut num_leaders = 0;
        for handle in handles {
            let wait_result = handle.await.unwrap();
            if wait_result.is_leader() {
                num_leaders += 1;
            }
        }

        assert_eq!(num_leaders, 1);
    });
}

fn await_semaphore() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // 只有3个信号灯的信号量
        let semaphore = Arc::new(Semaphore::new(3));

        // 5个并发任务，每个任务执行前都先获取信号灯
        // 因此，同一时刻最多只有3个任务进行并发
        for i in 1..=5 {
            let semaphore = semaphore.clone();
            tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                println!("{}, {}", i, now());
                time::sleep(Duration::from_secs(1)).await;
            });
        }

        time::sleep(Duration::from_secs(3)).await;
    });
}

/// tokio tcp server
fn test_tcp_server() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();
        loop {
            let (client, client_sock_addr) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                // 该任务负责处理client
            });
        }
    });
}

fn test_split_read_write() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let conn = TcpStream::connect("127.0.0.1:8888").await.unwrap();
        let (mut read_half, mut write_half) = conn.into_split();
    });
}

fn test_tokio_stream() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut st = stream::iter(vec![1, 2, 3]);
        while let Some(value) = futures_util::StreamExt::next(&mut st).await {
            println!("value:{}", value);
        }
    })
}

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_util::{codec::{self, Framed, LinesCodec}, sync::CancellationToken};

fn test_frame_line() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let server = TcpListener::bind("127.0.0.1:8888").await.unwrap();
        while let Ok((client_stream, client_addr)) = server.accept().await {
            tokio::spawn(async move { process_client(client_stream).await });
        }
    })
}

async fn process_client(client_stream: TcpStream) {
    let framed = Framed::new(client_stream, LinesCodec::new());
    let (frame_writer, frame_reader) = framed.split::<String>();
    let (msg_tx, msg_rx) = mpsc::channel::<String>(100);

    let mut read_task = tokio::spawn(async move {
        read_from_client(frame_reader, msg_tx).await;
    });

    let mut write_task = tokio::spawn(async move {
        write_to_client(frame_writer, msg_rx).await;
    });

    if tokio::try_join!(&mut read_task, &mut write_task).is_err() {
        eprintln!("read_task/write_task terminated");
        read_task.abort();
        write_task.abort();
    };
}

type LineFramedStream = SplitStream<Framed<TcpStream, LinesCodec>>;
type LineFramedSink = SplitSink<Framed<TcpStream, LinesCodec>, String>;

async fn read_from_client(mut reader: LineFramedStream, msg_tx: mpsc::Sender<String>) {
    loop {
        match tokio_stream::StreamExt::next(&mut reader).await {
            None => {
                println!("client closed");
                break;
            }
            Some(Err(e)) => {
                eprintln!("read from client error: {}", e);
                break;
            }
            Some(Ok(str)) => {
                println!("read from client. content: {}", str);
                // 将内容发送给writer，让writer响应给客户端，
                // 如果无法发送给writer，继续从客户端读取内容将没有意义，因此break退出
                if msg_tx.send(str).await.is_err() {
                    eprintln!("receiver closed");
                }
            }
        }
    }
}

async fn write_to_client(mut writer: LineFramedSink, mut msg_rx: mpsc::Receiver<String>) {
    while let Some(str) = msg_rx.recv().await {
        println!("write_to_client:{}", str);
        if writer.send(str).await.is_err() {
            eprintln!("write to client failed");
            break;
        }
    }
}

// /// 请求
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Request {
//     pub sym: String,
//     pub from: u64,
//     pub to: u64,
// }

// /// 响应
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Response(pub Option<Klines>);

// /// 对请求和响应的封装，之后客户端和服务端都将通过Sink和Stream来基于该类型通信
// #[derive(Debug, Serialize, Deserialize)]
// pub enum RstResp {
//     Request(Request),
//     Response(Response),
// }

// pub struct RstRespCodec;
// impl RstRespCodec {
//     /// 最多传送1G数据
//     const MAX_SIZE: usize = 1024 * 1024 * 1024 * 8;
// }

// impl codec::Encoder<RstResp> for RstRespCodec {
//     type Error = bincode::Error;
//     // 本示例中使用bincode将RstResp转换为&[u8]，也可以使用serde_json::to_vec()，前者效率更高一些
//     fn encode(&mut self, item: RstResp, dst: &mut BytesMut) -> Result<(), Self::Error> {
//         let data = bincode::serialize(&item)?;
//         let data = data.as_slice();

//         // 要传输的实际数据的长度
//         let data_len = data.len();
//         if data_len > Self::MAX_SIZE {
//             return Err(bincode::Error::new(bincode::ErrorKind::Custom(
//                 "frame is too large".to_string(),
//             )));
//         }

//         // 最大传输u32的数据(可最多512G)，
//         // 表示数据长度的u32数值占用4个字节
//         dst.reserve(data_len + 4);

//         // 先将长度值写入dst，即帧首，
//         // 写入的字节序是大端的u32，读取时也要大端格式读取，
//         // 也有小端的方法`put_u32_le()`，读取时也得小端读取
//         dst.put_u32(data_len as u32);

//         // 再将实际数据放入帧尾
//         dst.extend_from_slice(data);
//         Ok(())
//     }
// }

// /// 实现Decoder，将字节数据转换为RstResp
// impl codec::Decoder for RstRespCodec {
//     type Item = RstResp;
//     type Error = std::io::Error;
//     // 从不断被填充的Bytes buf中读取数据，并将其转换到目标类型
//     fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
//         let buf_len = src.len();

//         // 如果buf中的数据量连长度声明的大小都不足，则先跳过等待后面更多数据的到来
//         if buf_len < 4 { return Ok(None); }

//         // 先读取帧首，获得声明的帧中实际数据大小
//         let mut length_bytes = [0u8; 4];
//         length_bytes.copy_from_slice(&src[..4]);
//         let data_len = u32::from_be_bytes(length_bytes) as usize;
//         if data_len > Self::MAX_SIZE {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::InvalidData,
//                 format!("Frame of length {} is too large.", data_len),
//             ));
//         }

//         // 帧的总长度为 4 + frame_len
//         let frame_len = data_len + 4;

//         // buf中数据量不够，跳过，并预先申请足够的空闲空间来存放该帧后续到来的数据
//         if buf_len < frame_len {
//             src.reserve(frame_len - buf_len);
//             return Ok(None);
//         }

//         // 数据量足够了，从buf中取出数据转编成帧，并转换为指定类型后返回
//         // 需同时将buf截断(split_to会截断)
//         let frame_bytes = src.split_to(frame_len);
//         match bincode::deserialize::<RstResp>(&frame_bytes[4..]) {
//             Ok(frame) => Ok(Some(frame)),
//             Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
//         }
//     }
// }

#[tokio::main]
async fn main() {
    //test_frame_line();
    test_else().await;
}
