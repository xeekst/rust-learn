use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

//derive 是一个过程宏
//derive 实际是在 读取了输入TokenStream =》 添加代码 =》 输出TokenStream
//获取默认实现
#[derive(HelloMacro)]
struct Pancakes2NewName;

fn main() {
    Pancakes2NewName::hello_macro();
}