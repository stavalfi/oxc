use oxc_macros::declare_oxc_lint;

use crate::rule::Rule;

#[derive(Debug, Default, Clone)]
pub struct NoExportRename;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallows renaming a binding with `as` inside an `export { ... }` list.
    ///
    /// ### Why is this bad?
    ///
    /// The exported name no longer matches the declaration, so searching for one never finds the other.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// export { TENANT_CONFIGS as CATEGORIES_ETL_TENANT_CONFIGS } from "./zod-schemas.ts";
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// export { CATEGORIES_ETL_TENANT_CONFIGS } from "./zod-schemas.ts";
    /// ```
    NoExportRename(tsgolint),
    custom_tsgolint_oxlint_rules,
    restriction,
    version = "1.86.0",
    short_description = "Disallow `as` renames in exports.",
);

impl Rule for NoExportRename {}
