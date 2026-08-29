// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The letters of Russian and what they are.
//!
//! This is the floor everything else stands on. A syllable is a vowel, so
//! stress is counted here. A stem is hard or soft by its last consonant, so
//! declension and conjugation ask here. Half the code of 1956 is about which
//! letter may follow which, so the rules ask here too.
//!
//! Thirty-three letters, unchanged since 1918, and nothing above may hold an
//! opinion of its own about them.
//!
//! # A letter is a type, not a predicate
//!
//! A letter is exactly one of three things — a vowel, a consonant, a sign —
//! and it is written down that way. Nothing has to test that the vowels and
//! the consonants do not overlap, because there is nowhere to write a letter
//! that is both.
//!
//! Everything a letter is asked, it is asked once. [`Letter::of`] reads a
//! character into a letter, folding a capital on the way, and what follows
//! reads facts off the letter rather than matching the character again.
//!
//! | To know | Ask |
//! | --- | --- |
//! | is it a letter at all, and which | [`Letter::of`], or `char::try_into` |
//! | how it is written, small or capital, and where it stands | [`Letter`] |
//! | does the vowel soften what precedes it, is it written under stress, what it pairs with, what a glide before it writes | [`Vowel`] |
//! | voicing and its pair, hardness before a given letter, sibilant, back, sonorant | [`Consonant`] |
//! | does the sign soften or part | [`Sign`] |
//! | which Russian letter a Latin twin is drawn like | [`lookalike`] |
//! | is it a stress mark written over a letter | [`Mark`] |
//!
//! Of a written word: [`spell`] cuts it into letters and refuses anything that
//! is not one, [`syllables`] counts its vowels, [`lookalike::is_mixed`] reports
//! whether two alphabets got into it, and [`mark::bare`] takes the stress marks
//! off before a dictionary is asked.
//!
//! The free functions over `char` beside the types — [`is_vowel`],
//! [`is_sibilant`] and the rest — are the same answers for a caller holding a
//! character rather than a letter. They read the letter and ask it; nothing is
//! stated twice.

pub mod consonant;
pub mod letter;
pub mod lookalike;
pub mod mark;
pub mod sign;
pub mod vowel;

pub use self::{
    consonant::{Consonant, Hardness, Voicing},
    letter::{Letter, folded},
    mark::Mark,
    sign::Sign,
    vowel::Vowel
};

/// The alphabet, every letter of it, in its own order.
///
/// Stated on the letter itself rather than beside it: the list is the type's
/// own inventory, and a caller reaches it as [`Letter::ALL`].
const ALL: &[Letter; 33] = &[
    Letter::Vowel(Vowel::A),
    Letter::Consonant(Consonant::Be),
    Letter::Consonant(Consonant::Ve),
    Letter::Consonant(Consonant::Ge),
    Letter::Consonant(Consonant::De),
    Letter::Vowel(Vowel::Je),
    Letter::Vowel(Vowel::Jo),
    Letter::Consonant(Consonant::Zhe),
    Letter::Consonant(Consonant::Ze),
    Letter::Vowel(Vowel::I),
    Letter::Consonant(Consonant::Glide),
    Letter::Consonant(Consonant::Ka),
    Letter::Consonant(Consonant::El),
    Letter::Consonant(Consonant::Em),
    Letter::Consonant(Consonant::En),
    Letter::Vowel(Vowel::O),
    Letter::Consonant(Consonant::Pe),
    Letter::Consonant(Consonant::Er),
    Letter::Consonant(Consonant::Es),
    Letter::Consonant(Consonant::Te),
    Letter::Vowel(Vowel::U),
    Letter::Consonant(Consonant::Ef),
    Letter::Consonant(Consonant::Ha),
    Letter::Consonant(Consonant::Tse),
    Letter::Consonant(Consonant::Che),
    Letter::Consonant(Consonant::Sha),
    Letter::Consonant(Consonant::Shcha),
    Letter::Sign(Sign::Hard),
    Letter::Vowel(Vowel::Yi),
    Letter::Sign(Sign::Soft),
    Letter::Vowel(Vowel::E),
    Letter::Vowel(Vowel::Ju),
    Letter::Vowel(Vowel::Ja)
];

