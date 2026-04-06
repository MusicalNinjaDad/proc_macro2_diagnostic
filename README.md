# Use `Diagnostic` compiler messages from proc_macro2 code with `Result`-like syntax

Provides a DiagnosticResult which makes it easy to implement multi-level compiler messages
based upon the experimental `proc_macro::Diagnostic` and allows simple idiomatic error handling
via `?` while ensuring errors & warnings are properly emitted by the compiler.

## Note

This crate is deliberately opinionated and focusses on making it easy to create good compiler
errors and handle them easily:

- Top level diagnostics must be either an `Error` or a `Warning`
- (Only) `Help` (& `Note`s -> still to do) can be added to a diagnostic
- `Error`s always span the original call site - add a Help or Note to add information related
  to other spans
- `Warning`s will always finish with a `Note` detailing the original call site
- Multi-level nesting is not possible
- We do not provide a implementation of the full `proc_macro::Diagnostic` API. Other crates
  attempt to do this, if that is what you are after.

## Alternatives

- The similarly named [proc_macro2_diagnostics](https://crates.io/crates/proc-macro2-diagnostics) attempts to provide the full Diagnostic API, also on stable. Although it doesn't allow simple handling via `?` or guaranteed emission; uses a not-recommended hack to identify stable/nightly and in our experience tends to break in its attempt to color output. But it's very popular and complete, just not what we, the authors, were looking for.
