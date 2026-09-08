// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The rules of Russian, stated one paragraph to a file.
//!
//! A rule of the language is not policy and not a check: it is a statement
//! about what Russian requires, and it belongs beside the categories it speaks
//! of. `§ 40` is about the case of a noun, its stress and its stem — all three
//! are here in [`crate::grammar`], and nowhere else does the rule have to
//! reach.
//!
//! # The shape
//!
//! Every rule states three things and no more.
//!
//! | It states | Which is |
//! | --- | --- |
//! | where it is written down | [`Citation`] — the paragraph and point of the code of 1956 |
//! | what it is about | [`Scope`] — the class of word it speaks of, so the checker asks only the rules that could answer |
//! | what it finds wrong | its own `found`, which reads the writing and judges it |
//!
//! The scope is what makes a hundred rules cheap: a noun in the prepositional
//! meets the handful of rules about nouns in the prepositional, not all of
//! them.
//!
//! # What a rule may not do
//!
//! A rule reads facts and judges the writing. It does not read a file, does
//! not see the rest of the sentence unless its scope says so, and does not
//! decide how badly a breach reads — that is the checker's to weigh, and
//! keeping it out of here is what lets the same rule be a refusal in one
//! setting and a remark in another.

pub mod citation;
pub mod facts;
pub mod found;
pub mod rule;
pub mod scope;
pub mod svod;

pub use self::{
    citation::Citation,
    facts::{Facts, Parts},
    found::{Findings, Found},
    rule::Rule,
    scope::Scope
};

/// Every rule the engine may ask, in the order they stand in the code.
///
/// # Arguments
///
/// Takes no arguments.
///
/// # Returns
///
/// A list of every paragraph the engine knows, each as a [`Rule`].
///
/// # Examples
///
/// ```
/// use rusem::rules::{Rule, all};
///
/// let rules = all();
/// assert!(!rules.is_empty());
///
/// for rule in rules {
///     assert!(rule.cites().is_stated());
/// }
/// ```
#[must_use]
pub fn all() -> Vec<&'static dyn Rule> {
    svod::written::Rules::RULES.to_vec()
}
