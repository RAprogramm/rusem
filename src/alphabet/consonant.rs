// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The twenty-one consonants, and the three things Russian asks of each.
//!
//! **Voicing.** Six pairs — `б`/`п`, `в`/`ф`, `г`/`к`, `д`/`т`, `ж`/`ш`,
//! `з`/`с` — differ in nothing but the voice. That is why a voiced consonant
//! at the end of a word is written as itself and read as its pair (`дуб`
//! sounds `дуп`), and why the prefixes on `з` are written `с` before a
//! voiceless consonant (§ 47).
//!
//! **Hardness.** Most consonants are hard or soft by the letter that follows.
//! Five are neither: `ж`, `ш`, `ц` are hard whatever follows, `ч`, `щ` soft,
//! and `й` soft. Those five are why § 1 and § 2 exist at all — the vowel after
//! them states nothing, so the code has to.
//!
//! **Place.** The back consonants `г`, `к`, `х` refuse `ы` after them and swap
//! before a front vowel: `пеку` — `печёшь`. The sibilants `ж`, `ч`, `ш`, `щ`
//! are named as a set by half the paragraphs of the first chapter. The
//! sonorants `л`, `м`, `н`, `р`, `й` are what a cluster may end in without a
//! fleeting vowel parting it.

pub mod articulation;
pub mod hardness;
pub mod voicing;

pub use self::{hardness::Hardness, voicing::Voicing};

/// One of the twenty-one consonants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Consonant {
    /// `б`
    Be,
    /// `в`
    Ve,
    /// `г`
    Ge,
    /// `д`
    De,
    /// `ж`
    Zhe,
    /// `з`
    Ze,
    /// `й`
    Glide,
    /// `к`
    Ka,
    /// `л`
    El,
    /// `м`
    Em,
    /// `н`
    En,
    /// `п`
    Pe,
    /// `р`
    Er,
    /// `с`
    Es,
    /// `т`
    Te,
    /// `ф`
    Ef,
    /// `х`
    Ha,
    /// `ц`
    Tse,
    /// `ч`
    Che,
    /// `ш`
    Sha,
    /// `щ`
    Shcha
}

impl Consonant {
    /// The consonant a small letter is, or nothing when it is not one.
    #[must_use]
    #[inline]
    pub const fn of(letter: char) -> Option<Self> {
        Some(match letter {
            'б' => Self::Be,
            'в' => Self::Ve,
            'г' => Self::Ge,
            'д' => Self::De,
            'ж' => Self::Zhe,
            'з' => Self::Ze,
            'й' => Self::Glide,
            'к' => Self::Ka,
            'л' => Self::El,
            'м' => Self::Em,
            'н' => Self::En,
            'п' => Self::Pe,
            'р' => Self::Er,
            'с' => Self::Es,
            'т' => Self::Te,
            'ф' => Self::Ef,
            'х' => Self::Ha,
            'ц' => Self::Tse,
            'ч' => Self::Che,
            'ш' => Self::Sha,
            'щ' => Self::Shcha,
            _ => return None
        })
    }

    /// How the consonant is written.
    #[must_use]
    #[inline]
    pub const fn written(self) -> char {
        match self {
            Self::Be => 'б',
            Self::Ve => 'в',
            Self::Ge => 'г',
            Self::De => 'д',
            Self::Zhe => 'ж',
            Self::Ze => 'з',
            Self::Glide => 'й',
            Self::Ka => 'к',
            Self::El => 'л',
            Self::Em => 'м',
            Self::En => 'н',
            Self::Pe => 'п',
            Self::Er => 'р',
            Self::Es => 'с',
            Self::Te => 'т',
            Self::Ef => 'ф',
            Self::Ha => 'х',
            Self::Tse => 'ц',
            Self::Che => 'ч',
            Self::Sha => 'ш',
            Self::Shcha => 'щ'
        }
    }
}

impl TryFrom<char> for Consonant {
    type Error = crate::alphabet::NotALetter;

    fn try_from(letter: char) -> Result<Self, Self::Error> {
        Self::of(crate::alphabet::folded(letter)).ok_or(crate::alphabet::NotALetter(letter))
    }
}

impl From<Consonant> for char {
    fn from(held: Consonant) -> Self {
        held.written()
    }
}

impl core::fmt::Display for Consonant {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{}", self.written())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphabet::consonants;

    fn all() -> Vec<Consonant> {
        consonants().collect()
    }

    #[test]
    fn every_consonant_reads_back_as_itself() {
        for held in all() {
            assert_eq!(Consonant::of(held.written()), Some(held));
        }
    }

    #[test]
    fn the_alphabet_holds_twenty_one_consonants() {
        assert_eq!(all().len(), 21);
    }

    #[test]
    fn the_sibilants_and_the_back_consonants_bar_yi() {
        for held in all() {
            assert_eq!(held.bars_yi(), held.is_sibilant() || held.is_back());
        }
        assert!(Consonant::Zhe.bars_yi());
        assert!(Consonant::Ka.bars_yi());
        assert!(!Consonant::Te.bars_yi());
    }

    #[test]
    fn no_consonant_is_both_a_sibilant_and_a_back_one() {
        assert!(
            !all()
                .iter()
                .any(|held| held.is_sibilant() && held.is_back())
        );
    }

    #[test]
    fn the_standard_conversions_hold_both_ways() {
        for held in consonants() {
            let written = char::from(held);

            assert_eq!(Consonant::try_from(written), Ok(held));
            assert_eq!(held.to_string(), written.to_string());
        }
    }

    #[test]
    fn a_conversion_names_what_it_refused() {
        assert_eq!(
            Consonant::try_from('q'),
            Err(crate::alphabet::NotALetter('q'))
        );
    }
}
