// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The endings of everything that declines by agreeing.
//!
//! An adjective, a participle, an ordinal numeral and a pronoun that stands in
//! for an adjective all decline the same way: they take the case, number and
//! gender of the noun they lean on, and they take one set of endings to state
//! it. Russian calls that set the adjectival declension, and it has the same
//! two shapes as the nominal one, hard and soft.
//!
//! The neuter differs from the masculine in two cells and repeats it in the
//! rest; the plural has no gender at all. The accusative, here as in the noun,
//! states no ending of its own outside the feminine — it repeats the nominative
//! for a thing and the genitive for a living being.

use super::Stem;
use crate::grammar::{Animacy, Case, Gender, Number};

/// The endings one gender and number of the adjectival declension takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Endings {
    /// Nominative.
    pub nominative:    &'static str,
    /// Genitive.
    pub genitive:      &'static str,
    /// Dative.
    pub dative:        &'static str,
    /// Accusative, when this gender states one of its own.
    pub accusative:    Option<&'static str>,
    /// Instrumental.
    pub instrumental:  &'static str,
    /// Prepositional.
    pub prepositional: &'static str
}

impl Endings {
    /// The ending this set puts on a word in the case asked for.
    #[must_use]
    pub fn of(&self, case: Case, animacy: Animacy) -> &'static str {
        match case.merged() {
            Case::Nominative | Case::Vocative => self.nominative,
            Case::Dative => self.dative,
            Case::Instrumental => self.instrumental,
            Case::Prepositional => self.prepositional,
            Case::Accusative => self.accusative.unwrap_or(match animacy {
                Animacy::Animate => self.genitive,
                Animacy::Inanimate => self.nominative
            }),
            _ => self.genitive
        }
    }
}

/// The endings an agreeing word takes in the gender and number asked for.
///
/// Whether the endings carry the stress has to be told, because the masculine
/// nominative is written by it: `новый` against `второй`. The dictionary form
/// of an adjective states the answer outright — a stressed hard ending spells
/// itself `-ой` and an unstressed one `-ый` or `-ий` — so a caller holding
/// the dictionary form is never guessing here.
#[must_use]
pub const fn table(shape: Stem, gender: Gender, number: Number, stressed: bool) -> Endings {
    match (number, gender) {
        (Number::Plural, _) => plural(shape),
        (Number::Singular, Gender::Feminine) => feminine(shape),
        (Number::Singular, Gender::Neuter) => neuter(shape, stressed),
        (Number::Singular, _) => masculine(shape, stressed)
    }
}

/// The masculine singular.
///
/// The hard nominative is the one cell the stress rewrites: `-ый` off the
/// ending and `-ой` under it — `новый`, `второй`, `большой`. Zaliznyak's
/// index writes the two as one declension under its schemes `a` and `b`, and
/// no other cell changes its letters for the stress alone. A soft stem has no
/// stressed nominative to part from `-ий`, so the stress does not reach it.
const fn masculine(shape: Stem, stressed: bool) -> Endings {
    match shape {
        Stem::Hard => Endings {
            nominative:    if stressed { "ой" } else { "ый" },
            genitive:      "ого",
            dative:        "ому",
            accusative:    None,
            instrumental:  "ым",
            prepositional: "ом"
        },
        Stem::Soft => Endings {
            nominative:    "ий",
            genitive:      "его",
            dative:        "ему",
            accusative:    None,
            instrumental:  "им",
            prepositional: "ем"
        }
    }
}

/// The neuter singular, which parts from the masculine in two cells.
const fn neuter(shape: Stem, stressed: bool) -> Endings {
    let held = masculine(shape, stressed);
    let nominative = match shape {
        Stem::Hard => "ое",
        Stem::Soft => "ее"
    };

    Endings {
        nominative,
        accusative: Some(nominative),
        ..held
    }
}

/// The feminine singular, the only one with an accusative of its own.
const fn feminine(shape: Stem) -> Endings {
    match shape {
        Stem::Hard => Endings {
            nominative:    "ая",
            genitive:      "ой",
            dative:        "ой",
            accusative:    Some("ую"),
            instrumental:  "ой",
            prepositional: "ой"
        },
        Stem::Soft => Endings {
            nominative:    "яя",
            genitive:      "ей",
            dative:        "ей",
            accusative:    Some("юю"),
            instrumental:  "ей",
            prepositional: "ей"
        }
    }
}

/// The plural, which states no gender.
const fn plural(shape: Stem) -> Endings {
    match shape {
        Stem::Hard => Endings {
            nominative:    "ые",
            genitive:      "ых",
            dative:        "ым",
            accusative:    None,
            instrumental:  "ыми",
            prepositional: "ых"
        },
        Stem::Soft => Endings {
            nominative:    "ие",
            genitive:      "их",
            dative:        "им",
            accusative:    None,
            instrumental:  "ими",
            prepositional: "их"
        }
    }
}
