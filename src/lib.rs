#![feature(never_type)]
#![feature(proc_macro_diagnostic)]
#![feature(try_trait_v2)]

//! Provides a DiagnosticResult which makes it easy to implement multi-level compiler messages
//! based upon the experimental [proc_macro::Diagnostic] and allows simple idiomatic error handling
//! via `?` while ensuring errors & warnings are properly emitted by the compiler.
//!
//! ## Note
//!
//! This crate is deliberately opinionated and focusses on making it easy to create good compiler
//! errors and handle them easily:
//!
//! - Top level diagnostics must be either an `Error` or a `Warning`
//! - (Only) `Help` (& `Note`s -> still to do) can be added to a diagnostic
//! - `Error`s always span the original call site - add a Help or Note to add information related
//!   to other spans
//! - `Warning`s will always finish with a `Note` detailing the original call site
//! - Multi-level nesting is not possible
//! - We do not provide a implementation of the full [proc_macro::Diagnostic] API. Other crates
//!   attempt to do this, if that is what you are after.

use std::fmt::Debug;

extern crate proc_macro;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::Span;

use crate::DiagnosticResult_::{Err, Ok as Ok_, Warning};
use crate::internal::*;

/// Prelude for easy `*`` imports: `use proc_macro2_diagnostic::prelude::*`
pub mod prelude {
    pub use super::DiagnosticResult;
    pub use super::DiagnosticStream;
    pub use super::{Ok, error, warn_spanned};
}

/// A convenience type which is designed to be returned from a proc_macro2-based macro
/// implementation.
///
/// ### Usage
/// 1. Shorten your proc_macro to `my_proc_macro2_impl(input.into()).into()`
/// 2. Return a DiagnosticStream from `my_proc_macro2_impl(input: proc_macro2::Tokenstream) -> DiagnosticStream`
/// 3. Use `Ok()`, `error` or `warn_spanned` within the function; return a `DiagnosticResult<_>`
///    from any supporting functions and handle it with `?`
///
/// All errors & warnings will be properly emitted by the compiler and nicely formatted.
pub type DiagnosticStream = DiagnosticResult<proc_macro2::TokenStream>;

#[derive(Clone, Debug)]
#[must_use = "this `DiagnosticResult` may be an error or a warning, which should be emitted"]
/// Result-like type which can represent a valid return value, an error or a warning accompanying
/// a valid return value. Warnings will be emitted upon `?`, allowing your code to continue with
/// the valid value. Errors will short-circuit upon `?` and be emitted upon final conversion to a
/// [proc_macro::TokenStream]
///
/// ### Usage
/// 1. Create a DiagnosticResult via `Ok()`, `error` or `warn_spanned`.
/// 2. Treat the DiagnosticResult as you would any other Result type and unpack it with `?` at a
///    suitable point in your code.
///
/// ### Implementing [std::convert::TryFrom]
/// As it is not possible to directly create a pure Diagnostic, use `Result<T, DiagnosticResult<T>>`
/// ```
/// #![feature(never_type)]
/// use proc_macro2_diagnostic::prelude::*;
/// use syn::{parse_quote, LitInt};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// struct Even(i32);
///
/// impl TryFrom<LitInt> for Even {
///     type Error = DiagnosticResult<Even>;
///     fn try_from(num: LitInt) -> Result<Even, DiagnosticResult<Even>> {
///         // TODO: #17 allow direct ? from syn::Error
///         let num: i32 = num.base10_parse().unwrap();
///         if num % 2 == 0 {
///             std::result::Result::Ok(Even(num))
///         } else {
///             std::result::Result::Err(error("odd number"))
///         }
///     }
/// }
///
/// fn is_even(num: LitInt) -> DiagnosticResult<Even> {
///     let even = Even::try_from(num)?;
///     Ok(even)
/// }
///
/// assert!(is_even(parse_quote!(1)).is_error());
/// assert!(is_even(parse_quote!(2)).is_ok());
/// ```
/// which is a little ugly, but will simplify to either `T` or an unwrapped `DiagnosticResult<T>`
/// on `?`.
///
/// ### Future changes
/// - TODO: #11 Provide complete Result API
pub struct DiagnosticResult<T> {
    inner: DiagnosticResult_<T>,
}

