// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Every form the core writes is spelled the way the code of 1956 requires.
//!
//! This is the law that has two halves of the engine grade each other. One
//! half declines and conjugates: it knows stems, endings and how a letter
//! bends where they meet. The other half is the code of orthography, written
//! paragraph by paragraph and knowing nothing of paradigms. Neither was
//! written from the other, and a form that the first writes and the second
//! refuses means one of them is wrong about Russian.
//!
//! It is also how the paragraphs get asked at all. A rule needs the form a
//! word stands in, and until the core could write a word out, that form came
//! from an analyzer outside. Now the word states what it is, the core writes
//! it, and the rule is asked on facts the core derived — nothing is handed in
//! from outside for it to trust.
//!
//! A rule that asks for a stress is asked with the stress unknown, and a rule
//! that cannot answer without one stays silent. That silence is honest: the
//! core does not hold the stress yet, and a law must not report as an error
//! what it has not got the facts to judge.

use super::{Broken, Law, Says};
use crate::{
    lexis::Word,
    phonetics::stress::Stressed,
    rules::{
        Facts,
        facts::{About, Writing},
        rule,
        svod::written::RULES
    }
};

/// The law that what the core writes, the code admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spelled;

impl Law for Spelled {
    fn states(&self) -> &'static str {
        "every form the core writes is spelled as the code of 1956 requires"
    }

    fn broken(&self, word: &Word) -> Vec<Broken> {
        let stress = Stressed::unknown();
        let mut found = Vec::new();

        for (cell, spellings) in &word.paradigm().cells {
            for written in spellings {
                let facts = Facts {
                    writing: Writing {
                        written: written.as_str(),
                        next:    None
                    },
                    about:   About {
                        form:   *cell,
                        stress: &stress,
                        native: word.native,
                        proper: word.proper,
                        parts:  word.parts
                    }
                };

                found.extend(refused(&facts).map(|law| Broken {
                    cell: *cell,
                    written: written.clone(),
                    law
                }));
            }
        }

        found
    }
}

/// The paragraph that refuses this form, if one does.
///
/// The rule judges; this only asks. A rule that finds nothing has judged the
/// form and let it through, and a rule the scope keeps out was never asked at
/// all — the two are told apart by the scope, not by the answer.
fn refused(facts: &Facts<'_>) -> Option<Says> {
    RULES.iter().find_map(|held| {
        rule::asked(*held, facts)
            .first()
            .map(|one| Says::Misspelled(one.cites))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        grammar::{Animacy, Gender, declension::Declension},
        lexis::{Lexeme, Noun},
        morphology::WordForm
    };

    fn word(written: &str) -> Word {
        Word::new(
            WordForm::parse(written).unwrap_or_else(|_| unreachable!("a written Russian word")),
            Lexeme::Noun(Noun {
                gender:     Gender::Masculine,
                animacy:    Animacy::Inanimate,
                declension: Declension::Second,
                index:      None
            })
        )
    }

    #[test]
    fn an_ordinary_word_is_spelled_as_the_code_requires() {
        assert!(Spelled.broken(&word("стол")).is_empty());
    }

    #[test]
    fn a_form_the_code_refuses_is_reported_with_the_paragraph_that_refuses_it() {
        let held = Spelled.broken(&word("розиск").prefixed(3));
        let first = held.first().map(|one| one.law);

        assert!(!held.is_empty());
        assert!(matches!(first, Some(Says::Misspelled(cites)) if cites.paragraph == 7));
    }

    #[test]
    fn the_same_word_with_its_build_unknown_is_left_alone() {
        assert!(Spelled.broken(&word("розиск")).is_empty());
    }

    #[test]
    fn a_paragraph_that_needs_a_stress_stays_silent_without_one() {
        assert!(Spelled.broken(&word("шопот").bare()).is_empty());
    }
}
