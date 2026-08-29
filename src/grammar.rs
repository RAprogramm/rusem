// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Grammatical categories of Russian, normalized away from any one tagset.
//!
//! Analyzers speak their own dialects — `OpenCorpora`, Universal Dependencies,
//! Mystem. Adapters translate into the categories below, so the reasoning
//! layers never depend on the tagset an adapter happens to use.
//!
//! One system to a module: what a word is, what case it stands in, how many,
//! of what gender, in what aspect, at what time — and the tag that carries all
//! of them at once.

pub mod aspect;
pub mod case;
pub mod closed;
pub mod conjugation;
pub mod declension;
pub mod dependency;
pub mod form;
pub mod gender;
pub mod number;
pub mod part;
pub mod stem;
pub mod tag;
pub mod tense;

pub use self::{
    aspect::{Aspect, Transitivity, Voice},
    case::Case,
    gender::{Animacy, Gender},
    number::Number,
    part::PartOfSpeech,
    tag::GrammarTag,
    tense::{Mood, Person, Tense}
};
