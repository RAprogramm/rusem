// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The Latin letters drawn with the same glyph as a Russian one.
//!
//! `о` and `o` are the same shape and different letters. A scanned page, a
//! keyboard switched mid-word, a name copied from a foreign document — any of
//! them puts a Latin letter inside a Russian word, and no reader notices. The
//! analyzer does: the word is simply not in the dictionary, and nothing says
//! why.
//!
//! Which glyphs coincide is a fact about the alphabets, so it is stated here
//! rather than in whatever layer happens to trip over it. Whether to restore a
//! word is a decision, and that stays with the caller: `сор` and `cop` are both
//! words, in different languages, and only the context settles which was meant.
//!
//! The pairing is not symmetrical in the small letters. Latin `y` is drawn like
//! Russian `у`, but Latin `p` is drawn like Russian `р` — the sounds have
//! nothing to do with each other, and a table built on sound would be wrong
//! here. This one is built on shape alone.

use super::Letter;

/// The Latin letters that share a glyph with a Russian one, in pairs.
///
/// Written out to hold the match above against something other than itself.
/// Small letters first, then capitals. `в`, `н`, `к`, `м`, `т` have Latin
/// twins among the capitals only: `B`, `H`, `K`, `M`, `T` are drawn like them,
/// while `b`, `h`, `k`, `m`, `t` are not drawn like `в`, `н`, `к`, `м`, `т`.
#[cfg(test)]
const PAIRS: &[(char, char)] = &[
    ('a', 'а'),
    ('c', 'с'),
    ('e', 'е'),
    ('o', 'о'),
    ('p', 'р'),
    ('x', 'х'),
    ('y', 'у'),
    ('A', 'а'),
    ('B', 'в'),
    ('C', 'с'),
    ('E', 'е'),
    ('H', 'н'),
    ('K', 'к'),
    ('M', 'м'),
    ('O', 'о'),
    ('P', 'р'),
    ('T', 'т'),
    ('X', 'х'),
    ('Y', 'у')
];

/// The Russian letter a Latin character is drawn like, if any.
///
/// # Examples
///
/// ```
/// use rusem::alphabet::{Letter, Vowel, lookalike};
///
/// assert_eq!(lookalike::behind('o'), Some(Letter::Vowel(Vowel::O)));
/// assert_eq!(lookalike::behind('O'), Some(Letter::Vowel(Vowel::O)));
/// assert_eq!(lookalike::behind('z'), None);
/// assert_eq!(lookalike::behind('о'), None);
/// ```
#[inline]
#[must_use]
pub const fn behind(latin: char) -> Option<Letter> {
    Letter::of(match latin {
        'a' | 'A' => 'а',
        'B' => 'в',
        'c' | 'C' => 'с',
        'e' | 'E' => 'е',
        'H' => 'н',
        'K' => 'к',
        'M' => 'м',
        'o' | 'O' => 'о',
        'p' | 'P' => 'р',
        'T' => 'т',
        'x' | 'X' => 'х',
        'y' | 'Y' => 'у',
        _ => return None
    })
}

/// Reports whether a character is a Latin letter drawn like a Russian one.
#[inline]
#[must_use]
pub const fn is_masked(latin: char) -> bool {
    behind(latin).is_some()
}

/// The word with every masked letter restored.
///
/// The size is kept: a capital Latin letter becomes a capital Russian one, so
/// `BOДA` comes back as `ВОДА` and not as `воДа`. Anything else in the word is
/// left as it stands, and a word holding no masked letter comes back unchanged.
///
/// # Examples
///
/// ```
/// use rusem::alphabet::lookalike::unmasked;
///
/// assert_eq!(unmasked("вoда"), "вода");
/// assert_eq!(unmasked("BOДA"), "ВОДА");
/// assert_eq!(unmasked("вода"), "вода");
/// assert_eq!(unmasked("code"), "соdе", "d has no twin and stays Latin");
/// ```
#[inline]
#[must_use]
pub fn unmasked(word: &str) -> String {
    word.chars()
        .map(|held| behind(held).map_or(held, |letter| letter.written_as(held.is_uppercase())))
        .collect()
}

