# proc_macro2_diagnostic changelog

## [v0.6.3](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/tree/v0.6.3)

### Bug fixes

- Fully remove circular dependency risk via `try_v2_derive` by directly using `try_v2_traits`

## [v0.6.2](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/tree/v0.6.2)

### Bug fixes

- Remove circular dependency risk via `try_v2_derive`

## [v0.6.1](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/tree/v0.6.1)

### New features

- Added traits `Transform` and `Extract` to (try_v2) `DiagnosticResult`
- Impl'd `FromIterator` for `DiagnosticResult`

## [v0.6.0](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/tree/v0.6.0)

### Breaking changes

- Added `diagnostic()` to `trait AsDiagnostic` allowing for `TryFrom` implementations to use `Error = Diagnostic`

## [v0.5.0](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/tree/v0.5.0)

### Breaking changes

- Removed `FromResidual<Result<!,DiagnosticResult<!>>` - this breaks invocations of `Option::ok_or(error())?;`, which was nightly-only: see below for improved syntax on both stable & nightly.

### New features

- Added `trait ToDiagnostic` and `impl ToDiagnostic for Option` allowing `Option::or_error()`, `Option::or_error_spanned()` & `Option::or_warn_spanned_with_default()` on both stable & nightly.

## [v0.4.0](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/tree/v0.4.0)

### Breaking changes

- Removed `From<DiagnosticStream> for TokenStream`

### New features

- Consistent interface across nightly & stable. Automatic & future-proof identification & handling of experimental features.

## [v0.3.1](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/tree/v0.3.1)

### Bugfixes

- Support nightly from 2026-04-16: fulfil new trait bound `Try::Residual: Residual<Self::Output>` [rustlang/rust commit 3efcdbc](https://github.com/rust-lang/rust/commit/3efcdbc43c0d8d00bbbd84989a920d9fb63f6066)

## [v0.3.0](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/tree/v0.3.0)

## Breaking changes

- Inter-conversion from `Result` is only possible for `Result<_, DiagnosticResult<!>>` (was `Result<_, DiagnosticResult<T>>`, which was broken - see Bugfixes)

### New features

- Use `?` on a `Result<T, syn::Error>` in a function that returns a `DiagnosticResult` (or a `Result<_, DiagnosticResult>`)
- `add_help()` or `add_note()` to a `Result<T, syn::Error>`

### Bugfixes

- Fix for using `?` on a `Result<T, DiagnosticResult<T>>` in a function which returns `DiagnosticResult<U>`.

## [v0.2.0](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/tree/v0.2.0) - Simplify & focus API

### Breaking changes

The API was deliberately narrowed, to minimise future breaking changes:

- `DiagnosticResult` is now a struct with an inner, private enum
- `Diagnostic` is not exposed publicly (was previously exposed but unusable)

### New features

- Added `prelude` for easy imports of main functionality
- Added `is_ok()`, `is_error()`, `is_warning()` & `kind()` to functionally replace pattern matching
- Added `add_note()`
- Added `error_spanned()`

### Improvements

- Automatic note, highlighting call site on both warning & error, is now only added if no other message already highlights the call site

### Stability

- Automated monthly tests to provide stability guarantee for all experimental versions of 2024 edition
- Handle upcoming stabilisation of `assert_matches`

## [v0.1.0](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/tree/v0.1.0) - Warnings

### New features

- Can store a compiler `warning`, emitted immediately upon `?`, alongside a valid value
- Constructor functions can take a single `Span`, a `Vec<Span>`, or an `&[Span]`
- Automatically adds a `note` to any warning highlighting the call site

### Improvements

- Must use `DiagnosticResult`s
- `DiagnosticResult` is `Clone`
- Reduce restriction on messages from `Display` to `ToString`

## [v0.0.1](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/tree/v0.0.1) - Initial implementation

### New features

- Basic DiagnosticResult enum
- Can store a compiler error
- Can add_help to an error
- Implements `Try` (?)
- Emits on conversion to TokenStream
