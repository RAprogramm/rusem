// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The endings of the nouns in `-а`.
//!
//! One paradigm with two shapes, hard and soft, and one cell that is neither:
//! the genitive plural, which has no ending at all and shows what the stem
//! ends in instead. A soft stem writes a sign there — `недель` — and a stem in
//! the glide writes the glide — `ахиней`, `химий` — because that is the letter
//! the other endings carried and it has nowhere else to stand.

use crate::grammar::{
    Animacy, Case, Number,
    declension::{index::Kind, stated::ending::soft}
};

/// The ending one cell of the first paradigm takes.
#[must_use]
pub const fn of(
    kind: Kind,
    case: Case,
    number: Number,
    animacy: Animacy,
    stressed: bool,
    parted: bool
) -> &'static str {
    match number {
        Number::Singular => singular(kind, case, stressed),
        Number::Plural => plural(kind, case, animacy, parted)
    }
}

/// The singular.
const fn singular(kind: Kind, case: Case, stressed: bool) -> &'static str {
    let soft = soft(kind);

    match case.merged() {
        Case::Genitive => {
            if soft {
                "и"
            } else {
                "ы"
            }
        }
        Case::Dative | Case::Prepositional => {
            if matches!(kind, Kind::Iotated) {
                "и"
            } else {
                "е"
            }
        }
        Case::Accusative => {
            if soft {
                "ю"
            } else {
                "у"
            }
        }
        Case::Instrumental => instrumental(soft, stressed),
        _ => {
            if soft {
                "я"
            } else {
                "а"
            }
        }
    }
}

/// The instrumental, where the stress decides the vowel.
const fn instrumental(soft: bool, stressed: bool) -> &'static str {
    match (soft, stressed) {
        (true, true) => "ёй",
        (true, false) => "ей",
        (false, _) => "ой"
    }
}

/// The plural.
const fn plural(kind: Kind, case: Case, animacy: Animacy, parted: bool) -> &'static str {
    let soft = soft(kind);

    match case.merged() {
        Case::Genitive => bare(kind, parted),
        Case::Dative => {
            if soft {
                "ям"
            } else {
                "ам"
            }
        }
        Case::Instrumental => {
            if soft {
                "ями"
            } else {
                "ами"
            }
        }
        Case::Prepositional => {
            if soft {
                "ях"
            } else {
                "ах"
            }
        }
        Case::Accusative => match animacy {
            Animacy::Animate => bare(kind, parted),
            Animacy::Inanimate => nominative(soft)
        },
        _ => nominative(soft)
    }
}

/// The nominative plural.
const fn nominative(soft: bool) -> &'static str {
    if soft { "и" } else { "ы" }
}

/// The genitive plural, which writes what the stem ends in and nothing else.
///
/// A stem that has just been parted by a fleeting vowel writes nothing at all:
/// the vowel is what shows the softness, and `песня` gives `песен`, not
/// `песень`.
///
/// `свечей`, `вожжей` and `толкотней` do take an ending here, and it is not
/// the stress that says so: `бед`, `сред`, `рук` and `сторон` are stressed on
/// the ending by their schemes and take none. The dictionary marks the ones
/// that do, and until that mark is read this paradigm writes what it has.
const fn bare(kind: Kind, parted: bool) -> &'static str {
    match kind {
        Kind::Soft if !parted => "ь",
        Kind::Glide | Kind::Iotated if !parted => "й",
        _ => ""
    }
}