/// A character that is not a letter of the Russian alphabet.
///
/// Carried by the conversions rather than an empty error, so that a caller
/// refusing a word can say which character it refused over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotALetter(pub char);

impl core::fmt::Display for NotALetter {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "{:?} is no letter of the Russian alphabet",
            self.0
        )
    }
}

impl core::error::Error for NotALetter {}

/// Every letter, in the order of the alphabet.
pub fn letters() -> impl Iterator<Item = Letter> {
    ALL.iter().copied()
}

/// Every vowel, in the order of the alphabet.
pub fn vowels() -> impl Iterator<Item = Vowel> {
    ALL.iter().filter_map(|held| held.vowel())
}

/// Every consonant, in the order of the alphabet.
pub fn consonants() -> impl Iterator<Item = Consonant> {
    ALL.iter().filter_map(|held| held.consonant())
}

/// Reports whether a character is a vowel.
///
/// A capital counts: the alphabet is the same letters however a text writes
/// them.
///
/// # Examples
///
/// ```
/// use rusem::alphabet::is_vowel;
///
/// assert!(is_vowel('о'));
/// assert!(is_vowel('О'));
/// assert!(!is_vowel('т'));
/// ```
#[must_use]
#[inline]
pub const fn is_vowel(letter: char) -> bool {
    matches!(Letter::of(letter), Some(Letter::Vowel(_)))
}

/// Reports whether a character is a consonant.
///
/// # Examples
///
/// ```
/// use rusem::alphabet::is_consonant;
///
/// assert!(is_consonant('т'));
/// assert!(is_consonant('Т'));
/// assert!(!is_consonant('ь'));
/// ```
#[must_use]
#[inline]
pub const fn is_consonant(letter: char) -> bool {
    matches!(Letter::of(letter), Some(Letter::Consonant(_)))
}

/// Reports whether a character is one of the two signs.
#[must_use]
#[inline]
pub const fn is_sign(letter: char) -> bool {
    matches!(Letter::of(letter), Some(Letter::Sign(_)))
}

/// Reports whether a character is a sibilant: `ж`, `ч`, `ш`, `щ`.
#[inline]
#[must_use]
pub const fn is_sibilant(letter: char) -> bool {
    match Letter::of(letter) {
        Some(Letter::Consonant(held)) => held.is_sibilant(),
        _ => false
    }
}

/// Reports whether a character is a back consonant: `г`, `к`, `х`.
#[inline]
#[must_use]
pub const fn is_back(letter: char) -> bool {
    match Letter::of(letter) {
        Some(Letter::Consonant(held)) => held.is_back(),
        _ => false
    }
}

/// Reports whether a character is a sonorant: `л`, `м`, `н`, `р`, `й`.
#[inline]
#[must_use]
pub const fn is_sonorant(letter: char) -> bool {
    match Letter::of(letter) {
        Some(Letter::Consonant(held)) => held.is_sonorant(),
        _ => false
    }
}

/// Reports whether a character refuses `ы` after it (§ 1, § 2).
#[inline]
#[must_use]
pub const fn bars_yi(letter: char) -> bool {
    match Letter::of(letter) {
        Some(Letter::Consonant(held)) => held.bars_yi(),
        _ => false
    }
}

/// Reports whether a character is a vowel that softens what precedes it.
#[inline]
#[must_use]
pub const fn softens(letter: char) -> bool {
    match Letter::of(letter) {
        Some(Letter::Vowel(held)) => held.softens(),
        _ => false
    }
}

/// Reports whether a character belongs to the alphabet at all.
#[inline]
#[must_use]
pub const fn is_letter(letter: char) -> bool {
    Letter::of(letter).is_some()
}

