// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Which of the three kinds a letter is.
//!
//! A vowel, a consonant or a sign, and never two at once. The type says so and
//! these read it back: the questions are asked often enough to be worth a name,
//! and none of them can disagree with another, because all three read the same
//! value.

use super::Letter;
use crate::alphabet::{Consonant, Hardness, Vowel};

impl Letter {
    /// The vowel this letter is, when it is one.
    #[must_use]
    #[inline]
    pub const fn vowel(self) -> Option<Vowel> {
        match self {
            Self::Vowel(held) => Some(held),
            _ => None
        }
    }

    /// The consonant this letter is, when it is one.
    #[must_use]
    #[inline]
    pub const fn consonant(self) -> Option<Consonant> {
        match self {
            Self::Consonant(held) => Some(held),
            _ => None
        }
    }

    /// Reports whether the letter is a vowel.
    #[must_use]
    #[inline]
    pub const fn is_vowel(self) -> bool {
        matches!(self, Self::Vowel(_))
    }

    /// Reports whether the letter is a consonant.
    #[must_use]
    #[inline]
    pub const fn is_consonant(self) -> bool {
        matches!(self, Self::Consonant(_))
    }

    /// Reports whether the letter is a sign.
    #[must_use]
    #[inline]
    pub const fn is_sign(self) -> bool {
        matches!(self, Self::Sign(_))
    }
}

/// Whether a letter stands soft, which needs the letter after it.
///
/// Kept apart from the kind above: what a letter *is* it knows alone, and how
/// it *stands* it does not.
impl Letter {
    /// Reports whether this letter is a consonant that stands soft before the
    /// letter after it.
    ///
    /// A paired consonant carries no hardness of its own: `нос` and `нёс`
    /// differ in nothing but the vowel after the `н`. So the question cannot
    /// be answered by the consonant alone, and what follows is asked for.
    ///
    /// Nothing following means the end of the word, where a paired consonant
    /// is hard: `кон` against `конь`. A letter that is no consonant answers
    /// `false` — it has no hardness to report.
    ///
    /// Asked here rather than of the consonant, because the answer is about a
    /// letter in a word and the consonant knows nothing of letters.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::alphabet::Letter;
    ///
    /// let en = Letter::of('н').expect("a letter");
    ///
    /// assert!(!en.is_soft_before(Letter::of('о')));
    /// assert!(en.is_soft_before(Letter::of('ё')));
    /// assert!(!en.is_soft_before(None));
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_soft_before(self, next: Option<Self>) -> bool {
        let Self::Consonant(held) = self else {
            return false;
        };

        match held.hardness() {
            Hardness::AlwaysHard => false,
            Hardness::AlwaysSoft => true,
            Hardness::Paired => match next {
                Some(Self::Vowel(vowel)) => vowel.softens(),
                Some(Self::Sign(sign)) => sign.softens(),
                _ => false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_paired_consonant_is_soft_by_what_follows_it() {
        let en = Letter::of('н').expect("a letter");

        assert!(!en.is_soft_before(Letter::of('о')));
        assert!(en.is_soft_before(Letter::of('ё')));
        assert!(en.is_soft_before(Letter::of('ь')));
        assert!(!en.is_soft_before(Letter::of('ъ')));
        assert!(!en.is_soft_before(Letter::of('ы')));
        assert!(!en.is_soft_before(None));
    }

    #[test]
    fn a_consonant_that_carries_its_hardness_ignores_what_follows() {
        let zhe = Letter::of('ж').expect("a letter");
        let che = Letter::of('ч').expect("a letter");

        for next in [Letter::of('о'), Letter::of('ё'), None] {
            assert!(!zhe.is_soft_before(next));
            assert!(che.is_soft_before(next));
        }
    }

    #[test]
    fn a_letter_that_is_no_consonant_reports_no_hardness() {
        for held in Letter::ALL.iter().filter(|held| !held.is_consonant()) {
            assert!(!held.is_soft_before(Letter::of('ё')));
        }
    }
}
