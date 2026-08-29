// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Reading a written verb back into the cells it could be standing in.
//!
//! The same way the noun is read: every cell the verb has is written out and
//! compared with what arrived, so the endings, the stems and the letters the
//! alphabet bends after a sibilant are stated once — in the writing — and the
//! reading cannot know them differently.
//!
//! The stress is what the reader does not have. In the first conjugation it
//! decides between `е` and `ё`, so both spellings are written out and either
//! may answer; and since `ё` is written as `е` at the writer's pleasure, the
//! comparison folds it away — where everything in the engine folds it,
//! [`crate::alphabet::vowel::same`].

use crate::{
    alphabet::vowel::same,
    grammar::{
        Gender, Number, Person, Tense,
        conjugation::inflect,
        form::{Bare, verb::VerbForm}
    }
};

/// One cell of the paradigm a written form could have come from.
///
/// The imperative states a number and no person: it is said to whoever is
/// addressed, and the language marks only whether that is one or many.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// The tense that cell states.
    pub tense:  Tense,
    /// The person that cell states, absent in the past.
    pub person: Option<Person>,
    /// The gender that cell states, absent outside the past singular.
    pub gender: Option<Gender>,
    /// The number that cell states.
    pub number: Number
}

/// The persons the present states a cell for.
const PERSONS: [Person; 3] = [Person::First, Person::Second, Person::Third];

/// The numbers every tense states a cell for.
const NUMBERS: [Number; 2] = [Number::Singular, Number::Plural];

/// Reads a written form back into every cell that could have written it.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Person, conjugation::reading::cells};
///
/// let held = cells("пишут", "писать");
/// assert!(held.iter().any(|cell| cell.person == Some(Person::Third)));
///
/// assert!(cells("читала", "читать").len() == 1);
/// ```
#[must_use]
pub fn cells(written: &str, infinitive: &str) -> Vec<Cell> {
    matching(|cell| writes(infinitive, cell, written))
}

/// Every finite cell the test given admits, walked in the table's order.
///
/// The walk over the present, the past and the imperative is the reading
/// itself, and it is stated once: the derived reader here and the stated
/// reader beside it, [`super::stated::reading`], differ in who writes a cell
/// out, not in how the table is walked.
pub(crate) fn matching(admitted: impl Fn(VerbForm) -> bool) -> Vec<Cell> {
    let mut found = Vec::new();

    for number in NUMBERS {
        for person in PERSONS {
            if admitted(VerbForm::Present {
                person,
                number
            }) {
                found.push(Cell {
                    tense: Tense::Present,
                    person: Some(person),
                    gender: None,
                    number
                });
            }
        }
    }
    for gender in Gender::STATED {
        if admitted(VerbForm::Past(Bare::Singular(gender))) {
            found.push(Cell {
                tense:  Tense::Past,
                person: None,
                gender: Some(gender),
                number: Number::Singular
            });
        }
    }
    if admitted(VerbForm::Past(Bare::Plural)) {
        found.push(Cell {
            tense:  Tense::Past,
            person: None,
            gender: None,
            number: Number::Plural
        });
    }
    for number in NUMBERS {
        if admitted(VerbForm::Imperative(number)) {
            found.push(Cell {
                tense: Tense::Present,
                person: None,
                gender: None,
                number
            });
        }
    }

    found
}

/// Reports whether a cell of this verb is written the way the form arrived.
///
/// Both places of the stress are tried, because the reader does not know it
/// and the first conjugation writes a different vowel for each. In the
/// imperative that is also why the unknown stress is not a reason to refuse:
/// `учи` and `учь` are both written by the rule, one of them is the word, and
/// a reader that took neither would refuse a form the language wrote.
fn writes(infinitive: &str, cell: VerbForm, written: &str) -> bool {
    [false, true].into_iter().any(|stressed| {
        inflect::written(infinitive, cell, stressed).is_some_and(|held| same(&held, written))
    })
}