/// The letter a glide and a following vowel are written as together.
///
/// A vowel that already carries the glide comes back as itself. A character
/// that is no vowel, and `ы`, which never stands after a glide, come back as
/// nothing.
///
/// # Examples
///
/// ```
/// use rusem::alphabet::carrying_glide;
///
/// assert_eq!(carrying_glide('у'), Some('ю'));
/// assert_eq!(carrying_glide('ю'), Some('ю'));
/// assert_eq!(carrying_glide('ы'), None);
/// assert_eq!(carrying_glide('т'), None);
/// ```
#[inline]
#[must_use]
pub const fn carrying_glide(letter: char) -> Option<char> {
    match Letter::of(letter) {
        Some(Letter::Vowel(held)) => match held.carrying_glide() {
            Some(merged) => Some(merged.written()),
            None => None
        },
        _ => None
    }
}

/// Counts the vowels of a written word, which is to count its syllables.
///
/// # Examples
///
/// ```
/// use rusem::alphabet::syllables;
///
/// assert_eq!(syllables("вода"), 2);
/// assert_eq!(syllables("Вода"), 2);
/// assert_eq!(syllables("стол"), 1);
/// ```
#[inline]
#[must_use]
pub fn syllables(written: &str) -> usize {
    written.chars().filter(|held| is_vowel(*held)).count()
}

/// Spells a written word out into its letters, one at a time.
///
/// A character that is no letter comes back as [`None`] in its place rather
/// than being dropped: the caller asked for letters and there is something
/// else there, and dropping it silently would let `парашют-2` be read as
/// `парашют`. A caller that wants the whole word or nothing takes
/// [`spelled`].
///
/// Nothing is allocated. The letters arrive as the characters are read.
///
/// # Examples
///
/// ```
/// use rusem::alphabet::spell;
///
/// assert_eq!(spell("вода").count(), 4);
/// assert!(spell("вода").all(|held| held.is_some()));
/// assert!(spell("во-да").any(|held| held.is_none()));
/// ```
#[inline]
pub fn spell(written: &str) -> impl Iterator<Item = Option<Letter>> {
    written.chars().map(Letter::of)
}

