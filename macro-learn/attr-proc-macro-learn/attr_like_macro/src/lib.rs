use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Error, Ident, Lit, Meta, MetaNameValue, NestedMeta, Path, Result, Token,
};

struct TestPrintAttr {
    name: String,
    times: i32,
}

impl Parse for TestPrintAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut name = None;
        let mut times = None;
        let meta = input.parse::<Meta>()?;

        match meta {
            Meta::List(ref list) => {
                for nested in &list.nested {
                    match nested {
                        NestedMeta::Meta(meta) => match meta {
                            Meta::NameValue(name_value) => match name_value.path.get_ident() {
                                Some(ident) if ident == "name" => {
                                    name = Some(&name_value.lit);
                                }
                                Some(ident) if ident == "times" => {
                                    times = Some(&name_value.lit);
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
                                        "unknown attribute 2",
                                    ));
                                }
                            },
                            _ => {
                                return Err(Error::new_spanned(meta, "unknown attribute 3"));
                            }
                        },
                        _ => {
                            return Err(Error::new_spanned(nested, "unknown attribute 4"));
                        }
                    }
                }
            }
            _ => {
                return Err(Error::new_spanned(meta, "unknown attribute Meta::List"));
            }
        }

        let name = name.ok_or_else(|| Error::new_spanned(&meta, "missing name attribute"))?;

        let name = match name {
            Lit::Str(s) => s.value(),
            _ => {
                return Err(Error::new_spanned(name, "name must be a string"));
            }
        };

        let times = times.ok_or_else(|| Error::new_spanned(&meta, "missing times attribute"))?;

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
    let name_values =
        syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated
            .parse(attr)
            .unwrap();

    let mut name = None;
    let mut times = None;

    for name_value in name_values {
        match name_value.path.get_ident() {
            Some(ident) if ident == "name" => {
                name = Some(name_value.lit.clone());
            }
            Some(ident) if ident == "times" => {
                times = Some(name_value.lit.clone());
            }
            Some(ident) => {
                panic!("unknown attribute by name_value.path.get_ident()");
            }
            None => {
                panic!("unknown attribute 2");
            }
        }
    }

    let name = match name.unwrap() {
        Lit::Str(s) => s.value(),
        _ => {
            panic!("name must be a string");
        }
    };

    let times = match times.unwrap() {
        Lit::Int(int) => int.base10_parse::<i32>().unwrap(),
        _ => {
            panic!("times must be an integer");
        }
    };

    let testprint_attr = TestPrintAttr { name, times };

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
