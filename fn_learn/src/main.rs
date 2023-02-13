// 该函数将闭包作为参数并调用它。
fn apply<F>(f: F)
where
    // 闭包没有输入值和返回值。
    F: FnOnce(),
{
    // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。

    f();
}

// 输入闭包，返回一个 `i32` 整型的函数。
fn apply_to_3<F>(f: F) -> i32
where
    // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
    F: Fn(i32) -> i32,
{
    f(3)
}

// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
fn call_me<F: Fn(i32) -> i32>(f: F) -> i32 {
    f(5)
}
// 定义一个满足 `Fn` 约束的封装函数（wrapper function）。
fn function(x: i32) -> i32 {
    println!("I'm a function! {}", x);

    11 + x
}

///////////// 返回闭包 /////////////////////
/// 闭包作为输入参数是可能的，所以返回闭包作为输出参数（output parameter）也应该是可能的。然而返回闭包类型会有问题，因为目前 Rust 只支持返回具体（非泛型）的类型。按照定义，匿名的闭包的类型是未知的，所以只有使用impl Trait才能返回一个闭包。
//返回闭包的有效特征是：Fn、FnMut、FnOnce
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn test_return() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
}

fn main() {
    // 定义一个满足 `Fn` 约束的闭包。
    let closure = |x: i32| {
        println!("I'm a closure!{}", x);
        x + 1
    };

    call_me(closure);
    call_me(function);
}
