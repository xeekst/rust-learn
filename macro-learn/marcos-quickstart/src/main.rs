use std::env::args;

use proc_macro::{attribute_proc_macro, funcation_like_proc_macro, test, PrintRR};

// Shorthand for initializing a `String`.
macro_rules! S {
    ($e:expr) => {
        String::from($e)
    };
}

macro_rules! one_expression {
    ($e:expr) => {
        println!("e:{}", stringify!($e))
    };
}

macro_rules! times_five {
    ($e:expr) => {
        5 * $e
    };
}

macro_rules! vec_strs {
    (
        $(
            $element: expr
        )
        ,
        *
    ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}",$element));
            )
            *

            v
        }
    };
}

macro_rules! repeat_two {
    ($($i:ident)*,$($i2:ident)*) => {
        $(let $i:(); let $i2:();)*
    };
}

#[macro_export]
macro_rules! my_vec {
    // 匹配空输入，创建一个新的 vector
    () => {
        std::vec::Vec::new()
    };

    // 匹配类似于 vec![0; 10] 的输入
    ( $elem: expr ; $n: expr ) => {
        std::vec::from_elem($elem, $n)
    };

    // 匹配类似于 vec![1, 2, 3] 的输入
    ( $( $elem: expr ),* ) => {
        // 由于我们将生成多条语句，因此必须再用 {} 包起来
        {
            let mut v = std::vec::Vec::new();
            $( v.push($elem); )*
            v
        }
    };

    // 匹配类似于 vec![1, 2, 3, ] 的输入
    ( $( $elem: expr, )* ) => {
        // 递归调用
        my_vec![ $( $elem ),* ]
    };
}

// fn main() {
//     let world = S!("World");
//     one_expression!(1 + 1);
//     let l = args().len();
//     for i in 0..l {
//         let s = vec_strs![i];
//         println!("{:?}", s);
//     }
//     repeat_two!( a b c d e f, u v w x y z );
//     println!("Hello, {}!, times_five:{}", world, times_five!(55));
// }

macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
//  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ changed
}

macro_rules! recurrence {
    ( $seq:ident [ $ind:ident ]: $sty:ty = $($inits:expr),+ ; ... ; $recur:expr ) => {
        {
            /*
                What follows here is *literally* the code from before,
                cut and pasted into a new position. No other changes
                have been made.
            */

            use std::ops::Index;
            const MEM_SIZE: usize = count_exprs!($($inits),+);

            struct Recurrence {
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }

            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
                offset: usize,
            }

            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;

                fn index<'b>(&'b self, index: usize) -> &'b $sty {
                    use std::num::Wrapping;

                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);

                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }

            impl Iterator for Recurrence {
                type Item = $sty;

                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $ind = self.pos;
                            let $seq = IndexOffset { slice: &self.mem, offset: $ind };
                            $recur
                        };

                        {
                            use std::mem::swap;

                            let mut swap_tmp = next_val;
                            for i in (0..MEM_SIZE).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }

                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }

            Recurrence { mem: [$($inits),+], pos: 0 }
        }
    };
}

// let fib = {
//         use std::ops::Index;

//         struct Recurrence {
//             mem: [u64; 2],
//             pos: usize,
//         }

//         struct IndexOffset<'a> {
//             slice: &'a [u64; 2],
//             offset: usize,
//         }

//         impl<'a> Index<usize> for IndexOffset<'a> {
//             type Output = u64;

//             #[inline(always)]
//             fn index<'b>(&'b self, index: usize) -> &'b u64 {
//                 use std::num::Wrapping;

//                 let index = Wrapping(index);
//                 let offset = Wrapping(self.offset);
//                 let window = Wrapping(2);

//                 let real_index = index - offset + window;
//                 println!("self.slice:{:?}:", self.slice);
//                 &self.slice[real_index.0]
//             }
//         }

//         impl Iterator for Recurrence {
//             type Item = u64;

//             #[inline]
//             fn next(&mut self) -> Option<u64> {
//                 if self.pos < 2 {
//                     let next_val = self.mem[self.pos];
//                     self.pos += 1;
//                     Some(next_val)
//                 } else {
//                     let next_val = {
//                         let n = self.pos;
//                         let a = IndexOffset {
//                             slice: &self.mem,
//                             offset: n,
//                         };
//                         println!("a[n-2]:{} + a[n-1]:{}", a[n - 2], a[n - 1]);
//                         a[n - 2] + a[n - 1]
//                     };

//                     {
//                         use std::mem::swap;

