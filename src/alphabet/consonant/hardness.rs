// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Whether a consonant is hard or soft, and what settles it.
//!
//! Most consonants are neither until the letter after them says so: `нос` and
//! `нёс` differ in the vowel, not in the `н`. Six carry the answer themselves —
//! `ж`, `ш`, `ц` are hard whatever follows, `ч`, `щ`, `й` soft — and those six
//! are why § 1 and § 2 exist: the vowel after them states nothing, so the code
//! has to.

use super::Consonant;

/// Whether a consonant carries its hardness itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Hardness {
    /// Hard whatever follows: `ж`, `ш`, `ц`.
    AlwaysHard,
    /// Soft whatever follows: `ч`, `щ`, `й`.
    AlwaysSoft,
    /// Hard or soft by the letter after it: everything else.
    Paired
}

impl Consonant {
    /// Whether the consonant carries its hardness itself.
    #[must_use]
    #[inline]
    pub const fn hardness(self) -> Hardness {
        match self {
            Self::Zhe | Self::Sha | Self::Tse => Hardness::AlwaysHard,
            Self::Che | Self::Shcha | Self::Glide => Hardness::AlwaysSoft,
            _ => Hardness::Paired
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphabet::consonants;

    #[test]
    fn six_consonants_carry_their_hardness_themselves() {
        let held = consonants()
            .filter(|held| !matches!(held.hardness(), Hardness::Paired))
            .count();

        assert_eq!(held, 6);
    }
}
