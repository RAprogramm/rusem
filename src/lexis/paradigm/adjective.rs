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
//! The short cells and the comparative stand beside the full ones, written
//! from the same stem by the rules beside the writer,
//! [`crate::grammar::declension::adjective::short`] and
//! [`crate::grammar::declension::adjective::compared`]. What those rules
//! cannot derive they refuse, and a refused cell is simply not here: a
//! relational adjective's short forms, a suppletive comparative, a neuter
//! whose letter hangs on an unstated stress. Silence, not a guess.

use crate::{
    grammar::{
        Animacy, Case, Gender, Number,
        declension::adjective,
        form::{Adjectival, Agreed, Bare, Form}
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
    for gender in Gender::STATED {
        short(&mut cells, lemma, Bare::Singular(gender));
    }
    short(&mut cells, lemma, Bare::Plural);
    compared(&mut cells, lemma);

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

/// Writes one short cell into the table, if the core can spell it.
///
/// Animacy does not enter: the short form is said of its noun rather than
/// agreeing with it in a case, and only the accusative ever shows animacy.
fn short(cells: &mut Vec<(Form, Vec<WordForm>)>, lemma: &WordForm, held: Bare) {
    let Some(written) = adjective::short::written(lemma.as_str(), held) else {
        return;
    };
    let spellings = super::spelled(&written);
    if !spellings.is_empty() {
        cells.push((Form::Adjective(Adjectival::Short(held)), spellings));
    }
}

/// Writes the comparative into the table, if the core can derive it.
fn compared(cells: &mut Vec<(Form, Vec<WordForm>)>, lemma: &WordForm) {
    let Some(written) = adjective::compared::written(lemma.as_str()) else {
        return;
    };
    let spellings = super::spelled(&written);
    if !spellings.is_empty() {
        cells.push((Form::Adjective(Adjectival::Compared), spellings));
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

    fn short_of(table: &Paradigm, held: Bare) -> Vec<String> {
        table
            .fills(Form::Adjective(Adjectival::Short(held)))
            .iter()
            .map(|written| written.as_str().to_owned())
            .collect()
    }

    fn compared_of(table: &Paradigm) -> Vec<String> {
        table
            .fills(Form::Adjective(Adjectival::Compared))
            .iter()
            .map(|written| written.as_str().to_owned())
            .collect()
    }

    #[test]
    fn an_adjective_states_its_full_short_and_compared_cells() {
        assert_eq!(of(&form("новый")).len(), 29);
    }

    #[test]
    fn the_short_cells_are_written_from_the_bare_stem() {
        let table = of(&form("красный"));

        assert_eq!(
            short_of(&table, Bare::Singular(Gender::Masculine)),
            ["красен"]
        );
        assert_eq!(
            short_of(&table, Bare::Singular(Gender::Feminine)),
            ["красна"]
        );
        assert_eq!(short_of(&table, Bare::Singular(Gender::Neuter)), ["красно"]);
        assert_eq!(short_of(&table, Bare::Plural), ["красны"]);
    }

    #[test]
    fn a_stem_closed_by_the_suffix_parts_it_in_the_bare_masculine() {
        let table = of(&form("важный"));

        assert_eq!(
            short_of(&table, Bare::Singular(Gender::Masculine)),
            ["важен"]
        );
    }

    #[test]
    fn the_comparative_takes_its_ending_by_the_stem() {
        assert_eq!(compared_of(&of(&form("новый"))), ["новее"]);
        assert_eq!(compared_of(&of(&form("громкий"))), ["громче"]);
        assert_eq!(compared_of(&of(&form("тихий"))), ["тише"]);
    }

    #[test]
    fn a_relational_adjective_keeps_only_its_full_cells() {
        let table = of(&form("русский"));

        assert_eq!(table.len(), 24);
        assert!(short_of(&table, Bare::Singular(Gender::Masculine)).is_empty());
        assert!(compared_of(&table).is_empty());
    }

    #[test]
    fn a_cell_that_hangs_on_an_unstated_fact_stays_unstated() {
        let table = of(&form("хороший"));

        assert_eq!(
            short_of(&table, Bare::Singular(Gender::Masculine)),
            ["хорош"]
        );
        assert!(short_of(&table, Bare::Singular(Gender::Neuter)).is_empty());
        assert!(compared_of(&table).is_empty());
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
