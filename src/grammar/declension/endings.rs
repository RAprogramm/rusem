// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The endings each declension puts on a noun.
//!
//! This is the paradigm itself, the closed system the language declines by.
//! The accusative is deliberately absent from it: outside the first declension
//! Russian has no accusative ending of its own, it repeats the nominative for a
//! thing and the genitive for a living being, and writing it out as a row would
//! be inventing an ending the language does not have.

use super::{Declension, Stem};
use crate::grammar::{Animacy, Case, Gender, Number};

/// The endings of one number of one paradigm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Endings {
    /// Nominative.
    pub nominative:    &'static str,
    /// Genitive.
    pub genitive:      &'static str,
    /// Dative.
    pub dative:        &'static str,
    /// Accusative, when the paradigm states one of its own.
    pub accusative:    Option<&'static str>,
    /// Instrumental.
    pub instrumental:  &'static str,
    /// Prepositional.
    pub prepositional: &'static str
}

impl Endings {
    /// The ending this paradigm puts on a noun in the case asked for.
    ///
    /// Animacy is asked for because the accusative is answered by it whenever
    /// the paradigm states no accusative of its own. The cases the paradigm
    /// does not state a row for read the row of the case they count as — the
    /// partitive the genitive's, the locative the prepositional's, as
    /// [`Case::merged`] says — and the vocative reads the nominative's.
    #[must_use]
    pub fn of(&self, case: Case, animacy: Animacy) -> &'static str {
        match case {
            Case::Nominative | Case::Vocative => self.nominative,
            Case::Genitive | Case::Partitive => self.genitive,
            Case::Dative => self.dative,
            Case::Instrumental => self.instrumental,
            Case::Prepositional | Case::Locative => self.prepositional,
            Case::Accusative => self.accusative.unwrap_or(match animacy {
                Animacy::Animate => self.genitive,
                Animacy::Inanimate => self.nominative
            })
        }
    }
}

/// The endings a noun takes in the number asked for.
///
/// An indeclinable noun has no paradigm, and neither has a gender that the
/// second declension does not tell apart.
#[must_use]
pub const fn table(
    declension: Declension,
    stem: Stem,
    gender: Gender,
    number: Number
) -> Option<Endings> {
    match (declension, number) {
        (Declension::First, Number::Singular) => Some(first_singular(stem)),
        (Declension::First, Number::Plural) => Some(plural(
            stem,
            "",
            match stem {
                Stem::Hard => "ы",
                Stem::Soft => "и"
            }
        )),
        (Declension::Second, Number::Singular) => second_singular(stem, gender),
        (Declension::Second, Number::Plural) => Some(second_plural(stem, gender)),
        (Declension::Third, Number::Singular) => Some(third_singular()),
        (Declension::Third, Number::Plural) => Some(plural(Stem::Soft, "ей", "и")),
        (Declension::Mixed, Number::Singular) => Some(mixed_singular()),
        (Declension::Mixed, Number::Plural) => Some(plural(Stem::Soft, "ён", "а")),
        (Declension::Adjectival, _) => Some(adjectival(stem, gender, number)),
        (Declension::Indeclinable, _) => None
    }
}

/// The eleven mixed nouns in the singular.
///
/// They take the endings of the third declension everywhere but the
/// instrumental, where they take the second: `времени`, `временем`. The
/// growth of `-ен-` before them is not an ending and is added by the caller
/// that builds the stem.
const fn mixed_singular() -> Endings {
    Endings {
        nominative:    "я",
        genitive:      "и",
        dative:        "и",
        accusative:    Some("я"),
        instrumental:  "ем",
        prepositional: "и"
    }
}

/// A noun that declines as an adjective declines as an adjective.
///
/// `мороженое` and `столовая` were adjectives and kept their endings whole, so
/// there is nothing of the noun paradigm in them: the attributive table beside
/// this one is the whole answer.
const fn adjectival(stem: Stem, gender: Gender, number: Number) -> Endings {
    let held = super::attributive::table(stem, gender, number);

    Endings {
        nominative:    held.nominative,
        genitive:      held.genitive,
        dative:        held.dative,
        accusative:    held.accusative,
        instrumental:  held.instrumental,
        prepositional: held.prepositional
    }
}

/// The first declension in the singular.
const fn first_singular(stem: Stem) -> Endings {
    match stem {
        Stem::Hard => Endings {
            nominative:    "а",
            genitive:      "ы",
            dative:        "е",
            accusative:    Some("у"),
            instrumental:  "ой",
            prepositional: "е"
        },
        Stem::Soft => Endings {
            nominative:    "я",
            genitive:      "и",
            dative:        "е",
            accusative:    Some("ю"),
            instrumental:  "ей",
            prepositional: "е"
        }
    }
}

/// The second declension in the singular, which splits by gender.
const fn second_singular(stem: Stem, gender: Gender) -> Option<Endings> {
    match (gender, stem) {
        (Gender::Masculine, Stem::Hard) => Some(Endings {
            nominative:    "",
            genitive:      "а",
            dative:        "у",
            accusative:    None,
            instrumental:  "ом",
            prepositional: "е"
        }),
        (Gender::Masculine, Stem::Soft) => Some(Endings {
            nominative:    "ь",
            genitive:      "я",
            dative:        "ю",
            accusative:    None,
            instrumental:  "ем",
            prepositional: "е"
        }),
        (Gender::Neuter, Stem::Hard) => Some(Endings {
            nominative:    "о",
            genitive:      "а",
            dative:        "у",
            accusative:    Some("о"),
            instrumental:  "ом",
            prepositional: "е"
        }),
        (Gender::Neuter, Stem::Soft) => Some(Endings {
            nominative:    "е",
            genitive:      "я",
            dative:        "ю",
            accusative:    Some("е"),
            instrumental:  "ем",
            prepositional: "е"
        }),
        _ => None
    }
}

/// The third declension in the singular.
const fn third_singular() -> Endings {
    Endings {
        nominative:    "ь",
        genitive:      "и",
        dative:        "и",
        accusative:    Some("ь"),
        instrumental:  "ью",
        prepositional: "и"
    }
}

/// The second declension in the plural, whose nominative splits by gender.
const fn second_plural(stem: Stem, gender: Gender) -> Endings {
    match (gender, stem) {
        (Gender::Neuter, Stem::Hard) => plural(Stem::Hard, "", "а"),
        (Gender::Neuter, Stem::Soft) => plural(Stem::Soft, "ей", "я"),
        (_, Stem::Hard) => plural(Stem::Hard, "ов", "ы"),
        (_, Stem::Soft) => plural(Stem::Soft, "ей", "и")
    }
}

/// The plural, whose oblique endings are one set for the whole language.
///
/// The nominative and the genitive are what the declensions differ in, so
/// both are handed in whole. A genitive handed in empty is the genuine zero
/// ending of `книг` and `окон`, not a blank to be filled: nothing here
/// stands in for a value the caller did not state.
const fn plural(stem: Stem, genitive: &'static str, nominative: &'static str) -> Endings {
    let (dative, instrumental, prepositional) = match stem {
        Stem::Hard => ("ам", "ами", "ах"),
        Stem::Soft => ("ям", "ями", "ях")
    };

    Endings {
        nominative,
        genitive,
        dative,
        accusative: None,
        instrumental,
        prepositional
    }
}
