// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 86, пункт 2. Сложные междометия и звукоподражания пишутся через дефис.
//!
//! `ей-богу`, `ей-же-ей`, `о-го-го`, `ха-ха-ха`, `ой-ой-ой`, `цып-цып`,
//! `динь-динь-динь`.
//!
//! # Why this is a rule and not a list
//!
//! Every other paragraph here spells a word already known to be Russian. This
//! one does more: it says how an interjection is built, and a thing built by
//! rule can be recognised by rule.
//!
//! The interjections cannot be listed. `вау` and `окей` arrived in one
//! lifetime and more will arrive in the next, so a perfect list of them is
//! complete only until someone says something. But the shape the paragraph
//! names does not change. A word that repeats a piece of itself and joins the
//! pieces with a hyphen is an interjection whatever the piece is, and
//! [`repeats`] answers for `бла-бла-бла` and `тук-тук` without having heard
//! either.
//!
//! So the paragraph is read in two directions. Forward it requires the hyphen
//! of a word written without one. Backward it tells a checker that a word it
//! has never met stands outside the sentence and is not to be looked up,
//! declined or agreed with.

use crate::{
    grammar::PartOfSpeech,
    rules::{Citation, Findings, Found, Scope}
};

/// Where this rule is written.
pub const CITES: Citation = Citation::point(86, 2);

/// What this rule is about.
///
/// Interjections, because that is what the paragraph is about.
///
/// The repetition alone does not make one: `мама`, `папа` and `дядя` say a
/// piece of themselves twice and are nouns, written solid. Reading the class
/// off the repetition would hyphenate all three, so the class is asked for
/// instead — and the engine now has it, because the word states what it is
/// and the form comes from its own paradigm.
pub const SCOPE: Scope = Scope {
    parts:   &[PartOfSpeech::Interjection],
    cases:   &[],
    numbers: &[],
    needs:   crate::rules::scope::NOTHING
};

/// The fewest characters a repeated piece may have.
///
/// One letter repeats too readily to mean anything: `оо` in `зоопарк` is no
/// interjection, and neither is the doubled letter of any ordinary word.
const SHORTEST: usize = 2;

/// Reports whether a written word says a piece of itself twice.
///
/// Two readings, because the paragraph is about a word written two ways.
///
/// A word already hyphenated is read by its pieces: `о-го-го` says `го` twice
/// and `ей-же-ей` says `ей` twice, neither of them from the front and neither
/// of them throughout. Any piece said twice is the repetition the paragraph
/// means.
///
/// A word written solid has no pieces to read, so it is measured instead: a
/// piece is looked for that tiles the whole of it. `хахаха` is `ха` three
/// times over and `цыпцып` is `цып` twice.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::hyphen_compound::interjections::repeats;
///
/// assert!(repeats("хахаха"));
/// assert!(repeats("о-го-го"));
/// assert!(repeats("ей-же-ей"));
/// assert!(!repeats("вода"));
/// assert!(!repeats("зоопарк"));
/// ```
#[must_use]
pub fn repeats(written: &str) -> bool {
    let held = written.to_lowercase();

    if held.contains('-') {
        return said_twice(&held);
    }

    tiled(&held)
}

/// Reports whether a hyphenated word says one of its pieces twice.
fn said_twice(held: &str) -> bool {
    let pieces: Vec<&str> = held.split('-').filter(|piece| !piece.is_empty()).collect();

    pieces
        .iter()
        .enumerate()
        .any(|(place, piece)| pieces[place + 1..].contains(piece))
}

/// Reports whether a solid word is one piece laid down over and over.
fn tiled(held: &str) -> bool {
    let letters: Vec<char> = held.chars().collect();

    (SHORTEST..=letters.len() / 2).any(|piece| tiles(&letters, piece))
}

/// Reports whether a piece of a length tiles the whole word.
fn tiles(letters: &[char], piece: usize) -> bool {
    if !letters.len().is_multiple_of(piece) {
        return false;
    }

    letters
        .chunks(piece)
        .all(|chunk| chunk == &letters[..piece])
}

/// What the paragraph says when it is broken.
const SAYS: &str = "повторяющееся междометие пишется через дефис";

