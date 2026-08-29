// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Where a word may part into syllables.
//!
//! Two questions are asked of a word and they are not the same question.
//!
//! **Where does it part?** One answer, and the law of rising sonority gives it:
//! a syllable rises to its vowel, so the boundary falls where the rise begins.
//! `би-тва`, `кар-ман`, `се-мья`. That is [`parts`].
//!
//! **Where *may* it part?** Several answers. The school accepts `со-лнце` and
//! `сол-нце`, `кла-ссный` and `клас-сный`, and § 119 of the code offers
//! `буль-он` and `бу-льон` as both correct. That is [`places`], and it is what
//! a line break reads: § 117 asks for syllables and then forbids several of
//! them, so a hyphenation rule takes every place and refuses some.
//!
//! One thing holds in both and is not negotiable: `ь`, `ъ` and `й` are never
//! torn from what stands before them.
//!
//! Three things hold whatever the tradition. A syllable has exactly one vowel.
//! A sonorant before an obstruent stays behind, because the wave has already
//! fallen — `пол-ка`, `чай-ка`. And a sign goes with the consonant it speaks
//! of: it is no sound and cannot open a syllable.
//!
//! This is the phonetic division and not the one a line break uses. § 117 asks
//! for syllables and then forbids several of them, so a hyphenation rule reads
//! this and refuses some of what it offers.

use crate::{alphabet::Letter, phonetics::sonority::Sonority};

/// One syllable of a word, as a range of letters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Syllable {
    /// Where the syllable starts, counting letters from zero.
    pub from: usize,
    /// Where it ends, one past its last letter.
    pub upto: usize
}

impl Syllable {
    /// How many letters the syllable holds.
    #[must_use]
    #[inline]
    pub const fn len(self) -> usize {
        self.upto.saturating_sub(self.from)
    }

    /// Reports whether the syllable holds no letters at all.
    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// The letters of the syllable, cut out of the word it came from.
    #[must_use]
    #[inline]
    pub fn of(self, letters: &[Letter]) -> &[Letter] {
        letters.get(self.from..self.upto).unwrap_or(&[])
    }
}

/// Every place a word may part, in order.
///
/// A place is where a syllable could begin. [`breaks`] takes the one the law
/// gives; this offers every one the school allows and a line break may use.
///
/// # Examples
///
/// ```
/// use rusem::{alphabet::spelled, phonetics::syllable};
///
/// let held = spelled("битва").expect("letters");
/// assert_eq!(syllable::places(&held), vec![2, 3]);
///
/// let milk = spelled("молоко").expect("letters");
/// assert_eq!(syllable::places(&milk), vec![2, 4]);
/// ```
#[must_use]
pub fn places(letters: &[Letter]) -> Vec<usize> {
    (1..letters.len())
        .filter(|at| offered_at(letters, *at))
        .collect()
}

/// The places the law itself takes, in order.
///
/// One place to a syllable: the first that the rise allows, since a syllable
/// holds exactly one vowel and a second break inside the same cluster would
/// cut where there is nothing to cut.
///
/// # Examples
///
/// ```
/// use rusem::{alphabet::spelled, phonetics::syllable::breaks};
///
/// let held = spelled("битва").expect("letters");
/// assert_eq!(breaks(&held), vec![2]);
/// ```
#[must_use]
pub fn breaks(letters: &[Letter]) -> Vec<usize> {
    if !letters.iter().any(|held| held.is_vowel()) {
        return Vec::new();
    }

    let mut held: Vec<usize> = Vec::new();
    for at in 1..letters.len() {
        if !taken_at(letters, at) {
            continue;
        }
        if held
            .last()
            .is_some_and(|before| !parted_by_a_vowel(letters, *before, at))
        {
            continue;
        }
        held.push(at);
    }

    held
}

/// The syllables of a word.
#[must_use]
pub fn parts(letters: &[Letter]) -> Vec<Syllable> {
    if !letters.iter().any(|held| held.is_vowel()) {
        return Vec::new();
    }

    let mut held = Vec::new();
    let mut from = 0_usize;
    for at in breaks(letters) {
        held.push(Syllable {
            from,
            upto: at
        });
        from = at;
    }
    held.push(Syllable {
        from,
        upto: letters.len()
    });

    held
}

/// The syllables of a word, written out.
///
/// # Examples
///
/// ```
/// use rusem::{alphabet::spelled, phonetics::syllable::written};
///
/// let held = spelled("битва").expect("letters");
/// assert_eq!(written(&held), vec!["би", "тва"]);
/// ```
#[must_use]
pub fn written(letters: &[Letter]) -> Vec<String> {
    parts(letters)
        .into_iter()
        .map(|held| {
            held.of(letters)
                .iter()
                .map(|letter| letter.written())
                .collect()
        })
        .collect()
}

