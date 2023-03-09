extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    // rust 代码 转换为 一个语法树
    let ast = syn::parse(input).unwrap(); // 必须 panic

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    //quote! 把语法树再次转换回 rust 代码
    let gen = quote! {
        impl HelloMacro for #name{
            fn hello_macro(){
                println!("Hello, Macro!! my name is {}!",stringify!(#name));
            }
        }
    };
    gen.into()
}
