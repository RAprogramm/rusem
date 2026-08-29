// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! How a stem ends, which is what decides the shape of the ending after it.
//!
//! Russian states one set of endings and writes it two ways: `столом` beside
//! `конём`, `читают` beside `строят`. The difference is not in the ending but
//! in what the stem ends with, so it is stated once here and both paradigms —
//! the declension of a noun and the conjugation of a verb — ask the same
//! question of it.

use crate::alphabet::{Hardness, Letter};

/// How a stem ends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Stem {
    /// The stem ends hard, and the endings open on а, о, у, ы.
    Hard,
    /// The stem ends soft, and the same endings open on я, е, ю, и.
    Soft
}

impl Stem {
    /// Names how a stem ends, given its last letter.
    ///
    /// The letter alone settles it for the consonants that carry their
    /// softness themselves — `ч`, `щ` and `й` are soft wherever they stand,
    /// `ж`, `ш` and `ц` hard — and for the soft sign and the soft vowels,
    /// which are written precisely to state a soft stem. Everything else is
    /// hard until a following letter says otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::grammar::stem::Stem;
    ///
    /// assert_eq!(Stem::of('т'), Stem::Hard);
    /// assert_eq!(Stem::of('ь'), Stem::Soft);
    /// assert_eq!(Stem::of('ч'), Stem::Soft);
    /// assert_eq!(Stem::of('ш'), Stem::Hard);
    /// ```
    #[must_use]
    pub const fn of(last: char) -> Self {
        match Letter::of(last) {
            Some(Letter::Consonant(held)) => match held.hardness() {
                Hardness::AlwaysSoft => Self::Soft,
                Hardness::AlwaysHard | Hardness::Paired => Self::Hard
            },
            Some(Letter::Vowel(held)) if held.softens() => Self::Soft,
            Some(Letter::Sign(held)) if held.softens() => Self::Soft,
            _ => Self::Hard
        }
    }

    /// Reports whether the stem takes soft-shaped endings.
    #[must_use]
    pub const fn is_soft(self) -> bool {
        matches!(self, Self::Soft)
    }
}

/// Reports whether a letter is a sibilant.
///
/// A sibilant is neither hard nor soft for the purpose of the ending: it takes
/// the hard vowel in some cells and the soft one in others, and the paradigms
/// that care ask for it by name. Stated by the alphabet and re-exported here,
/// where the paradigms already look.
pub use crate::alphabet::is_sibilant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_consonant_that_carries_its_softness_settles_itself() {
        assert_eq!(Stem::of('ч'), Stem::Soft);
        assert_eq!(Stem::of('щ'), Stem::Soft);
        assert_eq!(Stem::of('ж'), Stem::Hard);
        assert_eq!(Stem::of('ц'), Stem::Hard);
    }

    #[test]
    fn the_soft_sign_and_the_soft_vowels_state_a_soft_stem() {
        assert_eq!(Stem::of('ь'), Stem::Soft);
        assert_eq!(Stem::of('я'), Stem::Soft);
        assert_eq!(Stem::of('ё'), Stem::Soft);
    }

    #[test]
    fn everything_else_is_hard() {
        assert_eq!(Stem::of('т'), Stem::Hard);
        assert_eq!(Stem::of('а'), Stem::Hard);
    }

    #[test]
    fn the_sibilants_are_named() {
        assert!(is_sibilant('ж'));
        assert!(is_sibilant('щ'));
        assert!(!is_sibilant('ц'));
    }
}
