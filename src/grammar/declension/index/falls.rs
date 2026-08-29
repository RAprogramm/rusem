// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Where the stress falls in one cell, given the scheme the word declines by.
//!
//! A scheme is a statement about the whole paradigm, so this is where it is
//! read cell by cell: `c` says the stress is on the stem through the singular
//! and on the ending through the plural, and asked about the dative plural it
//! answers the ending.
//!
//! The primed schemes are each their parent with one cell taken out, and they
//! are written that way rather than as tables of their own — `d′` is `d` with
//! the accusative singular pulled back onto the stem, and saying it twice
//! would let the two drift.
//!
//! A cell with no ending at all is answered honestly: the scheme may well say
//! the stress is on the ending, and there is no ending to carry it, so it
//! lands on the stem. That is not an exception to the scheme; it is what the
//! scheme means when nothing follows the stem.

use crate::grammar::{Case, Number, declension::index::Accent};

/// What carries the stress in a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Falls {
    /// On the stem.
    Stem,
    /// On the ending.
    Ending
}

/// Where the stress falls in one cell.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Case, Number,
///     declension::index::{
///         Accent,
///         falls::{Falls, on}
///     }
/// };
///
/// assert_eq!(on(Accent::A, Case::Genitive, Number::Plural), Falls::Stem);
/// assert_eq!(on(Accent::B, Case::Genitive, Number::Plural), Falls::Ending);
/// assert_eq!(
///     on(Accent::C, Case::Nominative, Number::Singular),
///     Falls::Stem
/// );
/// assert_eq!(
///     on(Accent::C, Case::Nominative, Number::Plural),
///     Falls::Ending
/// );
/// assert_eq!(
///     on(Accent::F, Case::Nominative, Number::Singular),
///     Falls::Stem
/// );
/// assert_eq!(on(Accent::F, Case::Dative, Number::Singular), Falls::Ending);
/// ```
#[must_use]
pub const fn on(accent: Accent, case: Case, number: Number) -> Falls {
    let case = case.merged();

    match accent {
        Accent::A => Falls::Stem,
        Accent::B => Falls::Ending,
        Accent::BPrime => pulled(case, number, Case::Instrumental, Number::Singular),
        Accent::C => by_number(number, Number::Singular),
        Accent::D => by_number(number, Number::Plural),
        Accent::DPrime => {
            if same(case, number, Case::Accusative, Number::Singular) {
                Falls::Stem
            } else {
                by_number(number, Number::Plural)
            }
        }
        Accent::E => opening(case, number),
        Accent::F => pulled(case, number, Case::Nominative, Number::Singular),
        Accent::FPrime => pulled(case, number, Case::Accusative, Number::Singular),
        Accent::FDouble => doubled(case, number)
    }
}

/// The stem in one number and the ending in the other.
const fn by_number(number: Number, stem: Number) -> Falls {
    if matches!((number, stem), (Number::Singular, Number::Singular))
        || matches!((number, stem), (Number::Plural, Number::Plural))
    {
        Falls::Stem
    } else {
        Falls::Ending
    }
}

/// The ending everywhere but one cell, which keeps the stem.
const fn pulled(case: Case, number: Number, held: Case, of: Number) -> Falls {
    if same(case, number, held, of) {
        Falls::Stem
    } else {
        Falls::Ending
    }
}

/// Scheme `e`: the stem through the singular and into the nominative plural.
const fn opening(case: Case, number: Number) -> Falls {
    if matches!(number, Number::Singular) || matches!(case, Case::Nominative) {
        Falls::Stem
    } else {
        Falls::Ending
    }
}

/// Scheme `f″`: the ending, but the stem in the instrumental singular and the
/// nominative plural.
const fn doubled(case: Case, number: Number) -> Falls {
    if same(case, number, Case::Instrumental, Number::Singular)
        || same(case, number, Case::Nominative, Number::Plural)
    {
        return Falls::Stem;
    }

    Falls::Ending
}

/// Reports whether a cell is the one named.
const fn same(case: Case, number: Number, held: Case, of: Number) -> bool {
    matches!(
        (case, held),
        (Case::Nominative, Case::Nominative)
            | (Case::Genitive, Case::Genitive)
            | (Case::Dative, Case::Dative)
            | (Case::Accusative, Case::Accusative)
            | (Case::Instrumental, Case::Instrumental)
            | (Case::Prepositional, Case::Prepositional)
    ) && matches!(
        (number, of),
        (Number::Singular, Number::Singular) | (Number::Plural, Number::Plural)
    )
}
