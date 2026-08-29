// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What a word is built of, as against how it sounds.
//!
//! A morpheme is the smallest part of a word that means anything. That is what
//! sets this level apart from [`crate::phonetics`]: a syllable is a wave of
//! sound and means nothing, a morpheme means something and may fall anywhere.
//! `водный` divides `во-дный` by sound and `вод-н-ый` by sense, and neither
//! division is a refinement of the other.
//!
//! Russian states much of its meaning here. A reader who has never met
//! `перестройка` still reads it — `пере-` over the root `строй` under `-к-а` —
//! and knows it names a rebuilding rather than a building. That is what this
//! level gives a machine.
//!
//! # Layout
//!
//! | Module | Holds |
//! | --- | --- |
//! | [`affix`] | the tables: prefixes, suffixes, endings, postfixes, interfixes |
//! | [`alternation`] | the consonants a root swaps: `рук-а` against `руч-ной` |
//! | [`derivation`] | what a morpheme is, a word cut into morphemes, and which word a word was built from |
//! | [`root`] | the one root behind its shapes: `рук`/`руч`, `сон`/`сн` |
//! | [`cut`] | the cutting the tables alone can do |
//!
//! # What this level cannot do alone
//!
//! [`cut`] reads the tables and nothing else, so it offers every cut the
//! tables admit and chooses between none of them: `нос` may be a bare root or
//! a root `нос` under no suffix, and `косточка` may be cut at `-очк-` or at
//! `-к-`. Choosing needs a dictionary — whether what remains is a word — and a
//! dictionary is not something the domain reads. The layer above asks.

pub mod affix;
pub mod alternation;
pub mod cut;
pub mod derivation;
pub mod root;

pub use self::derivation::{
    DerivationChain, DerivationStep, DerivationWay, Morpheme, MorphemeKind, Segment, Segmentation
};
