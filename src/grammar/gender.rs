// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Gender and animacy: what a nominal is.

/// Grammatical gender.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Gender {
    /// Masculine.
    Masculine,
    /// Feminine.
    Feminine,
    /// Neuter.
    Neuter,
    /// Common gender, resolved by context.
    Common
}

impl Gender {
    /// The genders a paradigm states a cell for, in the order the tables
    /// walk them.
    ///
    /// Three of the four: the common gender is resolved by the context a word
    /// stands in — `сирота` declines the same whichever it resolves to — so
    /// no table holds a row of its own for it. Every table walks its genders
    /// in this order, so the order is stated beside the genders rather than
    /// once per table.
    pub const STATED: [Self; 3] = [Self::Masculine, Self::Feminine, Self::Neuter];
}

/// Animacy of a nominal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Animacy {
    /// Animate.
    Animate,
    /// Inanimate.
    Inanimate
}
