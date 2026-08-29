// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Where the stress falls in a written form.
//!
//! Russian is written without stress and cannot be read without it. That much
//! is a fact about reading aloud. What makes stress a grammatical category and
//! not a convenience is the other half: half the orthography hangs on it.
//!
//! § 40 writes `и` in an unstressed ending and `е` otherwise. § 4 writes `о`
//! after a sibilant under stress and `е` without it. § 18, § 35, § 41 — every
//! one of them asks the same question first, and a checker that cannot answer
//! it cannot check any of them.
//!
//! So the place of the stress belongs here, beside case and number, and not in
//! a layer above. What stays outside is only the table of two and a half
//! million words that says where it falls: that is data, and the domain reads
//! no files. Everything a rule needs to ask — where the stress is, whether the
//! ending carries it, whether the word is settled at all — is stated here.

pub mod table;
pub mod text;

pub use self::{
    table::{Placement, Table},
    text::{Marked, Reading, needs_mark, place, place_word}
};

/// Where the stress falls in a form.
///
/// Counted in vowels rather than in letters, because that is what a syllable
/// is and what every rule that asks about stress means. The first vowel of the
/// word is zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Stress {
    /// The stress falls on the vowel at this place, counting from the first.
    On(usize),
    /// The word has one vowel, so the stress has nowhere else to be.
    Fixed,
    /// The word carries no stress of its own: a preposition, a particle, an
    /// enclitic. It leans on the word beside it.
    Leaning
}

impl Stress {
    /// The vowel the stress falls on, absent when the word carries none.
    #[must_use]
    pub const fn vowel(self) -> Option<usize> {
        match self {
            Self::On(held) => Some(held),
            Self::Fixed => Some(0),
            Self::Leaning => None
        }
    }

    /// Reports whether the stress falls on the vowel at a place.
    #[must_use]
    pub const fn falls_on(self, vowel: usize) -> bool {
        match self.vowel() {
            Some(held) => held == vowel,
            None => false
        }
    }
}

/// Where a written form is stressed, and how far that is trusted.
///
/// A spelling may be stressed in more than one place and mean a different word
/// each time — `за́мок` and `замо́к`, `сто́ит` and `стои́т`. The answer is
/// therefore a list, and the caller settles it by the reading it has already
/// chosen. An empty list is not an error: it says the table does not know, and
/// a wrong stress is worse than none.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stressed {
    /// The places the form may be stressed in.
    pub places: Vec<Stress>
}

impl Stressed {
    /// A form whose stress is settled.
    #[must_use]
    pub fn settled(stress: Stress) -> Self {
        Self {
            places: std::vec![stress]
        }
    }

    /// A form the table says nothing about.
    #[must_use]
    pub const fn unknown() -> Self {
        Self {
            places: Vec::new()
        }
    }

    /// Reports whether the stress is settled: one place and no other.
    #[must_use]
    pub const fn is_settled(&self) -> bool {
        self.places.len() == 1
    }

    /// The place the stress falls in, when only one is possible.
    #[must_use]
    pub const fn only(&self) -> Option<Stress> {
        match self.places.as_slice() {
            [held] => Some(*held),
            _ => None
        }
    }

    /// Reports whether the last vowel of a form of this many vowels carries
    /// the stress.
    ///
    /// This is the question the orthography actually asks: § 40 and § 4 do not
    /// care which vowel is stressed, only whether the ending is. A form whose
    /// stress is not settled answers `false`, because a rule must not fire on
    /// a guess.
    #[must_use]
    pub fn ending_stressed(&self, vowels: usize) -> bool {
        let Some(last) = vowels.checked_sub(1) else {
            return false;
        };

        self.only().is_some_and(|held| held.falls_on(last))
    }
}

pub use crate::alphabet::syllables as vowels;

/// Where a written word is stressed, when the spelling alone settles it.
///
/// Two things settle a stress without a table.
///
/// A word of one vowel has nowhere else to put it. A word holding `ё` is
/// stressed on it, because `ё` is written where `е` stands under stress and
/// nowhere else — that letter carries the answer in itself.
///
/// A compound holds more than one `ё` and only the last carries the main
/// stress: `трёхколёсный` is stressed on the second. The earlier ones take a
/// secondary stress, which is not what a rule of the orthography asks about.
///
/// A word the spelling does not settle comes back as [`Stressed::unknown`],
/// and the table is asked instead.
///
/// # Examples
///
/// ```
/// use rusem::phonetics::stress::{Stress, of_spelling};
///
/// assert_eq!(of_spelling("стол").only(), Some(Stress::Fixed));
/// assert_eq!(of_spelling("тёмный").only(), Some(Stress::On(0)));
/// assert_eq!(of_spelling("трёхколёсный").only(), Some(Stress::On(2)));
/// assert!(!of_spelling("вода").is_settled());
/// ```
#[must_use]
pub fn of_spelling(written: &str) -> Stressed {
    let held = vowels(written);
    if held == 0 {
        return Stressed::unknown();
    }
    if held == 1 {
        return Stressed::settled(Stress::Fixed);
    }

    let mut at = None;
    for (vowel, letter) in written
        .chars()
        .filter_map(crate::alphabet::Letter::of)
        .filter_map(crate::alphabet::Letter::vowel)
        .enumerate()
    {
        if letter.is_stressed() {
            at = Some(vowel);
        }
    }

    at.map_or_else(Stressed::unknown, |held| {
        Stressed::settled(Stress::On(held))
    })
}

