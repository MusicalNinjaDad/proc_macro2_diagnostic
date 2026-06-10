# Use `Diagnostic` compiler messages from proc_macro2 code with `Result`-like syntax

Provides a DiagnosticResult which makes it easy to implement multi-level compiler messages based upon the experimental `proc_macro::Diagnostic` and allows simple idiomatic error handling via `?` while ensuring errors & warnings are properly emitted by the compiler.

Provides a "fire and forget" API which works (and is tested) on nightly, stable and will automatically provide new features on stable as they become available. We have taken great care to provide an experience which automatically gets better as experimental features are stabilised and correctly and safely identifies the features available at build time.

## Note

This crate is deliberately opinionated and focuses on making it easy to create good compiler errors and handle them easily:

- Top level diagnostics must be either an `Error` or a `Warning`
- (Only) `Help` & `Note`s can be added as children
- A note highlighting the original call site will be added to any `Error`s / `Warning`s which do not span the call site themselves, or contain a child `Note` / `Help` which does.
- Multi-level nesting is not possible
- We do not provide an implementation of the full `proc_macro::Diagnostic` API. Other crates attempt to do this, if that is what you are after.

## Stability & MSRV

Given that this crate exposes an experimental API from std it works best on a nightly toolchain. I have taken great care to craft an API which works consistently on stable and which will leverage experimental features as soon as they stabilise, without additional intervention.

> 🔬 **Experimental Features**
>
> This crate makes use of two groups of experimental features on nightly, and implements the following technical compromises on where these features are unavailable:
>
> ### Diagnostic compiler messages
>
> - [`#![feature(proc_macro_diagnostic)]`](https://github.com/rust-lang/rust/issues/54140)
>
> On systems where the experimental API is not available:
>
> - Multi-Span messages are not possible. The *first* `Span` (based on insertion order, not code position) will be used and any additional `Spans` will be disregarded
> - All messages will be output as compiler errors. `note: ...` becomes `error: note: ...`
>
> ### Custom Try types
>
> - [`#![feature(never_type)]`](https://github.com/rust-lang/rust/issues/35121)
> - [`#![feature(try_trait_v2)]`](https://github.com/rust-lang/rust/issues/84277)
> - [`#![feature(try_trait_v2_residual)]`](https://github.com/rust-lang/rust/issues/91285)
>
> On systems where custom try types are not available, handling `Warning`s is less ergonomic:
>
> - `Warning`s are immediately emitted upon construction
> - any `Help` or `Note`s added to a `Warning` will not be emitted
> - any custom handling will not activate

### Stability guarantees

We recognise that you probably do not have control over the toolchain used to compile your crate, that is decided by someone downstream. This crate is therefore constructed in a way that will ensure your code *always* compiles, regardless of whether it is ultimately built on a nightly toolchain, the current stable toolchain, or a future stable toolchain where some of the above experimental features are stabilised.

We do this by using the amazing [autocfg](https://crates.io/crates/autocfg/) to securely identify the availability of each feature we use as well as the need to enable an experimental feature flag.

We run automated tests **every month** to ensure no fundamental changes affect this crate and test every PR against the current nightly, as well as the current equivalent beta & stable. We test & lint every push 4 times: against current nightly, current stable and nightly with only try / diagnostic enabled.

We recommend you also test your crate on *at least* stable & nightly before publishing.

If you find an issue before we do, please [raise an issue on github](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/issues).

The only thing you need to worry about: **Please do NOT expand the type alias `DiagnosticResult<T>` on stable**, if you do your crate will not work on nightly, and will break at some point in the future when try_trait_v2 is stabilised.

### MSRV

This crate supports every version of edition 2024 (rust 1.85.1 onwards, released as stable on 20225-03-18).

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

[build-dependencies]
├── autocfg
└── ninja-build_rs
    └── autocfg
```

## Alternatives

- The similarly named [proc_macro2_diagnostics](https://crates.io/crates/proc-macro2-diagnostics) attempts to provide the full Diagnostic API, also on stable. Although it doesn't allow simple handling via `?` or guaranteed emission; uses a **not-recommended hack to identify stable/nightly** and in our experience tends to break in its attempt to color output. But it's very popular and complete, just not what we, the authors, were looking for.
