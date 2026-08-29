// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 86 п. 3. Слова с частицами `кое-`, `кой-`, `-ка`, `-либо`, `-нибудь`,
//! `-то`, `-тка`, `-с`, `-де` пишутся через дефис.
//!
//! `кое-что`, `кое-кто`, `кой-куда`, `кто-нибудь`, `кто-либо`, `кто-то`,
//! `давай-ка`, `как-нибудь`, `ну-тка`, `да-с`.
//!
//! # Where the making lives
//!
//! `кто-то`, `кто-либо`, `кто-нибудь` and `кое-кто` are four words from one
//! asking word and four particles, and every asking word makes the same four.
//! Twenty-odd indefinite pronouns and adverbs are therefore a handful of
//! particles and the ten words that ask.
//!
//! That making is a fact about the language and lives in [`asking`]. Here is
//! only what the code of 1956 adds to it: the hyphen.
//!
//! # The exception is again a space
//!
//! Note 1 says `кое-кто` before a preposition falls into three words: `кое у
//! кого`, `кое в чём`. The hyphen is what joins, and a preposition unjoins.
//! [`required`] therefore reports only a run of two words.

use crate::{
    grammar::closed::asking,
    rules::{Citation, Findings, Found, Scope, scope}
};

/// Where this rule is written.
pub const CITES: Citation = Citation::point(86, 3);

/// What this rule is about.
pub const SCOPE: Scope = scope::ANY;

/// The particles the paragraph writes in front.
const LEADING: &[&str] = &["кое", "кой"];

/// The particles the paragraph writes behind.
const TRAILING: &[&str] = &["нибудь", "либо", "тка", "ка", "то", "де", "с"];

/// What the paragraph says when it is broken.
const SAYS: &str = "частицы кое-, -то, -либо, -нибудь пишутся через дефис";

/// What the paragraph finds in a written pair of words.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::hyphen_compound::particles::found;
///
/// let held = found("кто то");
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].instead, "кто-то");
///
/// assert!(found("кто-то").is_empty());
/// assert!(found("кое у кого").is_empty());
/// ```
#[must_use]
pub fn found(written: &str) -> Findings {
    let mut held = Findings::new();
    let lowered = written.to_lowercase();
    let mut said = lowered.split_whitespace();
    let (Some(first), Some(rest), None) = (said.next(), said.next(), said.next()) else {
        return held;
    };

    let joined = if LEADING.contains(&first) {
        asking::asks(rest)
    } else {
        TRAILING.contains(&rest) && asking::asks(first)
    };
    if !joined {
        return held;
    }

    held.push(Found::new(
        CITES,
        first.chars().count(),
        SAYS,
        std::format!("{first}-{rest}")
    ));
    held
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_paragraph_and_its_point_are_cited() {
        assert_eq!(CITES.paragraph, 86);
        assert_eq!(CITES.point, 3);
    }

    #[test]
    fn every_word_the_paragraph_names_is_one_the_making_reaches() {
        for held in [
            "кое-что",
            "кое-кто",
            "кое-какой",
            "кой-куда",
            "кто-нибудь",
            "кто-либо",
            "кто-то"
        ] {
            assert!(
                asking::built_from(held).is_some(),
                "the paragraph names {held} and the making does not reach it"
            );
        }
    }

    #[test]
    fn a_particle_written_off_its_word_is_found_and_spelled_with_the_hyphen() {
        let one = found("кто то");
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].cites, CITES);
        assert_eq!(one[0].instead, "кто-то");

        assert_eq!(found("кое что")[0].instead, "кое-что");
        assert_eq!(found("где нибудь")[0].instead, "где-нибудь");
    }

    #[test]
    fn a_word_already_hyphenated_is_left_alone() {
        assert!(found("кто-то").is_empty());
        assert!(found("кое-что").is_empty());
    }

    #[test]
    fn a_preposition_between_them_is_the_exception_the_note_names() {
        assert!(found("кое у кого").is_empty());
        assert!(found("кое в чём").is_empty());
    }

    #[test]
    fn two_words_that_are_no_such_pair_are_left_alone() {
        assert!(found("на столе").is_empty());
        assert!(found("стол то").is_empty());
        assert!(found("").is_empty());
    }

    #[test]
    fn the_case_a_word_is_written_in_does_not_matter() {
        assert_eq!(found("КТО ТО")[0].instead, "кто-то");
    }
}
