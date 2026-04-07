# Use `Diagnostic` compiler messages from proc_macro2 code with `Result`-like syntax

Provides a DiagnosticResult which makes it easy to implement multi-level compiler messages based upon the experimental `proc_macro::Diagnostic` and allows simple idiomatic error handling via `?` while ensuring errors & warnings are properly emitted by the compiler.

## Note

This crate is deliberately opinionated and focuses on making it easy to create good compiler errors and handle them easily:

- Top level diagnostics must be either an `Error` or a `Warning`
- (Only) `Help` & `Note`s can be added to a diagnostic
- `Error`s always span the original call site - add a Help or Note to add information related to other spans
- `Warning`s will always finish with a `Note` detailing the original call site
- Multi-level nesting is not possible
- We do not provide an implementation of the full `proc_macro::Diagnostic` API. Other crates attempt to do this, if that is what you are after.

## Stability & MSRV

Given that this crate exposes an experimental API from std it makes use of experimental features which require a nightly toolchain.

> 🔬 **Experimental Features**
>
> This crate makes use of the following experimental features:
>
> - [`#![feature(assert_matches)]`](https://github.com/rust-lang/rust/issues/82775) (stable since 2026-02-12)
> - [`#![feature(never_type)]`](https://github.com/rust-lang/rust/issues/35121)
> - [`#![feature(proc_macro_diagnostic)]`](https://github.com/rust-lang/rust/issues/54140)
> - [`#![feature(try_trait_v2)]`](https://github.com/rust-lang/rust/issues/84277)
>
> This list includes any unstable features used by direct & transitive dependencies (currently, none).
>
> The authors consider all of the above features to be reliable and already well advanced in the stabilisation process.

You do not need to enable these in your own code, the list is for information only.

### Stability guarantees

We run automated tests **every month** to ensure no fundamental changes affect this crate and test every PR against the current nightly, as well as the current equivalent beta & stable. If you find an issue before we do, please [raise an issue on github](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/issues).

### MSRV

For those of you working with a pinned nightly (etc.) this crate supports every version of edition 2024 (rust 1.85.1 onwards, released as stable on 20225-03-18). We use [autocfg](https://crates.io/crates/autocfg/) to seamlessly handle features which have been stabilised since then.

### Dependencies

We deliberately keep the dependency list short and pay attention to any transitive dependencies we bring in.

Current dependency tree:

```text
proc_macro2_diagnostic <- This crate
└── proc-macro2
    └── unicode-ident
└── syn
    ├── quote
    │   └── proc-macro2
    └── unicode-ident
```

## Alternatives

- The similarly named [proc_macro2_diagnostics](https://crates.io/crates/proc-macro2-diagnostics) attempts to provide the full Diagnostic API, also on stable. Although it doesn't allow simple handling via `?` or guaranteed emission; uses a not-recommended hack to identify stable/nightly and in our experience tends to break in its attempt to color output. But it's very popular and complete, just not what we, the authors, were looking for.
