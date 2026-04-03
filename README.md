# Diagnostic-based compiler errors for proc_macro2

An implementation of `Diagnostic` for proc_macro2 which leverages `Try`

Provides a DiagnosticResult which stores a Diagnostic with the same (target) API as
[proc_macro::Diagnostic] and allows `?` usage to return early from proc_macro2 code.

```rust
#![feature(never_type)]
#![feature(try_trait_v2)]

# extern crate proc_macro;

use proc_macro2_diagnostic::{DiagnosticResult,DiagnosticStream};
use quote::quote;

fn zst(name: &str) -> DiagnosticStream {
    match name {
        "fail" => DiagnosticResult::error("failed")?,
        _ => DiagnosticResult::Ok(quote!{struct #name;}),
    }
}

```
