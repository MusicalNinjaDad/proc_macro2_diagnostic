#![feature(never_type)]
#![feature(proc_macro_diagnostic)]
#![feature(try_trait_v2)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2_diagnostic::{DiagnosticResult, DiagnosticStream};
use quote::{format_ident, quote};

fn zst(name: &str) -> DiagnosticStream {
    match name {
        "fail" => DiagnosticResult::error("failed").add_help(Span::call_site(), "haha")?,
        _ => {
            let name = format_ident!("{name}");
            DiagnosticResult::Ok(quote! {struct #name;})
        }
    }
}

#[proc_macro_attribute]
pub fn oops(_: TokenStream, _: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("fail"))
}

#[proc_macro]
pub fn bingo(_: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("bingo"))
}
