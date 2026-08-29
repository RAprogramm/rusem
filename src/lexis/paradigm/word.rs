// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The paradigm of a word, whatever kind of word it is, and the way back.
//!
//! One entry above the tables: give it the dictionary form and what the word
//! is, and it answers with the table, choosing the pattern by the kind of word
//! rather than being told which builder to call.
//!
//! The way back is the same table read the other way round. A form arrives
//! written, the word that owns it makes its own table, and the cells that
//! spell the form are the cells it could be standing in — no ranking, no
//! preference, and nothing found that the table does not spell. What settles
//! between several cells is agreement and government above, not a guess here.
//!
//! A word whose forms the core cannot write yet answers with an empty table,
//! and the reader gets no cells at all. That is the honest answer: nothing is
//! known, as against `стол` in the nominative, which is known.

use super::{adjective, noun, verb};
use crate::{
    grammar::form::Form,
    lexis::{Lexeme, Paradigm},
    morphology::WordForm
};

/// Builds the paradigm of a word from what the word is.
///
/// # Examples
///
/// ```
/// use rusem::{
///     grammar::{Animacy, Gender, declension::Declension},
///     lexis::{Lexeme, Noun, paradigm::word},
///     morphology::WordForm
/// };
///
/// let table = word::of(
///     &WordForm::parse("книга")?,
///     Lexeme::Noun(Noun {
///         gender:     Gender::Feminine,
///         animacy:    Animacy::Inanimate,
///         declension: Declension::First,
///         index:      None
///     })
/// );
///
/// assert_eq!(table.len(), 12);
/// # Ok::<(), rusem::error::CoreError>(())
/// ```
#[must_use]
pub fn of(lemma: &WordForm, lexeme: Lexeme) -> Paradigm {
    match lexeme {
        Lexeme::Noun(held) => noun::of(lemma, held),
        Lexeme::Verb(held) => verb::of(lemma, held),
        Lexeme::Adjective => adjective::of(lemma),
        _ => Paradigm::bare(lemma.clone())
    }
}

/// Reads a written form back into the cells of its own word that spell it.
///
/// # Examples
///
/// ```
/// use rusem::{
///     grammar::{
///         Animacy, Case, Gender,
///         declension::Declension,
///         form::{Agreed, Form}
///     },
///     lexis::{Lexeme, Noun, paradigm::word},
///     morphology::WordForm
/// };
///
/// let stol = Lexeme::Noun(Noun {
///     gender:     Gender::Masculine,
///     animacy:    Animacy::Inanimate,
///     declension: Declension::Second,
///     index:      None
/// });
/// let cells = word::cells(&WordForm::parse("столы")?, &WordForm::parse("стол")?, stol);
///
/// assert!(cells.contains(&Form::Noun(Agreed::Plural {
///     case: Case::Nominative
/// })));
/// # Ok::<(), rusem::error::CoreError>(())
/// ```
#[must_use]
pub fn cells(written: &WordForm, lemma: &WordForm, lexeme: Lexeme) -> Vec<Form> {
    of(lemma, lexeme).cells_of(written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        grammar::{
            Animacy, Aspect, Case, Gender, Number, Transitivity,
            conjugation::Conjugation,
            declension::Declension,
            form::{Agreed, Bare, verb::VerbForm}
        },
        lexis::{Noun, Verb}
    };

    fn form(written: &str) -> WordForm {
        WordForm::parse(written).unwrap_or_else(|_| unreachable!("a written Russian word"))
    }

    fn table() -> Lexeme {
        Lexeme::Noun(Noun {
            gender:     Gender::Masculine,
            animacy:    Animacy::Inanimate,
            declension: Declension::Second,
            index:      None
        })
    }

    fn reading() -> Lexeme {
        Lexeme::Verb(Verb {
            aspect:       Aspect::Imperfective,
            transitivity: Transitivity::Transitive,
            reflexive:    false,
            conjugation:  Conjugation::First,
            index:        None
        })
    }

    #[test]
    fn a_noun_and_a_verb_each_get_the_table_of_their_kind() {
        assert_eq!(of(&form("стол"), table()).len(), 12);
        assert_eq!(of(&form("читать"), reading()).len(), 109);
    }

    #[test]
    fn a_word_whose_forms_the_core_cannot_write_gets_an_empty_table() {
        assert!(of(&form("быстро"), Lexeme::Adverb).is_empty());
    }

    #[test]
    fn a_noun_form_is_read_into_every_cell_that_spells_it() {
        let cells = cells(&form("стола"), &form("стол"), table());

        assert_eq!(
            cells,
            std::vec![Form::Noun(Agreed::Singular {
                case:   Case::Genitive,
                gender: Gender::Masculine
            })]
        );
    }

    #[test]
    fn a_verb_form_is_read_into_the_cell_that_spells_it() {
        let cells = cells(&form("читала"), &form("читать"), reading());

        assert_eq!(
            cells,
            std::vec![Form::Verb(VerbForm::Past(Bare::Singular(Gender::Feminine)))]
        );
    }

    #[test]
    fn a_form_the_word_does_not_have_reads_into_no_cell() {
        assert!(cells(&form("столами"), &form("книга"), table()).is_empty());
        assert!(cells(&form("читаю"), &form("стол"), table()).is_empty());
    }

    #[test]
    fn one_spelling_that_fills_two_cells_answers_both() {
        let cells = cells(&form("столы"), &form("стол"), table());

        assert_eq!(cells.len(), 2);
        assert!(
            cells
                .iter()
                .all(|held| held.number() == Some(Number::Plural))
        );
    }
}
