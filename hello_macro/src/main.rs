use hello_macro::HelloMacro;

#[derive(hello_macro_derive::HelloMacro)]
struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }

fn main() {
    Pancakes::hello_macro();
    println!("1+1:{}", stringify!(1 < 4 > 5 & 6 + 1));
}
