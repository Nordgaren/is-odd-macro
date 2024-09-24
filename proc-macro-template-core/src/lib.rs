#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::TokenStream;


pub fn proc_macro_impl(args: TokenStream) -> TokenStream {
    TokenStream::new()
}
