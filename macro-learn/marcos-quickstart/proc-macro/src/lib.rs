use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input};

// Funcation-like : 实际是把输入 TokenStream 映射到 另一种 TokenStream
#[proc_macro]
pub fn funcation_like_proc_macro(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn test(input: TokenStream) -> TokenStream {
    let range = 1..4;
    let q = quote!(0 #(+ #range)*);
    println!("{}", q); // 0 + 1i32 + 2i32 + 3i32
    dbg!(q).into() //
}

//attribute : input 为 #[attribute_proc_macro(input)], fn => annotated_item
#[proc_macro_attribute]
pub fn attribute_proc_macro(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    println!("attr input:{:#?}", input);
    println!("annotated_item input:{:#?}", annotated_item);
    //let p_input = parse_macro_input!(input);

    annotated_item
}

//derive : 派生 主要用于field、enum，struct或者union 这些，实现默认的 trait
#[proc_macro_derive(PrintRR)]
pub fn derive_proc_macro(input: TokenStream) -> TokenStream {
    println!("derive_proc_macro input:{:#?}", input);
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    // rust 代码 转换为 一个语法树
    let ast = syn::parse(input).unwrap(); // 必须 panic

    // Build the trait implementation
    impl_printrr_macro(&ast)
}

fn impl_printrr_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    //quote! 把语法树再次转换回 rust 代码
    let gen = quote! {
        impl PrintRR for #name{
            fn doit(){
                println!("Hello, Macro!! my name is {}!",stringify!(#name));
            }
        }
    };
    gen.into()
}
