#![cfg(test)]

use crate::proc_macro_impl;
use quote::quote;

#[test]
fn test() {
    let after = proc_macro_impl(quote!());
    assert_ne!(
        after.to_string(),
        ""
    );
}
