// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Where a consonant is made, which is what the code of 1956 keeps asking.
//!
//! The sibilants `ж`, `ч`, `ш`, `щ` open half the paragraphs of the first
//! chapter. The back consonants `г`, `к`, `х` refuse the same vowel after them
//! and swap before a front one: `пеку` — `печёшь`. The sonorants `л`, `м`,
//! `н`, `р`, `й` are what a cluster may end in without a fleeting vowel
//! parting it.

use super::Consonant;

impl Consonant {
    /// Reports whether the consonant is a sibilant: `ж`, `ч`, `ш`, `щ`.
    #[must_use]
    #[inline]
    pub const fn is_sibilant(self) -> bool {
        matches!(self, Self::Zhe | Self::Che | Self::Sha | Self::Shcha)
    }

    /// Reports whether the consonant is a back one: `г`, `к`, `х`.
    #[must_use]
    #[inline]
    pub const fn is_back(self) -> bool {
        matches!(self, Self::Ge | Self::Ka | Self::Ha)
    }

    /// Reports whether the consonant is a sonorant: `л`, `м`, `н`, `р`, `й`.
    #[must_use]
    #[inline]
    pub const fn is_sonorant(self) -> bool {
        matches!(
            self,
            Self::El | Self::Em | Self::En | Self::Er | Self::Glide
        )
    }

    /// Reports whether the consonant refuses `ы` after it.
    ///
    /// The sibilants and the back consonants, by § 1 and § 2. This is the one
    /// question the first chapter of the code asks most often.
    #[must_use]
    #[inline]
    pub const fn bars_yi(self) -> bool {
        self.is_sibilant() || self.is_back()
    }
}
