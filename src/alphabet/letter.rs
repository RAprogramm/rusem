// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! A letter of the Russian alphabet, and what kind of letter it is.
//!
//! Thirty-three letters, each exactly one of three things: a vowel, a
//! consonant, or a sign. Stating that as a type rather than as three
//! predicates is what makes it true — a letter cannot be a vowel and a
//! consonant at once because there is nowhere to write that down, and no test
//! has to hold the two lists in step.
//!
//! Case is settled at the door. `Letter::of` folds a capital to its small
//! letter, so nothing above ever has to remember to, and nothing answers
//! `false` about `О` because it was written at the start of a sentence.

pub mod kind;
pub mod writing;

use super::{consonant::Consonant, sign::Sign, vowel::Vowel};

/// One letter of the alphabet.
///
/// The order is the alphabet's, not the order the variants are written in. A
/// derived ordering would put every vowel before every consonant and sort
/// `абя` as `аяб`, which is not what a dictionary does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Letter {
    /// A vowel.
    Vowel(Vowel),
    /// A consonant.
    Consonant(Consonant),
    /// A hard or soft sign, which stands for no sound of its own.
    Sign(Sign)
}

impl Letter {
    /// The alphabet, every letter of it, in its own order.
    ///
    /// This is the one list. The letters are values rather than characters, so
    /// a walk over the alphabet is a walk over the type, and how each is
    /// written is read off the letter rather than stated a second time.
    ///
    /// The order is the alphabet's and not the character codes': `ё` stands
    /// sixth, between `е` and `ж`, and Unicode puts it after `я`. A dictionary
    /// is ordered by this.
    pub const ALL: &'static [Self; 33] = super::ALL;

    /// The letter a character is, or nothing when it is not one.
    ///
    /// A capital is folded to its small letter: the alphabet is the same
    /// thirty-three letters whatever a text does with their size, and where
    /// the capital matters — a proper name, the start of a sentence — that is
    /// a fact about the word rather than about the letter.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::alphabet::{Letter, Vowel};
    ///
    /// assert_eq!(Letter::of('о'), Some(Letter::Vowel(Vowel::O)));
    /// assert_eq!(Letter::of('О'), Some(Letter::Vowel(Vowel::O)));
    /// assert_eq!(Letter::of('a'), None);
    /// ```
    #[must_use]
    #[inline]
    pub const fn of(letter: char) -> Option<Self> {
        let small = folded(letter);

        if let Some(held) = Vowel::of(small) {
            return Some(Self::Vowel(held));
        }
        if let Some(held) = Consonant::of(small) {
            return Some(Self::Consonant(held));
        }

        match Sign::of(small) {
            Some(held) => Some(Self::Sign(held)),
            None => None
        }
    }

    /// The place of the letter in the alphabet, counting `а` as zero.
    ///
    /// This is what orders a dictionary, and it is not the order of the
    /// character codes: `ё` stands sixth, between `е` and `ж`, and Unicode
    /// puts it after `я`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::alphabet::Letter;
    ///
    /// let je = Letter::of('е').expect("a letter");
    /// let jo = Letter::of('ё').expect("a letter");
    /// let zhe = Letter::of('ж').expect("a letter");
    ///
    /// assert_eq!(jo.place(), je.place() + 1);
    /// assert_eq!(zhe.place(), jo.place() + 1);
    /// ```
    #[must_use]
    #[inline]
    pub const fn place(self) -> usize {
        match self {
            Self::Vowel(Vowel::A) => 0,
            Self::Consonant(Consonant::Be) => 1,
            Self::Consonant(Consonant::Ve) => 2,
            Self::Consonant(Consonant::Ge) => 3,
            Self::Consonant(Consonant::De) => 4,
            Self::Vowel(Vowel::Je) => 5,
            Self::Vowel(Vowel::Jo) => 6,
            Self::Consonant(Consonant::Zhe) => 7,
            Self::Consonant(Consonant::Ze) => 8,
            Self::Vowel(Vowel::I) => 9,
            Self::Consonant(Consonant::Glide) => 10,
            Self::Consonant(Consonant::Ka) => 11,
            Self::Consonant(Consonant::El) => 12,
            Self::Consonant(Consonant::Em) => 13,
            Self::Consonant(Consonant::En) => 14,
            Self::Vowel(Vowel::O) => 15,
            Self::Consonant(Consonant::Pe) => 16,
            Self::Consonant(Consonant::Er) => 17,
            Self::Consonant(Consonant::Es) => 18,
            Self::Consonant(Consonant::Te) => 19,
            Self::Vowel(Vowel::U) => 20,
            Self::Consonant(Consonant::Ef) => 21,
            Self::Consonant(Consonant::Ha) => 22,
            Self::Consonant(Consonant::Tse) => 23,
            Self::Consonant(Consonant::Che) => 24,
            Self::Consonant(Consonant::Sha) => 25,
            Self::Consonant(Consonant::Shcha) => 26,
            Self::Sign(Sign::Hard) => 27,
            Self::Vowel(Vowel::Yi) => 28,
            Self::Sign(Sign::Soft) => 29,
            Self::Vowel(Vowel::E) => 30,
            Self::Vowel(Vowel::Ju) => 31,
            Self::Vowel(Vowel::Ja) => 32
        }
    }
}

