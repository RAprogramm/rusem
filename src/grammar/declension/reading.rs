// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Reading a written noun back into the cells it could be standing in.
//!
//! The paradigm beside this one builds a form from a cell. This goes the other
//! way, which is the direction a reader goes: a form arrives already written,
//! and what has to be found is which cell of the paradigm produced it.
//!
//! It goes that way by asking the same paradigm. Every cell is written out and
//! compared with what arrived, so there is one statement of how a noun
//! declines and not two that could drift apart. What is added here is what
//! reading adds and writing does not: a form is admitted when it differs from
//! the written cell only in a way the language itself leaves open.
//!
//! Two such ways. `ё` is written as `е` wherever the writer of a text pleases,
//! so `тёмен` and `темен` are the same form. And the genitive plural of a noun
//! with no ending may part its last two consonants with a fleeting vowel —
//! `сосна` gives `сосен` — which the spelling of the dictionary form does not
//! decide, so both are read.
//!
//! The answer is rarely one cell. Russian spells several cells alike, and a
//! form read alone is honestly several cases at once. Which of them the
//! sentence means is settled above, by agreement and by government; here every
//! cell that could have written this form is returned, and none is preferred.

use super::{noun, spelling};
use crate::{
    alphabet::vowel::same,
    grammar::{Animacy, Case, Gender, Number}
};

/// One cell of the paradigm a written form could have come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// The case that cell states.
    pub case:   Case,
    /// The number that cell states.
    pub number: Number
}

/// Reads a written form back into every cell that could have written it.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Animacy, Case, Gender, declension::reading::cells};
///
/// let held = cells("времени", "время", Gender::Neuter, Animacy::Inanimate);
/// assert!(held.iter().any(|cell| cell.case == Case::Genitive));
///
/// let plural = cells("времён", "время", Gender::Neuter, Animacy::Inanimate);
/// assert!(plural.iter().any(|cell| cell.case == Case::Genitive));
/// ```
#[must_use]
pub fn cells(written: &str, nominative: &str, gender: Gender, animacy: Animacy) -> Vec<Cell> {
    matching(
        written,
        |case, number| noun::written(nominative, gender, animacy, case, number),
        |case, number| parted(nominative, gender, case, number, written)
    )
}

/// Every cell whose written form, put down by the writer given, is the same
/// form as what arrived.
///
/// The walk over both numbers and the stated cases is the reading itself, and
/// it is stated once: the derived reader here and the stated reader beside it
/// differ in who writes a cell out and in what else they admit, not in how
/// the table is walked. The comparison folds `ё` where everything folds it,
/// [`crate::alphabet::vowel::same`], and the second question is what a reader
/// admits beyond the letter-for-letter match — the derived reader passes the
/// fleeting-vowel admission, the stated reader passes nothing.
pub(crate) fn matching(
    written: &str,
    writes: impl Fn(Case, Number) -> Option<String>,
    admits: impl Fn(Case, Number) -> bool
) -> Vec<Cell> {
    let mut found = Vec::new();

    for number in [Number::Singular, Number::Plural] {
        for case in Case::STATED {
            let Some(held) = writes(case, number) else {
                continue;
            };
            if same(&held, written) || admits(case, number) {
                found.push(Cell {
                    case,
                    number
                });
            }
        }
    }

    found
}

/// Reports whether the form is the cell written with a fleeting vowel.
///
/// Only the genitive plural without an ending can be: that is the one cell
/// where two consonants close the word, and Russian parts them.
fn parted(nominative: &str, gender: Gender, case: Case, number: Number, written: &str) -> bool {
    if !matches!((case.merged(), number), (Case::Genitive, Number::Plural)) {
        return false;
    }
    let Some(stem) = noun::stem(nominative, gender, case, number) else {
        return false;
    };
    let Some(ending) = noun::ending(nominative, gender, Animacy::Inanimate, case, number) else {
        return false;
    };
    if !ending.is_empty() {
        return false;
    }

    [false, true]
        .into_iter()
        .any(|stressed| same(&spelling::parted(&stem, stressed), written))
}
