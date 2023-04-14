use std::{thread, time::Duration};

use sled::Db;

struct Data {
    pub name: String,
    pub value: i32,
}

fn open_db() -> Db {
    sled::open("welcome-to-sled2.sled").expect("open")
}

fn insert_or_override() {
    let db = open_db();
    db.insert("ISKEY", "vvvvv").unwrap();
}

fn compare_and_swap() {
    let db = open_db();
    // 当ISKEY 下没值的时候才插入值
    db.compare_and_swap("ISKEY", Option::None as Option<&str>, Some("NEWV"))
        .unwrap();
    // 当ISKEY 下值等于输入的值时，将替换为新值
    db.compare_and_swap("ISKEY", Some("NEWV2"), Some("NEWV"))
        .unwrap();
    // 当ISKEY 下值等于输入值，新值为 None 将删除 旧值
    db.compare_and_swap("ISKEY", Some("NEWV2"), Option::None as Option<&str>)
        .unwrap();
}

fn scan_prefix() {
    let db = open_db();
    for item in db.scan_prefix("ISK") {
        println!("item:{:?}", item);
    }
}

fn get_gt_next() {
    let db = open_db();
    let kv = db.get_gt("KEY").unwrap();
    println!("kv:{:?}", kv);
}

fn get_lt_next() {
    let db = open_db();
    let kv = db.get_lt("KEY").unwrap();
    println!("kv:{:?}", kv);
}

fn get_range(){
    let db = open_db();
    for item in db.range("K1".."K2") {
        println!("item:{:?}", item);
    }
}

fn watch_event() {
    let db = open_db();
    let mut subscriber = db.watch_prefix(vec![]);
    for event in subscriber.take(0) {
        match event {
            sled::Event::Insert{ key, value } => assert_eq!(key.as_ref(), &[0]),
            sled::Event::Remove {key } => {}
        }
    }
}

fn main() {
    let tree = sled::open("welcome-to-sled2.sled").expect("open");
    let t1 = tree.open_tree("tree1").unwrap();
    let names = tree.tree_names();
    for n in names {
        println!("name:{}", std::str::from_utf8(&n).unwrap());
    }
    let size = tree.size_on_disk().unwrap();
    println!("size:{}", size);

    let iv = tree.get("KEY1").unwrap().unwrap();
    let val = std::str::from_utf8(&iv).unwrap();
    println!("get KEY1:{val}");

    // insert and get, similar to std's BTreeMap
    tree.insert("KEY1", "VAL1").unwrap();
    tree.insert("KEY3", "VAL421").unwrap();
    assert_eq!(tree.get(&"KEY1"), Ok(Some(sled::IVec::from("VAL1"))));

    thread::spawn(move || {
        let tree2 = sled::open("welcome-to-sled.sled").expect("open");
        tree2.insert("KEY3", "VAL1").unwrap();
        assert_eq!(tree2.get(&"KEY1"), Ok(Some(sled::IVec::from("VAL1"))));
    });

    thread::sleep(Duration::from_secs(3));
    // range queries
    for kv in tree.range("KEY1".."KEY9") {
        println!("kv:{:?}", kv);
    }
    let iv = tree.get("KEY1").unwrap().unwrap();
    let val = std::str::from_utf8(&iv).unwrap();
    println!("get KEY1:{val}");

    // atomic compare and swap
    let r = tree
        .compare_and_swap("KEY1", Some("VAL1"), Some("VAL23"))
        .unwrap();

    println!("compare_and_swap:{:?}", r);

    let iv = tree.get("KEY1").unwrap().unwrap();
    let val = std::str::from_utf8(&iv).unwrap();
    println!("after compare_and_swap, get KEY1:{val}");

    // deletion
    //tree.remove(&"KEY1").unwrap();

    // block until all operations are stable on disk
    // (flush_async also available to get a Future)

    for i in 0..10 {
        tree.insert(&[i], vec![i])
            .expect("should write successfully");
    }

    let lt = tree.get_lt(&[5]).unwrap();
    println!("lt:{:?}", lt);

    let gt = tree.get_gt(&[5]).unwrap();
    println!("gt:{:?}", gt);

    for item in tree.iter() {
        println!("iter item:{:?}", item);
    }

    for item in tree.scan_prefix("KEY") {
        println!("scan_prefix item:{:?}", item);
    }

    tree.flush().unwrap();
    println!("Hello, world!");
}
