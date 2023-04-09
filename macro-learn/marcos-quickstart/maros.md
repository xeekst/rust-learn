```rust
macro_rules! $name{
    $rule0;
    $rule1;
    //....
    $rulen;
}

$rule0 => ( $matcher ) => { $expansion };
($( $elem:expr ),*) => { ... }

其实，重复捕获的一般形式为 $ ( ... ) sep rep，这里：

(...) 就是反复匹配的模式；
sep 是可选的分隔标记，常见的有 , 和 ;；
rep 是必须的重复操作符，可以为：
    ?：最多一次重复；
    *：0 次或多次重复；
    +：1 次或多次重复；

$( $elem:expr ),* 中间以,分割的表达式 e1,e2,e3
( $( $elem: expr, )* ) 匹配末尾,的表达式 e1,e2,e3
```

1. 
```
Input:TokenStream => syn::parse_macro_input! => input:syn::parse::Parse 的
```