#[derive(Clone, Debug)]
/// The type of top-level message contained in the DiagnosticResult
pub enum DiagnosticResultKind {
    Ok,
    Warning,
    Error,
}

#[derive(Clone, Debug)]
enum DiagnosticResult_<T> {
    Ok(T),
    Warning(T, Diagnostic),
    Err(Diagnostic),
}

/// Create an `Ok` result.
#[expect(non_snake_case, reason = "same feel as a Result type alias")]
pub fn Ok<T>(val: T) -> DiagnosticResult<T> {
    DiagnosticResult {
        inner: DiagnosticResult_::Ok(val),
    }
}

/// Create an `Err` result containing an `Error` diagnostic **spanning the macro call_site**
///
/// The message can be anything that implements `ToString` (incl. everything `Display`),
/// this means you can use format_args!() to avoid intermediate allocations.
pub fn error<T, MSG: ToString>(message: MSG) -> DiagnosticResult<T> {
    DiagnosticResult {
        inner: Err(Diagnostic::new(Level::Error, Span::call_site(), message)),
    }
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
    DiagnosticResult {
        inner: Warning(value, Diagnostic::new(Level::Warning, span, message)),
    }
}

impl<T> DiagnosticResult<T> {
    /// Add a `Help` message to an existing result at one or more spans.
    ///
    /// The message can be anything that implements `ToString` (incl. everything `Display`),
    /// this means you can use format_args!() to avoid intermediate allocations.
    pub fn add_help<MSG: ToString, SPN: MultiSpan>(mut self, span: SPN, message: MSG) -> Self {
        match self.inner {
            Ok_(val) => Self {
                inner: Warning(val, Diagnostic::new(Level::Help, span, message)),
            },
            Warning(_, ref mut diagnostic) | Err(ref mut diagnostic) => {
                diagnostic.add_help(span, message);
                self
            }
        }
    }

    /// Add a `Note` to an existing result at one or more spans.
    ///
    /// The message can be anything that implements `ToString` (incl. everything `Display`),
    /// this means you can use format_args!() to avoid intermediate allocations.
    pub fn add_note<MSG: ToString, SPN: MultiSpan>(mut self, span: SPN, message: MSG) -> Self {
        match self.inner {
            Ok_(val) => Self {
                inner: Warning(val, Diagnostic::new(Level::Note, span, message)),
            },
            Warning(_, ref mut diagnostic) | Err(ref mut diagnostic) => {
                diagnostic.add_note(span, message);
                self
            }
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(&self.inner, &Ok_(_))
    }

    pub fn is_warning(&self) -> bool {
        matches!(&self.inner, &Warning(_, _))
    }

    pub fn is_error(&self) -> bool {
        matches!(&self.inner, &Err(_))
    }

    /// The type of top-level message
    pub fn kind(&self) -> DiagnosticResultKind {
        match self.inner {
            DiagnosticResult_::Ok(_) => DiagnosticResultKind::Ok,
            DiagnosticResult_::Warning(_, _) => DiagnosticResultKind::Warning,
            DiagnosticResult_::Err(_) => DiagnosticResultKind::Error,
        }
    }

    /// Return the Ok result or panic.
    pub fn unwrap(self) -> T
    where
        T: Debug,
    {
        match self.inner {
            Ok_(t) => t,
            Warning(val, warning) => panic!(
                "Called unwrap on value {:?} with accompanying warning: {:?}",
                val, warning
            ),
            Err(error) => panic!("Called unwrap on an error: {:?}", error),
        }
    }
}

mod internal {
    use std::fmt::Display;

    use proc_macro2::Span;

