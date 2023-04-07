use proc_macro::TokenStream;
use quote::quote;
use syn;


//attribute : input 为 #[attribute_proc_macro(input)], fn => annotated_item
#[proc_macro_attribute]
pub fn attribute_proc_macro(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    println!("attr input:{:#?}", input);
    println!("annotated_item input:{:#?}", annotated_item);

    let gen = quote! {
        impl PrintRR for #name{
            fn doit(){
                println!("Hello, Macro!! my name is {}!",stringify!(#name));
            }
        }
    };
    gen.into()
    
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
