// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 72. Мягкий знак перед твёрдой согласной.
//!
//! Буква ь пишется для обозначения мягкости согласной в середине слова перед
//! твёрдой согласной: `молотьба`, `просьба`, `нянька`, `меньше`. В источнике
//! это утверждение стоит вне нумерации — пункты 1 и 2 параграф отводит
//! только согласной перед мягкой, — поэтому ссылка идёт на параграф целиком.

use super::{After, handed_over, stands, with_sign, without_sign};
use crate::rules::{Citation, Findings, Found, Scope, scope};

/// The point as a rule: what it cites and what it is about.
///
/// Holds the citation and the scope together so the engine can list the
/// point alongside the other paragraphs. The judging function [`found`]
/// stays free.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::soft_sign::before_hard::Rule;
///
/// assert_eq!(Rule::CITES.paragraph, 72);
/// ```
pub struct Rule;

impl Rule {
    /// Where this rule is written.
    ///
    /// The before-a-hard-consonant sentence is unnumbered prose in § 72, so the
    /// citation is the paragraph whole.
    pub const CITES: Citation = Citation::whole(72);

    /// What this point is about.
    pub const SCOPE: Scope = scope::Scope::ANY;
}

/// What the point says when the sign is missing.
const SAYS: &str = "мягкость согласной перед твёрдой обозначается мягким знаком";

/// What it says when the sign stands after a hard consonant.
const SAYS_NOT: &str = "после твёрдой согласной мягкий знак не пишется";

/// What the point finds.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::soft_sign::{After, before_hard::found};
///
/// assert!(found("просьба", 'с', true, After::Hard, 3).is_empty());
///
/// let held = found("просба", 'с', true, After::Hard, 3);
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].instead, "просьба");
/// ```
#[must_use]
pub fn found(word: &str, written: char, soft: bool, after: After, at: usize) -> Findings {
    let mut held = Findings::new();
    if !matches!(after, After::Hard) || handed_over(written) {
        return held;
    }

    let written_sign = stands(word, at);
    if written_sign == soft {
        return held;
    }

    let (says, instead) = if soft {
        (SAYS, with_sign(word, at))
    } else {
        (SAYS_NOT, without_sign(word, at))
    };

    held.push(Found::new(Rule::CITES, at + 1, says, instead));
    held
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_paragraph_is_cited_whole() {
        assert_eq!(Rule::CITES.paragraph, 72);
        assert_eq!(Rule::CITES.point, 0);
    }

    #[test]
    fn a_soft_consonant_before_a_hard_one_takes_the_sign() {
        assert!(found("просьба", 'с', true, After::Hard, 3).is_empty());

        let held = found("просба", 'с', true, After::Hard, 3);
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "просьба");
    }

    #[test]
    fn a_hard_consonant_before_a_hard_one_takes_none() {
        let held = found("косьба", 'с', false, After::Hard, 2);

        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "косба");
    }

    #[test]
    fn what_stands_at_the_end_is_another_point() {
        assert!(found("кон", 'н', true, After::End, 2).is_empty());
    }
}