/// Reports whether a vowel stands between two places.
///
/// A syllable has exactly one vowel, so two breaks with no vowel between them
/// would cut where there is nothing to cut.
fn parted_by_a_vowel(letters: &[Letter], from: usize, upto: usize) -> bool {
    letters
        .get(from..upto)
        .is_some_and(|held| held.iter().any(|letter| letter.is_vowel()))
}

/// Reports whether the sign before a place closes the syllable there.
///
/// The hard sign always does: it stands after a prefix and parts it from what
/// follows — `подъ-езд`, `объ-явление`.
///
/// The soft sign does when a consonant follows it — `боль-шой` — and does not
/// when a vowel does. There it is the parting sign, and what it parts belongs
/// to one syllable: `се-мья`, not `семь-я`.
fn closes(letters: &[Letter], at: usize) -> bool {
    let before = at.checked_sub(1).and_then(|held| letters.get(held));

    match before {
        Some(Letter::Sign(held)) if held.softens() => {
            letters.get(at).is_none_or(|next| !next.is_vowel())
        }
        _ => true
    }
}

/// The consonants standing between a place and the vowel after it.
///
/// Empty when a vowel stands at the place itself, which is the commonest
/// break of all — two vowels running are two syllables.
fn run(letters: &[Letter], at: usize) -> Option<Vec<Letter>> {
    if !letters.get(..at)?.iter().any(|held| held.is_vowel()) {
        return None;
    }
    let rest = letters.get(at..)?;
    if !rest.iter().any(|held| held.is_vowel()) {
        return None;
    }

    Some(
        rest.iter()
            .take_while(|held| !held.is_vowel())
            .copied()
            .collect()
    )
}

/// Reports whether the sonority rises along a run.
///
/// This is the law itself. A syllable rises to its vowel, so the consonants
/// opening it must not fall: `тв` in `битва` rises from a voiceless to a
/// voiced and may open one, `лк` in `полка` falls and may not.
///
/// Two sounds of equal loudness are a rise when they are obstruents — `ст` in
/// `сестра` opens a syllable, `се-стра` — or when they are the same letter
/// written twice: `лл` in `аллея` is one sound and opens one, `а-ллея`.
///
/// Two different sonorants are not: the wave has already reached them, and the
/// first stays behind. `кар-ман`, not `ка-рман`.
fn rises(held: &[Letter]) -> bool {
    let sounded: Vec<(Letter, Sonority)> = held
        .iter()
        .filter_map(|letter| Sonority::of(*letter).map(|loud| (*letter, loud)))
        .collect();

    sounded.windows(2).all(|pair| {
        let ((first_letter, first), (second_letter, second)) = (pair[0], pair[1]);

        first < second
            || (first == second && (first < Sonority::Sonorant || first_letter == second_letter))
    })
}

/// Reports whether the law puts a syllable boundary at a place.
///
/// A sign may not open one: it stands for no sound and belongs to the
/// consonant before it. It closes a syllable instead, so whatever follows a
/// sign opens the next — `подъ-езд`, `боль-шой` — and a cluster holding a sign
/// cannot be crossed.
///
/// A vowel may open a syllable, but only after another vowel: a consonant is
/// never parted from the vowel after it (§ 118).
fn taken_at(letters: &[Letter], at: usize) -> bool {
    let (Some(before), Some(here)) = (
        at.checked_sub(1).and_then(|held| letters.get(held)),
        letters.get(at)
    ) else {
        return false;
    };
    if here.is_sign() {
        return false;
    }
    if before.is_sign() {
        return closes(letters, at) && run(letters, at).is_some();
    }
    if here.is_vowel() {
        return before.is_vowel() && run(letters, at).is_some();
    }

    let Some(held) = run(letters, at) else {
        return false;
    };
    if held
        .iter()
        .enumerate()
        .any(|(step, letter)| letter.is_sign() && closes(letters, at + step + 1))
    {
        return false;
    }
    rises(&held)
}

