fn main() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    let v = vec![String::from("asf")];
    for mut i in v.into_iter() {
        println!("{}", i);
        i.push_str("string")
    }
    // into_iter 的调用方是 self 不是 &self 所以ownership被转移了
    //println!("v:{:?}", v);        

    let v23 = vec![1, 2, 3];
    let v2 = vec![String::from("asf")];
    for i in v2.iter() {
        println!("&String {}", i);
        println!("String {}", *i);
        //i.push_str("string")
    }
    println!("v:{:?}", v2);

    let mut v3 = vec![String::from("asf")];
    for i in v3.iter_mut() {
        println!("{}", i);
        i.push_str(" string");
        //i.push_str("string")
    }

    //let a: Vec<&str> = iter.map(|&x| x).collect();
    println!("{:?}", v3);
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.
fn calculate_length(s: &String) -> usize {
    s.len()
}

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
