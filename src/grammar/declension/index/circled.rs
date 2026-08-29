// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The circled numerals of an index, each an instruction about one cell.
//!
//! Zaliznyak's `Грамматический словарь` prints them as ①, ② and ③, and a
//! dictionary that cannot print a circle writes them in parentheses: `дом`
//! is `1c(1)` and `глаз` is `1c(1)(2)`. Each numeral names one cell of the
//! paradigm and says that the cell takes its ending from the other pattern —
//! `по чужому образцу`, as the dictionary's own introduction puts it.
//!
//! ① is the nominative plural: a masculine writes the neuter row's `-а́` —
//! `дома́`, `снега́` — and a neuter writes the masculine row's `-ы` —
//! `я́блоки`. ② is the genitive plural the same way round: a masculine
//! writes the neuter's bare stem — `сапо́г`, `глаз` — and a neuter writes the
//! masculine's `-ов`. ③ is the prepositional singular of the stems the index
//! writes `7`, which trade their `-ии` for `-е`: `о Баби́е`.
//!
//! A numeral in doubled parentheses — `баклажан` is `1a((2))`, `жвало` is
//! `1a((1))` — says the departure is optional and both forms live:
//! `баклажа́нов` beside `баклажа́н`, `жва́ла` beside `жва́лы`. The pattern's
//! own form is one of the two, so writing by the pattern stays right, and the
//! numeral is kept as the fact that the other form lives too.

/// How firmly a circled numeral holds its cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Reach {
    /// `(N)`: the cell is the other pattern's and nothing else.
    Whole,
    /// `((N))`: both forms live, the pattern's own and the other's.
    Either
}

/// The circled numerals one index carries.
///
/// Each field is one numeral, absent when the index does not write it. An
/// index may carry several at once — `глаз` is `1c(1)(2)` — which is why they
/// are three facts rather than one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circled {
    /// ①: the nominative plural by the other pattern.
    pub nominative:    Option<Reach>,
    /// ②: the genitive plural by the other pattern.
    pub genitive:      Option<Reach>,
    /// ③: the prepositional singular written `-е` against the `-и` of the
    /// stems the index writes `7`.
    pub prepositional: Option<Reach>
}

impl Circled {
    /// A record with no numerals, which is what most indexes carry.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            nominative:    None,
            genitive:      None,
            prepositional: None
        }
    }
}
