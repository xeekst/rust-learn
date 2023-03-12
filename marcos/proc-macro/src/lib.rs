use proc_macro::TokenStream;
use quote::quote;
use syn;

// Funcation-like : 实际是把输入 TokenStream 映射到 另一种 TokenStream
#[proc_macro]
pub fn funcation_like_proc_macro(input: TokenStream) -> TokenStream {
    input
}

//attribute : input 为 #[attribute_proc_macro(input)], fn => annotated_item
#[proc_macro_attribute]
pub fn attribute_proc_macro(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    annotated_item
}

//derive : 派生 主要用于field、enum，struct或者union 这些，实现默认的 trait
#[proc_macro_derive(PrintRR)]
pub fn derive_proc_macro(input: TokenStream) -> TokenStream {
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