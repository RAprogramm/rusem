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
    let mut found = Vec::new();

    found.extend(present(written, infinitive));
    found.extend(gone(written, infinitive));
    found.extend(bidden(written, infinitive));

    found
}

/// The imperative cells a form could be standing in.
///
/// Here the unknown stress is not a reason to refuse: `учи` and `учь` are both
/// written by the rule, one of them is the word, and a reader that took
/// neither would refuse a form the language wrote.
fn bidden(written: &str, infinitive: &str) -> Vec<Cell> {
    let mut found = Vec::new();

    for number in NUMBERS {
        if writes(infinitive, VerbForm::Imperative(number), written) {
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

/// The present cells a form could be standing in.
fn present(written: &str, infinitive: &str) -> Vec<Cell> {
    let mut found = Vec::new();

    for number in NUMBERS {
        for person in PERSONS {
            let cell = VerbForm::Present {
                person,
                number
            };
            if writes(infinitive, cell, written) {
                found.push(Cell {
                    tense: Tense::Present,
                    person: Some(person),
                    gender: None,
                    number
                });
            }
        }
    }

    found
}

/// The past cells a form could be standing in.
fn gone(written: &str, infinitive: &str) -> Vec<Cell> {
    let mut found = Vec::new();

    for gender in Gender::STATED {
        if writes(infinitive, VerbForm::Past(Bare::Singular(gender)), written) {
            found.push(Cell {
                tense:  Tense::Past,
                person: None,
                gender: Some(gender),
                number: Number::Singular
            });
        }
    }
    if writes(infinitive, VerbForm::Past(Bare::Plural), written) {
        found.push(Cell {
            tense:  Tense::Past,
            person: None,
            gender: None,
            number: Number::Plural
        });
    }

    found
}

/// Reports whether a cell of this verb is written the way the form arrived.
///
/// Both places of the stress are tried, because the reader does not know it
/// and the first conjugation writes a different vowel for each.
fn writes(infinitive: &str, cell: VerbForm, written: &str) -> bool {
    [false, true].into_iter().any(|stressed| {
        inflect::written(infinitive, cell, stressed).is_some_and(|held| same(&held, written))
    })
}
