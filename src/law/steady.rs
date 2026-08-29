// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What belongs to the word does not change from cell to cell.
//!
//! A noun has one gender. It agrees a different adjective in the plural than
//! in the singular, but it does not become neuter there, and a table that
//! wrote it neuter in one cell would be stating that `стол` and `столо` are
//! one word. The same holds of the class: every cell of a noun is a cell of a
//! noun, and a verb has no cell that is a noun's.
//!
//! This is the law that catches a table built by the wrong builder — the one
//! mistake the writing itself cannot show, because every form in such a table
//! is spelled correctly and only belongs to another word.

use super::{Broken, Law, Says};
use crate::{grammar::form::Form, lexis::Word};

/// The law that a word keeps what is its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Steady;

impl Law for Steady {
    fn states(&self) -> &'static str {
        "what belongs to the word is the same in every cell of it"
    }

    fn broken(&self, word: &Word) -> Vec<Broken> {
        let mut found = Vec::new();

        for (cell, spellings) in &word.paradigm().cells {
            if strays(*cell, word) {
                for written in spellings {
                    found.push(Broken {
                        cell:    *cell,
                        written: written.clone(),
                        law:     Says::Astray
                    });
                }
            }
        }

        found
    }
}

/// Reports whether a cell states something the word does not have.
///
/// Only a noun is asked about its gender. An adjective and a participle have a
/// gender in every cell and none as words: theirs is the noun's, and a cell
/// that states it is stating whose noun, not whose word.
fn strays(cell: Form, word: &Word) -> bool {
    let Form::Noun(_) = cell else {
        return false;
    };
    let Some(own) = word.lexeme.gender() else {
        return true;
    };

    cell.gender().is_some_and(|held| held != own)
}
