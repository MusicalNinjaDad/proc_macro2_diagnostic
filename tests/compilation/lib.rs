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
        "warn" => {
            let name = format_ident!("{name}");
            let zst = quote! {struct #name;};
            let zst = DiagnosticResult::warn_spanned(zst, Span::call_site(), "be careful")?;
            DiagnosticResult::Ok(zst)
        }
        "helpful_warning" => {
            let name = format_ident!("{name}");
            let zst = quote! {struct #name;};
            let zst = DiagnosticResult::warn_spanned(zst, Span::call_site(), "be careful")
                .add_help(Span::call_site(), "this might help you understand")?;
            DiagnosticResult::Ok(zst)
        }
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

#[proc_macro_attribute]
pub fn error(_: TokenStream, _: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("fail"))
}

#[proc_macro]
pub fn no_error(_: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("bingo"))
}

#[proc_macro]
pub fn warn(_: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("warn"))
}

#[proc_macro]
pub fn helpful_warning(_: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("helpful_warning"))
}
#[proc_macro_derive(warn_multispan)]
pub fn warn_multispan(input: TokenStream) -> TokenStream {
    warn_multispan::impl_(input.into()).into()
}

mod warn_multispan {
    use proc_macro2::{Span, TokenStream};
    use proc_macro2_diagnostic::{DiagnosticResult, DiagnosticStream};
    use syn::{DeriveInput, spanned::Spanned};

    pub(super) fn impl_(input: TokenStream) -> DiagnosticStream {
        let ast: DeriveInput = syn::parse2(input).expect("derive macro");
        let attributes = &ast.attrs;
        let ident = &ast.ident;
        DiagnosticResult::warn_spanned(
            TokenStream::new(),
            ident.span(),
            format_args!("{} attributes counted", attributes.len()),
        )
        .extend_span(attributes.iter().next().unwrap().span())
    }
}
