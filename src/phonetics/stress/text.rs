// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Placing the stress over a running text.
//!
//! Three things can happen to a word. The table knows one stress for it, and
//! the mark goes in. The table knows two, and the reading settles which — this
//! is where a spelling like `за́мок` parts from `замо́к`. Or nothing settles it,
//! and the word is left as written, because a wrong stress names another word.
//!
//! A word of one syllable takes no mark. Neither does a word holding `ё`,
//! which is stressed by its own spelling.

use std::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec
};

use crate::phonetics::stress::table::Table;

/// The combining acute accent Russian marks stress with.
///
/// Named by the alphabet, which is where the marks are stated, and re-exported
/// here because this is where they are written.
pub const ACUTE: char = crate::alphabet::Mark::Acute.written();

/// What an analyzer settled on for one word.
///
/// The same shape a placement in the table has, and deliberately the same
/// type: what the table stores about a spelling and what an analyzer says
/// about it are the same two facts, and two declarations of them would drift.
pub use crate::phonetics::stress::table::Reading;

/// One word of a text, with the stress placed if it could be.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Marked {
    /// The word as written, with the mark inserted where one was placed.
    pub written:   String,
    /// Whether a mark was placed.
    pub placed:    bool,
    /// Whether the spelling takes the stress in more than one place.
    pub homograph: bool
}

/// Reports whether a word needs a mark at all.
///
/// A word of one syllable has nowhere else to put the stress, and a word
/// holding `ё` is already marked: that letter is stressed by its own spelling.
#[must_use]
pub fn needs_mark(word: &str) -> bool {
    syllables(word) > 1 && !word.contains('ё')
}

/// Places the stress over one word.
///
/// The reading is what settles a homograph. Without one, a spelling stressed
/// in two places is left bare rather than guessed at.
#[must_use]
pub fn place_word(table: &Table, word: &str, reading: Option<&Reading>) -> Marked {
    let bare = word.to_lowercase();
    let homograph = table.is_homograph(&bare);

    if !needs_mark(&bare) {
        return Marked {
            written: word.to_owned(),
            placed: false,
            homograph
        };
    }

    let placements = table.get(&bare);
    let vowel = match placements {
        [] => None,
        [single] => Some(single.vowel),
        many => reading
            .and_then(|held| settle(many, held))
            .or_else(|| chief(many))
    };

    let Some(vowel) = vowel else {
        return Marked {
            written: word.to_owned(),
            placed: false,
            homograph
        };
    };

    Marked {
        written: marked(word, vowel),
        placed: true,
        homograph
    }
}

/// Places the stress over a whole text.
///
/// Everything between words — spaces, punctuation, digits — is carried through
/// untouched, so the text comes back as it went in but readable aloud. The
/// readings, when given, line up with the words in the order they appear.
#[must_use]
pub fn place(table: &Table, text: &str, readings: &[Option<Reading>]) -> String {
    let mut written = String::with_capacity(text.len() + text.len() / 8);
    let mut word = String::new();
    let mut seen = 0_usize;

    let flush = |word: &mut String, written: &mut String, seen: &mut usize| {
        if word.is_empty() {
            return;
        }

        let reading = readings.get(*seen).and_then(Option::as_ref);
        written.push_str(&place_word(table, word, reading).written);
        *seen += 1;
        word.clear();
    };

    for letter in text.chars() {
        if letter.is_alphabetic() || letter == '-' {
            word.push(letter);
            continue;
        }

        flush(&mut word, &mut written, &mut seen);
        written.push(letter);
    }
    flush(&mut word, &mut written, &mut seen);

    written
}

/// Chooses among several placements by what the analyzer read the word as.
///
/// The lemma settles most homographs — `сто́ит` belongs to `стоить` and `стои́т`
/// to `стоять`. Where one lemma takes the stress in two places, the grammar
/// settles it: `во́ду` is accusative and `воды́` genitive. Where neither does,
/// nothing is placed.
fn settle(
    placements: &[crate::phonetics::stress::table::Placement],
    reading: &Reading
) -> Option<usize> {
    let by_lemma: Vec<&crate::phonetics::stress::table::Placement> = placements
        .iter()
        .filter(|placed| placed.reading.lemma == reading.lemma)
        .collect();
    if by_lemma.is_empty() {
        return None;
    }

    if let Some(vowel) = agreed(&by_lemma) {
        return Some(vowel);
    }

    let by_tags: Vec<&crate::phonetics::stress::table::Placement> = by_lemma
        .iter()
        .copied()
        .filter(|placed| {
            !placed.reading.tags.is_empty()
                && placed
                    .reading
                    .tags
                    .iter()
                    .all(|tag| reading.tags.iter().any(|held| held == tag))
        })
        .collect();

    agreed(&by_tags)
}

