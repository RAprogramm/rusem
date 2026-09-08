// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 5. В русских словах после ж, ч, ш, щ в безударных слогах о не пишется.
//!
//! `горошек` against `петушок`, `сторожем` against `чижом`, `большего` against
//! `большого`. The letter written in the unstressed syllable is `е`.
//!
//! The paragraph says «in Russian words», and that is the one thing this rule
//! cannot see: `шофёр` and `жокей` are loanwords and keep their `о`. So the
//! caller says whether the word is native, and a caller that does not know
//! gets silence rather than a refusal.
//!
//! This is the paragraph that made the stress worth wiring in. Without it the
//! rule cannot be asked at all: every `о` after a sibilant is right in one
//! syllable and wrong in the next, and only the stress tells them apart.

use crate::{
    grammar::stem,
    phonetics::stress::Stressed,
    rules::{Citation, Findings, Found, Scope, found::spelled, scope}
};

/// The paragraph as a rule: what it cites and what it is about.
///
/// Holds the citation and the scope together so the engine can list the
/// paragraph alongside the others. The judging function [`found`] stays
/// free.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::unstressed_o::Rule;
///
/// assert_eq!(Rule::CITES.paragraph, 5);
/// ```
pub struct Rule;

impl Rule {
    /// Where this rule is written.
    pub const CITES: Citation = Citation::whole(5);

    /// What this rule is about.
    pub const SCOPE: Scope = Scope {
        parts:   &[],
        cases:   &[],
        numbers: &[],
        needs:   scope::Needs {
            stress: true,
            parts:  false
        }
    };
}

/// What the paragraph says when it is broken.
const SAYS: &str = "в неударяемом слоге после шипящей пишется е, а не о";

/// What the paragraph finds in a written word.
///
/// It judges only what it can: a borrowing is outside it, and a word whose
/// stress is unknown cannot be judged at all, since the paragraph is about the
/// unstressed syllable. Both come back with nothing found.
///
/// # Examples
///
/// ```
/// use rusem::{
///     phonetics::stress::{Stress, Stressed},
///     rules::svod::unstressed_o::found
/// };
///
/// let third = Stressed::settled(Stress::On(2));
/// assert!(found("петушок", true, &third).is_empty());
///
/// let first = Stressed::settled(Stress::On(1));
/// let held = found("горошок", true, &first);
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].at, 5);
/// assert_eq!(held[0].instead, "горошек");
/// ```
#[must_use]
pub fn found(written: &str, native: bool, stress: &Stressed) -> Findings {
    let mut held = Findings::new();
    if !native || !stress.is_settled() {
        return held;
    }

    let letters: std::vec::Vec<char> = written.chars().collect();
    let mut vowel = 0_usize;

    for (at, letter) in letters.iter().enumerate() {
        if !crate::alphabet::is_vowel(*letter) {
            continue;
        }
        let place = vowel;
        vowel += 1;

        if *letter != REFUSED {
            continue;
        }
        if !at
            .checked_sub(1)
            .and_then(|before| letters.get(before))
            .is_some_and(|before| stem::is_sibilant(*before))
        {
            continue;
        }
        if stress.only().is_some_and(|one| one.falls_on(place)) {
            continue;
        }

        held.push(Found::new(
            Rule::CITES,
            at,
            SAYS,
            spelled(written, at, WRITTEN)
        ));
    }

    held
}

/// The letter the paragraph refuses in an unstressed syllable.
const REFUSED: char = 'о';

/// The letter it writes there instead.
const WRITTEN: char = 'е';

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phonetics::stress::Stress;

    #[test]
    fn the_paragraph_is_cited() {
        assert_eq!(Rule::CITES.paragraph, 5);
        assert!(Rule::CITES.is_stated());
    }

    #[test]
    fn a_stressed_o_after_a_sibilant_is_left_alone() {
        let third = Stressed::settled(Stress::On(2));

        assert!(found("петушок", true, &third).is_empty());
    }

    #[test]
    fn an_unstressed_o_after_a_sibilant_is_found_and_spelled_out() {
        let first = Stressed::settled(Stress::On(1));
        let held = found("горошок", true, &first);

        assert_eq!(held.len(), 1);
        assert_eq!(held[0].cites, Rule::CITES);
        assert_eq!(held[0].at, 5);
        assert_eq!(held[0].instead, "горошек");
    }

    #[test]
    fn a_borrowing_is_outside_the_paragraph() {
        let first = Stressed::settled(Stress::On(1));

        assert!(found("горошок", false, &first).is_empty());
    }

    #[test]
    fn a_word_whose_stress_is_unknown_is_not_judged() {
        assert!(found("горошок", true, &Stressed::unknown()).is_empty());
    }

    #[test]
    fn an_o_after_anything_else_is_not_this_paragraph() {
        let first = Stressed::settled(Stress::On(1));

        assert!(found("голова", true, &first).is_empty());
    }
}
