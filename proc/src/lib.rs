use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Expr, Ident, ItemFn, Result, ReturnType, Token, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token,
};

struct Arg {
    name: Ident,
    _equal: Token![=],
    _brace_token: token::Paren,
    fields: Punctuated<Expr, Token![,]>,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;

        Ok(Arg {
            name: input.parse()?,
            _equal: input.parse()?,
            _brace_token: parenthesized!(content in input),
            fields: content.parse_terminated(Expr::parse, Token![,])?,
        })
    }
}

struct Args {
    args: Punctuated<Arg, Token![,]>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Args {
            args: input.parse_terminated(Arg::parse, Token![,])?,
        })
    }
}

#[proc_macro_attribute]
pub fn paramtest(attr: TokenStream, item: TokenStream) -> TokenStream {
    build_macro(
        attr,
        item,
        |test_fn_name, fn_name, _, args| {
            quote! {
                #[test]
                fn #test_fn_name() {
                    #fn_name(#(#args),*);
                }
            }
        },
        |input_fn, test_fns| {
            let fn_name = &input_fn.sig.ident;
            let fn_vis = &input_fn.vis;
            let fn_attrs = &input_fn.attrs;
            let fn_block = &input_fn.block;
            let fn_inputs = &input_fn.sig.inputs;
            let fn_output = &input_fn.sig.output;

            quote! {
                #(#fn_attrs)*
                #fn_vis fn #fn_name(#fn_inputs) #fn_output
                #fn_block

                #(#test_fns)*
            }
        },
    )
}

#[cfg(feature = "tokio")]
#[proc_macro_attribute]
pub fn tokio_paramtest(attr: TokenStream, item: TokenStream) -> TokenStream {
    build_macro(
        attr,
        item,
        |test_fn_name, fn_name, output, args| {
            let is_result = match output {
                ReturnType::Type(_, ty) => {
                    if let syn::Type::Path(type_path) = &**ty {
                        if let Some(seg) = type_path.path.segments.last() {
                            seg.ident == "Result"
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            };

            if is_result {
                quote! {
                    #[tokio::test]
                    async fn #test_fn_name() -> Result<(), Box<dyn std::error::Error>> {
                        #fn_name(#(#args),*).await?;

                        Ok(())
                    }
                }
            } else {
                quote! {
                    #[tokio::test]
                    async fn #test_fn_name() {
                        #fn_name(#(#args),*).await;
                    }
                }
            }
        },
        |input_fn, test_fns| {
            let fn_name = &input_fn.sig.ident;
            let fn_vis = &input_fn.vis;
            let fn_attrs = &input_fn.attrs;
            let fn_block = &input_fn.block;
            let fn_inputs = &input_fn.sig.inputs;
            let fn_output = &input_fn.sig.output;

            quote! {
                #(#fn_attrs)*
                #fn_vis async fn #fn_name(#fn_inputs) #fn_output
                #fn_block

                #(#test_fns)*
            }
        },
    )
}

fn build_macro<TestFunc, OriginalFunc>(
    attr: TokenStream,
    item: TokenStream,
    test_func: TestFunc,
    original_func: OriginalFunc,
) -> TokenStream
where
    TestFunc: Fn(&Ident, &Ident, &ReturnType, &Vec<Expr>) -> proc_macro2::TokenStream,
    OriginalFunc: Fn(ItemFn, Vec<proc_macro2::TokenStream>) -> proc_macro2::TokenStream,
{
    // Parse the input tokens into a syntax tree
    let args = parse_macro_input!(attr as Args);
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract the function name and signature
    let fn_name = &input_fn.sig.ident;
    let fn_output = &input_fn.sig.output;

    // Collect test cases from the attribute arguments
    let mut test_cases = Vec::new();
    for arg in args.args {
        let mut test_case = Vec::new();

        for field in arg.fields {
            test_case.push(field);
        }

        test_cases.push((arg.name, test_case));
    }

    // Generate a test function for each test case
    let test_fns = test_cases
        .iter()
        .map(|(name, case)| {
            let test_fn_name = format_ident!("{}_{}", fn_name, name);
            let args = case;

            test_func(&test_fn_name, fn_name, fn_output, args)
        })
        .collect::<Vec<_>>();

    // Generate the original function (not marked as #[test])
    let output = original_func(input_fn, test_fns);

    output.into()
}
