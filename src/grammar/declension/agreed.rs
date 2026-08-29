// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Reading a written adjective back into the cells it could be standing in.
//!
//! An agreeing word carries three categories at once, and its endings collapse
//! them heavily: `красной` is genitive, dative, instrumental and prepositional
//! feminine all in one spelling. A form read alone is all of them, and which
//! one it is comes from the noun it leans on, not from the form.

use super::{
    adjective::{ending_stressed, parted},
    attributive, spelling
};
use crate::grammar::{Animacy, Case, Gender, Number};

/// One cell of the adjectival paradigm a written form could have come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    /// The case that cell states.
    pub case:   Case,
    /// The number that cell states.
    pub number: Number,
    /// The gender that cell states, absent in the plural.
    pub gender: Option<Gender>
}

/// Reads a written agreeing word back into every cell that could have written
/// it.
///
/// The stress is not guessed at: the dictionary form states it — `-ой`
/// against `-ый` and `-ий` — and the reader holds the dictionary form, so it
/// reads with the same stress the writer wrote with.
#[must_use]
pub fn cells(written: &str, dictionary: &str, animacy: Animacy) -> Vec<Cell> {
    let Some((base, shape)) = parted(dictionary) else {
        return Vec::new();
    };
    let stressed = ending_stressed(dictionary);

    let mut found = Vec::new();
    for gender in Gender::STATED {
        for case in Case::STATED {
            let table = attributive::table(shape, gender, Number::Singular, stressed);
            if writes(&base, table.of(case, animacy), written, stressed) {
                found.push(Cell {
                    case,
                    number: Number::Singular,
                    gender: Some(gender)
                });
            }
        }
    }

    let table = attributive::table(shape, Gender::Masculine, Number::Plural, stressed);
    for case in Case::STATED {
        if writes(&base, table.of(case, animacy), written, stressed) {
            found.push(Cell {
                case,
                number: Number::Plural,
                gender: None
            });
        }
    }

    found
}

/// Reports whether a stem and an ending, once the alphabet has bent them,
/// spell the form that arrived.
fn writes(stem: &str, ending: &str, written: &str, stressed: bool) -> bool {
    joins(stem, &spelling::fitted(stem, ending, stressed), written)
}

/// Reports whether a stem followed by an ending spells a form.
fn joins(stem: &str, ending: &str, written: &str) -> bool {
    written.chars().count() == stem.chars().count() + ending.chars().count()
        && written.starts_with(stem)
        && written.ends_with(ending)
}
