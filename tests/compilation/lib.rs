use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2_diagnostic::prelude::*;
use quote::{format_ident, quote};

/// Return a `struct #name`, or produce:
/// - simple error if name == "fail"
/// - simple error with additional help if name == "helpme"
fn zst(name: &str) -> DiagnosticStream {
    match name {
        "fail" => error("failed")?,
        "helpme" => error("failed").add_help(Span::call_site(), "haha")?,
        "warn" => {
            let name = format_ident!("{name}");
            let zst = quote! {struct #name;};
            let zst = warn_spanned(zst, Span::call_site(), "be careful")?;
            Ok(zst)
        }
        "helpful_warning" => {
            let name = format_ident!("{name}");
            let zst = quote! {struct #name;};
            let zst = warn_spanned(zst, Span::call_site(), "be careful")
                .add_help(Span::call_site(), "this might help you understand")?;
            Ok(zst)
        }
        _ => {
            let name = format_ident!("{name}");
            Ok(quote! {struct #name;})
        }
    }
}

#[proc_macro_attribute]
pub fn error_and_help(_: TokenStream, _: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("helpme"))
}

#[proc_macro_attribute]
pub fn error_no_help(_: TokenStream, _: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("fail"))
}

#[proc_macro]
pub fn spanned_error(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let first_item: Span = input
        .into_iter()
        .find_map(|tt| match tt {
            proc_macro2::TokenTree::Ident(ident) => Some(ident.span()),
            _ => None,
        })
        .unwrap();
    error_spanned(first_item, "spanned error").into()
}

#[proc_macro]
pub fn no_error(_: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("bingo"))
}

#[proc_macro]
pub fn vec_span_warn(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let spans: Vec<Span> = input
        .into_iter()
        .filter_map(|tt| match tt {
            proc_macro2::TokenTree::Ident(ident) => Some(ident.span()),
            _ => None,
        })
        .collect();
    let ts = quote! { struct VecSpanWarn; };
    let result = warn_spanned(ts, spans, "warning with multiple spans");
    proc_macro::TokenStream::from(result)
}

#[proc_macro]
pub fn span_slice_help(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let spans: Vec<Span> = input
        .into_iter()
        .filter_map(|tt| match tt {
            proc_macro2::TokenTree::Ident(ident) => Some(ident.span()),
            _ => None,
        })
        .collect();
    let ts = quote! { struct SpanSliceHelp; };
    let result = warn_spanned(ts, Span::call_site(), "warning")
        .add_help(&spans[..], "help with multiple spans");
    proc_macro::TokenStream::from(result)
}

#[proc_macro]
pub fn warn(_: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("warn"))
}

#[proc_macro]
pub fn helpful_warning(_: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(zst("helpful_warning"))
}

#[proc_macro]
pub fn just_a_note(_: TokenStream) -> TokenStream {
    let my_struct = zst("Bob").add_note(Span::call_site(), "this is Bob");
    my_struct.into()
}
