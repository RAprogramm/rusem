// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Every form a word has, and what is written in each.
//!
//! A paradigm is a table: one row to a cell the language gives the word, and
//! in each row the spelling that fills it. `стол` has twelve cells and twelve
//! spellings; `пальто` has twelve cells and one spelling in all of them, which
//! is what makes it indeclinable rather than what makes it short of forms.
//!
//! The cell is a [`Form`] and not a flat tag. That is the whole difference
//! from the paradigm an adapter answers with: a tag lets a cell be written
//! that the language has not got — a verb in the genitive, a past tense in the
//! first person — and a table of them cannot be walked without asking, at
//! every row, whether the row means anything. Here every row does.
//!
//! Two spellings may fill one cell — `чаю` and `чая` are both the genitive of
//! `чай` — and one spelling may fill several: `стола` is genitive singular and
//! nothing else, but `столы` is nominative and accusative plural at once. So a
//! cell holds a list, and reading backwards answers a list.

pub mod adjective;
pub mod noun;
pub mod participle;
pub mod verb;
pub mod word;

use crate::{grammar::form::Form, morphology::WordForm};

/// The spellings that fill a cell whose written form the core states.
///
/// One, or none: a written form that does not read back as a word is the one
/// thing building can produce that the alphabet refuses, and it fills the
/// cell with nothing rather than dropping the cell. A paradigm the core
/// misspells is then visibly short of a spelling instead of silently short
/// of a cell — the difference between evidence and its absence.
pub(crate) fn spelled(written: &str) -> Vec<WordForm> {
    WordForm::parse(written).map_or_else(|_| Vec::new(), |held| std::vec![held])
}

/// Every form a word has.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Paradigm {
    /// The dictionary form the paradigm belongs to.
    pub lemma: WordForm,
    /// The cells, each with the spellings that fill it.
    pub cells: Vec<(Form, Vec<WordForm>)>
}

impl Paradigm {
    /// A word with no forms recorded but its own.
    ///
    /// Not an empty paradigm: an indeclinable word genuinely has one spelling
    /// for every cell, and a caller that finds nothing here should say the
    /// paradigm is unknown rather than that the word has no forms.
    #[must_use]
    #[inline]
    pub const fn bare(lemma: WordForm) -> Self {
        Self {
            lemma,
            cells: Vec::new()
        }
    }

    /// Reports whether the paradigm records no cells at all.
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// How many cells the paradigm records.
    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.cells.len()
    }
}

/// What a paradigm says about the word it belongs to.
///
/// Held apart from the table itself: holding cells is one job, and answering
/// questions about them is another that grows as the engine learns to ask
/// more.
impl Paradigm {
    /// The spellings that fill one cell.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::{
    ///     grammar::{
    ///         Case, Gender,
    ///         form::{Agreed, Form}
    ///     },
    ///     lexis::Paradigm,
    ///     morphology::WordForm
    /// };
    ///
    /// let cell = Form::Noun(Agreed::Singular {
    ///     case:   Case::Genitive,
    ///     gender: Gender::Masculine
    /// });
    /// let held = Paradigm {
    ///     lemma: WordForm::parse("стол")?,
    ///     cells: vec![(cell, vec![WordForm::parse("стола")?])]
    /// };
    ///
    /// assert_eq!(held.fills(cell).len(), 1);
    /// # Ok::<(), rusem::error::CoreError>(())
    /// ```
    #[must_use]
    pub fn fills(&self, cell: Form) -> &[WordForm] {
        self.cells
            .iter()
            .find(|(held, _)| *held == cell)
            .map_or(&[], |(_, spellings)| spellings.as_slice())
    }

    /// The cells one spelling fills.
    ///
    /// A list, because a spelling rarely fills one: `столы` is the nominative
    /// and the accusative plural at once, and a reader that took the first
    /// answer would settle a case the word does not settle.
    #[must_use]
    pub fn cells_of(&self, written: &WordForm) -> Vec<Form> {
        self.cells
            .iter()
            .filter(|(_, spellings)| spellings.contains(written))
            .map(|(held, _)| *held)
            .collect()
    }

    /// Every spelling the word takes, in the order the cells stand, each once.
    #[must_use]
    pub fn spellings(&self) -> Vec<WordForm> {
        let mut held: Vec<WordForm> = Vec::new();

        for (_, spellings) in &self.cells {
            for one in spellings {
                if !held.contains(one) {
                    held.push(one.clone());
                }
            }
        }

        held
    }

    /// Reports whether the word is written the same way in every cell.
    ///
    /// That is what an indeclinable word is: `пальто` has all its cells and one
    /// spelling. A word with no cells recorded is not indeclinable, only
    /// unknown, and answers `false`.
    #[must_use]
    pub fn is_indeclinable(&self) -> bool {
        !self.is_empty() && self.spellings().len() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{
        Case, Gender, Number,
        form::{Agreed, Form}
    };

    fn form(text: &str) -> WordForm {
        WordForm::parse(text).expect("a word form")
    }

    fn one(case: Case) -> Form {
        Form::Noun(Agreed::Singular {
            case,
            gender: Gender::Masculine
        })
    }

    fn many(case: Case) -> Form {
        Form::Noun(Agreed::Plural {
            case
        })
    }

    fn table() -> Paradigm {
        Paradigm {
            lemma: form("стол"),
            cells: std::vec![
                (one(Case::Nominative), std::vec![form("стол")]),
                (one(Case::Genitive), std::vec![form("стола")]),
                (many(Case::Nominative), std::vec![form("столы")]),
                (many(Case::Accusative), std::vec![form("столы")]),
            ]
        }
    }

    #[test]
    fn a_cell_answers_with_what_fills_it() {
        assert_eq!(table().fills(one(Case::Genitive)), [form("стола")]);
    }

    #[test]
    fn a_cell_the_paradigm_does_not_hold_answers_with_nothing() {
        assert!(table().fills(one(Case::Dative)).is_empty());
    }

    #[test]
    fn a_spelling_that_fills_two_cells_names_both() {
        let held = table().cells_of(&form("столы"));

        assert_eq!(held.len(), 2);
        assert!(
            held.iter()
                .all(|cell| cell.number() == Some(Number::Plural))
        );
    }

    #[test]
    fn a_spelling_the_paradigm_does_not_hold_names_no_cell() {
        assert!(table().cells_of(&form("столом")).is_empty());
    }

    #[test]
    fn every_spelling_is_listed_once() {
        let held = table().spellings();

        assert_eq!(held.len(), 3);
        assert!(held.contains(&form("столы")));
    }

    #[test]
    fn a_word_written_one_way_in_every_cell_is_indeclinable() {
        let held = Paradigm {
            lemma: form("пальто"),
            cells: std::vec![
                (one(Case::Nominative), std::vec![form("пальто")]),
                (one(Case::Genitive), std::vec![form("пальто")]),
            ]
        };

        assert!(held.is_indeclinable());
        assert!(!table().is_indeclinable());
    }

    #[test]
    fn a_paradigm_with_no_cells_is_unknown_and_not_indeclinable() {
        let held = Paradigm::bare(form("стол"));

        assert!(held.is_empty());
        assert_eq!(held.len(), 0);
        assert!(!held.is_indeclinable());
        assert!(held.spellings().is_empty());
    }
}
