// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! How a letter is written, small or capital.
//!
//! The alphabet is the same thirty-three letters in either size, so the size is
//! a way of writing rather than a letter of its own. Both are named rather than
//! computed: a shift of the character code is right for thirty-two of them and
//! wrong for `ё`, which Unicode puts elsewhere.

use super::Letter;
use crate::alphabet::{Consonant, Sign, Vowel};

impl Letter {
    /// How the letter is written, as a small letter.
    #[must_use]
    #[inline]
    pub const fn written(self) -> char {
        match self {
            Self::Vowel(held) => held.written(),
            Self::Consonant(held) => held.written(),
            Self::Sign(held) => held.written()
        }
    }

    /// How the letter is written as a capital.
    ///
    /// The alphabet is the same thirty-three letters in either size, so this
    /// is a way of writing rather than a letter of its own. It is stated here
    /// because a proper name, a sentence and a restored word all need it, and
    /// none of them should be folding character codes by hand.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::alphabet::Letter;
    ///
    /// let jo = Letter::of('ё').expect("a letter");
    /// let a = Letter::of('а').expect("a letter");
    ///
    /// assert_eq!(jo.capital(), 'Ё');
    /// assert_eq!(a.capital(), 'А');
    /// ```
    #[must_use]
    #[inline]
    pub const fn capital(self) -> char {
        match self {
            Self::Vowel(Vowel::A) => 'А',
            Self::Consonant(Consonant::Be) => 'Б',
            Self::Consonant(Consonant::Ve) => 'В',
            Self::Consonant(Consonant::Ge) => 'Г',
            Self::Consonant(Consonant::De) => 'Д',
            Self::Vowel(Vowel::Je) => 'Е',
            Self::Vowel(Vowel::Jo) => 'Ё',
            Self::Consonant(Consonant::Zhe) => 'Ж',
            Self::Consonant(Consonant::Ze) => 'З',
            Self::Vowel(Vowel::I) => 'И',
            Self::Consonant(Consonant::Glide) => 'Й',
            Self::Consonant(Consonant::Ka) => 'К',
            Self::Consonant(Consonant::El) => 'Л',
            Self::Consonant(Consonant::Em) => 'М',
            Self::Consonant(Consonant::En) => 'Н',
            Self::Vowel(Vowel::O) => 'О',
            Self::Consonant(Consonant::Pe) => 'П',
            Self::Consonant(Consonant::Er) => 'Р',
            Self::Consonant(Consonant::Es) => 'С',
            Self::Consonant(Consonant::Te) => 'Т',
            Self::Vowel(Vowel::U) => 'У',
            Self::Consonant(Consonant::Ef) => 'Ф',
            Self::Consonant(Consonant::Ha) => 'Х',
            Self::Consonant(Consonant::Tse) => 'Ц',
            Self::Consonant(Consonant::Che) => 'Ч',
            Self::Consonant(Consonant::Sha) => 'Ш',
            Self::Consonant(Consonant::Shcha) => 'Щ',
            Self::Sign(Sign::Hard) => 'Ъ',
            Self::Vowel(Vowel::Yi) => 'Ы',
            Self::Sign(Sign::Soft) => 'Ь',
            Self::Vowel(Vowel::E) => 'Э',
            Self::Vowel(Vowel::Ju) => 'Ю',
            Self::Vowel(Vowel::Ja) => 'Я'
        }
    }

    /// How the letter is written, in the size asked for.
    #[must_use]
    #[inline]
    pub const fn written_as(self, capital: bool) -> char {
        if capital {
            self.capital()
        } else {
            self.written()
        }
    }
}
