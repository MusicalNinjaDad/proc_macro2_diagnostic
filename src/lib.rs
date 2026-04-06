#![feature(import_trait_associated_functions)]
#![feature(never_type)]
#![feature(proc_macro_diagnostic)]
#![feature(try_trait_v2)]

//! Provides a DiagnosticResult which stores a Diagnostic based upon the API of
//! [proc_macro::Diagnostic] and allows `?` usage to return early from proc_macro2 code.
//!
//! ## Note
//!
//! This crate is a little opinionated in an attempt to make it simpler to create good compiler errors:
//!
//! - Top level diagnostics must be either an `Error` or a `Warning`
//! - (Only) `Help` (& `Note`s -> still to do) can be added to a diagnostic
//! - `Error`s always span the original call site - add a Help or Note to add information related to other spans
//! - `Warning`s will always finish with a `Note` detailing the original call site
//! - Multi-level nesting is not possible
//!
//! ## Usage
//!
//! ```
//! #![feature(never_type)]
//! #![feature(try_trait_v2)]
//!
//! # extern crate proc_macro;
//!
//! use proc_macro2_diagnostic::prelude::*;
//! use quote::quote;
//!
//! fn zst(name: &str) -> DiagnosticStream {
//!     match name {
//!         "fail" => error("failed")?,
//!         _ => Ok(quote!{struct #name;}),
//!     }
//! }
//!
//! ```

use std::fmt::Debug;

extern crate proc_macro;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::Span;

use crate::DiagnosticResult::{Err, Ok, Warning};

/// Prelude for easy `*`` imports: `use proc_macro2_diagnostic::prelude::*`
pub mod prelude {
    pub use super::DiagnosticResult::{self, Ok};
    pub use super::DiagnosticStream;
    pub use super::{error, warn_spanned};
}

/// A convenience type which is designed to be returned from a proc_macro2-based macro
/// implementation.
/// Call `into`/`from`, not `?`, on this to return and convert the contained TokenStream
/// and/or emit the diagnostic messages.
pub type DiagnosticStream = DiagnosticResult<proc_macro2::TokenStream>;

#[derive(Clone, Debug)]
#[must_use = "this `DiagnosticResult` may be an `Err` variant, which should be handled, or a Warning, which should be emitted"]
#[non_exhaustive]
/// Result-like type which wraps any Ok-type and provides a `Diagnostic`-like API &
/// functionality for non-OK cases.
///
/// ### Usage
/// It is deliberately not possible to directly create an `Err` etc., prefer usage of `error()`,
/// `warn_spanned()` which ensure all invariants are maintained.
///
/// ### Future changes
/// - TODO: #11 Provide complete Result API
pub enum DiagnosticResult<T> {
    Ok(T),
    Warning(T, Diagnostic),
    Err(Diagnostic),
}

/// Create an `Err` result containing an `Error` diagnostic **spanning the macro call_site**
///
/// The message can be anything that implements `ToString` (incl. everything `Display`),
/// this means you can use format_args!() to avoid intermediate allocations.
pub fn error<T, MSG: ToString>(message: MSG) -> DiagnosticResult<T> {
    Err(Diagnostic {
        level: Level::Error,
        message: message.to_string(),
        spans: vec![Span::call_site()],
        children: vec![],
    })
}

/// Create a `Warning` result containing _both_ a `Warning` diagnostic at one or more spans
/// _and_ a valid value.
///
/// The message can be anything that implements `ToString` (incl. everything `Display`),
/// this means you can use format_args!() to avoid intermediate allocations.
///
/// A note will be added to the warning when emitted, which highlights the original call site.
pub fn warn_spanned<T, MSG: ToString, SPN: MultiSpan>(
    value: T,
    span: SPN,
    message: MSG,
) -> DiagnosticResult<T> {
    Warning(
        value,
        Diagnostic {
            level: Level::Warning,
            message: message.to_string(),
            spans: span.into_spans(),
            children: vec![],
        },
    )
}

impl<T> DiagnosticResult<T> {
    /// Add a `Help` message to an existing result at one or more spans.
    ///
    /// The message can be anything that implements `ToString` (incl. everything `Display`),
    /// this means you can use format_args!() to avoid intermediate allocations.
    pub fn add_help<MSG: ToString, SPN: MultiSpan>(mut self, span: SPN, message: MSG) -> Self {
        match self {
            Ok(_) => todo!("Handle attempt to attach a help message to an OK value"),
            DiagnosticResult::Warning(_, ref mut diagnostic) | Err(ref mut diagnostic) => {
                diagnostic.children.push(Diagnostic {
                    level: Level::Help,
                    message: message.to_string(),
                    spans: span.into_spans(),
                    children: vec![],
                });
                self
            }
        }
    }

    // TODO: #18 pub fn add_note()

    /// Return the Ok result or panic.
    pub fn unwrap(self) -> T
    where
        T: Debug,
    {
        match self {
            Ok(t) => t,
            Self::Warning(val, warning) => panic!(
                "Called unwrap on value {:?} with accompanying warning: {:?}",
                val, warning
            ),
            Err(error) => panic!("Called unwrap on an error: {:?}", error),
        }
    }
}

