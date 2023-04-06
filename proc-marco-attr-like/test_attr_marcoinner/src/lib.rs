use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Error, Expr, Ident, Lit, Meta, MetaList, MetaNameValue, Path, Result, Token, Type,
};

struct TestPrintAttr {
    name: String,
    times: i32,
}

impl Parse for TestPrintAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut name = None;
        let mut times = None;

        // 第一种 解析为：Token![,] 分割的数组
        let name_values = input.parse_terminated(Meta::parse, Token![,])?;
        println!("name_values len:{}", name_values.len());
        for nv in name_values {
            match nv {
                Meta::NameValue(ref name_value) => match name_value.path.get_ident() {
                    Some(ident) if ident == "name" => {
                        name = match &name_value.value {
                            Expr::Lit(lit) => Some(lit.lit.clone()),
                            _ => todo!(),
                        }
                    }
                    Some(ident) if ident == "times" => {
                        times = match &name_value.value {
                            Expr::Lit(lit) => Some(lit.lit.clone()),
                            _ => todo!(),
                        }
                    }
                    Some(ident) => {
                        return Err(Error::new_spanned(
                            ident,
                            "unknown attribute by name_value.path.get_ident()",
                        ));
                    }
                    None => {
                        return Err(Error::new_spanned(
                            &name_value.path,
                            format!("unknown attribute"),
                        ));
                    }
                },
                Meta::Path(_) => todo!(),
                Meta::List(_) => todo!(),
            }
        }

        // 第二种 按固定顺序解析
        // let m1 = input.parse::<Meta>()?;
        // match m1 {
        //     Meta::NameValue(ref name_value) => match name_value.path.get_ident() {
        //         Some(ident) if ident == "name" => {
        //             name = match &name_value.value {
        //                 Expr::Lit(lit) => Some(lit.lit.clone()),
        //                 _ => todo!(),
        //             }
        //         }
        //         Some(ident) => {
        //             return Err(Error::new_spanned(
        //                 ident,
        //                 "unknown attribute by name_value.path.get_ident()",
        //             ));
        //         }
        //         None => {
        //             return Err(Error::new_spanned(&name_value.path, "unknown attribute 2"));
        //         }
        //     },
        //     Meta::Path(_) => todo!(),
        //     Meta::List(_) => todo!(),
        // }

        // let _: Token![,] = input.parse()?;

        // let m2 = input.parse::<Meta>()?;
        // match m2 {
        //     Meta::NameValue(ref name_value) => match name_value.path.get_ident() {
        //         Some(ident) if ident == "times" => {
        //             println!("times");
        //             times = match &name_value.value {
        //                 Expr::Lit(lit) => Some(lit.lit.clone()),
        //                 _ => todo!(),
        //             }
        //         }
        //         Some(ident) => {
        //             return Err(Error::new_spanned(
        //                 ident,
        //                 "unknown attribute by name_value.path.get_ident()",
        //             ));
        //         }
        //         None => {
        //             return Err(Error::new_spanned(&name_value.path, "unknown attribute 2"));
        //         }
        //     },
        //     Meta::Path(_) => todo!(),
        //     Meta::List(_) => todo!(),
        // }

        let name = name.unwrap();
        let name = match name {
            Lit::Str(s) => s.value(),
            _ => {
                return Err(Error::new_spanned(name, "name must be a string"));
            }
        };

        let times = times.unwrap();
        let times = match times {
            Lit::Int(int) => int.base10_parse::<i32>()?,
            _ => {
                return Err(Error::new_spanned(times, "times must be an integer"));
            }
        };

        Ok(TestPrintAttr { name, times })
    }
}

#[proc_macro_attribute]
pub fn testprint(attr: TokenStream, item: TokenStream) -> TokenStream {
    let testprint_attr = parse_macro_input!(attr as TestPrintAttr);
    let input_fn = parse_macro_input!(item as syn::ItemFn);

    let fn_name = &input_fn.sig.ident;
    let visibility = &input_fn.vis;
    let input_args = &input_fn.sig.inputs;
    let output = &input_fn.sig.output;
    let block = &input_fn.block;

    let name = &testprint_attr.name;
    let times = &testprint_attr.times;

    let print_code = quote_spanned! {block.span()=>
        for _ in 0..#times {
            println!("Test print {}: {}", #name, stringify!(#block));
            (||#block)()
        }
        #block
    };

    let output = quote! {
        #visibility fn #fn_name(#input_args) #output {
            #print_code
        }
    };

    TokenStream::from(output)
}
