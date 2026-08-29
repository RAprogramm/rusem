// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Two words the core writes into one cell stand together.
//!
//! The other laws ask about a word alone. This one asks about the joint, and
//! it is the joint that a sentence is made of: an adjective in the cell the
//! noun is in must agree with it, and an adjective in any other cell must not.
//!
//! Both halves matter, and the second is the one that catches a dependency
//! worth nothing. A rule that answered yes to every pair would let `нового
//! стол` through and would still pass any test that only tried correct
//! phrases; so the law tries the wrong pairing too and demands a no.
//!
//! What is being checked is the core against itself again. The cells come from
//! the tables the core builds, the answer from the dependency the core states,
//! and the two were written apart: the tables know nothing of agreement, and
//! agreement knows nothing of endings.

use super::{Broken, Law, Says};
use crate::{
    grammar::{
        dependency,
        form::{Adjectival, Agreed, Form}
    },
    lexis::{Lexeme, Word},
    morphology::WordForm
};

/// The word every noun is tried against.
///
/// An adjective of the hard shape, whose whole paradigm the core writes: what
/// the law needs is a second word to stand beside the first, and a word that
/// declines by the ordinary adjectival pattern is the plainest one there is.
const AGREEING: &str = "новый";

/// The law that a word and its dependent stand together, and only together.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Held;

impl Law for Held {
    fn states(&self) -> &'static str {
        "an agreeing word stands with the noun in its cell and with no other"
    }

    fn broken(&self, word: &Word) -> Vec<Broken> {
        let Some(other) = agreeing() else {
            return unasked(word);
        };
        let mut found = Vec::new();

        for (cell, spellings) in &word.paradigm().cells {
            let Some(held) = dependency::agreeing(*cell) else {
                continue;
            };
            if !matches!(cell, Form::Noun(_)) {
                continue;
            }
            for (mine, theirs) in other.paradigm().cells.iter().map(|(one, _)| (*cell, *one)) {
                if wrong(mine, theirs, held) {
                    found.extend(spellings.iter().map(|written| Broken {
                        cell:    *cell,
                        written: written.clone(),
                        law:     Says::Unheld
                    }));
                    break;
                }
            }
        }

        found
    }
}

/// Reports whether the pairing of two cells is judged the wrong way round.
///
/// The same cell must be held, a different one must not. `Agreed` is what the
/// cells state, so the two are compared by that and not by the forms, which
/// differ in class and always would.
fn wrong(noun: Form, other: Form, held: Agreed) -> bool {
    let Form::Adjective(Adjectival::Full(theirs)) = other else {
        return false;
    };
    let stands = dependency::agrees(noun, other);

    stands != same(held, theirs)
}

/// Reports whether two cells state the same case, number and gender.
fn same(held: Agreed, theirs: Agreed) -> bool {
    match (held, theirs) {
        (
            Agreed::Singular {
                case,
                gender
            },
            Agreed::Singular {
                case: theirs,
                gender: stated
            }
        ) => case.merged() == theirs.merged() && gender == stated,
        (
            Agreed::Plural {
                case
            },
            Agreed::Plural {
                case: theirs
            }
        ) => case.merged() == theirs.merged(),
        _ => false
    }
}

/// The cells the law meant to check, each named unchecked.
///
/// A law whose probe the core cannot write has checked nothing, and silence
/// there would pass every word exactly when the core is most wrong; so every
/// cell the loop above would have judged is returned as [`Says::Unasked`]
/// instead of being waved through.
fn unasked(word: &Word) -> Vec<Broken> {
    word.paradigm()
        .cells
        .iter()
        .filter(|(cell, _)| matches!(cell, Form::Noun(_)) && dependency::agreeing(*cell).is_some())
        .flat_map(|(cell, spellings)| {
            spellings.iter().map(|written| Broken {
                cell:    *cell,
                written: written.clone(),
                law:     Says::Unasked
            })
        })
        .collect()
}

/// The word every noun is tried against, written out.
///
/// `None` cannot happen for a fixed Russian word, and the caller does not
/// quietly assume so: a probe that fails to build turns into [`unasked`]
/// findings rather than into silence.
fn agreeing() -> Option<Word> {
    WordForm::parse(AGREEING)
        .ok()
        .map(|lemma| Word::new(lemma, Lexeme::Adjective))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        grammar::{Animacy, Case, Gender, declension::Declension},
        lexis::Noun
    };

    fn word(written: &str, gender: Gender, declension: Declension) -> Word {
        Word::new(
            WordForm::parse(written).unwrap_or_else(|_| unreachable!("a written Russian word")),
            Lexeme::Noun(Noun {
                gender,
                animacy: Animacy::Inanimate,
                declension,
                index: None
            })
        )
    }

    #[test]
    fn a_noun_stands_with_the_adjective_in_its_own_cell_and_no_other() {
        for held in [
            word("стол", Gender::Masculine, Declension::Second),
            word("книга", Gender::Feminine, Declension::First),
            word("время", Gender::Neuter, Declension::Mixed)
        ] {
            assert_eq!(Held.broken(&held), Vec::new(), "{}", held.lemma.as_str());
        }
    }

    #[test]
    fn the_law_is_tried_against_a_word_the_core_can_write() {
        let other = agreeing().map(|held| held.paradigm().len());

        assert_eq!(other, Some(24));
    }

    #[test]
    fn a_probe_that_cannot_be_built_names_the_unchecked_cells() {
        let held = word("стол", Gender::Masculine, Declension::Second);
        let named = unasked(&held);

        assert!(!named.is_empty());
        assert!(named.iter().all(|broken| broken.law == Says::Unasked));
    }

    #[test]
    fn the_pairing_of_one_cell_with_another_is_judged_apart() {
        let genitive = Agreed::Singular {
            case:   Case::Genitive,
            gender: Gender::Feminine
        };
        let dative = Agreed::Singular {
            case:   Case::Dative,
            gender: Gender::Feminine
        };

        assert!(same(genitive, genitive));
        assert!(!same(genitive, dative));
    }
}
