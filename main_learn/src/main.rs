fn main() {
    let constValue = 5;
    let x = 5;
    // 使用let 表示重新定义变量
    let x = x + 2;
    let mut value = 11;

    //常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let spaces = "   ";
    let spaces = spaces.len();

    let mut spaces = "    ";
    let spaces = spaces.len();

    let y: f32 = 3.0;
    let x: u32 = 3;

    //四个字节的字符
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //复合类型 —— 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //模式匹配来解构元组值
    let (x, y, z) = tup;

    let nulltup = ();

    println!("{}", x);
    //直接索引使用
    println!("{}", tup.0);

    //复合类型 —— 数组
    let array = [1, 2, 3, 4, 5, 6];
    let a0 = array[0];
    let a1 = array[1];
    let array2: [i32; 2] = [1, 2];
    println!("Hello, world.");
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is: {x}");

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
        println!("counter:{counter}");
    };
    println!("The result is {result}");

    // 内层循环直接结束外层循环
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    for number in (1..50).rev() {
        println!("for number:{number}")
    }
    println!("LIFTOFF!!!");

    let s1 = String::from("hello");
    //移动 s1 失效
    let s2 = s1;
    println!("{},world", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello"); // s comes into scope
    let s_b = s.clone();
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
                        // s 已经被释放
    println!("s:{}", s_b);

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterwar

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    //return 可以把变量s2重新移动到s3
    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    println!("s3:{}", s3);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");

    //使用引用 不转移 控制权
    let len = calculate_length2(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 可变引用
    let mut s = String::from("hello");

    change(&mut s);
    println!("The change string:{}", s);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("the first word is: {}", word);
    s.clear(); // error!

    println!("the first word is: {}", s);

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("eaagagwe@l.com");

    let black = Color(1, 3, 4);
    let origin = Point(1, 4, 5);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i64, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    print!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
