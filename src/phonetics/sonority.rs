// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! How loud a sound is, which is what settles where a syllable ends.
//!
//! A syllable of Russian is a wave of sonority rising to a vowel. Where the
//! wave falls, the syllable ends. That is the whole of the law, and every rule
//! of syllable division below is a consequence of it rather than a rule of its
//! own.
//!
//! Four steps, as Avanesov numbered them: a vowel is the loudest, then the
//! sonorants, then the voiced obstruents, and the voiceless are the quietest.
//! The signs have no sound and take no step.

use crate::alphabet::{Letter, Voicing};

/// How loud a sound is, on the four-step scale.
///
/// Ordered: a vowel is greater than a sonorant, a sonorant than a voiced
/// obstruent, and that than a voiceless one. The ordering is what the law of
/// rising sonority is stated in, so it is derived rather than written out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Sonority {
    /// A voiceless obstruent: `п`, `ф`, `к`, `т`, `ш`, `с`, `х`, `ц`, `ч`, `щ`.
    Voiceless,
    /// A voiced obstruent: `б`, `в`, `г`, `д`, `ж`, `з`.
    Voiced,
    /// A sonorant: `л`, `м`, `н`, `р`, `й`.
    Sonorant,
    /// A vowel, which is what a syllable is built around.
    Vowel
}

impl Sonority {
    /// How loud a letter is, or nothing when it stands for no sound.
    ///
    /// The hard and soft signs answer nothing: they say something about the
    /// consonant beside them and are not sounds themselves.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::{alphabet::Letter, phonetics::Sonority};
    ///
    /// let o = Letter::of('о').expect("a letter");
    /// let el = Letter::of('л').expect("a letter");
    /// let te = Letter::of('т').expect("a letter");
    /// let soft = Letter::of('ь').expect("a letter");
    ///
    /// assert_eq!(Sonority::of(o), Some(Sonority::Vowel));
    /// assert!(Sonority::of(el) > Sonority::of(te));
    /// assert_eq!(Sonority::of(soft), None);
    /// ```
    #[must_use]
    #[inline]
    pub const fn of(letter: Letter) -> Option<Self> {
        Some(match letter {
            Letter::Vowel(_) => Self::Vowel,
            Letter::Consonant(held) if held.is_sonorant() => Self::Sonorant,
            Letter::Consonant(held) => match held.voicing() {
                Voicing::Voiced => Self::Voiced,
                Voicing::Voiceless => Self::Voiceless
            },
            Letter::Sign(_) => return None
        })
    }

    /// Reports whether the sound can be the peak a syllable is built around.
    ///
    /// Only a vowel can in Russian. A language where a sonorant may — Czech
    /// `vlk`, Serbian `srce` — would answer differently here and nowhere else.
    #[must_use]
    #[inline]
    pub const fn is_peak(self) -> bool {
        matches!(self, Self::Vowel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphabet::Letter;

    fn of(letter: char) -> Option<Sonority> {
        Sonority::of(Letter::of(letter).expect("a letter"))
    }

    #[test]
    fn the_scale_rises_from_the_voiceless_to_the_vowel() {
        assert!(Sonority::Voiceless < Sonority::Voiced);
        assert!(Sonority::Voiced < Sonority::Sonorant);
        assert!(Sonority::Sonorant < Sonority::Vowel);
    }

    #[test]
    fn every_letter_that_sounds_has_a_loudness() {
        for held in Letter::ALL {
            assert_eq!(Sonority::of(*held).is_some(), !held.is_sign());
        }
    }

    #[test]
    fn a_vowel_is_the_loudest_and_the_only_peak() {
        assert_eq!(of('о'), Some(Sonority::Vowel));
        assert_eq!(of('я'), Some(Sonority::Vowel));
        assert!(Sonority::Vowel.is_peak());
        assert!(!Sonority::Sonorant.is_peak());
    }

    #[test]
    fn a_sonorant_is_louder_than_any_obstruent() {
        for held in ['л', 'м', 'н', 'р', 'й'] {
            assert_eq!(of(held), Some(Sonority::Sonorant));
            assert!(of(held) > of('б'));
            assert!(of(held) > of('п'));
        }
    }

    #[test]
    fn a_voiced_obstruent_is_louder_than_a_voiceless_one() {
        assert_eq!(of('б'), Some(Sonority::Voiced));
        assert_eq!(of('п'), Some(Sonority::Voiceless));
        assert!(of('б') > of('п'));
        assert!(of('з') > of('с'));
    }

    #[test]
    fn a_sign_has_no_loudness() {
        assert_eq!(of('ь'), None);
        assert_eq!(of('ъ'), None);
    }
}
