use std::collections::HashMap;

fn main() {
    // let v: Vec&<i32> = Vec::new();
    // let mut& v = vec![1, 2, 3];

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // {
    //     let v2 = vec![1, 3, 5, 6];
    // }

    let v = vec![1, 2, 3, 4, 5];

    let mut third: &i32 = &v[2];
    third = &10;
    let sec: &i32 = &v[2];
    println!("The 1 third element is {}", third);
    println!("The v[2] element is {}", &v[2]);
    match v.get(2) {
        Some(value) => println!("The 2 third element is {}", value),
        None => println!("There is no third element."),
    }
    println!("The 3 third element is {}", third);
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // push的时候。需要把 vec copy到新内存中，此时却存在一个不可变借用，导致会出异常
    //v.push(6);

    println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 412, 512];
    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let data = "initial contents";

    let s = data.to_string();

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s2:{s2},s3:{s3}");

    let hello = String::from("Hola");
    let ch = &hello[1..3];
    println!("char:{ch}");

    println!("chars");
    for c in hello.chars() {
        println!("{c}");
    }

    println!("bytes");
    for b in hello.bytes() {
        println!("{b}");
    }

    //hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 不可用 field_name 被map借用了
    //println!("field_name:{field_name}");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(v) => println!("score:{v}"),
        None => print!("None"),
    };

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    println!("Hello, world!");
}
