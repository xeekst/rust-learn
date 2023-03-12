use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, Ident, ItemFn};

#[proc_macro_attribute]
pub fn logging_proc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let vis = &input.vis;
    let ident = &input.sig.ident;
    let block = &input.block;

    let gen = quote! {
        #vis fn #ident() {
            println!("An hhhhhhhhhhhhhhhhhhhhh");
            #block
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn deco(attr: TokenStream, item: TokenStream) -> TokenStream {
    let decorator = parse_macro_input!(attr as Ident);
    let decoratee = parse_macro_input!(item as ItemFn);

    let vis = &decoratee.vis;
    let ident = &decoratee.sig.ident;
    let block = &decoratee.block;
    let inputs = &decoratee.sig.inputs;
    let output = &decoratee.sig.output;

    let arguments: Vec<_> = inputs
        .iter()
        .map(|input| match input {
            FnArg::Typed(val) => &val.pat,
            _ => unreachable!(),
        })
        .collect();

    let caller = quote! {
        #vis fn #ident(#inputs) #output {
            let func = #decorator(original_fn);
            return func(#(#arguments), *);

            fn original_fn(#inputs) #output #block
        }
    };

    caller.into()
}
