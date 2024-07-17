
use proc_macro::TokenStream;

extern crate proc_macro;

mod heap;


#[proc_macro]
pub fn heap(input: TokenStream) -> TokenStream {
    heap::proc_macro(input)
}