//                         let mut swap_tmp = next_val;
//                         for i in [1, 0] {
//                             println!("swap_tmp:{} + self.mem[{}]:{}", swap_tmp, i, self.mem[i]);
//                             swap(&mut swap_tmp, &mut self.mem[i]);
//                             println!(
//                                 "swaped: swap_tmp:{} + self.mem[{}]:{}",
//                                 swap_tmp, i, self.mem[i]
//                             );
//                         }
//                     }

//                     self.pos += 1;
//                     Some(next_val)
//                 }
//             }
//         }

//         Recurrence {
//             mem: [0, 1],
//             pos: 0,
//         }
//     };

//     for e in fib.take(10) {
//         println!("{}", e)
//     }

macro_rules! expressions {
    ($($expr:expr)*) => {
        $(
            println!("expr:{}",stringify!($expr));
        )
        *
    }

}

macro_rules! idents {
    ($($ident:ident)*) => {
        $(
            println!("ident:{}",stringify!($ident));
        )
        *
    };
}

macro_rules! items {
    ($($item:item)*) => {
        $(
            println!("items:{}",stringify!($item));
        )
        *
    };
}

macro_rules! lifetimes {
    ($($lifetime:lifetime)*) => {
        $(
            println!("lifetime:{}",stringify!($lifetime));
        )
        *
    };
}

macro_rules! literals {
    ($($literal:literal)*) =>{
        $(
            println!("literals:{}",stringify!($literal));
        )
        *
    };
}

macro_rules! metas {
    ($($meta:meta)*) => {
        $(
            println!("meta:{}",stringify!($meta));
        )
        *
    };
}

macro_rules! patterns {
    ($($pat:pat)*) => {
        $(
            println!("pat:{}",stringify!($pat));
        )
        *
    };
}

macro_rules! pattern_params {
    ($( $( $pat:pat_param )|+ )*) => {
        $(
            $(
                println!("pattern_params pat:{}",stringify!($pat));
            )
            +
        )
        *
    };
}

macro_rules! types {
    ($($type:ty)*) => {
        $(
            println!("type type:{}",stringify!($type));
        )
        *
    };
}

macro_rules! it_is_opaque {
    (()) => {
        "()"
    };
    (($tt:tt)) => {
        concat!("$tt is ", stringify!($tt))
    };
    ($vis:vis ,) => {
        it_is_opaque!(($vis));
    };
}

macro_rules! ambiguty {
    ($($i:ident)* $j:ident) => {};
}

fn test_attribute_macro() {}
#[attribute_proc_macro]
fn foo() {}

#[attribute_proc_macro(attributes are pretty handsome)]
fn bar() {}

pub trait PrintRR {
    fn doit() {}
}

#[derive(proc_macro::PrintRR)]
struct FooRR;


fn main() {
    let fib = recurrence![a[n]: u64 = 0, 1; ...; a[n-2] + a[n-1]];

    for e in fib.take(10) {
        println!("{}", e)
    }
    println!("recurrence!(f[i]: f64 = 1.0; ...; f[i-1] * i as f64).take(10)");
    for e in recurrence!(f[i]: f64 = 1.0; ...; f[i-1] * i as f64).take(10) {
        println!("{}", e)
    }

    expressions! {
        "literal"
        funcall()
        future.await
        break 'foo bar
    }

    println!("======== idents:()");
    idents! (
            // _ <- This is not an ident, it is a pattern
            foo
            async
            O_________O
            _____O_____
            literal
    );

    println!("======== idents:{{}}");
    idents! {
            // _ <- This is not an ident, it is a pattern
            foo
            async
            O_________O
            _____O_____
            literal
    }

    items! {
        struct Foo;
        enum Bar {
            Baz
        }
        impl Foo {}
        pub use crate::foo;
        trait TraitGOO{
            fn doit(i:i32);
        }
    }

    lifetimes! {
        'static
        'shiv
        '_
    }

    literals! {
        -1
        "hello world"
        2.3
        b'b'
        true
    }

    metas! {
        ASimplePath
        super::man
        path = "home"
        foo(bar)
        Foo {}
    }

    patterns! {
        "literal"
        _
        0..5
        ref mut PatternsAreNice
        0 | 1 | 2 | 3
    }

    pattern_params! {
        "literal"
        _
        0..5
        ref mut PatternsAreNice
        0 | 1 | 2 | 3
    }

    types! {
        foo::bar
        bool
        [u8]
        impl IntoIterator<Item = u32>
    }

    println!("{}", it_is_opaque!(,));

    test_funcation_like();
    println!("proc_attr:");
    dbg!(bar());
    foo();
    //let f = FooRR{};
    FooRR::doit();

    let q = test!();
    dbg!(q); // q = 6

}

//######################### 过程宏 ########################
fn test_funcation_like() {
    funcation_like_proc_macro!({
        let a = 1 + 1;

        a
    });
}

