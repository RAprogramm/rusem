// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The endings of the masculines and the neuters.
//!
//! They share every cell but two: the nominative and the accusative, where a
//! masculine writes what its stem ends in and a neuter writes a vowel. That is
//! the whole of the difference, and it is why the two are one paradigm here.
//!
//! The genitive plural is where the kinds part company. A hard stem takes
//! `-ов`, a sibilant takes `-ей`, `ц` takes `-ев` unstressed and `-ов` under
//! stress — `пальцев` against `отцов` — and a stem in the glide takes `-ев` or
//! `-ёв` the same way: `чаёв`, `киёв`.

use crate::grammar::{
    Animacy, Case, Gender, Number,
    declension::{index::Kind, stated::ending::soft}
};

/// The ending one cell of the second paradigm takes.
#[must_use]
pub const fn of(
    gender: Gender,
    kind: Kind,
    case: Case,
    number: Number,
    animacy: Animacy,
    stressed: bool
) -> &'static str {
    match number {
        Number::Singular => singular(gender, kind, case, animacy, stressed),
        Number::Plural => plural(gender, kind, case, animacy, stressed)
    }
}

/// The singular.
const fn singular(
    gender: Gender,
    kind: Kind,
    case: Case,
    animacy: Animacy,
    stressed: bool
) -> &'static str {
    let soft = soft(kind);

    match case.merged() {
        Case::Genitive => vowel(soft, "я", "а"),
        Case::Dative => vowel(soft, "ю", "у"),
        Case::Instrumental => instrumental(soft, stressed),
        Case::Prepositional => {
            if matches!(kind, Kind::Iotated) {
                "и"
            } else {
                "е"
            }
        }
        Case::Accusative => accusative(gender, kind, animacy, stressed),
        _ => nominative(gender, kind, stressed)
    }
}

/// The nominative singular, which a masculine writes with its stem's own
/// letter and a neuter with a vowel.
const fn nominative(gender: Gender, kind: Kind, stressed: bool) -> &'static str {
    if matches!(gender, Gender::Neuter) {
        return match (soft(kind), stressed) {
            (true, true) => "ё",
            (true, false) => "е",
            (false, _) => "о"
        };
    }

    match kind {
        Kind::Soft => "ь",
        Kind::Glide | Kind::Iotated => "й",
        _ => ""
    }
}

/// The accusative singular, which repeats the nominative for a thing and the
/// genitive for a living being.
///
/// A neuter repeats the nominative whichever it names. `чудовище` and `быдло`
/// are alive and are seen, not `чудовища`: the neuter shows animacy in the
/// plural alone, which is where `чудовищ` stands.
const fn accusative(gender: Gender, kind: Kind, animacy: Animacy, stressed: bool) -> &'static str {
    if matches!(gender, Gender::Neuter) {
        return nominative(gender, kind, stressed);
    }

    match animacy {
        Animacy::Animate => vowel(soft(kind), "я", "а"),
        Animacy::Inanimate => nominative(gender, kind, stressed)
    }
}

/// The instrumental singular, where the stress decides the vowel.
const fn instrumental(soft: bool, stressed: bool) -> &'static str {
    match (soft, stressed) {
        (true, true) => "ём",
        (true, false) => "ем",
        (false, _) => "ом"
    }
}

/// The plural.
const fn plural(
    gender: Gender,
    kind: Kind,
    case: Case,
    animacy: Animacy,
    stressed: bool
) -> &'static str {
    let soft = soft(kind);

    match case.merged() {
        Case::Genitive => genitive(gender, kind, stressed),
        Case::Dative => vowel(soft, "ям", "ам"),
        Case::Instrumental => vowel(soft, "ями", "ами"),
        Case::Prepositional => vowel(soft, "ях", "ах"),
        Case::Accusative => match animacy {
            Animacy::Animate => genitive(gender, kind, stressed),
            Animacy::Inanimate => opening(gender, soft)
        },
        _ => opening(gender, soft)
    }
}

/// The nominative plural.
const fn opening(gender: Gender, soft: bool) -> &'static str {
    if matches!(gender, Gender::Neuter) {
        return if soft { "я" } else { "а" };
    }

    if soft { "и" } else { "ы" }
}

/// The genitive plural, which the kind of stem settles.
const fn genitive(gender: Gender, kind: Kind, stressed: bool) -> &'static str {
    if matches!(gender, Gender::Neuter) {
        return match kind {
            Kind::Soft => "ей",
            Kind::Glide | Kind::Iotated => "й",
            _ => ""
        };
    }

    match (kind, stressed) {
        (Kind::Hard | Kind::Velar, _) | (Kind::Tse, true) => "ов",
        (Kind::Sibilant | Kind::Soft | Kind::Third, _) => "ей",
        (Kind::Tse | Kind::Glide | Kind::Iotated, false) => "ев",
        (Kind::Glide | Kind::Iotated, true) => "ёв"
    }
}

/// One of two vowels, by the shape of the stem.
const fn vowel(soft: bool, held: &'static str, hard: &'static str) -> &'static str {
    if soft { held } else { hard }
}
