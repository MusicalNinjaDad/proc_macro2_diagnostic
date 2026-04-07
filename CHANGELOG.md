# proc_macro2_diagnostic changelog

## [v0.3.0](https://github.com/MusicalNinjaDad/proc_macro2_diagnostic/tree/v0.3.0)

### New features

- Use `?` on a syn::Error in a function that returns a `DiagnosticResult` (or a `Result<_, DiagnosticResult>`)

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