/// Spells a written word out, refusing it whole if anything in it is not a
/// letter.
///
/// The collecting half of [`spell`], for a caller that wants the letters in
/// hand rather than one at a time.
///
/// # Examples
///
/// ```
/// use rusem::alphabet::spelled;
///
/// assert_eq!(spelled("вода").map(|held| held.len()), Some(4));
/// assert_eq!(spelled("во-да"), None);
/// ```
#[inline]
#[must_use]
pub fn spelled(written: &str) -> Option<Vec<Letter>> {
    spell(written).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_alphabet_holds_every_letter_once() {
        let mut held = Letter::ALL.to_vec();
        held.sort_unstable();
        held.dedup();

        assert_eq!(held.len(), 33);
    }

    #[test]
    fn the_order_is_not_the_order_of_the_codes() {
        let written: Vec<char> = Letter::ALL.iter().map(|held| held.written()).collect();
        let coded = {
            let mut held = written.clone();
            held.sort_unstable();
            held
        };

        assert_ne!(coded, written);
    }

    #[test]
    fn the_vowels_and_the_consonants_are_walked_in_order() {
        assert_eq!(vowels().count(), 10);
        assert_eq!(consonants().count(), 21);
        assert_eq!(vowels().next(), Some(Vowel::A));
        assert_eq!(consonants().next(), Some(Consonant::Be));
    }

    #[test]
    fn a_capital_is_read_as_its_small_letter() {
        assert!(is_vowel('О'));
        assert!(is_consonant('Щ'));
        assert!(is_letter('Ё'));
        assert_eq!(syllables("Вода"), 2);
    }

    #[test]
    fn a_letter_of_another_alphabet_is_nothing_here() {
        assert!(!is_letter('a'));
        assert!(!is_vowel('a'));
        assert!(!is_consonant('b'));
    }

    #[test]
    fn the_counts_of_each_kind_are_what_the_alphabet_states() {
        let held = Letter::ALL.iter().filter(|letter| letter.is_sign()).count();

        assert_eq!(vowels().count(), 10);
        assert_eq!(consonants().count(), 21);
        assert_eq!(held, 2);
        assert_eq!(vowels().count() + consonants().count() + held, 33);
    }

    #[test]
    fn a_word_of_letters_is_spelled_and_one_with_anything_else_is_not() {
        assert_eq!(spelled("вода").map(|held| held.len()), Some(4));
        assert_eq!(spelled("Вода").map(|held| held.len()), Some(4));
        assert_eq!(spelled("во-да"), None);
        assert_eq!(spelled("вода2"), None);
        assert_eq!(spell("вода").count(), 4);
        assert!(spell("во-да").any(|held| held.is_none()));
    }

    #[test]
    fn the_standard_conversions_hold_both_ways() {
        for held in Letter::ALL {
            let written = char::from(*held);

            assert_eq!(Letter::try_from(written), Ok(*held));
            assert_eq!(held.to_string(), written.to_string());
        }
    }

    #[test]
    fn a_conversion_names_the_character_it_refused() {
        assert_eq!(Letter::try_from('a'), Err(NotALetter('a')));
        assert_eq!(
            NotALetter('a').to_string(),
            "'a' is no letter of the Russian alphabet"
        );
    }

    #[test]
    fn the_alphabet_is_walked_as_letters() {
        assert_eq!(letters().count(), 33);
        assert_eq!(letters().next(), Some(Letter::Vowel(Vowel::A)));
    }

    #[test]
    fn the_predicates_agree_with_the_types() {
        for held in Letter::ALL {
            let written = held.written();

            assert_eq!(is_vowel(written), held.is_vowel());
            assert_eq!(is_consonant(written), held.is_consonant());
            assert_eq!(is_sign(written), held.is_sign());
            assert!(is_letter(written));
        }
    }

    #[test]
    fn every_question_over_a_character_answers_as_the_letter_does() {
        for held in Letter::ALL {
            let written = held.written();
            let consonant = held.consonant();

            assert_eq!(
                is_sibilant(written),
                consonant.is_some_and(Consonant::is_sibilant)
            );
            assert_eq!(is_back(written), consonant.is_some_and(Consonant::is_back));
            assert_eq!(
                is_sonorant(written),
                consonant.is_some_and(Consonant::is_sonorant)
            );
            assert_eq!(bars_yi(written), consonant.is_some_and(Consonant::bars_yi));
            assert_eq!(softens(written), held.vowel().is_some_and(Vowel::softens));
            assert_eq!(
                carrying_glide(written),
                held.vowel()
                    .and_then(Vowel::carrying_glide)
                    .map(Vowel::written)
            );
        }
    }

    #[test]
    fn a_character_that_is_no_letter_answers_no_to_everything() {
        for held in ['q', '-', '0', ' '] {
            assert!(!is_vowel(held));
            assert!(!is_consonant(held));
            assert!(!is_sign(held));
            assert!(!is_sibilant(held));
            assert!(!is_back(held));
            assert!(!is_sonorant(held));
            assert!(!bars_yi(held));
            assert!(!softens(held));
            assert!(!is_letter(held));
            assert_eq!(carrying_glide(held), None);
        }
    }

    #[test]
    fn a_capital_answers_as_its_small_letter() {
        for held in Letter::ALL {
            let capital = held.capital();

            assert_eq!(is_sibilant(capital), is_sibilant(held.written()));
            assert_eq!(bars_yi(capital), bars_yi(held.written()));
            assert_eq!(softens(capital), softens(held.written()));
            assert_eq!(carrying_glide(capital), carrying_glide(held.written()));
        }
    }
}
