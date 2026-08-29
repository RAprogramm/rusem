// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 1. После ж, ч, ш, щ не пишутся ю, я, ы, а пишутся у, а, и.
//!
//! `чудо`, `щука`, `час`, `роща`, `жир`, `шить`. The paragraph allows `ю` and
//! `я` after these consonants in loanwords — `жюри`, `парашют`, `Сен-Жюст` —
//! and in abbreviations, where the code permits any letters at all (§ 110).
//!
//! That exception is why the paragraph is asked here about `ы` alone: `парашют`
//! is a correct word, so a rule that refused `ю` after `ш` would refuse
//! correct Russian. What holds without exception is the other half — `ы` never
//! stands after a sibilant, in a loanword or out of one — and it is what the
//! paragraph judges by.

use crate::{
    grammar::stem,
    rules::{Citation, Findings, Found, Scope, found::spelled, scope}
};

/// Where this rule is written.
pub const CITES: Citation = Citation::point(1, 1);

/// What this rule is about.
///
/// Every word: a sibilant takes the same letters after it whatever the word
/// is, and § 1 names no part of speech.
pub const SCOPE: Scope = scope::ANY;

/// What the paragraph says when it is broken.
const SAYS: &str = "после шипящей пишется и, а не ы";

/// The letter that never stands after a sibilant.
const REFUSED: char = 'ы';

/// The letter the paragraph writes in its place.
const WRITTEN: char = 'и';

/// The letters the paragraph refuses outside loanwords.
///
/// Stated so that a caller which does know a word to be native may ask the
/// stricter question. Nothing in the engine knows that yet, and nothing calls
/// this.
pub const REFUSED_IN_NATIVE: &[char] = &['ю', 'я'];

/// What the paragraph finds in a written word.
///
/// Every sibilant is examined and every breach after one is reported, each
/// with the word as it should be written.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::sibilant_vowels::found;
///
/// assert!(found("жир").is_empty());
/// assert!(found("парашют").is_empty());
///
/// let held = found("жыр");
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].at, 1);
/// assert_eq!(held[0].instead, "жир");
/// ```
#[must_use]
pub fn found(written: &str) -> Findings {
    let letters: std::vec::Vec<char> = written.chars().collect();
    let mut held = Findings::new();

    for (at, letter) in letters.iter().enumerate() {
        if !stem::is_sibilant(*letter) {
            continue;
        }
        let after = at + 1;
        if letters.get(after) != Some(&REFUSED) {
            continue;
        }

        held.push(Found::new(
            CITES,
            after,
            SAYS,
            spelled(written, after, WRITTEN)
        ));
    }

    held
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_paragraph_is_cited() {
        assert_eq!(CITES.paragraph, 1);
        assert!(CITES.is_stated());
    }

    #[test]
    fn a_word_the_paragraph_admits_is_left_alone() {
        for held in ["жир", "чудо", "щука", "шить", "роща", "час"] {
            assert!(found(held).is_empty(), "{held}");
        }
    }

    #[test]
    fn a_loanword_in_ju_is_left_alone() {
        assert!(found("жюри").is_empty());
        assert!(found("парашют").is_empty());
    }

    #[test]
    fn yi_after_a_sibilant_is_found_and_spelled_out() {
        let held = found("жыр");

        assert_eq!(held.len(), 1);
        assert_eq!(held[0].cites, CITES);
        assert_eq!(held[0].at, 1);
        assert_eq!(held[0].says, SAYS);
        assert_eq!(held[0].instead, "жир");
    }

    #[test]
    fn every_breach_of_the_paragraph_is_reported() {
        let held = found("жыршы");

        assert_eq!(held.len(), 2);
        assert_eq!(held[0].at, 1);
        assert_eq!(held[1].at, 4);
    }

    #[test]
    fn yi_after_anything_else_is_not_this_paragraph() {
        assert!(found("быт").is_empty());
        assert!(found("сыр").is_empty());
    }
}