/// Where a written word is stressed, asking the table only when the spelling
/// does not settle it.
///
/// This is the order the engine should ask in. A word of one vowel and a word
/// holding `ё` answer themselves, and by the count of the table those are a
/// large share of Russian text: every monosyllable, and every form that put
/// its stress on an `е`. The table is opened for what is left.
///
/// The table is passed as a closure rather than as a type, because the domain
/// does not know what a table is — only that something can be asked.
///
/// # Examples
///
/// ```
/// use rusem::phonetics::stress::{Stress, Stressed, of_word};
///
/// let never_asked = |_: &str| {
///     unreachable!("the spelling settled it");
/// };
///
/// assert_eq!(of_word("тёмный", never_asked).only(), Some(Stress::On(0)));
/// assert_eq!(of_word("стол", never_asked).only(), Some(Stress::Fixed));
///
/// let table = |_: &str| Stressed::settled(Stress::On(1));
/// assert_eq!(of_word("вода", table).only(), Some(Stress::On(1)));
/// ```
#[must_use]
pub fn of_word<F>(written: &str, table: F) -> Stressed
where
    F: FnOnce(&str) -> Stressed
{
    let held = of_spelling(written);
    if held.is_settled() {
        return held;
    }

    table(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_place_is_counted_in_vowels() {
        assert_eq!(vowels("вода"), 2);
        assert_eq!(vowels("молоко"), 3);
        assert_eq!(vowels("стол"), 1);
    }

    #[test]
    fn a_leaning_word_carries_no_stress() {
        assert_eq!(Stress::Leaning.vowel(), None);
        assert!(!Stress::Leaning.falls_on(0));
    }

    #[test]
    fn a_one_vowel_word_is_stressed_where_it_can_be() {
        assert_eq!(Stress::Fixed.vowel(), Some(0));
        assert!(Stress::Fixed.falls_on(0));
    }

    #[test]
    fn an_ending_is_stressed_when_the_last_vowel_is() {
        let held = Stressed::settled(Stress::On(1));

        assert!(held.ending_stressed(2));
        assert!(!held.ending_stressed(3));
    }

    #[test]
    fn a_word_of_no_vowels_has_no_stressed_ending() {
        assert!(!Stressed::settled(Stress::On(0)).ending_stressed(0));
    }

    #[test]
    fn a_homograph_settles_nothing_and_fires_nothing() {
        let held = Stressed {
            places: std::vec![Stress::On(0), Stress::On(1)]
        };

        assert!(!held.is_settled());
        assert_eq!(held.only(), None);
        assert!(!held.ending_stressed(2));
    }

    #[test]
    fn a_word_of_one_vowel_is_settled_by_its_spelling() {
        assert_eq!(of_spelling("стол").only(), Some(Stress::Fixed));
        assert_eq!(of_spelling("вплоть").only(), Some(Stress::Fixed));
    }

    #[test]
    fn a_word_holding_jo_is_settled_by_its_spelling() {
        assert_eq!(of_spelling("тёмный").only(), Some(Stress::On(0)));
        assert_eq!(of_spelling("нашёл").only(), Some(Stress::On(1)));
        assert_eq!(of_spelling("ружьё").only(), Some(Stress::On(1)));
    }

    #[test]
    fn a_compound_is_stressed_on_its_last_jo() {
        assert_eq!(of_spelling("трёхколёсный").only(), Some(Stress::On(2)));
    }

    #[test]
    fn a_word_the_spelling_does_not_settle_asks_the_table() {
        assert!(!of_spelling("вода").is_settled());
        assert!(!of_spelling("молоко").is_settled());
    }

    #[test]
    fn a_word_without_vowels_is_settled_by_nothing() {
        assert!(!of_spelling("").is_settled());
        assert!(!of_spelling("вств").is_settled());
    }

    #[test]
    fn the_table_is_not_asked_when_the_spelling_settles_it() {
        let never = |_: &str| {
            unreachable!("the spelling settled it");
        };

        assert_eq!(of_word("тёмный", never).only(), Some(Stress::On(0)));
        assert_eq!(of_word("стол", never).only(), Some(Stress::Fixed));
    }

    #[test]
    fn the_table_is_asked_when_the_spelling_does_not() {
        let table = |_: &str| Stressed::settled(Stress::On(1));

        assert_eq!(of_word("вода", table).only(), Some(Stress::On(1)));
    }

    #[test]
    fn a_word_the_table_does_not_know_fires_nothing() {
        let held = Stressed::unknown();

        assert!(!held.is_settled());
        assert!(!held.ending_stressed(2));
    }
}
