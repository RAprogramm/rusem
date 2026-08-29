// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What every rule of Russian is, seen from outside.
//!
//! A rule states where it is written, what it is about, and what it finds
//! wrong in what was written. Nothing else: it does not read files, does not
//! see the sentence past what its scope allows, and does not weigh how badly a
//! breach reads. That last one is the checker's, and keeping it out is what
//! lets the same paragraph be a refusal in one setting and a remark in
//! another.
//!
//! # Why a trait and not two hundred functions
//!
//! The paragraphs were written one by one, each with its own arguments, and
//! nothing could hold them together. A checker had to name every one of them,
//! so adding a paragraph meant editing the checker, and forgetting to edit it
//! meant the paragraph was written and never asked.
//!
//! One trait fixes both. [`Rule::found`] takes the same facts for every
//! paragraph, so the paragraphs live in a list; [`Rule::scope`] says which
//! words a paragraph could speak of, so a word meets the handful that could
//! answer rather than all two hundred; and a new paragraph is a file and one
//! line in the list.
//!
//! # Why it judges rather than requires
//!
//! An earlier shape had the rule hand out a requirement and let the checker
//! compare it against the writing. That put the comparison outside the rule,
//! and with it the decision of what counts as broken — and it left the rule
//! with one answer, silence, for three different things: the writing is right,
//! the rule is not about this word, and the rule has not got the facts to
//! judge. A pass and a gap looked the same.
//!
//! So the rule judges. What it returns is what it found: nothing when the
//! writing is right, and otherwise the place, the words of the paragraph and
//! the spelling that should stand there. Whether it could judge at all is the
//! scope's answer, given before the rule is asked.

use crate::rules::{Citation, Findings, Scope, facts::Facts};

/// A rule of Russian, as the checker sees it.
pub trait Rule: Sync {
    /// Where the rule is written down.
    fn cites(&self) -> Citation;

    /// What class of word the rule speaks of.
    ///
    /// The checker indexes by this, so a rule that names a narrow scope is
    /// asked about few words. A rule about spelling names none and is asked
    /// about all of them.
    fn scope(&self) -> Scope;

    /// What the rule finds wrong in the writing it is given.
    ///
    /// An empty answer means the writing satisfies this paragraph. It does not
    /// mean the paragraph was skipped: a paragraph that cannot judge is kept
    /// out by its scope and never reaches here.
    fn found(&self, facts: &Facts<'_>) -> Findings;
}

/// Reports whether a rule could speak about a word at all.
///
/// Two gates, both the scope's: the form must be one the rule speaks of, and
/// the facts the rule needs must be known. A rule kept out by either was not
/// asked — which is exactly what tells a gap in the facts apart from a word
/// the rule let through.
#[must_use]
pub fn admits(rule: &dyn Rule, facts: &Facts<'_>) -> bool {
    let scope = rule.scope();

    scope.admits(facts.about.form) && scope.judgeable(&facts.about)
}

/// What a rule finds, or nothing when it is out of scope.
///
/// The one way a caller should ask a rule anything: it applies the scope
/// first, so a rule cannot fire on a word it was never about.
#[must_use]
pub fn asked(rule: &dyn Rule, facts: &Facts<'_>) -> Findings {
    if admits(rule, facts) {
        rule.found(facts)
    } else {
        Findings::new()
    }
}
