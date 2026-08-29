// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The endings of the third declension.
//!
//! The nouns the index writes `8`: feminine, written with a sign, and holding
//! one vowel through four cells of the singular. The instrumental is the odd
//! one — `-ью` where everything else is `-и` — and it is the only cell of the
//! Russian noun that is written so.

use crate::grammar::{Animacy, Case, Number};

/// The ending one cell of the third paradigm takes.
///
/// Every cell has one: the paradigm is whole, and an answer that could not
/// be given would have nothing to be silent about.
#[must_use]
pub const fn of(case: Case, number: Number, animacy: Animacy) -> &'static str {
    match number {
        Number::Singular => singular(case),
        Number::Plural => plural(case, animacy)
    }
}

/// The singular.
const fn singular(case: Case) -> &'static str {
    match case.merged() {
        Case::Genitive | Case::Dative | Case::Prepositional => "и",
        Case::Instrumental => "ью",
        _ => "ь"
    }
}

/// The plural.
const fn plural(case: Case, animacy: Animacy) -> &'static str {
    match case.merged() {
        Case::Genitive => "ей",
        Case::Dative => "ям",
        Case::Instrumental => "ями",
        Case::Prepositional => "ях",
        Case::Accusative => match animacy {
            Animacy::Animate => "ей",
            Animacy::Inanimate => "и"
        },
        _ => "и"
    }
}
