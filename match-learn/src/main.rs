// // fn main() {
// //     let x = Some(5);
// //     let y = 10;

// //     match x {
// //         Some(50) => println!("Got 50"),
// //         // 匹配任何 Some 类型 值为dynamic
// //         Some(dynamic) => println!("Matched, dynamic = {:?}", dynamic),
// //         _ => println!("Default case, x = {:?}", x),
// //     }

// //     println!("at the end: x = {:?}, y = {:?}", x, y);
// // }
// fn main() {
//     let x = 4;

//     match x {
//         1 | 2 => println!("one or two"),
//         3..=4 => println!("3 or 4"),
//         5 => println!("three"),
//         _ => println!("anything"),
//     }

//     let p = Point { x: 1, y: 2 };
//     let Point { x: a, y: b } = p;
//     print!("a: {a},b: {b}");
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}
