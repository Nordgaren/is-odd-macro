#![doc = include_str!("../README.md")]

use is_odd_macro_core::is_odd_impl;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro]
/// Preprocess for all eternity...
///
/// Perfect example of how to use this crate...
/// # Example
/// ```rust
/// use crate::is_odd_macro::is_odd;
/// fn test_number() -> bool {
///     if is_odd!(10) {
///         println!("Odd!");
///         true
///     } else {
///         panic!("Not odd!")
///     }
/// }
///
/// fn test_ident(n: i32) -> bool {
///     if is_odd!(n) {
///         println!("Odd!");
///         true
///     } else {
///         panic!("Not odd!")
///     }
/// }
pub fn is_odd(args: TokenStream) -> TokenStream {
    is_odd_impl(args.into()).into()
}