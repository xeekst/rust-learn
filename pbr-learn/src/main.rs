extern crate pbr;

use pbr::ProgressBar;
use std::thread;

fn main() {
	let chs = "aa".as_bytes();
	//.chars();
	println!(
        "bytes size: {}, bytes:{:?}, len:{},item0 size:{}",
        std::mem::size_of_val(&chs),
        chs,
        chs.len(),
		std::mem::size_of_val(&chs[0]),
    );
	let chs = "aa".chars();
    println!("chs is {:?} ({})", chs, chs.clone().into_iter().collect::<Vec<char>>().len());
    let c= chs.clone().next().unwrap();
    println!("💩 is {} ({}) size:{}", c, c as u32,std::mem::size_of_val(&c));

    println!("char : {}", std::mem::size_of::<char>());
    println!("u8 : {}", std::mem::size_of::<u8>());
    let s = "aaaaaasssssssssssssssssssssssssssss";
    println!("s : {}", std::mem::size_of_val(&s));
    let bytes = s.as_bytes();
    println!(
        "bytes size: {}, bytes:{:?}, len:{},item0 size:{}",
        std::mem::size_of_val(&bytes),
        bytes,
        bytes.len(),
		std::mem::size_of_val(&bytes[0]),
    );
    let chs = s.chars().next().unwrap();
    println!("chs : {}", std::mem::size_of_val(&chs));
    return;

    let count = 0;
    let mut pb = ProgressBar::new(count);
    pb.format("╢▌▌░╟");
    //pb.show_percent = true;
    for i in 0..1000 {
        //pb.add(1);
        //let last_count = pb.inc();
        let last_count = pb.inc();

        pb = ProgressBar::new(pb.total + 2);
        pb.set(i + 2);
        //break;

        //println!("last:{}",last_count);
        //pb.set_max_refresh_rate(Some(Du));

        //
        thread::sleep_ms(1);
    }
    pb.finish_print("done");
}
