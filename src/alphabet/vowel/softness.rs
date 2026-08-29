// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What a vowel says about the consonant before it.
//!
//! Russian writes softness with the vowel and not with the consonant: `нос`
//! and `нёс` differ in one letter, and that letter is the vowel. Five vowels
//! state a soft consonant before them, five state a hard one, and they pair
//! off — `а`/`я`, `о`/`ё`, `у`/`ю`, `э`/`е`, `ы`/`и`.
//!
//! Four of the soft ones carry a glide as well: `я` is `й` and `а` written
//! together. `и` is the odd one — soft without a glide of its own, though it
//! absorbs one standing before it.

use super::Vowel;

impl Vowel {
    /// Reports whether the vowel states that the consonant before it is soft.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::alphabet::Vowel;
    ///
    /// assert!(Vowel::Ja.softens());
    /// assert!(!Vowel::A.softens());
    /// ```
    #[must_use]
    #[inline]
    pub const fn softens(self) -> bool {
        matches!(self, Self::Ja | Self::Je | Self::Jo | Self::Ju | Self::I)
    }

    /// The vowel that stands for the same sound after a consonant of the other
    /// hardness.
    ///
    /// `а` gives `я` and `я` gives `а`; `ы` gives `и`. The pairing is its own
    /// inverse, so asking twice comes back where it started.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::alphabet::Vowel;
    ///
    /// assert_eq!(Vowel::A.paired(), Vowel::Ja);
    /// assert_eq!(Vowel::Ja.paired(), Vowel::A);
    /// assert_eq!(Vowel::Yi.paired(), Vowel::I);
    /// ```
    #[must_use]
    #[inline]
    pub const fn paired(self) -> Self {
        match self {
            Self::A => Self::Ja,
            Self::Ja => Self::A,
            Self::O => Self::Jo,
            Self::Jo => Self::O,
            Self::U => Self::Ju,
            Self::Ju => Self::U,
            Self::E => Self::Je,
            Self::Je => Self::E,
            Self::Yi => Self::I,
            Self::I => Self::Yi
        }
    }

    /// The vowel a glide and this vowel are written as together.
    ///
    /// Four vowels carry a glide in them: `я` is `й` and `а`, `ю` is `й` and
    /// `у`, `е` is `й` and `э`, `ё` is `й` and `о`. A stem ending in the glide
    /// loses it before them — `читай-` and `-у` are written `читаю` — and `и`
    /// absorbs it the same way, without changing: `строй-` and `-ит` are
    /// written `строит`.
    ///
    /// `ы` answers nothing. It has no iotated form and never stands after a
    /// glide; the pairing that turns it into `и` is the one about hardness,
    /// and using it here would invent a spelling Russian does not have.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::alphabet::Vowel;
    ///
    /// assert_eq!(Vowel::U.carrying_glide(), Some(Vowel::Ju));
    /// assert_eq!(Vowel::Ju.carrying_glide(), Some(Vowel::Ju));
    /// assert_eq!(Vowel::I.carrying_glide(), Some(Vowel::I));
    /// assert_eq!(Vowel::Yi.carrying_glide(), None);
    /// ```
    #[must_use]
    #[inline]
    pub const fn carrying_glide(self) -> Option<Self> {
        Some(match self {
            Self::A | Self::Ja => Self::Ja,
            Self::U | Self::Ju => Self::Ju,
            Self::E | Self::Je => Self::Je,
            Self::O | Self::Jo => Self::Jo,
            Self::I => Self::I,
            Self::Yi => return None
        })
    }
}