/// Reports whether a word mixes the two alphabets.
///
/// This is the question worth asking before restoring anything: a word written
/// wholly in Latin is a foreign word and none of the engine's business, and a
/// word written wholly in Russian needs nothing done to it. It is the mixture
/// that is a typing accident.
///
/// The Latin side is the basic Latin alphabet, where every twin of this
/// module lives. A letter of some third alphabet — Greek, or a Ukrainian
/// vowel — is neither Russian nor Latin, so a word holding one is not a
/// mixture of these two and is not reported as one.
///
/// # Examples
///
/// ```
/// use rusem::alphabet::lookalike::is_mixed;
///
/// assert!(is_mixed("вoда"));
/// assert!(!is_mixed("вода"));
/// assert!(!is_mixed("code"));
/// assert!(!is_mixed("стоλ"));
/// ```
#[inline]
#[must_use]
pub fn is_mixed(word: &str) -> bool {
    let russian = word.chars().any(|held| Letter::of(held).is_some());
    let latin = word.chars().any(|held| held.is_ascii_alphabetic());

    russian && latin
}

/// Reports whether a word becomes a word of Russian letters once the masking
/// is undone.
///
/// A word that does not is not a masked Russian word, whatever it holds:
/// `вoda` mixes the alphabets and stays mixed after the swap, so restoring it
/// would invent a word nobody wrote.
///
/// # Examples
///
/// ```
/// use rusem::alphabet::lookalike::is_restorable;
///
/// assert!(is_restorable("вoда"));
/// assert!(!is_restorable("вoda"));
/// ```
#[inline]
#[must_use]
pub fn is_restorable(word: &str) -> bool {
    is_mixed(word)
        && word
            .chars()
            .all(|held| Letter::of(held).is_some() || is_masked(held))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphabet::Vowel;

    #[test]
    fn a_latin_twin_names_the_russian_letter_behind_it() {
        assert_eq!(behind('o'), Some(Letter::Vowel(Vowel::O)));
        assert_eq!(behind('O'), Some(Letter::Vowel(Vowel::O)));
        assert_eq!(behind('p'), Letter::of('р'));
        assert_eq!(behind('H'), Letter::of('н'));
    }

    #[test]
    fn a_latin_letter_with_no_twin_names_nothing() {
        assert_eq!(behind('z'), None);
        assert_eq!(behind('q'), None);
        assert_eq!(behind('b'), None);
    }

    #[test]
    fn a_russian_letter_is_not_a_twin_of_itself() {
        for held in Letter::ALL {
            assert_eq!(behind(held.written()), None);
        }
    }

    #[test]
    fn every_twin_names_the_letter_the_table_states() {
        for (latin, russian) in PAIRS {
            assert_eq!(
                behind(*latin),
                Letter::of(*russian),
                "{latin} is drawn like {russian}"
            );
            assert!(is_masked(*latin));
            assert_eq!(Letter::of(*latin), None, "{latin} is no Russian letter");
        }
    }

    #[test]
    fn the_table_and_the_match_name_the_same_twins() {
        let held = (0_u32..0x0250)
            .filter_map(char::from_u32)
            .filter(|letter| is_masked(*letter))
            .count();

        assert_eq!(held, PAIRS.len());
    }

    #[test]
    fn a_masked_word_is_restored_and_a_clean_one_is_untouched() {
        assert_eq!(unmasked("вoда"), "вода");
        assert_eq!(unmasked("вода"), "вода");
        assert_eq!(unmasked("стол"), "стол");
    }

    #[test]
    fn the_size_of_a_restored_letter_is_kept() {
        assert_eq!(unmasked("BOДA"), "ВОДА");
        assert_eq!(unmasked("Вoда"), "Вода");
        assert_eq!(unmasked("вoда"), "вода");
    }

    #[test]
    fn a_mixed_word_is_named_as_mixed() {
        assert!(is_mixed("вoда"));
        assert!(!is_mixed("вода"));
        assert!(!is_mixed("code"));
        assert!(!is_mixed(""));
    }

    #[test]
    fn a_third_alphabet_is_not_taken_for_latin() {
        assert!(!is_mixed("стоλ"), "Greek is neither of the two alphabets");
        assert!(!is_mixed("стіл"), "a wholly Ukrainian word mixes nothing");
    }

    #[test]
    fn a_word_that_stays_mixed_after_the_swap_is_not_restorable() {
        assert!(is_restorable("вoда"));
        assert!(!is_restorable("вoda"));
        assert!(!is_restorable("вода"));
    }

    #[test]
    fn restoring_a_restorable_word_leaves_only_russian_letters() {
        let held = unmasked("вoда");

        assert!(held.chars().all(|letter| Letter::of(letter).is_some()));
    }
}