    #[derive(Debug, Clone)]
    /// The internal Diagnostic stored within DiagnosticResult.
    /// Not (currently) designed for direct usage.
    ///
    /// 1:1 structure to match [proc_macro::Diagnostic]
    pub struct Diagnostic {
        pub level: Level,
        pub message: String,
        pub spans: Vec<Span>,
        pub children: Vec<Diagnostic>,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    /// 1:1 match for [proc_macro::Level].
    pub enum Level {
        Error,
        Warning,
        Note,
        Help,
    }

    impl Display for Level {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Level::Error => write!(f, "error"),
                Level::Warning => write!(f, "warning"),
                Level::Note => write!(f, "note"),
                Level::Help => write!(f, "help"),
            }
        }
    }

    impl Diagnostic {
        pub fn new<MSG: ToString, SPN: MultiSpan>(level: Level, span: SPN, message: MSG) -> Self {
            Self {
                level,
                message: message.to_string(),
                spans: span.into_spans(),
                children: vec![],
            }
        }

        pub fn add_help<MSG: ToString, SPN: MultiSpan>(&mut self, span: SPN, message: MSG) {
            self.children.push(Diagnostic {
                level: Level::Help,
                message: message.to_string(),
                spans: span.into_spans(),
                children: vec![],
            });
        }

        pub fn add_note<MSG: ToString, SPN: MultiSpan>(&mut self, span: SPN, message: MSG) {
            self.children.push(Diagnostic {
                level: Level::Note,
                message: message.to_string(),
                spans: span.into_spans(),
                children: vec![],
            })
        }

        /// Does any message _exactly_ span the call_site (not just across it)?
        fn spans_call_site(&self) -> bool {
            let call_site = Span::call_site();
            let cs_file = call_site.local_file();
            let cs_start = call_site.start();
            let cs_end = call_site.end();
            let is_call_site = |span: &&Span| {
                span.local_file() == cs_file && span.start() == cs_start && span.end() == cs_end
            };

            self.spans.iter().find(is_call_site).is_some()
                || self
                    .children
                    .iter()
                    .find(|child| child.spans_call_site())
                    .is_some()
        }

        /// Convert to a [proc_macro::Diagnostic] and then emit.
        pub fn emit(mut self) {
            if !self.spans_call_site() {
                self.add_note(
                    Span::call_site(),
                    format!(
                        "this {} originates from the macro invocation here",
                        self.level
                    ),
                );
            };
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
        Self { inner: Ok_(output) }
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self.inner {
            // BUG RISK?? Removal of Self - confusion between <T> and <!>??
            Ok_(t) => std::ops::ControlFlow::Continue(t),
            Warning(t, d) => {
                d.emit();
                std::ops::ControlFlow::Continue(t)
            }
            Err(d) => std::ops::ControlFlow::Break(DiagnosticResult { inner: Err(d) }),
        }
    }
}

impl<T> std::ops::FromResidual<DiagnosticResult<!>> for DiagnosticResult<T> {
    fn from_residual(residual: DiagnosticResult<!>) -> Self {
        match residual.inner {
            Err(residual) => Self {
                inner: Err(residual),
            },
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
        match result.inner {
            Ok_(t) => t.into(),
            Warning(t, warning) => {
                warning.emit();
                t.into()
            }
            Err(error) => {
                error.emit();
                TokenStream1::new()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_ok() {
        assert!(Ok(()).is_ok());
    }

    #[test]
    fn is_warning() {
        assert!(warn_spanned((), Span::call_site(), "foo").is_warning());
    }

    #[test]
    fn is_error() {
        assert!(error::<(), &str>("foo").is_error());
    }

    #[test]
    fn kind() {
        match Ok(()).kind() {
            DiagnosticResultKind::Ok => (),
            DiagnosticResultKind::Warning => panic!("not a warning"),
            DiagnosticResultKind::Error => panic!("not an error"),
        }
    }
}
