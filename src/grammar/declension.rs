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

impl Declension {
    /// The eleven nouns that take endings from more than one pattern.
    ///
    /// Ten of them end in `-мя` and grow a `-ен-`: in the singular before every
    /// ending but the nominative and the accusative — `время`, `времени` — and
    /// in the plural throughout, except the genitive, whose ending carries
    /// the growth in itself — `времена`, `временам`, but `времён`. `путь`
    /// is masculine and declines as a feminine noun in `ь` except in the
    /// instrumental, where it says `путём`; it grows nothing.
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
}

/// The endings only the adjectival declension writes.
///
/// A noun declines as an adjective when it was one: `мороженое` is a
/// substantivized neuter adjective and takes `-ого`, not `-а`. The dictionary
/// form betrays that history only where no noun paradigm spells the same
/// ending. `-ый` is such an ending, because the second declension writes a
/// bare stem before its final `й` and no noun stem ends in `ы`; so are `-ое`
/// and `-ее`, where a neuter noun folds the glide into `-ьё` or `-ие` instead
/// — `ружьё`, `житие`; so is `-яя`, which would ask the first declension for
/// a stem ending in `я` itself; and so is the plural `-ые` of the nouns that
/// have no singular, `чаевые`. The endings a noun also writes prove nothing
/// and are not here: `-ой` is `герой`, `-ий` is `гений`, `-ая` is `стая`,
/// `-ие` is `собрание`.
const ADJECTIVAL: &[&str] = &["ый", "ое", "ее", "яя", "ые"];

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
/// ordinary ones and are not. Then the endings only the adjectival declension
/// writes. What is left is settled by the dictionary form and the gender
/// between them.
///
/// A substantivized adjective whose ending a noun also writes cannot be told
/// from a noun by its form: `рабочий` ends as `гений` does, `выходной` as
/// `герой`, `столовая` as `стая`. The form is the only fact held here, so
/// such a word is read as the noun it is spelled like; naming it adjectival
/// takes a dictionary, and a word a dictionary holds states its declension
/// outright rather than asking here.
#[must_use]
pub fn of(nominative: &str, gender: Gender) -> Declension {
    if Declension::MIXED.contains(&nominative) {
        return Declension::Mixed;
    }
    if ADJECTIVAL
        .iter()
        .any(|held| nominative.ends_with(held) && nominative.chars().count() > 3)
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
