use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct LinkSignalFirstStatement;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// In a class that inherits `linkSignal`, requires every async instance
    /// method that takes the caller's `AbortSignal` to start with
    /// `const signal = super.linkSignal({ signal: <caller signal> });`, and
    /// reports any later `super.linkSignal` of that same signal.
    ///
    /// ### Why is this bad?
    ///
    /// Linking inline at each call site links the same signal several times
    /// and leaves the statements before the first link running after the
    /// instance was closed. One link, first, covers the whole body.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// class A extends ADisposable {
    ///   async run({ signal }: { signal: AbortSignal }): Promise<void> {
    ///     await fetch(url, { signal: super.linkSignal({ signal }) });
    ///   }
    /// }
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// class A extends ADisposable {
    ///   async run({ signal: callerSignal }: { signal: AbortSignal }): Promise<void> {
    ///     const signal = super.linkSignal({ signal: callerSignal });
    ///     await fetch(url, { signal });
    ///   }
    /// }
    /// ```
    LinkSignalFirstStatement(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description =
        "Require async methods to link the caller's signal once, as their first statement.",
);

impl Rule for LinkSignalFirstStatement {}
