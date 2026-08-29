// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Where the stress falls in every spelling a dictionary listed.
//!
//! The table is read from a file of one spelling to a line: the spelling, a
//! tab, then the lemmas it belongs to and the stressed vowel under each. The
//! vowel is counted in characters from the start of the word, so putting the
//! mark back is one insertion and needs no search.

use std::{
    collections::HashMap,
    io::BufRead,
    string::{String, ToString},
    vec::Vec
};

/// Where the stress falls in one spelling, under one lemma.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Placement {
    /// Which word, and what the dictionary said it is.
    pub reading: Reading,
    /// The stressed vowel, counted in characters from the start.
    pub vowel:   usize,
    /// Which article of the headword the form came from, counted from zero.
    pub article: usize
}

/// What is said about one spelling: which word it is a form of, and what the
/// analyzer or the dictionary called it.
///
/// One type for both sides. A table stores this about a spelling and an
/// analyzer answers with it, and two declarations of the same two facts would
/// drift apart.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reading {
    /// The dictionary form the spelling belongs to.
    pub lemma: String,
    /// What the form was called, in the same words the table uses.
    pub tags:  Vec<String>
}

/// Every spelling a dictionary showed, and where its stress falls.
#[derive(Debug, Default, Clone)]
pub struct Table {
    of: HashMap<String, Vec<Placement>>
}

impl Table {
    /// Reads a table written by the converter.
    ///
    /// A malformed line is skipped rather than refused: the table is data, and
    /// one bad line is no reason to leave a text unread.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read.
    pub fn read<R: BufRead>(reader: R) -> std::io::Result<Self> {
        let mut of: HashMap<String, Vec<Placement>> = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let Some((spelling, rest)) = line.split_once('\t') else {
                continue;
            };

            let mut placements = Vec::new();
            for held in rest.split_whitespace() {
                let mut fields = held.split(':');
                let (Some(lemma), Some(vowel)) = (fields.next(), fields.next()) else {
                    continue;
                };
                let Ok(vowel) = vowel.parse::<usize>() else {
                    continue;
                };
                let tags = fields
                    .next()
                    .filter(|held| !held.is_empty())
                    .map(|held| held.split(',').map(ToString::to_string).collect())
                    .unwrap_or_default();
                let article = fields
                    .next()
                    .and_then(|held| held.parse().ok())
                    .unwrap_or(0);

                placements.push(Placement {
                    reading: Reading {
                        lemma: lemma.to_string(),
                        tags
                    },
                    vowel,
                    article
                });
            }

            if !placements.is_empty() {
                of.insert(spelling.to_lowercase(), placements);
            }
        }

        Ok(Self {
            of
        })
    }

    /// Builds a table from placements already in hand.
    #[must_use]
    pub const fn from_parts(of: HashMap<String, Vec<Placement>>) -> Self {
        Self {
            of
        }
    }

    /// Reports whether a spelling takes the stress in more than one place.
    #[must_use]
    pub fn is_homograph(&self, spelling: &str) -> bool {
        let held = self.get(spelling);
        let Some(first) = held.first() else {
            return false;
        };

        held.iter().any(|placed| placed.vowel != first.vowel)
    }
}

/// What the table holds, asked of it.
///
/// Kept apart from the reading and the building: filling a table is one job
/// and questioning it is another.
impl Table {
    /// Every placement a spelling has.
    #[must_use]
    pub fn get(&self, spelling: &str) -> &[Placement] {
        self.of.get(spelling).map_or(&[], Vec::as_slice)
    }

    /// How many spellings the table holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.of.len()
    }

    /// Reports whether the table holds nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.of.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> Table {
        Table::read(
            "замок\tзамок:1:singular,nominative:0 замок:3:singular,nominative:0\nвода\tвода:3:singular,nominative:0\n"
                .as_bytes(),
        )
        .expect("the table reads")
    }

    #[test]
    fn the_table_reports_how_much_it_holds() {
        assert_eq!(table().len(), 2);
        assert!(!table().is_empty());
        assert_eq!(Table::default().len(), 0);
        assert!(Table::default().is_empty());
    }

    #[test]
    fn a_line_the_table_cannot_read_is_passed_over() {
        let held = Table::read(
            "нетабуляции\nслово\tбездвоеточий\nиное\tиное:нечисло:0\nстол\tстол:2::0\n".as_bytes()
        )
        .expect("a table");

        assert_eq!(held.len(), 1, "only the well-formed line was read");
        assert!(held.get("слово").is_empty());
        assert!(held.get("иное").is_empty());
        assert!(!held.get("стол").is_empty());
    }

    #[test]
    fn a_spelling_carries_every_placement_it_was_given() {
        assert_eq!(table().get("вода").len(), 1);
        assert_eq!(table().get("замок").len(), 2);
    }

    #[test]
    fn a_spelling_stressed_in_two_places_is_a_homograph() {
        assert!(table().is_homograph("замок"));
        assert!(!table().is_homograph("вода"));
        assert!(!table().is_homograph("нетуттакого"));
    }
}
