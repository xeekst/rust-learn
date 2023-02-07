use std::time::Duration;

use tokio::{ self, runtime::Runtime, sync };

fn main() {
    let rt = Runtime::new().unwrap();
    let (tx, mut rx) = sync::mpsc::channel::<i32>(10);

    rt.block_on(async {
        tokio::spawn(async move {
            for i in 1..=12 {
                // if let Err(_) = tx.send(i).await {}
                if tx.send(i).await.is_err() {
                    println!("receiver closed");
                    return;
                }
            }
        });
    });

    std::thread::sleep(Duration::from_secs(3));
    while let Ok(i) = rx.try_recv() {
        println!("received: {}", i);
    }

    println!("end exit.");
}
