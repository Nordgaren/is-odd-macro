#![doc = include_str!("../README.md")]

use std::str::FromStr;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse2, Expr};

/// Should be isize::MIN. -40 for testing.
const ISIZE_RANGE_START: isize = isize::MIN;
/// Set to 0 to be efficient, so you don't check against any non-negative integers. :-)
const ISIZE_RANGE_END: isize = 0;
/// Should be usize::MIN.
const USIZE_RANGE_START: usize = usize::MIN;
/// Set to 40 for testing.
const USIZE_RANGE_END: usize = usize::MAX;

/// Pass in an expression or an identifier, and generate the appropriate code for each.
pub fn is_odd_impl(args: TokenStream) -> TokenStream {
    // proc_marco2 version of "parse_macro_input!(input as LitStr)"
    match parse2::<Ident>(args.clone()) {
        Ok(ident) => is_odd_macro_ident(ident),
        Err(_) => match parse2::<Expr>(args) {
            Ok(expr) => is_odd_macro_expr(expr),
            Err(error) => error.to_compile_error(),
        },
    }
}

/// Pass any expressions starting with a negative sign to isize, and the rest to usize. User needs
/// to pass in a correct expression.
fn is_odd_macro_expr(expr: Expr) -> TokenStream {
    let string = expr.to_token_stream().to_string();
    match string.starts_with('-') {
        true => is_odd_macro_expr_isize(expr),
        false => is_odd_macro_expr_usize(expr),
    }
}

/// Generates code for any negative integer passed into the macro.
fn is_odd_macro_expr_isize(expr: Expr) -> TokenStream {
    let min = ISIZE_RANGE_START;
    let mut out = quote! {
        if #expr == #min {
            false
        }
    };

    let mut rest = vec![];

    let t = TokenStream::from_str("true").unwrap();
    let f = TokenStream::from_str("false").unwrap();

    let start = min + 1;
    for i in start..ISIZE_RANGE_END {
        let is_odd = if i % 2 != 0 {
            &t
        } else {
            &f
        };
        let next = quote! {
            else if #expr == #i {
                #is_odd
            }
        };

        rest.push(next);
    }

    let is_odd = if ISIZE_RANGE_END % 2 != 0 {
        &t
    } else {
        &f
    };

    rest.push(quote! {
        else {
            #is_odd
        }
    });

    out.append_all(rest);

    out

}

/// Generates code for non-negative integer.
fn is_odd_macro_expr_usize(ident: Expr) -> TokenStream {
    let min = USIZE_RANGE_START;
    let mut out = quote! {
        if #ident == #min {
            false
        }
    };

    let mut rest = vec![];

    let t = TokenStream::from_str("true").unwrap();
    let f = TokenStream::from_str("false").unwrap();

    let start = min + 1;
    for i in start..USIZE_RANGE_END {
        let is_odd = if i % 2 != 0 {
            &t
        } else {
            &f
        };
        let next = quote! {
            else if #ident == #i {
                #is_odd
            }
        };

        rest.push(next);
    }

    let is_odd = if USIZE_RANGE_END % 2 != 0 {
        &t
    } else {
        &f
    };

    rest.push(quote! {
        else {
            #is_odd
        }
    });

    out.append_all(rest);

    out
}

/// For identifiers, like local variables.
fn is_odd_macro_ident(ident: Ident) -> TokenStream {
    let min = USIZE_RANGE_START;
    let mut out = quote! {
        if #ident as usize == 0usize {
            false
        }
    };

    let mut rest = vec![];

    let t = TokenStream::from_str("true").unwrap();
    let f = TokenStream::from_str("false").unwrap();

    let start = min + 1;
    for i in start..USIZE_RANGE_END {
        let is_odd = if i % 2 != 0 {
            &t
        } else {
            &f
        };
        let next = quote! {
            else if #ident as usize == #i {
                #is_odd
            }
        };

        rest.push(next);
    }

    let is_odd = if USIZE_RANGE_END % 2 != 0 {
        &t
    } else {
        &f
    };

    rest.push(quote! {
        else {
            #is_odd
        }
    });

    out.append_all(rest);

    out
}
