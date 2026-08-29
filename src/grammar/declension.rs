// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The declension a noun belongs to, and the stem it declines on.
//!
//! Russian declines its nouns in three patterns, and which pattern a noun takes
//! is not a property to be looked up word by word: it follows from the gender
//! and from how the dictionary form ends. A feminine or masculine noun in -а
//! declines the first way, a masculine noun with a bare ending and a neuter in
//! -о the second, a feminine noun in -ь the third. Nothing else is needed, and
//! no noun is named here.
//!
//! Across all three the endings come in two shapes, hard and soft, and the
//! shape follows from the last sound of the stem in the same mechanical way.

pub mod adjective;
pub mod agreed;
pub mod attributive;
pub mod endings;
pub mod index;
pub mod noun;
pub mod numeral;
pub mod pronominal;
pub mod reading;
pub mod spelling;
pub mod stated;

use crate::{
    alphabet::is_consonant,
    grammar::{Gender, stem::Stem}
};

/// The pattern a noun declines by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Declension {
    /// Feminine and masculine nouns whose dictionary form ends in а or я.
    First,
    /// Masculine nouns with a bare ending and neuter nouns in о or е.
    Second,
    /// Feminine nouns whose dictionary form ends in ь.
    Third,
    /// The ten nouns in `-мя`, and `путь`, which take endings from more than
    /// one pattern: `время`, `времени`, `временем`.
    Mixed,
    /// Nouns that decline as adjectives, because they were adjectives:
    /// `мороженое`, `столовая`, `рабочий`.
    Adjectival,
    /// Nouns that take no endings at all.
    Indeclinable
}

/// The eleven nouns that take endings from more than one pattern.
///
/// Ten of them end in `-мя` and grow a `-ен-` before every ending but the
/// nominative and the accusative; `путь` is masculine and declines as a
/// feminine noun in `ь` except in the instrumental, where it says `путём`.
pub const MIXED: &[&str] = &[
    "бремя",
    "время",
    "вымя",
    "знамя",
    "имя",
    "пламя",
    "племя",
    "семя",
    "стремя",
    "темя",
    "путь"
];

/// The letters the growth of a mixed noun in `-мя` is written with.
pub const GROWTH: &str = "ен";

/// The endings a noun of the adjectival declension is written with.
///
/// A noun declines as an adjective when it was one: `мороженое` is a
/// substantivized neuter adjective and takes `-ого`, not `-а`. The endings are
/// the adjectival ones, so the pattern says only that they are used.
const ADJECTIVAL: &[&str] = &["ый", "ий", "ой", "ая", "яя", "ое", "ее", "ые", "ие"];

/// The words in `-ий`, `-ия`, `-ие` whose prepositional takes `и` and not `е`.
///
/// `о гении`, `в армии`, `о собрании` — the stem ends in the glide, and the
/// ending after it does not soften a second time.
#[must_use]
pub fn on_glide(nominative: &str) -> bool {
    ["ий", "ия", "ие"]
        .iter()
        .any(|held| nominative.ends_with(held) && nominative.chars().count() > 3)
}

/// Reports which pattern a noun declines by.
///
/// The closed list comes first, because the eleven mixed nouns look like
/// ordinary ones and are not. Then the adjectival endings, which overrule the
/// gender: `рабочий` is masculine and does not decline as `конь`. What is left
/// is settled by the dictionary form and the gender between them.
#[must_use]
pub fn of(nominative: &str, gender: Gender) -> Declension {
    if MIXED.contains(&nominative) {
        return Declension::Mixed;
    }
    if ADJECTIVAL
        .iter()
        .any(|held| nominative.ends_with(held) && nominative.chars().count() > 3)
        && !on_glide(nominative)
    {
        return Declension::Adjectival;
    }

    let Some(last) = nominative.chars().last() else {
        return Declension::Indeclinable;
    };

    match (last, gender) {
        ('а' | 'я', Gender::Feminine | Gender::Masculine | Gender::Common) => Declension::First,
        ('ь', Gender::Feminine) => Declension::Third,
        ('ь' | 'й' | 'о' | 'е' | 'ё', Gender::Masculine | Gender::Neuter) => {
            Declension::Second
        }
        (held, Gender::Masculine) if is_consonant(held) => Declension::Second,
        _ => Declension::Indeclinable
    }
}

/// Reports whether the noun declines on a hard or a soft stem.
///
/// The vowel of the dictionary form says it outright — а against я, о against
/// е — and a bare masculine form says it by its last consonant, where й and ь
/// are the soft ones.
#[must_use]
pub fn stem(nominative: &str) -> Stem {
    match nominative.chars().last() {
        Some('я' | 'е' | 'ё' | 'ь' | 'й') => Stem::Soft,
        _ => Stem::Hard
    }
}
