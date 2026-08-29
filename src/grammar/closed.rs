// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The closed classes of Russian: the words that can be listed.
//!
//! A class is closed when a person can write down every member and the list
//! will still be complete in fifty years. Conjunctions, prepositions,
//! particles, pronouns. Nothing new joins them, and that is what makes them
//! grammar rather than vocabulary.
//!
//! This is why they are stated here rather than loaded from a dictionary. A
//! pack of nouns that fails to load leaves the engine knowing fewer words; a
//! pack of conjunctions that fails to load leaves it unable to find a clause.
//! One is a degraded engine and the other is a broken one, and only the first
//! is worth the risk.
//!
//! # One list to a class
//!
//! Every list here is named for the class it holds and appears once. That is
//! the point: before this module the checker held five different lists of
//! coordinating conjunctions under one name, and at most one of them was
//! right. A gate that needs a narrower set says so in its own words and cites
//! this one, rather than quietly keeping a fifth copy.

pub mod adverb;
pub mod asking;
pub mod conjunction;
pub mod copula;
pub mod interjection;
pub mod numeral;
pub mod parenthetical;
pub mod particle;
pub mod predicative;
pub mod preposition;
pub mod pronoun;

pub use self::{
    conjunction::{COORDINATING, SUBORDINATING},
    numeral::{CARDINAL, COLLECTIVE, ORDINAL},
    particle::{DENYING, ENCLITIC, PARTICLES},
    preposition::{PREPOSITIONS, governs},
    pronoun::{Class as PronounClass, DEMONSTRATIVE, PERSONAL, POSSESSIVE}
};

/// Reports whether a written word belongs to any closed class.
///
/// A word that does is a function word: the sense gate skips it, because a
/// dictionary defines `и` and `в` in a way no engine can use, and agreement
/// and government say everything about them that matters.
#[must_use]
pub fn is_function_word(written: &str) -> bool {
    let held = written.to_lowercase();

    [COORDINATING, SUBORDINATING, PARTICLES, PREPOSITIONS]
        .iter()
        .any(|class| class.contains(&held.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_class_is_sorted_and_holds_no_word_twice() {
        for class in [
            COORDINATING,
            SUBORDINATING,
            PARTICLES,
            PREPOSITIONS,
            DEMONSTRATIVE,
            POSSESSIVE,
            PERSONAL
        ] {
            let mut held = class.to_vec();
            held.sort_unstable();
            held.dedup();

            assert_eq!(held.len(), class.len(), "{class:?} holds a word twice");
        }
    }

    #[test]
    fn every_word_of_every_class_is_russian_and_small() {
        for class in [
            COORDINATING,
            SUBORDINATING,
            PARTICLES,
            PREPOSITIONS,
            adverb::DEMONSTRATIVE,
            adverb::ASKING,
            adverb::DEFINITIVE,
            asking::PRONOUNS,
            asking::ADVERBS,
            adverb::POSSESSIVE,
            parenthetical::ALWAYS,
            parenthetical::NEVER,
            parenthetical::EITHER
        ] {
            for held in class {
                assert!(
                    held.chars()
                        .all(|letter| crate::alphabet::is_letter(letter) || letter == '-'),
                    "{held} is no Russian word"
                );
                assert_eq!(*held, held.to_lowercase(), "{held} is not written small");
            }
        }
    }

    #[test]
    fn a_function_word_is_named_as_one() {
        assert!(is_function_word("и"));
        assert!(is_function_word("в"));
        assert!(is_function_word("не"));
        assert!(is_function_word("чтобы"));
        assert!(is_function_word("И"));
    }

    #[test]
    fn a_word_that_means_something_is_not_a_function_word() {
        assert!(!is_function_word("стол"));
        assert!(!is_function_word("читать"));
        assert!(!is_function_word(""));
    }
}
