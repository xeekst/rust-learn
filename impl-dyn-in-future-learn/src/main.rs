use futures::channel::mpsc;
use futures::executor;
use futures::executor::ThreadPool;
use futures::StreamExt;
use std::future::Future;
use std::pin::Pin;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    test02().await;
}

// T: Unpin，那么 Pin<T> 和 &mut T 完全一样，也就是Pin也没效果，该移动还是一样移动
// 绝大部分的标准库类型都实现了Unpin，但是 async/await 生成的 Future 没有实现 Unpin
// 可以固定到栈上 unsafe{ Pin::new_unchecked(&mut T) }，也可以固定到堆上 Box::pin(T)

async fn value(x: u8) -> u8 {
    5 + x
}

fn value_p(x: u8) -> Pin<Box<dyn Future<Output = u8> + Send>> {
    Box::pin(async move { 5 + x })
}

fn return_impl_func() -> impl Future<Output = u8> {
    let feature = async {
        let x = value(4).await;
        x + 5
    };

    feature
}

fn test01() {
    //创建一个executor线程池
    let pool = ThreadPool::new().unwrap();
    //创建一个unbounded mpsc channel来进行任务间消息通信
    let (tx, rx) = mpsc::unbounded::<i32>();

    let fut_values = async {
        let fut_tx_result = async move {
            (0..100).for_each(|v| {
                tx.unbounded_send(v).expect("Failed to send");
            })
        };
        // 通过线程池调用fut_tx_result future, poll其结果
        pool.spawn_ok(fut_tx_result);
        // 将rx这个接收消息的Stream转换为储存结果的future
        let fut_values = rx.map(|v| v * 2).collect();
        // 等待接收结果的future执行完毕
        fut_values.await
    };

    //开始执行fut_values, 将会调用Future::poll
    let values: Vec<i32> = executor::block_on(fut_values);
    println!("Values={:?}", values);
}

async fn test02() {
    let f = async { value(4).await };
    let v = input_async(f).await;
    println!("input_async:{v}");

    let v2 = input_async_fn(value_p).await;
    println!("input_async_fn:{v2}");
}

//callback 的入参是由该函数外部确定，也就是实际传入了一个实例
async fn input_async(callback: impl Future<Output = u8>) -> u8 {
    let r = callback.await;

    r
}

// `impl Trait` only allowed in function and inherent method return types, not in `Fn` trait return
//async fn input_async_fn(callback: Fn(u8) -> impl Future<Output = u8>) -> u8 {}

//callback 的入参是该函数内部决定，也就是传入了一个函数
async fn input_async_fn<F>(callback: F) -> u8
where
    F: Fn(u8) -> Pin<Box<dyn Future<Output = u8> + Send>>,
{
    let x1 = callback(2).await;

    x1 + 5
}
