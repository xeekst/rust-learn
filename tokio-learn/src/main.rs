use chrono::Local;
use std::thread;
use tokio::{
    self,
    runtime::Runtime,
    task::{self, JoinError},
    time,
};

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

fn main() {
  await_try_all_finish();
}
