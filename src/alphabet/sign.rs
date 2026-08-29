// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The two signs, which stand for no sound and are letters all the same.
//!
//! The soft sign says the consonant before it is soft — `конь` against `кон` —
//! and after a sibilant it says nothing about the sound at all, standing only
//! to mark the form: `рожь`, `режь`, `настежь` (§ 58).
//!
//! The hard sign parts a prefix from a soft vowel and stops the vowel from
//! softening what came before: `съесть`, `объехать`, `предъявить` (§ 70).

/// One of the two signs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Sign {
    /// `ь`, which softens the consonant before it.
    Soft,
    /// `ъ`, which parts a prefix from a soft vowel.
    Hard
}

impl Sign {
    /// The sign a small letter is, or nothing when it is not one.
    #[must_use]
    #[inline]
    pub const fn of(letter: char) -> Option<Self> {
        Some(match letter {
            'ь' => Self::Soft,
            'ъ' => Self::Hard,
            _ => return None
        })
    }

    /// How the sign is written.
    #[must_use]
    #[inline]
    pub const fn written(self) -> char {
        match self {
            Self::Soft => 'ь',
            Self::Hard => 'ъ'
        }
    }

    /// Reports whether the sign softens the consonant before it.
    #[must_use]
    #[inline]
    pub const fn softens(self) -> bool {
        matches!(self, Self::Soft)
    }

    /// Reports whether the sign parts what stands before it from what follows.
    #[must_use]
    #[inline]
    pub const fn parts(self) -> bool {
        matches!(self, Self::Hard)
    }
}

impl TryFrom<char> for Sign {
    type Error = crate::alphabet::NotALetter;

    fn try_from(letter: char) -> Result<Self, Self::Error> {
        Self::of(crate::alphabet::folded(letter)).ok_or(crate::alphabet::NotALetter(letter))
    }
}

impl From<Sign> for char {
    fn from(held: Sign) -> Self {
        held.written()
    }
}

impl core::fmt::Display for Sign {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{}", self.written())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_signs_read_back_as_themselves() {
        assert_eq!(Sign::of('ь'), Some(Sign::Soft));
        assert_eq!(Sign::of('ъ'), Some(Sign::Hard));
        assert_eq!(Sign::of('о'), None);
    }

    #[test]
    fn each_sign_does_one_thing_and_not_the_other() {
        assert!(Sign::Soft.softens());
        assert!(!Sign::Soft.parts());
        assert!(Sign::Hard.parts());
        assert!(!Sign::Hard.softens());
    }

    #[test]
    fn a_sign_is_written_as_it_was_read() {
        for held in [Sign::Soft, Sign::Hard] {
            assert_eq!(Sign::of(held.written()), Some(held));
        }
    }

    #[test]
    fn the_standard_conversions_hold_both_ways() {
        for held in [Sign::Soft, Sign::Hard] {
            let written = char::from(held);

            assert_eq!(Sign::try_from(written), Ok(held));
            assert_eq!(held.to_string(), written.to_string());
        }
    }

    #[test]
    fn a_conversion_names_what_it_refused() {
        assert_eq!(Sign::try_from('q'), Err(crate::alphabet::NotALetter('q')));
    }
}
