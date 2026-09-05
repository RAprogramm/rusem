// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Reading a written form into the cells of a word, without building its table.
//!
//! There are two ways to ask which cell a form stands in, and the engine needs
//! both. One is to write the whole word out and look for the spelling in it,
//! which is what a table is for. The other is to work from the ending back:
//! take the stem the dictionary form gives, take the endings the pattern
//! gives, and keep every cell whose ending, once the alphabet has bent it,
//! spells what arrived.
//!
//! The second is what a reader does, and it is what this module is. It is also
//! what makes the first checkable: two ways of knowing the same thing, written
//! apart, which must answer the same. Where they part, the core is wrong about
//! the language, and the law beside them says so.

pub mod agreeing;
pub mod verb;

use crate::{
    grammar::{
        Case, Gender, Number,
        declension::{Declension, reading, stated},
        form::{Adjectival, Agreed, Form}
    },
    lexis::{
        lexeme::{Lexeme, Noun, Road},
        word::Word
    },
    morphology::WordForm
};

/// The cells of a word a written form could be standing in.
///
/// # Examples
///
/// ```
/// use rusem::{
///     grammar::{
///         Animacy, Case, Gender,
///         declension::Declension,
///         form::{Agreed, Form}
///     },
///     lexis::{Lexeme, Noun, Word, reading},
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
/// let cells = reading::of(&word, &WordForm::parse("столом")?);
/// assert_eq!(
///     cells,
///     vec![Form::Noun(Agreed::Singular {
///         case:   Case::Instrumental,
///         gender: Gender::Masculine
///     })]
/// );
/// # Ok::<(), rusem::error::CoreError>(())
/// ```
#[must_use]
pub fn of(word: &Word, written: &WordForm) -> Vec<Form> {
    match word.lexeme {
        Lexeme::Noun(held) => noun(word.lemma.as_str(), held, written.as_str()),
        Lexeme::Adjective => agreeing::of(word.lemma.as_str(), written.as_str()),
        Lexeme::Verb(held) => verb::of(word.lemma.as_str(), held, written.as_str()),
        _ => Vec::new()
    }
}

/// The cells of a noun a written form could be standing in.
///
/// A noun that was an adjective is read as one, and its cells are then stated
/// in its own gender: `столовой` is read off the adjectival endings, but the
/// word is a feminine noun and the cell says so.
fn noun(lemma: &str, held: Noun, written: &str) -> Vec<Form> {
    match held.declension {
        Declension::Indeclinable => standing(held.gender, written == lemma),
        Declension::Adjectival => agreeing::of(lemma, written)
            .into_iter()
            .filter_map(|form| nominal(form, held.gender))
            .collect(),
        _ => declined(lemma, held, written)
            .into_iter()
            .map(|cell| Form::Noun(agreed(cell.case, cell.number, held.gender)))
            .collect()
    }
}

/// The cells of a declining noun, read by the road the word was written by.
///
/// Which road that is — the stated index or the worked-out pattern — is not
/// decided here: [`Noun::road`] states it once, for this reader and for the
/// writer alike. Reading a form down one road that was written down the
/// other would make the two directions disagree about words they both know.
fn declined(lemma: &str, held: Noun, written: &str) -> Vec<reading::Cell> {
    match held.road() {
        Road::Stated(index) => {
            stated::reading::cells(written, lemma, held.gender, held.animacy, index)
        }
        Road::Derived => reading::cells(written, lemma, held.gender, held.animacy)
    }
}

/// Every cell an indeclinable noun has, when the form is the word itself.
fn standing(gender: Gender, itself: bool) -> Vec<Form> {
    if !itself {
        return Vec::new();
    }

    Case::STATED
        .into_iter()
        .flat_map(|case| {
            [
                Form::Noun(agreed(case, Number::Singular, gender)),
                Form::Noun(agreed(case, Number::Plural, gender))
            ]
        })
        .collect()
}

/// The nominal cell an adjectival reading names, in the noun's own gender.
fn nominal(form: Form, gender: Gender) -> Option<Form> {
    let Form::Adjective(Adjectival::Full(held)) = form else {
        return None;
    };

    match held {
        Agreed::Singular {
            case,
            gender: stated
        } if stated == gender => Some(Form::Noun(agreed(case, Number::Singular, gender))),
        Agreed::Plural {
            case
        } => Some(Form::Noun(agreed(case, Number::Plural, gender))),
        Agreed::Singular {
            ..
        } => None
    }
}

/// The cell a case, a number and a gender name.
pub(crate) const fn agreed(case: Case, number: Number, gender: Gender) -> Agreed {
    match number {
        Number::Singular => Agreed::Singular {
            case,
            gender
        },
        Number::Plural => Agreed::Plural {
            case
        }
    }
}