#[derive(Debug, Clone)]
/// The internal Diagnostic stored within DiagnosticResult.
/// Not (currently) designed for direct usage.
///
/// 1:1 structure to match [proc_macro::Diagnostic]
///
/// ### Implementing [std::convert::TryFrom]
/// As it is not possible to directly create a `Diagnostic`, use
/// ```ignore code-snippet
/// impl TryFrom ... {
///     type Error = DiagnosticResult<T>
///     fn try_from ... -> Result<T, DiagnosticResult<T>> {
///         ...
///     }
/// }
/// ```
/// which is a little ugly, but will simplify to either `T` or an unwrapped `DiagnosticResult<T>`
/// on `?`.
pub struct Diagnostic {
    level: Level,
    message: String,
    spans: Vec<Span>,
    children: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// 1:1 match for [proc_macro::Level].
enum Level {
    Error,
    Warning,
    Note,
    Help,
}

impl From<Level> for proc_macro::Level {
    fn from(level: Level) -> Self {
        match level {
            Level::Error => Self::Error,
            Level::Help => Self::Help,
            Level::Note => Self::Note,
            Level::Warning => Self::Warning,
        }
    }
}

/// A helper trait for APIs that accept one or more `Span`s.
///
/// This mirrors the behavior of [proc_macro::diagnostic::MultiSpan] and allows
/// callers to pass a `Span`, `Vec<Span>`, or `&[Span]` to supported APIs.
pub trait MultiSpan {
    /// Consume `self` and convert into an owned `Vec<Span>`.
    fn into_spans(self) -> Vec<Span>;
}

impl MultiSpan for Span {
    fn into_spans(self) -> Vec<Span> {
        vec![self]
    }
}

impl MultiSpan for Vec<Span> {
    fn into_spans(self) -> Vec<Span> {
        self
    }
}

impl MultiSpan for &[Span] {
    fn into_spans(self) -> Vec<Span> {
        self.to_vec()
    }
}

/// Will emit diagnostics in non-fatal cases:
/// - `Ok(val)?` -> `val`
/// - `Warning(val, diag)` -> `val` _and_ emits `diag` immediately
/// - `Err(diag)` -> short-circuits with `Err(diag)` but _does NOT emit_ `diag` as this would lead to
///   repeat emissions
impl<T> std::ops::Try for DiagnosticResult<T> {
    type Output = T;

    type Residual = DiagnosticResult<!>;

    fn from_output(output: Self::Output) -> Self {
        Self::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Ok(t) => std::ops::ControlFlow::Continue(t),
            Self::Warning(t, d) => {
                d.emit();
                std::ops::ControlFlow::Continue(t)
            }
            Self::Err(d) => std::ops::ControlFlow::Break(DiagnosticResult::Err(d)),
        }
    }
}

impl<T> std::ops::FromResidual<DiagnosticResult<!>> for DiagnosticResult<T> {
    fn from_residual(residual: DiagnosticResult<!>) -> Self {
        match residual {
            DiagnosticResult::Err(residual) => DiagnosticResult::Err(residual),
        }
    }
}

/// If you inadvertently (or for "reasons") create a `Result<U,DiagnosticResult<T>>` then `?` will
/// convert and `Err` to a simple `DiagnosticResult<T>::Err`.
impl<T> std::ops::FromResidual<Result<std::convert::Infallible, DiagnosticResult<T>>>
    for DiagnosticResult<T>
{
    fn from_residual(result: Result<std::convert::Infallible, DiagnosticResult<T>>) -> Self {
        match result {
            Result::Err(e) => e,
        }
    }
}

// TODO: #17 impl<T> FromResidual<Result<!, syn::Error>> for DiagnosticResult<T>

/// Convert the underlying [proc_macro2::TokenStream] to a [proc_macro::TokenStream] and/or convert
/// and emit the contained [Diagnostic] as per [proc_macro::Diagnostic], returning an empty
/// [proc_macro::TokenStream] in case of [DiagnosticResult::Err].
impl From<DiagnosticStream> for TokenStream1 {
    fn from(result: DiagnosticStream) -> Self {
        match result {
            DiagnosticResult::Ok(t) => t.into(),
            DiagnosticResult::Warning(t, warning) => {
                warning.emit();
                t.into()
            }
            DiagnosticResult::Err(error) => {
                error.emit();
                TokenStream1::new()
            }
        }
    }
}

impl Diagnostic {
    /// Convert to a [proc_macro::Diagnostic] and then emit.
    fn emit(mut self) {
        if matches!(self.level, Level::Warning) {
            let source_note = Diagnostic {
                level: Level::Note,
                message: "this warning originates from the macro invocation here".to_string(),
                spans: vec![Span::call_site()],
                children: vec![],
            };
            self.children.push(source_note);
        }
        let spans = self.as_spans();
        let mut pm_diagnostic =
            proc_macro::Diagnostic::spanned(spans, self.level.into(), self.message);
        for child in self.children {
            pm_diagnostic = child.add_as_child(pm_diagnostic);
        }
        pm_diagnostic.emit();
    }
}

/// Supporting functions for the conversion to the proc_macro world.
impl Diagnostic {
    /// Add this [Diagnostic] as the child of a [proc_macro::Diagnostic].
    /// Consumes both and returns a new [proc_macro::Diagnostic].
    fn add_as_child(self, parent: proc_macro::Diagnostic) -> proc_macro::Diagnostic {
        let msg = self.message.clone();
        match self.level {
            Level::Error => parent.span_error(self.as_spans(), msg),
            Level::Warning => parent.span_warning(self.as_spans(), msg),
            Level::Note => parent.span_note(self.as_spans(), msg),
            Level::Help => parent.span_help(self.as_spans(), msg),
        }
    }

    /// Get and convert the spans to use in a new [proc_macro::Diagnostic].
    fn as_spans(&self) -> Vec<proc_macro::Span> {
        self.spans.iter().map(|span| span.unwrap()).collect()
    }
}
