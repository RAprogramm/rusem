// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! A word: how the dictionary writes it and what it is.
//!
//! The two are useless apart. `стол` alone does not say whether it is a noun
//! or the stem of a verb; a masculine inanimate noun of the second declension
//! is not a word until something is written. Together they are everything the
//! core needs to write out the whole word and to read any form of it back, and
//! nothing else in the engine has to be consulted for that.

use crate::{
    grammar::form::Form,
    lexis::{Lexeme, Paradigm, paradigm::word},
    morphology::WordForm,
    rules::Parts
};

/// A word, as the dictionary holds it.
///
/// Two of the four fields are there because the code of 1956 asks for them and
/// the spelling cannot answer. Whether a word is Russian or a borrowing
/// decides `о` against `е` after a sibilant — `шёпот` against `шоколад` — and
/// whether it names one particular thing decides `ы` against `и` after `ц`:
/// `цыган` is a word, `Цицин` a name. Neither follows from the letters, so
/// both are stated by the word.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Word {
    /// How the dictionary writes it.
    pub lemma:  WordForm,
    /// What it is.
    pub lexeme: Lexeme,
    /// Whether the word is Russian rather than a borrowing.
    pub native: bool,
    /// Whether the word names one particular thing.
    pub proper: bool,
    /// How the word is put together, as far as anything knows.
    pub parts:  Parts
}

impl Word {
    /// Builds an ordinary Russian word: one of the language's own, naming a
    /// kind of thing rather than one particular thing.
    #[must_use]
    pub const fn new(lemma: WordForm, lexeme: Lexeme) -> Self {
        Self {
            lemma,
            lexeme,
            native: true,
            proper: false,
            parts: Parts::Unknown
        }
    }

    /// The same word, said to be a borrowing.
    #[must_use]
    pub const fn borrowed(mut self) -> Self {
        self.native = false;
        self
    }

    /// The same word, said to name one particular thing.
    #[must_use]
    pub const fn named(mut self) -> Self {
        self.proper = true;
        self
    }

    /// The same word, said to be built with a prefix that many letters long.
    #[must_use]
    pub const fn prefixed(mut self, letters: usize) -> Self {
        self.parts = Parts::Prefixed(letters);
        self
    }

    /// The same word, said to be built without a prefix.
    #[must_use]
    pub const fn bare(mut self) -> Self {
        self.parts = Parts::Bare;
        self
    }

    /// Every form the word has.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::{
    ///     grammar::{Animacy, Gender, declension::Declension},
    ///     lexis::{Lexeme, Noun, Word},
    ///     morphology::WordForm
    /// };
    ///
    /// let word = Word::new(
    ///     WordForm::parse("стол")?,
    ///     Lexeme::Noun(Noun {
    ///         gender:     Gender::Masculine,
    ///         animacy:    Animacy::Inanimate,
    ///         declension: Declension::Second,
    ///         index:      None
    ///     })
    /// );
    ///
    /// assert_eq!(word.paradigm().len(), 12);
    /// # Ok::<(), rusem::error::CoreError>(())
    /// ```
    #[must_use]
    pub fn paradigm(&self) -> Paradigm {
        word::of(&self.lemma, self.lexeme)
    }

    /// The cells of this word a written form could be standing in.
    #[must_use]
    pub fn cells(&self, written: &WordForm) -> Vec<Form> {
        word::cells(written, &self.lemma, self.lexeme)
    }
}
