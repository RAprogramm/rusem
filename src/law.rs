// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What cannot be otherwise, whatever word is asked.
//!
//! A rule of the code of 1956 says how a word is written. A law here says
//! something about the language itself that no word can break — and if a word
//! the core builds breaks one, it is the core that is wrong, not the word.
//! That is what makes these worth having: they are the only check the engine
//! can run on itself, needing no corpus, no dictionary and no one's judgement.
//!
//! Four of them so far. Two are about the core knowing one thing rather than
//! two, one has the code of 1956 judge what the tables write, and one asks
//! whether two words the core writes can stand together. The core writes a form
//! from a cell and reads a cell from a form, and those are the same knowledge
//! seen from two sides: if what it writes is not what it reads, it holds two
//! answers where the language holds one. And the categories a word owns — the
//! gender of a noun, the class it belongs to — cannot come out different in one
//! cell than in another, because they belong to the word and not to the form.
//!
//! # How a law is asked
//!
//! Every law takes a word and returns where that word breaks it, which is
//! usually nowhere. A law that has nothing to say about a word says nothing:
//! silence is the ordinary answer, and a law that fires on every word is a
//! broken law rather than a broken language.

pub mod held;
pub mod mirrored;
pub mod spelled;
pub mod steady;

use crate::{grammar::form::Form, lexis::Word, morphology::WordForm, rules::Citation};

/// What a law found the core doing wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Broken {
    /// The cell the core wrote.
    pub cell:    Form,
    /// What it wrote there.
    pub written: WordForm,
    /// Which law it broke.
    pub law:     Says
}

/// What a law says was wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Says {
    /// The form the core wrote does not read back into the cell that wrote it.
    ///
    /// One of the two directions is wrong and the law does not say which: it
    /// says they disagree, which is enough to know the core is not holding one
    /// piece of knowledge.
    Unread,
    /// The cell states a category the word itself does not have.
    ///
    /// A noun is masculine in every cell or it is not masculine; a cell of one
    /// word cannot belong to another class than the word does.
    Astray,
    /// Two words the language holds together are not held together.
    ///
    /// Either a dependent in the head's own cell was refused, or one in
    /// another cell was allowed. Both are the same mistake: a dependency that
    /// does not tell the two apart is not a dependency.
    Unheld,
    /// The form is spelled the way a paragraph of the code refuses.
    ///
    /// The paragraph is named: a law that says a form is wrong without saying
    /// what refuses it cannot be argued with, and this one can be — either the
    /// declension is wrong or the paragraph is.
    Misspelled(Citation),
    /// The law could not build the word it checks against, so this cell went
    /// unchecked.
    ///
    /// The probe is the core's own writing, not the word's: a core that
    /// cannot write it has already gone wrong, and the cells it left
    /// unchecked are named rather than passed in silence — a self-check that
    /// swallowed its own breakage would pass every word exactly when it was
    /// most wrong.
    Unasked
}

/// A law of the language, as the core checks itself against it.
pub trait Law: Sync {
    /// What the law says, in the shortest words that say it.
    fn states(&self) -> &'static str;

    /// Where the word breaks it.
    fn broken(&self, word: &Word) -> Vec<Broken>;
}

/// Every law the core knows.
#[must_use]
pub fn all() -> Vec<&'static dyn Law> {
    std::vec![
        &mirrored::Mirrored as &dyn Law,
        &steady::Steady as &dyn Law,
        &spelled::Spelled as &dyn Law,
        &held::Held as &dyn Law
    ]
}

/// Everything the core does wrong on one word.
///
/// # Examples
///
/// ```
/// use rusem::{
///     grammar::{Animacy, Gender, declension::Declension},
///     law,
///     lexis::{Lexeme, Noun, Word},
///     morphology::WordForm
/// };
///
/// let word = Word::new(
///     WordForm::parse("книга")?,
///     Lexeme::Noun(Noun {
///         gender:     Gender::Feminine,
///         animacy:    Animacy::Inanimate,
///         declension: Declension::First,
///         index:      None
///     })
/// );
///
/// assert!(law::breaks(&word).is_empty());
/// # Ok::<(), rusem::error::CoreError>(())
/// ```
#[must_use]
pub fn breaks(word: &Word) -> Vec<Broken> {
    all().into_iter().flat_map(|law| law.broken(word)).collect()
}