/// Reports whether a syllable may begin at a place at all.
///
/// Laxer than [`taken_at`]: a cluster between vowels may be parted anywhere
/// that leaves a vowel on either side and does not tear a sign or a glide from
/// what stands before it. That is what the school accepts and what a line
/// break reads.
fn offered_at(letters: &[Letter], at: usize) -> bool {
    if taken_at(letters, at) {
        return true;
    }
    let (Some(before), Some(here)) = (
        at.checked_sub(1).and_then(|held| letters.get(held)),
        letters.get(at)
    ) else {
        return false;
    };
    if here.is_sign() || matches!(here.consonant(), Some(crate::alphabet::Consonant::Glide)) {
        return false;
    }
    if here.is_vowel() && !before.is_vowel() {
        return false;
    }

    run(letters, at).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphabet::spelled;

    fn cut(word: &str) -> Vec<String> {
        written(&spelled(word).expect("letters"))
    }

    #[test]
    fn a_consonant_between_vowels_opens_the_next_syllable() {
        assert_eq!(cut("молоко"), ["мо", "ло", "ко"]);
        assert_eq!(cut("ракета"), ["ра", "ке", "та"]);
        assert_eq!(cut("окно"), ["о", "кно"]);
    }

    #[test]
    fn a_rising_cluster_opens_a_syllable_whole() {
        assert_eq!(cut("битва"), ["би", "тва"]);
        assert_eq!(cut("водный"), ["во", "дный"]);
        assert_eq!(cut("сестра"), ["се", "стра"]);
        assert_eq!(cut("утро"), ["у", "тро"]);
    }

    #[test]
    fn a_sonorant_before_a_quieter_sound_stays_behind() {
        assert_eq!(cut("полка"), ["пол", "ка"]);
        assert_eq!(cut("чайка"), ["чай", "ка"]);
        assert_eq!(cut("парта"), ["пар", "та"]);
    }

    #[test]
    fn two_different_sonorants_part() {
        assert_eq!(cut("карман"), ["кар", "ман"]);
    }

    #[test]
    fn one_sound_written_twice_opens_a_syllable() {
        assert_eq!(cut("аллея"), ["а", "лле", "я"]);
    }

    #[test]
    fn two_vowels_running_are_two_syllables() {
        assert_eq!(cut("аорта"), ["а", "ор", "та"]);
        assert_eq!(cut("моя"), ["мо", "я"]);
    }

    #[test]
    fn a_word_of_one_vowel_is_one_syllable() {
        assert_eq!(cut("стол"), ["стол"]);
        assert_eq!(cut("всплеск"), ["всплеск"]);
        assert_eq!(cut("класс"), ["класс"]);
    }

    #[test]
    fn a_hard_sign_closes_the_syllable_it_ends() {
        assert_eq!(cut("подъезд"), ["подъ", "езд"]);
    }

    #[test]
    fn a_soft_sign_closes_before_a_consonant_and_parts_before_a_vowel() {
        assert_eq!(cut("большой"), ["боль", "шой"]);
        assert_eq!(cut("семья"), ["се", "мья"]);
    }

    #[test]
    fn a_word_without_vowels_has_no_syllables() {
        assert!(cut("вств").is_empty());
        assert!(cut("").is_empty());
    }

    #[test]
    fn a_word_without_vowels_offers_no_place_either() {
        let letters = spelled("вств").expect("letters");

        assert!(places(&letters).is_empty());
        assert!(breaks(&letters).is_empty());
    }

    #[test]
    fn a_place_at_the_very_start_or_end_is_no_place() {
        let letters = spelled("аорта").expect("letters");

        assert!(!places(&letters).contains(&0));
        assert!(!places(&letters).contains(&letters.len()));
    }

    #[test]
    fn the_law_takes_only_places_the_word_offers() {
        for word in ["молоко", "битва", "полка", "подъезд", "аорта", "карман"]
        {
            let letters = spelled(word).expect("letters");
            let offered = places(&letters);

            assert!(
                breaks(&letters).iter().all(|at| offered.contains(at)),
                "{word}"
            );
        }
    }

    #[test]
    fn more_places_are_offered_than_the_law_takes() {
        let letters = spelled("битва").expect("letters");

        assert_eq!(places(&letters), std::vec![2, 3]);
        assert_eq!(breaks(&letters), std::vec![2]);
    }

    #[test]
    fn no_place_tears_a_sign_or_a_glide_from_what_precedes_it() {
        for word in ["подъезд", "большой", "чайка", "майка"] {
            let letters = spelled(word).expect("letters");

            for at in places(&letters) {
                let held = letters.get(at).expect("a letter");

                assert!(!held.is_sign(), "{word} parts before a sign");
                assert!(
                    !matches!(held.consonant(), Some(crate::alphabet::Consonant::Glide)),
                    "{word} parts before the glide"
                );
            }
        }
    }

    #[test]
    fn the_syllables_cover_the_word_and_do_not_overlap() {
        for word in ["молоко", "битва", "полка", "подъезд", "аорта"] {
            let letters = spelled(word).expect("letters");
            let held = parts(&letters);

            assert_eq!(held.first().map(|first| first.from), Some(0));
            assert_eq!(held.last().map(|last| last.upto), Some(letters.len()));
            for pair in held.windows(2) {
                assert_eq!(pair[0].upto, pair[1].from);
            }
            assert!(held.iter().all(|one| !one.is_empty()));
        }
    }

    #[test]
    fn a_syllable_holds_exactly_one_vowel() {
        for word in [
            "молоко",
            "битва",
            "полка",
            "аорта",
            "стол",
            "подъезд",
            "семья"
        ] {
            let letters = spelled(word).expect("letters");

            for held in parts(&letters) {
                let vowels = held
                    .of(&letters)
                    .iter()
                    .filter(|letter| letter.is_vowel())
                    .count();

                assert_eq!(vowels, 1, "{word}");
            }
        }
    }
}
