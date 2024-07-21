//! This crate provides the `task!` macro, which is used to define the executor for the RTOS's task.
extern crate proc_macro;
use darling::ast::NestedMeta;
use proc_macro::TokenStream;

mod macros;
mod util;
use macros::*;
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::Token;



struct Args {
    meta: Vec<NestedMeta>,
}

impl Parse for Args {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        let meta = Punctuated::<NestedMeta, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            meta: meta.into_iter().collect(),
        })
    }
}
/// The `#[task]` attribute
#[proc_macro_attribute]
pub fn task(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as Args);
    let f = syn::parse_macro_input!(item as syn::ItemFn);

    task::run(&args.meta, f).unwrap_or_else(|x| x).into()
}