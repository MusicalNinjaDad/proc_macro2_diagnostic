#![feature(never_type)]
#![feature(proc_macro_diagnostic)]
#![feature(try_trait_v2)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2_diagnostic::{DiagnosticResult, DiagnosticStream};
use quote::{format_ident, quote};

/// Return a `struct #name`, or produce:
/// - simple error if name == "fail"
/// - simple error with additional help if name == "helpme"
fn zst(name: &str) -> DiagnosticStream {
    match name {
        "fail" => DiagnosticResult::error("failed")?,
        "helpme" => DiagnosticResult::error("failed").add_help(Span::call_site(), "haha")?,
        _ => {
            let name = format_ident!("{name}");
            DiagnosticResult::Ok(quote! {struct #name;})
        }
    }
}

#[proc_macro_attribute]
pub fn error_and_help(_: TokenStream, _: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("helpme"))
}

#[proc_macro]
pub fn no_error(_: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("bingo"))
}