/// What the paragraph finds in a written word.
///
/// A word that says a piece of itself twice and is written solid: the
/// paragraph writes it with hyphens, and the finding spells it out that way.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::hyphen_compound::interjections::found;
///
/// let held = found("хахаха");
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].instead, "ха-ха-ха");
///
/// assert!(found("ха-ха-ха").is_empty());
/// assert!(found("вода").is_empty());
/// ```
#[must_use]
pub fn found(written: &str) -> Findings {
    let mut held = Findings::new();
    if written.contains('-') || !repeats(written) {
        return held;
    }
    let Some(parted) = parted(written) else {
        return held;
    };

    held.push(Found::new(CITES, 0, SAYS, parted));
    held
}

/// The word written with hyphens between the pieces it repeats.
fn parted(written: &str) -> Option<std::string::String> {
    let letters: std::vec::Vec<char> = written.chars().collect();
    let piece = (SHORTEST..=letters.len() / 2).find(|piece| tiles(&letters, *piece))?;

    Some(
        letters
            .chunks(piece)
            .map(|chunk| chunk.iter().collect::<std::string::String>())
            .collect::<std::vec::Vec<std::string::String>>()
            .join("-")
    )
}

/// Reports whether a written word is an interjection by the shape of it.
///
/// The paragraph read backward. `ха-ха-ха` is an interjection because nothing
/// but an interjection is built this way, and the checker may say so without
/// having the word in any dictionary.
///
/// Only the hyphenated shape answers true. `хахаха` is the same word written
/// wrongly, and saying what it is belongs to [`found`], which reports the
/// breach rather than reading past it.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::hyphen_compound::interjections::interjects;
///
/// assert!(interjects("ха-ха-ха"));
/// assert!(interjects("бла-бла-бла"));
/// assert!(interjects("тук-тук"));
/// assert!(!interjects("хахаха"));
/// assert!(!interjects("по-русски"));
/// ```
#[must_use]
pub fn interjects(written: &str) -> bool {
    written.contains('-') && repeats(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_paragraph_and_its_point_are_cited() {
        assert_eq!(CITES.paragraph, 86);
        assert_eq!(CITES.point, 2);
        assert!(CITES.is_stated());
    }

    #[test]
    fn every_word_the_paragraph_names_repeats() {
        for held in [
            "ей-же-ей",
            "о-го-го",
            "ха-ха-ха",
            "ой-ой-ой",
            "цып-цып",
            "динь-динь-динь"
        ] {
            assert!(
                repeats(held),
                "{held} is named by the paragraph and repeats nothing"
            );
        }
    }

    #[test]
    fn an_ordinary_word_repeats_nothing() {
        for held in ["вода", "зоопарк", "стол", "длинный", "кооператив"]
        {
            assert!(!repeats(held), "{held} repeats");
        }
    }

    #[test]
    fn a_single_letter_is_too_short_to_be_a_piece() {
        assert!(!repeats("аа"));
        assert!(!repeats("ааа"));
    }

    #[test]
    fn a_repeat_written_solid_is_found_and_spelled_with_hyphens() {
        let one = found("хахаха");
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].cites, CITES);
        assert_eq!(one[0].instead, "ха-ха-ха");

        let two = found("цыпцып");
        assert_eq!(two[0].instead, "цып-цып");
    }

    #[test]
    fn a_repeat_already_hyphenated_is_left_alone() {
        assert!(found("ха-ха-ха").is_empty());
        assert!(found("динь-динь-динь").is_empty());
    }

    #[test]
    fn a_word_that_repeats_nothing_is_outside_the_paragraph() {
        assert!(found("вода").is_empty());
        assert!(found("по-русски").is_empty());
        assert!(found("").is_empty());
    }

    #[test]
    fn a_word_never_heard_of_is_read_as_an_interjection_by_its_shape() {
        assert!(interjects("бла-бла-бла"));
        assert!(interjects("тук-тук"));
        assert!(interjects("ТУК-ТУК"));
    }

    #[test]
    fn a_hyphenated_word_that_repeats_nothing_is_no_interjection() {
        assert!(!interjects("по-русски"));
        assert!(!interjects("кто-то"));
        assert!(!interjects("из-за"));
    }

    #[test]
    fn what_it_reports_is_the_word_written_the_way_the_paragraph_writes_it() {
        let held = found("хахаха");

        assert_eq!(held[0].instead, "ха-ха-ха");
        assert!(found(&held[0].instead).is_empty());
    }
}
