// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The paradigm of an adjective, built from its dictionary form alone.
//!
//! An adjective states nothing as a word: its gender, number and case are the
//! noun's, and all it brings of its own is the shape it declines on, which its
//! dictionary form says outright. So there is nothing to pass in beside the
//! word, and nothing to look up.
//!
//! The accusative is the one cell where the adjective cannot answer alone.
//! Russian writes `новый стол` and `нового брата` — same cell, two spellings,
//! and what parts them is the animacy of the noun, which the adjective does
//! not carry. The cell therefore holds both, and the noun beside it settles
//! which is meant. That is not an ambiguity to be resolved here: it is the
//! language stating that the form depends on something outside the word.
//!
//! The short form and the comparative are not here. `красен` parts its stem
//! with a vowel the spelling cannot predict, and a cell the core cannot write
//! is left unstated.

use crate::{
    grammar::{
        Animacy, Case, Gender, Number,
        declension::adjective,
        form::{Adjectival, Agreed, Form}
    },
    lexis::Paradigm,
    morphology::WordForm
};

/// Builds every declined cell of an adjective.
///
/// # Examples
///
/// ```
/// use rusem::{
///     grammar::{
///         Case, Gender,
///         form::{Adjectival, Agreed, Form}
///     },
///     lexis::paradigm::adjective,
///     morphology::WordForm
/// };
///
/// let table = adjective::of(&WordForm::parse("новый")?);
/// let cell = Form::Adjective(Adjectival::Full(Agreed::Singular {
///     case:   Case::Genitive,
///     gender: Gender::Feminine
/// }));
///
/// assert_eq!(
///     table.fills(cell).first().map(WordForm::as_str),
///     Some("новой")
/// );
/// # Ok::<(), rusem::error::CoreError>(())
/// ```
#[must_use]
pub fn of(lemma: &WordForm) -> Paradigm {
    let mut cells = Vec::new();

    for gender in Gender::STATED {
        for case in Case::STATED {
            push(&mut cells, lemma, case, Number::Singular, gender);
        }
    }
    for case in Case::STATED {
        push(&mut cells, lemma, case, Number::Plural, Gender::Masculine);
    }

    Paradigm {
        lemma: lemma.clone(),
        cells
    }
}

/// Writes one cell into the table, if the core can spell it.
fn push(
    cells: &mut Vec<(Form, Vec<WordForm>)>,
    lemma: &WordForm,
    case: Case,
    number: Number,
    gender: Gender
) {
    let spellings = spelled(lemma, case, number, gender);
    if !spellings.is_empty() {
        cells.push((cell(case, number, gender), spellings));
    }
}

/// The cell of the table a case, a number and a gender name.
const fn cell(case: Case, number: Number, gender: Gender) -> Form {
    Form::Adjective(Adjectival::Full(match number {
        Number::Singular => Agreed::Singular {
            case,
            gender
        },
        Number::Plural => Agreed::Plural {
            case
        }
    }))
}

/// What is written in one cell, once for every animacy that spells it
/// differently.
fn spelled(lemma: &WordForm, case: Case, number: Number, gender: Gender) -> Vec<WordForm> {
    let mut held: Vec<WordForm> = Vec::new();

    for animacy in [Animacy::Inanimate, Animacy::Animate] {
        let Some(written) = adjective::written(lemma.as_str(), case, number, gender, animacy)
        else {
            continue;
        };
        let Ok(one) = WordForm::parse(&written) else {
            continue;
        };
        if !held.contains(&one) {
            held.push(one);
        }
    }

    held
}

#[cfg(test)]
mod tests {
    use super::*;

    fn form(written: &str) -> WordForm {
        WordForm::parse(written).unwrap_or_else(|_| unreachable!("a written Russian word"))
    }

    fn cell_of(table: &Paradigm, case: Case, number: Number, gender: Gender) -> Vec<String> {
        table
            .fills(cell(case, number, gender))
            .iter()
            .map(|written| written.as_str().to_owned())
            .collect()
    }

    #[test]
    fn an_adjective_states_a_cell_for_three_genders_and_the_plural() {
        assert_eq!(of(&form("новый")).len(), 24);
    }

    #[test]
    fn the_hard_and_the_soft_shape_take_their_own_endings() {
        let hard = of(&form("новый"));
        let soft = of(&form("синий"));
        let cell = (Case::Instrumental, Number::Singular, Gender::Masculine);

        assert_eq!(cell_of(&hard, cell.0, cell.1, cell.2), ["новым"]);
        assert_eq!(cell_of(&soft, cell.0, cell.1, cell.2), ["синим"]);
    }

    #[test]
    fn a_stem_that_refuses_the_hard_vowel_declines_hard_all_the_same() {
        let table = of(&form("строгий"));

        assert_eq!(
            cell_of(&table, Case::Genitive, Number::Singular, Gender::Masculine),
            ["строгого"]
        );
        assert_eq!(
            cell_of(&table, Case::Nominative, Number::Plural, Gender::Masculine),
            ["строгие"]
        );
    }

    #[test]
    fn the_accusative_holds_both_spellings_the_noun_can_call_for() {
        let table = of(&form("новый"));

        assert_eq!(
            cell_of(
                &table,
                Case::Accusative,
                Number::Singular,
                Gender::Masculine
            ),
            ["новый", "нового"]
        );
        assert_eq!(
            cell_of(&table, Case::Accusative, Number::Plural, Gender::Masculine),
            ["новые", "новых"]
        );
    }

    #[test]
    fn the_feminine_accusative_states_an_ending_of_its_own() {
        let table = of(&form("новый"));

        assert_eq!(
            cell_of(&table, Case::Accusative, Number::Singular, Gender::Feminine),
            ["новую"]
        );
    }

    #[test]
    fn one_spelling_that_fills_four_cells_answers_all_four() {
        let table = of(&form("новый"));

        let cells = table.cells_of(&form("новой"));

        assert_eq!(cells.len(), 4);
    }
}
