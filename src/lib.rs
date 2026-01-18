extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn triple_verify(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let vis = &input.vis;
    let sig = &input.sig;
    let body = &input.block;
    let fn_name = &sig.ident;
    let generics = &sig.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let inputs = &sig.inputs;
    let output = &sig.output;

    let args = inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => {
                let pat = &pat_type.pat;
                quote! { #pat }
            }
            _ => quote! {},
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #vis #sig {
            fn inner #impl_generics (#inputs) #output #where_clause #body

            let result1 = inner #ty_generics (#(#args),*);
            let result2 = inner #ty_generics (#(#args),*);
            let result3 = inner #ty_generics (#(#args),*);

            if result1 == result2 && result2 == result3 {
                result1
            } else if result1 == result2 {
                eprintln!("WARNING: Function {}: Two results match, one differs", stringify!(#fn_name));
                result1
            } else if result1 == result3 {
                eprintln!("WARNING: Function {}: Two results match, one differs", stringify!(#fn_name));
                result1
            } else if result2 == result3 {
                eprintln!("WARNING: Function {}: Two results match, one differs", stringify!(#fn_name));
                result2
            } else {
                panic!("Function {}: All three results differ: {:?}, {:?}, {:?}", stringify!(#fn_name), result1, result2, result3);
            }
        }
    };

    TokenStream::from(expanded)
}
