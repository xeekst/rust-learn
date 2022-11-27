use std::sync::{Arc, RwLock};

#[derive(Default)]
struct Config {
    pub debug_mode: bool,
}

impl Config {
    pub fn current() -> Arc<Config> {
        CURRENT_CONFIG.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CURRENT_CONFIG.with(|c| *c.write().unwrap() = Arc::new(self))
    }
}

thread_local! {
    static CURRENT_CONFIG: RwLock<Arc<Config>> = RwLock::new(Default::default());
}

fn single_test() {
    Config { debug_mode: true }.make_current();
    if Config::current().debug_mode {
        // do something
    }
}
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
    v2: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item, v2: 3 }
    }
}

fn test_from_into() {
    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

fn test_closure() {
    use std::mem;

    let color = String::from("green");

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到
    // `print` 离开作用域。
    //
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("`color`: {}", color);

    // 使用借用来调用闭包 `color`。
    print();

    // `color` 可再次被不可变借用，因为闭包只持有一个指向 `color` 的不可变引用。
    let _reborrow = &color;
    print();

    // 在最后使用 `print` 之后，移动或重新借用都是允许的。
    let _color_moved = color;

    let mut count = 0;
    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包
    inc();

    // 因为之后调用闭包，所以仍然可变借用 `count`
    // 试图重新借用将导致错误
    //let _reborrow = &count;
    // ^ 试一试：将此行注释去掉。
    inc();

    // 闭包不再借用 `&mut count`，因此可以正确地重新借用
    let _count_reborrowed = &mut count;

    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    //consume();
    // ^ 试一试：将此行注释去掉。
}

fn test_closure_fn_fnonce_fnmut() {
    //当闭包函数其中的使用
    // 1. 涉及到移动所有权时 =》 FnOnce
    // 2. 涉及到使用可变变量时 =》 FnMut
    // 3. 涉及到只读引用的时候 =》 Fn

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

    use std::mem;

    let greeting = "hello";
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建有所有权的数据。
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
    let diary = || {
        // `greeting` 通过引用捕获，故需要闭包是 `Fn`。
        println!("I said {}.", greeting);

        // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 又要求闭包通过值获取 `farewell`。
        // 现在需要 `FnOnce`。
        //mem::drop(farewell);
    };

    // 以闭包作为参数，调用函数 `apply`。
    apply(diary);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

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

fn test_fn_output() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

fn input_and_return_fn() {
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 命令式（imperative）的写法
    // 声明累加器变量
    let mut acc = 0;
    // 迭代：0，1, 2, ... 到无穷大
    for n in 0.. {
        // 数字的平方
        let n_squared = n * n;

        if n_squared >= upper {
            // 若大于上限则退出循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数就计数
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // 函数式的写法
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // 所有自然数取平方
        .take_while(|&n| n < upper) // 取小于上限的
        .filter(|&n| is_odd(n)) // 取奇数
        .fold(0, |sum, i| sum + i); // 最后加起来
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

//条件编译 cfg
fn test_cfg() {
    #[cfg(target_os = "linux")]
    fn are_you_on_linux() {
        println!("You are running linux!")
    }

    // 而这个函数仅当目标系统 **不是** Linux 时才会编译
    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() {
        println!("You are *not* running linux!")
    }

    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}

fn partial_move() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // `name` 从 person 中移走，但 `age` 只是引用
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // 报错！部分移动值的借用：`person` 部分借用产生
    //println!("The person struct is {:?}", person);

    // `person` 不能使用，但 `person.age` 因为没有被移动而可以继续使用
    println!("The person's age from person struct is {}", person.age);
}

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// 此函数借用了一个 i32 类型
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn test_borrowing() {
    // 创建一个装箱的 i32 类型，以及一个存在栈中的 i32 类型。
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // 借用了 box 的内容，但没有取得所有权，所以 box 的内容之后可以再次借用。
    // 译注：请注意函数自身就是一个作用域，因此下面两个函数运行完成以后，
    // 在函数中临时创建的引用也就不复存在了。
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // 取得一个对 box 中数据的引用
        let _ref_to_i32: &i32 = &boxed_i32;

        // 报错！
        // 当 `boxed_i32` 里面的值之后在作用域中被借用时，不能将其销毁。
        //eat_box_i32(boxed_i32);
        // 改正 ^ 注释掉此行

        // 在 `_ref_to_i32` 里面的值被销毁后，尝试借用 `_ref_to_i32`
        //（译注：如果此处不借用，则在上一行的代码中，eat_box_i32(boxed_i32)可以将 `boxed_i32` 销毁。）
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` 离开作用域且不再被借用。
    }

    // `boxed_i32` 现在可以将所有权交给 `eat_i32` 并被销毁。
    //（译注：能够销毁是因为已经不存在对 `boxed_i32` 的引用）
    eat_box_i32(boxed_i32);
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// let 或者 解构 时候 使用 ref 来表示引用
fn test_ref() {
    let c = 'Q';

    // 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号。
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // 在解构一个结构体时 `ref` 同样有效。
    let _copy_of_x = {
        // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;

        // 返回一个 `point` 的 `x` 字段的拷贝。
        *ref_to_x
    };

    // `point` 的可变拷贝
    let mut mutable_point = point;

    {
        // `ref` 可以与 `mut` 结合以创建可变引用。
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;

        // 通过可变引用来改变 `mutable_point` 的字段 `y`。
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    // 包含一个指针的可变元组
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    println!("tuple is {:?}", mutable_tuple);
    {
        // 解构 `mutable_tuple` 来改变 `last` 的值。
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}

// `print_refs` 接受两个 `i32` 的引用，它们有不同的生命周期 `'a` 和 `'b`。
// 这两个生命周期都必须至少要和 `print_refs` 函数一样长。
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// 不带参数的函数，不过有一个生命周期参数 `'a`。
fn failed_borrow<'a>() {
    let _x = 12;

    // 报错：`_x` 的生命周期不够长
    //let y: &'a i32 = &_x;
    // 在函数内部使用生命周期 `'a` 作为显式类型标注将导致失败，因为 `&_x` 的
    // 生命周期比 `y` 的短。短生命周期不能强制转换成长生命周期。
}

fn test_borrow_life() {
    // 创建变量，稍后用于借用。
    let (four, nine) = (4, 9);

    // 两个变量的借用（`&`）都传进函数。
    print_refs(&four, &nine);
    // 任何被借用的输入量都必须比借用者生存得更长。
    // 也就是说，`four` 和 `nine` 的生命周期都必须比 `print_refs` 的长。

    failed_borrow();
    // `failed_borrow` 未包含引用，因此不要求 `'a` 长于函数的生命周期，
    // 但 `'a` 寿命确实更长。因为该生命周期从未被约束，所以默认为 `'static`。
}

// 一个拥有生命周期 `'a` 的输入引用，其中 `'a` 的存活时间
// 至少与函数的一样长。
fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

// 可变引用同样也可能拥有生命周期。
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// 拥有不同生命周期的多个元素。对下面这种情形，两者即使拥有
// 相同的生命周期 `'a` 也没问题，但对一些更复杂的情形，可能
// 就需要不同的生命周期了。
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

// 返回传递进来的引用也是可行的。
// 但必须返回正确的生命周期。
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

//fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// 上面代码是无效的：`'a` 存活的时间必须比函数的长。
// 这里的 `&String::from("foo")` 将会创建一个 `String` 类型，然后对它取引用。
// 数据在离开作用域时删掉，返回一个指向无效数据的引用。

fn test_lifetime() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

fn re<'a>() -> String {
    return String::from("a");
}

trait UsernameWidget {
    // 从这个 widget 中获取选定的用户名
    fn get(&self) -> String;
}

trait AgeWidget {
    // 从这个 widget 中获取选定的年龄
    fn get(&self) -> u8;
}

// 同时具有 UsernameWidget 和 AgeWidget 的表单
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn test_same_fn_in_other_trait() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    // 如果取消注释此行，则会收到一条错误消息，提示 “multiple `get` found”（找到了多个`get`）。
    // 因为毕竟有多个名为 `get` 的方法。
    // println!("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}

macro_rules! say_hello {
    () => {
        println!("Helllo!");
    };
}
macro_rules! create_function {
    // 此宏接受一个 `ident` 指示符表示的参数，并创建一个名为 `$func_name` 的函数。
    // `ident` 指示符用于变量名或函数名
    ($func_name:ident) => {
        fn $func_name() {
            // `stringify!` 宏把 `ident` 转换成字符串。
            println!("You called {:?}()", stringify!($func_name))
        }
    };
}

macro_rules! print_result {
    // 此宏接受一个 `expr` 类型的表达式，并将它作为字符串，连同其结果一起
    // 打印出来。
    // `expr` 指示符表示表达式。
    ($expression:expr) => {
        // `stringify!` 把表达式*原样*转换成一个字符串。
        println!("{:?} = {:?}", stringify!($expression), $expression)
    };
}

fn test_macro() {
    // 这个调用将会展开成 `println("Hello");`!
    say_hello!();
    create_function!(foo);
    create_function!(bar);
    foo();
    bar();
    print_result!(1u32 + 1);

    // 回想一下，代码块也是表达式！
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}

use std::num::ParseIntError;

// 组合算子
// and_then => 正确的话继续往下执行，错误直接返回该错误了
// map => 有正确结果读入 value ，然后转为其他值，错误直接返回该错误
// map_err => 有错误才执行，映射为其他error，正确值直接返回

// 就像 `Option` 那样，我们可以使用 `map()` 之类的组合算子。
// 除去写法外，这个函数与上面那个完全一致，它的作用是：
// 如果值是合法的，计算其乘积，否则返回错误。
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let default = first_number_str.parse::<i32>().map_err(|op| 0);
    println!("{:?}", default);
    let r = first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    });

    r
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // 这种情况下仍然会给出正确的答案。
    let twenty = multiply("10", "2");
    print(twenty);

    // 这种情况下就会提供一条更有用的错误信息。
    let tt = multiply("t", "2");
    print(tt);
}
