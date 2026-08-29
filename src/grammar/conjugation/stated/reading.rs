// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Reading a written form back through the index a dictionary states.
//!
//! The module beside this one writes a cell by the index. This goes the other
//! way, and it goes by the same road: every finite cell the index states is
//! written out and compared with what arrived, so there is one statement of
//! how the verb conjugates, read backwards, and not a second that could drift
//! from the first. The walk over the cells is the derived reader's walk,
//! [`reading::matching`](crate::grammar::conjugation::reading), and the
//! comparison folds `ё` to `е` where everything in the engine folds it,
//! [`crate::alphabet::vowel::same`], because a writer may always drop the
//! diaeresis.
//!
//! Where the derived reader tries both places of the stress, this one tries
//! none: the scheme letter of the index is the stress, so each cell has the
//! one spelling the writer states. A cell the writer refuses — a class whose
//! stem needs the dictionary's own note, an index marked `^` — is a cell no
//! form reads back into, because the core never claimed to know what stands
//! there.

use crate::{
    alphabet::vowel::same,
    grammar::{
        conjugation::{
            index::VerbIndex,
            reading::{Cell, matching}
        },
        form::verb::VerbForm
    }
};

/// Reads a written form back into every cell the index could have written it
/// from.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Person,
///     conjugation::{index, stated::reading::cells}
/// };
///
/// let held = index::read("3b").expect("a stated index");
/// let found = cells("толкнёшь", "толкнуть", held);
/// assert!(found.iter().any(|cell| cell.person == Some(Person::Second)));
///
/// let folded = cells("толкнешь", "толкнуть", held);
/// assert_eq!(found, folded);
/// ```
#[must_use]
pub fn cells(written: &str, infinitive: &str, index: VerbIndex) -> Vec<Cell> {
    matching(|cell: VerbForm| {
        super::written(infinitive, index, cell, false).is_some_and(|held| same(&held, written))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Gender, Number, Tense, conjugation::index};

    fn stated(written: &str) -> VerbIndex {
        index::read(written).unwrap_or_else(|| unreachable!("a stated index"))
    }

    #[test]
    fn a_stated_form_reads_back_with_the_diaeresis_dropped() {
        let found = cells("везешь", "везти", stated("7b/b"));

        assert!(found.contains(&Cell {
            tense:  Tense::Present,
            person: Some(crate::grammar::Person::Second),
            gender: None,
            number: Number::Singular
        }));
    }

    #[test]
    fn the_past_masculine_reads_back_through_its_yo() {
        let found = cells("вёз", "везти", stated("7b/b"));

        assert!(found.contains(&Cell {
            tense:  Tense::Past,
            person: None,
            gender: Some(Gender::Masculine),
            number: Number::Singular
        }));
    }

    #[test]
    fn what_the_index_does_not_state_reads_into_no_cell() {
        assert_eq!(cells("несёшь", "нести", stated("7b/b")), Vec::new());
        assert_eq!(cells("лжёшь", "лгать", stated("6°b/c^")), Vec::new());
    }
}
