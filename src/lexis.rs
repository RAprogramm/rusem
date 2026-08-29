// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The word, as against the form of a word.
//!
//! `стол`, `стола`, `столу` are three forms and one word. The level above the
//! morphemes is where that one word lives: what it is whatever form it takes,
//! and which forms it has.
//!
//! Two things are held apart on purpose and it is worth saying why. A
//! [`lexeme::Lexeme`] is what the word *is* — a noun of this gender, a verb of
//! this aspect — and does not change. A [`paradigm::Paradigm`] is what the
//! word *has*: a cell for every form the language gives it, and the spelling
//! that fills each cell. One is a fact about the word, the other a table over
//! it, and a type that mixed them would let a word have a gender in one cell
//! and another gender in the next.
//!
//! # Layout
//!
//! | Module | Holds |
//! | --- | --- |
//! | [`lexeme`] | what a word is, whatever form it takes |
//! | [`paradigm`] | the cells a word has and what is written in them |
//! | [`reading`] | the way back: which cells a written form fills |
//! | [`word`] | the lexeme and its spelling held together |

pub mod lexeme;
pub mod paradigm;
pub mod reading;
pub mod word;

pub use self::{
    lexeme::{Lexeme, Noun, Road, Verb},
    paradigm::Paradigm,
    word::Word
};
