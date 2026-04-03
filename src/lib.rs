#![feature(never_type)]
#![feature(proc_macro_diagnostic)]
#![feature(try_trait_v2)]

//! Provides a DiagnosticResult which stores a Diagnostic with the same (target) API as
//! [proc_macro::Diagnostic] and allows `?` usage to return early from proc_macro2 code.
//!
//! ```
//! #![feature(never_type)]
//! #![feature(try_trait_v2)]
//!
//! # extern crate proc_macro;
//!
//! use proc_macro2_diagnostic::{DiagnosticResult,DiagnosticStream};
//! use quote::quote;
//!
//! fn zst(name: &str) -> DiagnosticStream {
//!     match name {
//!         "fail" => DiagnosticResult::error("failed")?,
//!         _ => DiagnosticResult::Ok(quote!{struct #name;}),
//!     }
//! }
//!
//! ```

use std::fmt::Display;

extern crate proc_macro;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::Span;

/// A convenience type which is designed to be returned from a proc_macro2-based macro
/// implementation.
/// Call `into`/`from`, not `?`, on this to return and convert the contained TokenStream
/// and/or emit the diagnostic messages.
///
/// ### Future changes:
/// - TODO: #9 Consider changing usage of DiagnosticStream to be Try-based instead of From-based.
pub type DiagnosticStream = DiagnosticResult<proc_macro2::TokenStream>;

#[derive(Debug)]
#[non_exhaustive]
/// Result-like type which wraps any Ok-type and provides a `Diagnostic`-like API &
/// functionality for non-OK cases.
///
/// ### Usage
/// **Do not directly create an `Err`, prefer usage of `error()`**
///
/// ### Future changes
/// - TODO: #10 Extend to include Warnings etc. (emited on `?`)
/// - TODO: #11 Provide complete Result API
pub enum DiagnosticResult<T> {
    Ok(T),
    Err(Diagnostic),
}

impl<T> DiagnosticResult<T> {
    /// Create an `Err` result containing an `Error` diagnostic **spanning the macro call_site**
    ///
    /// The message can be anything that implements `Display` - this means you can use
    /// format_args!() to avoid intermediate allocations
    pub fn error<S: Display>(message: S) -> Self {
        Self::Err(Diagnostic {
            level: Level::Error,
            message: message.to_string(),
            spans: vec![Span::call_site()],
            children: vec![],
        })
    }

    /// Add a `Help` message to an existing result, at a given span.
    ///
    /// The message can be anything that implements `Display` - this means you can use
    /// format_args!() to avoid intermediate allocations
    pub fn add_help<S: Display>(mut self, span: Span, message: S) -> Self {
        let Self::Err(ref mut diagnostic) = self else {
            todo!()
        };
        diagnostic.children.push(Diagnostic {
            level: Level::Help,
            message: message.to_string(),
            spans: vec![span],
            children: vec![],
        });
        self
    }

    /// Return the Ok result or panic
    pub fn unwrap(self) -> T {
        let Self::Ok(t) = self else {
            panic!("Called unwrap on a not-OK value")
        };
        t
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    level: Level,
    message: String,
    spans: Vec<Span>,
    children: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Level {
    Error,
    #[expect(unused)]
    Warning,
    #[expect(unused)]
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

impl<T> std::ops::Try for DiagnosticResult<T> {
    type Output = T;

    type Residual = DiagnosticResult<!>;

    fn from_output(output: Self::Output) -> Self {
        Self::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Ok(t) => std::ops::ControlFlow::Continue(t),
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

impl<T> std::ops::FromResidual<Result<std::convert::Infallible, DiagnosticResult<T>>>
    for DiagnosticResult<T>
{
    fn from_residual(result: Result<std::convert::Infallible, DiagnosticResult<T>>) -> Self {
        match result {
            Err(e) => e,
        }
    }
}

impl Diagnostic {
    fn add_as_child(self, parent: proc_macro::Diagnostic) -> proc_macro::Diagnostic {
        let msg = self.message.clone();
        match self.level {
            Level::Error => parent.span_error(self.as_spans(), msg),
            Level::Warning => parent.span_warning(self.as_spans(), msg),
            Level::Note => parent.span_note(self.as_spans(), msg),
            Level::Help => parent.span_help(self.as_spans(), msg),
        }
    }
}

impl Diagnostic {
    fn as_spans(&self) -> Vec<proc_macro::Span> {
        self.spans.iter().map(|span| span.unwrap()).collect()
    }
}

impl From<DiagnosticStream> for TokenStream1 {
    fn from(result: DiagnosticStream) -> Self {
        match result {
            DiagnosticResult::Ok(t) => t.into(),
            DiagnosticResult::Err(diagnostic) => {
                // MSV: unwrap requires rustc 1.29+ *without* semver exempt features
                let spans = diagnostic.as_spans();
                let mut pm_diagnostic = proc_macro::Diagnostic::spanned(
                    spans,
                    diagnostic.level.into(),
                    diagnostic.message,
                );
                for child in diagnostic.children {
                    pm_diagnostic = child.add_as_child(pm_diagnostic);
                }
                pm_diagnostic.emit();
                TokenStream1::new()
            }
        }
    }
}