/// The stress the dictionary's own first article gives, if only one does.
///
/// A rare namesake should not cost a common word its stress. `вода` has a
/// second article for a dialect word stressed on the first syllable; the
/// first article is the water everyone means, and that is what a reader
/// wants when nothing else settles it.
fn chief(placements: &[crate::phonetics::stress::table::Placement]) -> Option<usize> {
    let first: Vec<&crate::phonetics::stress::table::Placement> = placements
        .iter()
        .filter(|placed| placed.article == 0)
        .collect();

    agreed(&first)
}

/// The stress these placements agree on, if they agree.
fn agreed(placements: &[&crate::phonetics::stress::table::Placement]) -> Option<usize> {
    let first = placements.first()?.vowel;

    placements
        .iter()
        .all(|placed| placed.vowel == first)
        .then_some(first)
}

use crate::alphabet::syllables;

/// Inserts the mark after one vowel of a word.
fn marked(word: &str, vowel: usize) -> String {
    let mut written = String::with_capacity(word.len() + ACUTE.len_utf8());

    for (position, letter) in word.chars().enumerate() {
        written.push(letter);
        if position == vowel {
            written.push(ACUTE);
        }
    }

    if written.chars().any(|letter| letter == ACUTE) {
        written
    } else {
        word.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::phonetics::stress::table::{Placement, Table};

    fn table() -> Table {
        let mut of = std::collections::HashMap::new();
        of.insert(
            "вода".to_string(),
            vec![Placement {
                reading: Reading {
                    lemma: "вода".to_string(),
                    tags:  vec!["singular".to_string(), "nominative".to_string()]
                },
                vowel:   3,
                article: 0
            }]
        );
        of.insert(
            "замок".to_string(),
            vec![
                Placement {
                    reading: Reading {
                        lemma: "замок".to_string(),
                        tags:  vec!["singular".to_string(), "nominative".to_string()]
                    },
                    vowel:   1,
                    article: 0
                },
                Placement {
                    reading: Reading {
                        lemma: "замок".to_string(),
                        tags:  vec!["singular".to_string(), "nominative".to_string()]
                    },
                    vowel:   3,
                    article: 0
                },
            ]
        );
        of.insert(
            "стол".to_string(),
            vec![Placement {
                reading: Reading {
                    lemma: "стол".to_string(),
                    tags:  vec!["singular".to_string(), "nominative".to_string()]
                },
                vowel:   2,
                article: 0
            }]
        );

        Table::from_parts(of)
    }

    #[test]
    fn a_word_with_one_stress_takes_the_mark() {
        let held = place_word(&table(), "вода", None);

        assert!(held.placed);
        assert_eq!(held.written, "вода\u{0301}");
    }

    #[test]
    fn a_word_of_one_syllable_is_left_alone() {
        let held = place_word(&table(), "стол", None);

        assert!(!held.placed);
        assert_eq!(held.written, "стол");
    }

    #[test]
    fn a_homograph_nothing_settles_is_left_bare() {
        let held = place_word(&table(), "замок", None);

        assert!(!held.placed);
        assert!(held.homograph);
        assert_eq!(held.written, "замок");
    }

    #[test]
    fn a_reading_settles_a_homograph_by_its_lemma() {
        let held = place_word(
            &table(),
            "замок",
            Some(&Reading {
                lemma: "замок".to_string(),
                tags:  std::vec!["singular".to_string(), "nominative".to_string()]
            })
        );

        assert!(!held.placed, "one lemma stressed two ways settles nothing");
        assert!(held.homograph);
    }

    #[test]
    fn a_lemma_stressed_one_way_needs_no_grammar_to_settle_it() {
        let held = place_word(
            &table(),
            "стол",
            Some(&Reading {
                lemma: "стол".to_string(),
                tags:  Vec::new()
            })
        );

        assert!(!held.placed, "one syllable takes no mark");
    }

    #[test]
    fn a_mark_asked_for_beyond_the_last_vowel_leaves_the_word_alone() {
        let held = place_word(&Table::default(), "вода", None);

        assert_eq!(held.written, "вода");
    }

    #[test]
    fn a_reading_of_another_lemma_settles_nothing() {
        let held = place_word(
            &table(),
            "замок",
            Some(&Reading {
                lemma: "запор".to_string(),
                tags:  Vec::new()
            })
        );

        assert!(!held.placed);
    }

    #[test]
    fn a_lemma_stressed_one_way_is_settled_by_the_reading() {
        let held = place_word(
            &table(),
            "вода",
            Some(&Reading {
                lemma: "вода".to_string(),
                tags:  std::vec!["singular".to_string()]
            })
        );

        assert!(held.placed);
        assert!(held.written.contains(ACUTE));
    }

    #[test]
    fn a_word_the_table_does_not_know_is_left_bare() {
        let held = place_word(&table(), "кирпич", None);

        assert!(!held.placed);
        assert_eq!(held.written, "кирпич");
    }

    #[test]
    fn a_text_comes_back_with_its_punctuation() {
        let held = place(&table(), "Вода, вода!", &[]);

        assert_eq!(held, "Вода\u{0301}, вода\u{0301}!");
    }
}
