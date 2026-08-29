// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Reading a written form back through the index a dictionary states.
//!
//! The module beside this one writes a cell by the index. This goes the other
//! way, and it goes by the same road: every cell the index states is written
//! out and compared with what arrived, so there is one statement of how the
//! word declines, read backwards, and not a second that could drift from the
//! first. The comparison folds `ё` to `е`, because a writer may always drop
//! the diaeresis, and it folds it in one place — the same place the derived
//! reader folds it.
//!
//! Three cells the exact path does not produce: the second genitive (`чаю`),
//! the second locative (`в лесу`) and the vocative. The index does not state
//! them — Zaliznyak marks their existence with separate signs the core has no
//! rules for yet — and a cell the index does not state is a cell the core
//! stays silent about rather than invents. A reader asking for them is not
//! turned away empty: the locative counts as the prepositional and the
//! partitive as the genitive wherever cases are compared, which is what
//! [`Case::merged`](crate::grammar::Case::merged) is for and what everything
//! above this module already does.

use crate::grammar::{
    Animacy, Gender, Number,
    declension::{
        index::Index,
        reading::{CASES, Cell, same}
    }
};

/// Reads a written form back into every cell the index could have written it
/// from.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Animacy, Case, Gender,
///     declension::{index, stated::reading::cells}
/// };
///
/// let held = index::read("2b").expect("a stated index");
/// let found = cells("конём", "конь", Gender::Masculine, Animacy::Animate, held);
/// assert!(found.iter().any(|cell| cell.case == Case::Instrumental));
///
/// let folded = cells("конем", "конь", Gender::Masculine, Animacy::Animate, held);
/// assert_eq!(found, folded);
/// ```
#[must_use]
pub fn cells(
    written: &str,
    lemma: &str,
    gender: Gender,
    animacy: Animacy,
    index: Index
) -> Vec<Cell> {
    let mut found = Vec::new();

    for number in [Number::Singular, Number::Plural] {
        for case in CASES {
            let Some(held) = super::written(lemma, gender, animacy, index, case, number) else {
                continue;
            };
            if same(&held, written) {
                found.push(Cell {
                    case,
                    number
                });
            }
        }
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Case, Gender, declension::index};

    fn stated(written: &str) -> Index {
        index::read(written).unwrap_or_else(|| unreachable!("a stated index"))
    }

    #[test]
    fn a_stated_form_reads_back_with_the_diaeresis_dropped() {
        let found = cells(
            "конем",
            "конь",
            Gender::Masculine,
            Animacy::Animate,
            stated("2b")
        );

        assert!(found.contains(&Cell {
            case:   Case::Instrumental,
            number: Number::Singular
        }));
    }

    #[test]
    fn a_locative_query_is_answered_by_the_prepositional_cell() {
        let found = cells(
            "коне",
            "конь",
            Gender::Masculine,
            Animacy::Animate,
            stated("2b")
        );

        assert!(found.contains(&Cell {
            case:   Case::Locative.merged(),
            number: Number::Singular
        }));
        assert!(!found.iter().any(|cell| cell.case == Case::Locative));
    }
}
