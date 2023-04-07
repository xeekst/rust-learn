use attr_like_macro::testprint;

#[testprint(name = "n1", times = 3)]
fn tp() {}

fn main() {
    tp();
    println!("Hello, world!");
}
