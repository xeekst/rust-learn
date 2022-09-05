// use rand::*;
// use std::collections::*;
// // use std::cmp::Ordering;
// // use std::io;
// use std::{cmp::Ordering, io};

// // fn main() {
// //     let string1 = String::from("abcd");
// //     let string2 = "xyz";

// //     let result = longest(string1.as_str(), string2);
// //     println!("The longest string is {}", result);


// //     let secret_number = rand::thread_rng().gen_range(1..=100);
// //     println!("Hello, world!");
// // }

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}