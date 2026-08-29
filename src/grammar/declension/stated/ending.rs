// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The ending a stated index puts on a noun.
//!
//! Three paradigms, chosen by how the dictionary form ends and by the index:
//! the first for the nouns in `-а` whatever their gender — `мужчина` and
//! `слуга` are masculine and decline like `книга` — the second for the rest of
//! the masculines and the neuters, the third for the nouns the index writes
//! `8`. Which of the two shapes an ending takes —
//! `-ой` or `-ей`, `-ом` or `-ем`, `-о` or `-е` — is settled by the kind of
//! stem and by whether the stress falls on the ending, and both are known
//! before this is asked.
//!
//! What the alphabet does afterwards is not here. `ы` after `к` and the
//! unstressed `о` after `ц` are the same two rules for every ending in the
//! language, and they are applied once, where the stem and the ending meet.

pub mod first;
pub mod second;
pub mod third;

use crate::grammar::{
    Animacy, Case, Gender, Number,
    declension::index::{Index, Kind}
};

/// What the cell's own shape has already settled about the word.
///
/// Three answers that the stem and the scheme give before an ending is chosen:
/// where the stress fell, whether the word declines by the first paradigm, and
/// whether a fleeting vowel has just been put back into the stem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    /// Whether the stress falls on the ending.
    pub stressed: bool,
    /// Whether the noun declines by the first paradigm.
    pub opens:    bool,
    /// Whether a fleeting vowel has been put back into the stem.
    pub parted:   bool
}

/// Reports whether the endings of this kind of stem are the soft ones.
#[must_use]
pub const fn soft(kind: Kind) -> bool {
    matches!(kind, Kind::Soft | Kind::Glide | Kind::Iotated | Kind::Third)
}

/// The ending one cell takes.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Animacy, Case, Gender, Number,
///     declension::{
///         index,
///         stated::ending::{Shape, of}
///     }
/// };
///
/// let held = index::read("2b").expect("a stated index");
/// let one = of(
///     Gender::Masculine,
///     held,
///     Case::Instrumental,
///     Number::Singular,
///     Animacy::Animate,
///     Shape {
///         stressed: true,
///         opens:    false,
///         parted:   false
///     }
/// );
/// assert_eq!(one, Some("ём"));
/// ```
#[must_use]
pub const fn of(
    gender: Gender,
    index: Index,
    case: Case,
    number: Number,
    animacy: Animacy,
    shape: Shape
) -> Option<&'static str> {
    if matches!(index.kind, Kind::Third) {
        return third::of(case, number, animacy);
    }
    if shape.opens {
        return Some(first::of(
            index.kind,
            case,
            number,
            animacy,
            shape.stressed,
            shape.parted
        ));
    }

    Some(second::of(
        gender,
        index.kind,
        case,
        number,
        animacy,
        shape.stressed
    ))
}
