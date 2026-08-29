// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Whether a consonant is voiced, and what it pairs with.
//!
//! Six pairs differ in nothing but the voice: `б`/`п`, `в`/`ф`, `г`/`к`,
//! `д`/`т`, `ж`/`ш`, `з`/`с`. That is why a voiced consonant at the end of a
//! word is written as itself and read as its pair — `дуб` sounds `дуп` — and
//! why the prefixes on `з` are written `с` before a voiceless consonant
//! (§ 47).
//!
//! The sonorants are voiced and pair with nothing: Russian writes no voiceless
//! `л`.

use super::Consonant;

/// Whether a consonant is voiced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Voicing {
    /// Voiced: `б`, `в`, `г`, `д`, `ж`, `з` and the sonorants.
    Voiced,
    /// Voiceless: `п`, `ф`, `к`, `т`, `ш`, `с`, `х`, `ц`, `ч`, `щ`.
    Voiceless
}

impl Consonant {
    /// Whether the consonant is voiced.
    ///
    /// The sonorants are voiced and have no voiceless pair: nothing in Russian
    /// is written for a voiceless `л`.
    #[must_use]
    #[inline]
    pub const fn voicing(self) -> Voicing {
        match self {
            Self::Be
            | Self::Ve
            | Self::Ge
            | Self::De
            | Self::Zhe
            | Self::Ze
            | Self::Glide
            | Self::El
            | Self::Em
            | Self::En
            | Self::Er => Voicing::Voiced,
            _ => Voicing::Voiceless
        }
    }

    /// The consonant that differs from this one in the voice alone.
    ///
    /// A sonorant, `х`, `ц`, `ч` and `щ` have none, and come back as
    /// themselves. The pairing is its own inverse.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::alphabet::Consonant;
    ///
    /// assert_eq!(Consonant::Be.voiced_pair(), Consonant::Pe);
    /// assert_eq!(Consonant::Pe.voiced_pair(), Consonant::Be);
    /// assert_eq!(Consonant::El.voiced_pair(), Consonant::El);
    /// ```
    #[must_use]
    #[inline]
    pub const fn voiced_pair(self) -> Self {
        match self {
            Self::Be => Self::Pe,
            Self::Pe => Self::Be,
            Self::Ve => Self::Ef,
            Self::Ef => Self::Ve,
            Self::Ge => Self::Ka,
            Self::Ka => Self::Ge,
            Self::De => Self::Te,
            Self::Te => Self::De,
            Self::Zhe => Self::Sha,
            Self::Sha => Self::Zhe,
            Self::Ze => Self::Es,
            Self::Es => Self::Ze,
            held => held
        }
    }

    /// Reports whether the consonant has a pair differing only in the voice.
    #[must_use]
    #[inline]
    pub const fn is_paired_by_voice(self) -> bool {
        !matches!(
            self,
            Self::Glide
                | Self::El
                | Self::Em
                | Self::En
                | Self::Er
                | Self::Ha
                | Self::Tse
                | Self::Che
                | Self::Shcha
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphabet::consonants;

    /// The six pairs the grammars state, written out to hold the match above
    /// against something other than itself.
    const PAIRED: &[(Consonant, Consonant)] = &[
        (Consonant::Be, Consonant::Pe),
        (Consonant::Ve, Consonant::Ef),
        (Consonant::Ge, Consonant::Ka),
        (Consonant::De, Consonant::Te),
        (Consonant::Zhe, Consonant::Sha),
        (Consonant::Ze, Consonant::Es)
    ];

    #[test]
    fn the_pairs_are_the_six_the_grammars_state() {
        for (voiced, voiceless) in PAIRED {
            assert_eq!(voiced.voiced_pair(), *voiceless);
            assert_eq!(voiceless.voiced_pair(), *voiced);
        }

        let held = consonants()
            .filter(|held| held.is_paired_by_voice())
            .count();

        assert_eq!(held, PAIRED.len() * 2);
    }

    #[test]
    fn the_pairing_is_its_own_inverse() {
        for held in consonants() {
            assert_eq!(held.voiced_pair().voiced_pair(), held);
        }
    }

    #[test]
    fn a_pair_holds_one_voiced_consonant_and_one_voiceless() {
        for held in consonants().filter(|held| held.is_paired_by_voice()) {
            assert_ne!(held.voicing(), held.voiced_pair().voicing());
        }
    }

    #[test]
    fn a_sonorant_is_voiced_and_has_no_pair() {
        for held in consonants().filter(|held| held.is_sonorant()) {
            assert_eq!(held.voicing(), Voicing::Voiced);
            assert!(!held.is_paired_by_voice());
        }
    }
}