impl Ord for Letter {
    /// Letters order as a dictionary orders them.
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.place().cmp(&other.place())
    }
}

impl PartialOrd for Letter {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<char> for Letter {
    type Error = super::NotALetter;

    fn try_from(letter: char) -> Result<Self, Self::Error> {
        Self::of(letter).ok_or(super::NotALetter(letter))
    }
}

impl From<Letter> for char {
    fn from(held: Letter) -> Self {
        held.written()
    }
}

impl core::fmt::Display for Letter {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{}", self.written())
    }
}

/// The small letter a character is written as.
///
/// Only the Russian capitals are folded, and each is named rather than
/// computed: a shift of the character code would be right for thirty-two of
/// the thirty-three and wrong for `Ё`, which Unicode puts elsewhere. Latin `A`
/// is not a Russian letter however it is folded, and folding it would let it
/// be mistaken for one.
#[inline]
#[must_use]
pub const fn folded(letter: char) -> char {
    match letter {
        'А' => 'а',
        'Б' => 'б',
        'В' => 'в',
        'Г' => 'г',
        'Д' => 'д',
        'Е' => 'е',
        'Ё' => 'ё',
        'Ж' => 'ж',
        'З' => 'з',
        'И' => 'и',
        'Й' => 'й',
        'К' => 'к',
        'Л' => 'л',
        'М' => 'м',
        'Н' => 'н',
        'О' => 'о',
        'П' => 'п',
        'Р' => 'р',
        'С' => 'с',
        'Т' => 'т',
        'У' => 'у',
        'Ф' => 'ф',
        'Х' => 'х',
        'Ц' => 'ц',
        'Ч' => 'ч',
        'Ш' => 'ш',
        'Щ' => 'щ',
        'Ъ' => 'ъ',
        'Ы' => 'ы',
        'Ь' => 'ь',
        'Э' => 'э',
        'Ю' => 'ю',
        'Я' => 'я',
        held => held
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_capital_is_the_same_letter_as_its_small() {
        assert_eq!(Letter::of('О'), Letter::of('о'));
        assert_eq!(Letter::of('Ё'), Letter::of('ё'));
        assert_eq!(Letter::of('Щ'), Letter::of('щ'));
        assert_eq!(Letter::of('Ь'), Letter::of('ь'));
    }

    #[test]
    fn a_letter_of_another_alphabet_is_no_letter() {
        assert_eq!(Letter::of('a'), None);
        assert_eq!(Letter::of('A'), None);
        assert_eq!(Letter::of('-'), None);
        assert_eq!(Letter::of(' '), None);
    }

    #[test]
    fn every_letter_is_exactly_one_kind() {
        for held in Letter::ALL {
            let kinds = usize::from(held.is_vowel())
                + usize::from(held.is_consonant())
                + usize::from(held.is_sign());

            assert_eq!(kinds, 1, "{held} is not exactly one kind");
        }
    }

    #[test]
    fn every_letter_reads_back_as_itself() {
        for held in Letter::ALL {
            assert_eq!(Letter::of(held.written()), Some(*held));
        }
    }

    #[test]
    fn the_place_is_the_order_of_the_alphabet_and_not_of_unicode() {
        let jo = Letter::of('ё').expect("a letter").place();
        let je = Letter::of('е').expect("a letter").place();
        let zhe = Letter::of('ж').expect("a letter").place();

        assert_eq!(jo, je + 1);
        assert_eq!(zhe, jo + 1);
    }

    #[test]
    fn the_place_of_every_letter_is_its_own() {
        for (at, held) in Letter::ALL.iter().enumerate() {
            assert_eq!(held.place(), at);
        }
    }

    #[test]
    fn every_letter_is_written_in_either_size_and_reads_back() {
        for held in Letter::ALL {
            assert_eq!(Letter::of(held.capital()), Some(*held));
            assert_eq!(held.written_as(true), held.capital());
            assert_eq!(held.written_as(false), held.written());
            assert_ne!(held.capital(), held.written());
        }
    }

    #[test]
    fn letters_sort_as_a_dictionary_sorts_them() {
        let mut held: Vec<Letter> = "яба".chars().filter_map(Letter::of).collect();
        held.sort_unstable();

        let written: String = held.iter().map(|letter| letter.written()).collect();

        assert_eq!(written, "абя");
    }

    #[test]
    fn the_whole_alphabet_sorts_into_its_own_order() {
        let mut held = Letter::ALL.to_vec();
        held.reverse();
        held.sort_unstable();

        assert_eq!(held.as_slice(), Letter::ALL.as_slice());
    }

    #[test]
    fn jo_sorts_between_je_and_zhe_and_not_after_ja() {
        let je = Letter::of('е').expect("a letter");
        let jo = Letter::of('ё').expect("a letter");
        let zhe = Letter::of('ж').expect("a letter");
        let ja = Letter::of('я').expect("a letter");

        assert!(je < jo);
        assert!(jo < zhe);
        assert!(jo < ja);
    }
}
