use std::{thread, time::Duration};

fn main() {
    let tree = sled::open("welcome-to-sled2.sled").expect("open");

    // insert and get, similar to std's BTreeMap
    tree.insert("KEY1", "VAL1").unwrap();
    assert_eq!(tree.get(&"KEY1"), Ok(Some(sled::IVec::from("VAL1"))));

    thread::spawn(move||{
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
        .compare_and_swap("KEY1", Some("VAL1"), Some("VAL2"))
        .unwrap();

    println!("compare_and_swap:{:?}", r);

    let iv = tree.get("KEY1").unwrap().unwrap();
    let val = std::str::from_utf8(&iv).unwrap();
    println!("after compare_and_swap, get KEY1:{val}");

    // deletion
    //tree.remove(&"KEY1").unwrap();

    // block until all operations are stable on disk
    // (flush_async also available to get a Future)
    tree.flush().unwrap();
    println!("Hello, world!");
}
