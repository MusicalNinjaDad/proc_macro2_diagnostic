# Diagnostic-based compiler errors for proc_macro2

Use `Diagnostic` compiler messages from proc_macro2 code with `Result`-like syntax.

Provides a DiagnosticResult which stores a Diagnostic based upon the API of
[proc_macro::Diagnostic] and allows `?` usage to return early from proc_macro2 code.

## Note

This crate is a little opinionated in an attempt to make it simpler to create good compiler errors:

- Top-level diagnostics must be either an `Error` or a `Warning`
- (Only) `Help` (& `Note`s -> still to do) can be added to a diagnostic
- `Error`s always span the original call site - add a Help or Note to add information related to other spans
- `Warning`s will always finish with a `Note` detailing the original call site
- Multi-level nesting is not possible

## Usage

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
