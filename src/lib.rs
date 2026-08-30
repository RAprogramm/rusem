// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Domain vocabulary of the Russian semantic engine.
//!
//! This crate states what the engine knows about and how the parts of that
//! knowledge fit together: word forms and their readings, the morphemes a word
//! is built from, the senses a dictionary lists, the relations between those
//! senses, the participants a predicate governs, and the verdict a phrase gets
//! when it is checked against all of it.
//!
//! Nothing here reads a file, opens a socket or calls a model. Every fact
//! arrives wrapped in [`evidence::Evidenced`], which pairs it with the source
//! that stated it — the engine has no path that produces an unsourced claim,
//! and that is the property the layers above are built to preserve.
//!
//! # The levels of the language
//!
//! The modules are stacked the way linguists stack the language itself, and
//! each level stands on the one below and knows nothing of the one above.
//!
//! | Level | Module | Unit |
//! | --- | --- | --- |
//! | writing | [`alphabet`] | the letter, the stress mark, the Latin twin |
//! | sound | [`phonetics`] | the syllable, the stress, the loudness of a sound |
//! | structure | [`morphemics`] | the morpheme, the cut of a word, the root behind its shapes |
//! | the word | [`lexis`] | the lexeme and its paradigm |
//! | its categories | [`grammar`] | case, number, tense, the form, the declension, the conjugation |
//! | the sentence | [`syntax`] | the dependency tree |
//!
//! A syllable is not a smaller morpheme and a morpheme is not a bigger
//! syllable: `водный` divides `во-дный` by sound and `вод-н-ый` by sense, and
//! neither boundary is a refinement of the other. So [`phonetics`] and
//! [`morphemics`] do not speak to each other, and both speak to
//! [`alphabet`].
//!
//! # What the levels are for
//!
//! | Module | Holds |
//! | --- | --- |
//! | [`rules`] | the code of 1956, one paragraph to a module |
//! | [`morphology`] | the reading an analyzer gives a written form |
//! | [`sense`] | dictionary senses, semantic classes, registers, idioms |
//! | [`relation`] | the relation inventory of the semantic network |
//! | [`frame`] | predicate frames, slots, selectional restrictions |
//! | [`evidence`] | sources, provenance, confidence |
//! | [`verdict`] | violations and the outcome of a check |
//! | [`engine`] | the checker that runs the rules over a phrase |
//! | [`ports`] | the interfaces the knowledge layers implement |
//! | [`id`] | stable identifiers of lemmas, senses, morphemes, frames, sources |
//! | [`error`] | failures of the domain layer |
//!
//! # Examples
//!
//! Reading a form and refusing what is not a word:
//!
//! ```
//! use rusem::morphology::WordForm;
//!
//! assert_eq!(WordForm::parse(" Столом ")?.as_str(), "столом");
//! assert!(WordForm::parse("table").is_err());
//! # Ok::<(), rusem::error::CoreError>(())
//! ```

pub mod alphabet;
pub mod engine;
pub mod error;
pub mod evidence;
pub mod frame;
pub mod grammar;
pub mod id;
pub mod law;
pub mod lexis;
pub mod morphemics;
pub mod morphology;
pub mod phonetics;
pub mod ports;
pub mod relation;
pub mod rules;
pub mod sense;
pub mod syntax;
pub mod verdict;
