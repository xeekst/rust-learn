use std::time::Duration;
use tokio::{self, runtime::Runtime, sync};

fn main() {
    let rt = Runtime::new().unwrap();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(5);

    rt.block_on(async {
        let handle = tokio::spawn(async move {
            for i in 1..=12 {
                tokio::time::sleep(Duration::from_millis(500)).await;
                println!("tx capacity:{}",tx.capacity());
                if tx.send(i).await.is_err() {
                    println!("receiver closed");
                    return;
                }
                println!("send :{i}");
            }
            println!("send finish: ");
        });
        tokio::spawn(async move {
            let mut idx = 0;
            loop {
                tokio::select! {
                    Some(r) = rx.recv() => {
                        println!("recv: {:?}", r);
                        tokio::time::sleep(Duration::from_secs(2)).await;
                    }
                    _ = tokio::time::sleep(Duration::from_secs(1)) => {
                        idx +=1;
                        println!("sleeping :{idx}");
                    }
                }
            }
        });
        //handle.await.unwrap(); // 等待异步任务完成
        println!("end: ");
    });

    println!("start sleep: ");
    loop {
        
    }

    println!("end exit.");
}
