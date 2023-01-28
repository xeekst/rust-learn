use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => { file },
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hellp.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("read_username_from_file3.txt")?.read_to_string(&mut s)?;

    return Ok(s);
}

fn read_username_from_file4() -> Result<String, io::Error> {
    let s = fs::read_to_string("hello.txt");
    println!("read_username_from_file4:{:?}", s);
    return s;
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    let f = read_username_from_file();
    println!("{:?}",f);
    return;

    let mut g = Guess::new(11);
    println!("Guess:{:?}", g.value);
    g.value = 22;
    println!("Guess:{:?}", g.value);

    let opt = last_char_of_first_line("\nhi");
    println!("opt:{:?}", opt);
    //let f = File::open("hello.txt")?;
    let v = read_username_from_file2();
    println!("v:{:?}", v);
    read_username_from_file4();
    //let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    // let v = vec![1, 3, 4];

    //v[99];
    //panic!("crash on here.");

    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };

    // let p = Point { x: 5, y: 10 };
    // println!("p.x={}", p.x());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    println!("Hello, world!");

    let s1 = Some("abcde");
    let s2 = Some(5);
    let s3 = Some(5);
    let s3_fn = || Some(5);
    let s3_a_fn = |_| Some(5);
    let s4 = Some(5);
    let s4_fn = || Option::<usize>::None;
    let s4_a_fn = |_| Option::<usize>::None;

    let n1: Option<&str> = None;
    let n2: Option<usize> = None;

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<usize, &str> = Ok(5);

    let e1: Result<&str, &str> = Err("abcde");
    let e2: Result<usize, &str> = Err("abcde");

    let fn_character_count = |s: &str| s.chars().count();
    let os = s2.or(s3).or(s4);
    let ands = s2.and(s3).and(s4);
    let orelse = s2.or_else(s3_fn).or_else(s4_fn);
    // 使用上一个值的正确的值作为输入继续执行
    let andthen = s2.and_then(s3_a_fn).and_then(s4_a_fn);
    println!("{:?},", s1.map(fn_character_count)); // Some1 map = Some2
    println!("{:?},", n1.map(fn_character_count)); // None map = None

    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() }; // 该函数返回一个 isize

    //map_err 只 map Err 类型的
    println!("{:?},{:?}", o1, o1.map_err(fn_character_count)); // Ok map_err = Ok
    println!("{:?},{:?}", e1, e1.map_err(fn_character_count)); // Err1 map_err = Err2
    return;
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn double_value(&self) -> i32 {
        self.value * 2
    }
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest2<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        if *item > largest {
            largest = item.clone();
        }
    }

    largest
}

fn largest3<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest4<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if *item > *largest {
            largest = item;
        }
    }

    largest
}
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// use clap::{ArgGroup, Parser};

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// #[command(group(
//             ArgGroup::new("vers")
//                 .required(true)
//                 .args(["set_ver", "major",  "patch"]),
//         ))]
// struct Cli {
//     /// set version manually
//     #[arg(long, value_name = "VER")]
//     set_ver: Option<String>,

//     /// auto inc major
//     #[arg(long)]
//     major: bool,

//     /// auto inc minor
//     #[arg(long)]
//     minor: bool,

//     /// auto inc patch
//     #[arg(long)]
//     patch: bool,

//     /// some regular input
//     #[arg(group = "input")]
//     input_file: Option<String>,

//     /// some special input argument
//     #[arg(long, group = "input")]
//     spec_in: Option<String>,

//     #[arg(short, requires = "input")]
//     config: Option<String>,
// }

// fn main() {
//     let cli = Cli::parse();

//     // Let's assume the old version 1.2.3
//     let mut major = 1;
//     let mut minor = 1;
//     let mut patch = 1;

//     // See if --set_ver was used to set the version manually
//     let version = if let Some(ver) = cli.set_ver.as_deref() {
//         ver.to_string()
//     } else {
//         // Increment the one requested (in a real program, we'd reset the lower numbers)
//         let (maj, min, pat) = (cli.major, cli.minor, cli.patch);
//         println!("{}.{}.{}", maj, min, pat);
//         match (maj, min, pat) {
//             (true, _, _) => major += 1,
//             (_, true, _) => minor += 1,
//             (_, _, true) => patch += 1,
//             _ => unreachable!(),
//         };
//         format!("{}.{}.{}", major, minor, patch)
//     };

//     println!("Version: {}", version);

//     // Check for usage of -c
//     if let Some(config) = cli.config.as_deref() {
//         let input = cli
//             .input_file
//             .as_deref()
//             .unwrap_or_else(|| cli.spec_in.as_deref().unwrap());
//         println!("Doing work using input {} and config {}", input, config);
//     }
// }
