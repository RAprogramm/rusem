// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Declining a noun by the index a dictionary states for it.
//!
//! The module beside this one works the pattern out from the gender and the
//! dictionary form, which is what has to be done for a word no dictionary
//! holds. Here the pattern is not worked out: it is read off the index, which
//! says what the stem ends in and where the stress falls, and the endings
//! follow from those two and the gender.
//!
//! That is the difference between `конем` and `конём`, between `лице` and
//! `лицо`, between `мужев` and `мужей`. None of them can be settled without
//! the stress or without knowing that the stem ends in a sibilant rather than
//! merely in a soft consonant, and the index says both.

pub mod ending;
pub mod fleeting;
pub mod reading;
pub mod stem;

use crate::grammar::{
    Animacy, Case, Gender, Number,
    declension::{
        index::{
            Index,
            falls::{self, Falls}
        },
        spelling
    }
};

/// The stem with its `ё` written `е` where the stress has left it.
///
/// `ё` is a stressed letter and nothing else in Russian: `жёлудь` keeps it
/// while the stress is on the stem and writes `желудей` when the scheme moves
/// the stress onto the ending. A cell with no ending at all keeps it, because
/// there is nowhere else for the stress to have gone: `кочерёг`, `сестёр`.
fn unstressed(base: &str, stressed: bool) -> String {
    if !stressed {
        return String::from(base);
    }

    base.replace('ё', "е")
}

/// Reports whether a noun declines by the first paradigm.
///
/// By its dictionary form and not by its gender: `мужчина` and `слуга` are
/// masculine and decline like `книга`, and `дядя` does too. Every noun written
/// with `-а` or `-я` belongs here, whoever it names.
#[must_use]
pub fn opens(lemma: &str) -> bool {
    matches!(lemma.chars().last(), Some('а' | 'я'))
}

/// Writes one cell of a noun that a dictionary states the index of.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Animacy, Case, Gender, Number,
///     declension::{index, stated::written}
/// };
///
/// let held = index::read("2b").expect("a stated index");
/// let one = written(
///     "конь",
///     Gender::Masculine,
///     Animacy::Animate,
///     held,
///     Case::Instrumental,
///     Number::Singular
/// );
/// assert_eq!(one.as_deref(), Some("конём"));
///
/// let tse = index::read("5d").expect("a stated index");
/// let two = written(
///     "лицо",
///     Gender::Neuter,
///     Animacy::Inanimate,
///     tse,
///     Case::Nominative,
///     Number::Singular
/// );
/// assert_eq!(two.as_deref(), Some("лицо"));
/// ```
#[must_use]
pub fn written(
    lemma: &str,
    gender: Gender,
    animacy: Animacy,
    index: Index,
    case: Case,
    number: Number
) -> Option<String> {
    let opens = opens(lemma);
    let base = stem::of(lemma, gender, index)?;
    let stressed = matches!(
        falls::on(index.accent, case, number, animacy),
        Falls::Ending
    );
    let parted = index.fleeting;
    let base = if parted {
        fleeting::of(
            &base,
            fleeting::Word {
                gender,
                kind: index.kind,
                animacy,
                opens
            },
            case,
            number,
            stressed
        )
    } else {
        base
    };
    let held = ending::of(
        gender,
        index,
        case,
        number,
        animacy,
        ending::Shape {
            stressed,
            opens,
            parted
        }
    );
    let base = unstressed(&base, stressed && !held.is_empty());

    Some(base.clone() + &spelling::fitted(&base, held, stressed))
}
