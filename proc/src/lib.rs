use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Expr, Ident, ItemFn, Result, Token, parenthesized,
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
    // Parse the input tokens into a syntax tree
    let args = parse_macro_input!(attr as Args);
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract the function name and signature
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_attrs = &input_fn.attrs;
    let fn_block = &input_fn.block;
    let fn_inputs = &input_fn.sig.inputs;
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
    let test_fns = test_cases.iter().map(|(name, case)| {
        let test_fn_name = format_ident!("{}_{}", fn_name, name);
        let args = case;
        quote! {
            #[test]
            fn #test_fn_name() {
                #fn_name(#(#args),*);
            }
        }
    });

    // Generate the original function (not marked as #[test])
    let output = quote! {
        #(#fn_attrs)*
        #fn_vis fn #fn_name(#fn_inputs) #fn_output
        #fn_block

        #(#test_fns)*
    };

    output.into()
}
