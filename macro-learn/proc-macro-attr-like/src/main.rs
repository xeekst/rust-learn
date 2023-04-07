use test_attr_macroinner::testprint;

#[testprint(name = "wow", times = 3)]
fn fp() {
    println!("just stringfy");
}

fn main() {
    fp();
    println!("Hello, world!");
}
