// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What belongs to a word rather than to any one of its forms.
//!
//! A noun does not change its gender when it changes case; a verb does not
//! change its aspect when it changes tense. These are facts about the word, and
//! stating them on every form — which is what a tag does — invites two forms of
//! the same word to disagree about them.
//!
//! So the categories are split in two. [`crate::grammar::form::Form`] holds
//! what varies from form to form; [`Lexeme`] holds what does not. Between them
//! they say everything a tag says, and they cannot contradict each other,
//! because neither states what the other states.

use crate::grammar::{
    Animacy, Aspect, Gender, Transitivity,
    conjugation::{Conjugation, index::VerbIndex},
    declension::{Declension, index::Index}
};

/// What a verb is, whatever form it takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Verb {
    /// Whether the action is put as bounded or as running.
    pub aspect:       Aspect,
    /// Whether the verb takes a direct object.
    pub transitivity: Transitivity,
    /// Whether the verb carries `-ся`.
    pub reflexive:    bool,
    /// Which set of personal endings the verb takes.
    pub conjugation:  Conjugation,
    /// The index a dictionary states for the verb, in Zaliznyak's notation.
    ///
    /// `Some` when a dictionary has spoken: the index then settles the finite
    /// forms — the class, the stress, the departures — and they are written
    /// from it exactly. `None` when no dictionary has spoken and the class
    /// must be derived from the infinitive by § 44 of the 1956 code, which is
    /// a rule of thumb rather than a fact about the word.
    pub index:        Option<VerbIndex>
}

impl Verb {
    /// Reports whether the verb can form a passive participle.
    ///
    /// Only a transitive verb can: a passive participle names the one the
    /// action is done to, and an intransitive verb has no such one. A
    /// reflexive verb is intransitive by its `-ся` whatever else it is.
    #[must_use]
    pub const fn takes_passive(self) -> bool {
        !self.reflexive && matches!(self.transitivity, Transitivity::Transitive)
    }

    /// Reports whether the verb has a present tense.
    ///
    /// A perfective verb does not: its present-shaped forms are the simple
    /// future. That is why `прочитает` is not `is reading` but `will read`,
    /// and why a perfective verb has no present participle.
    #[must_use]
    pub const fn has_present(self) -> bool {
        matches!(self.aspect, Aspect::Imperfective)
    }

    /// The road this verb's finite forms go by, stated once for both
    /// directions.
    #[must_use]
    pub const fn road(&self) -> Road<VerbIndex> {
        match self.index {
            Some(index) => Road::Stated(index),
            None => Road::Derived
        }
    }
}

/// What a noun is, whatever case it stands in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Noun {
    /// The gender the noun makes its agreeing words take.
    pub gender:     Gender,
    /// Whether the noun names something animate, which the accusative shows.
    pub animacy:    Animacy,
    /// Which pattern the noun declines by.
    pub declension: Declension,
    /// The index a dictionary states for the noun, in Zaliznyak's notation.
    ///
    /// `Some` when a dictionary has spoken: the index then settles the whole
    /// paradigm — the stress, the fleeting vowel, the `ё` — and the forms are
    /// written from it exactly. `None` when no dictionary has spoken and the
    /// pattern must be derived from the gender and the spelling, which is a
    /// rule of thumb rather than a fact about the word.
    pub index:      Option<Index>
}

/// The road a changing word's forms go by.
///
/// A stated word goes by the index its dictionary writes — a noun's
/// declension index, a verb's conjugation index — and a derived one by the
/// pattern worked out from its dictionary form. The decision is a fact about
/// the word, not about a direction: the writer fills the paradigm down this
/// road and the reader reads forms back down the same one. A form written
/// down one road and read down the other would let the two directions
/// disagree about a word they both know.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Road<Stated> {
    /// A dictionary has spoken, and the index settles the paradigm.
    Stated(Stated),
    /// No dictionary has spoken: the pattern is worked out from what the word
    /// is and its dictionary form.
    Derived
}

impl Noun {
    /// The road this noun's forms go by, stated once for both directions.
    #[must_use]
    pub const fn road(&self) -> Road<Index> {
        match self.index {
            Some(index) => Road::Stated(index),
            None => Road::Derived
        }
    }
}

/// What a word is, whatever form of it is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Lexeme {
    /// A noun.
    Noun(Noun),
    /// An adjective, which takes its categories from the noun it describes and
    /// carries none of its own.
    Adjective,
    /// A verb, including its participles.
    Verb(Verb),
    /// A numeral.
    Numeral,
    /// A pronoun.
    Pronoun,
    /// An adverb.
    Adverb,
    /// A preposition, a conjunction, a particle or an interjection: a word
    /// that has no forms and therefore no categories of its own.
    Function
}

impl Lexeme {
    /// The gender the word makes its agreeing words take, which only a noun
    /// states.
    ///
    /// An adjective has a gender in every form and none as a word: `красный`
    /// and `красная` are the same word, and the gender is the noun's.
    #[must_use]
    pub const fn gender(self) -> Option<Gender> {
        match self {
            Self::Noun(held) => Some(held.gender),
            _ => None
        }
    }

    /// Whether the word names something animate, which only a noun states.
    #[must_use]
    pub const fn animacy(self) -> Option<Animacy> {
        match self {
            Self::Noun(held) => Some(held.animacy),
            _ => None
        }
    }

    /// How the action is put, which only a verb states.
    #[must_use]
    pub const fn aspect(self) -> Option<Aspect> {
        match self {
            Self::Verb(held) => Some(held.aspect),
            _ => None
        }
    }

    /// Whether the word takes a direct object, which only a verb states.
    #[must_use]
    pub const fn transitivity(self) -> Option<Transitivity> {
        match self {
            Self::Verb(held) => Some(held.transitivity),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read() -> Verb {
        Verb {
            aspect:       Aspect::Imperfective,
            transitivity: Transitivity::Transitive,
            reflexive:    false,
            conjugation:  Conjugation::First,
            index:        None
        }
    }

    #[test]
    fn a_noun_states_its_gender_and_an_adjective_does_not() {
        let noun = Lexeme::Noun(Noun {
            gender:     Gender::Feminine,
            animacy:    Animacy::Inanimate,
            declension: Declension::First,
            index:      None
        });

        assert_eq!(noun.gender(), Some(Gender::Feminine));
        assert_eq!(Lexeme::Adjective.gender(), None);
    }

    #[test]
    fn a_verb_states_its_aspect_and_a_noun_does_not() {
        assert_eq!(Lexeme::Verb(read()).aspect(), Some(Aspect::Imperfective));
        assert_eq!(Lexeme::Adjective.aspect(), None);
    }

    #[test]
    fn a_noun_states_its_animacy_and_an_adverb_does_not() {
        let noun = Lexeme::Noun(Noun {
            gender:     Gender::Masculine,
            animacy:    Animacy::Animate,
            declension: Declension::Second,
            index:      None
        });

        assert_eq!(noun.animacy(), Some(Animacy::Animate));
        assert_eq!(Lexeme::Adverb.animacy(), None);
    }

    #[test]
    fn a_verb_states_its_transitivity_and_a_pronoun_does_not() {
        assert_eq!(
            Lexeme::Verb(read()).transitivity(),
            Some(Transitivity::Transitive)
        );
        assert_eq!(Lexeme::Pronoun.transitivity(), None);
        assert_eq!(Lexeme::Function.transitivity(), None);
    }

    #[test]
    fn a_perfective_verb_has_no_present() {
        let done = Verb {
            aspect: Aspect::Perfective,
            ..read()
        };

        assert!(read().has_present());
        assert!(!done.has_present());
    }

    #[test]
    fn a_reflexive_verb_forms_no_passive_participle() {
        let turned = Verb {
            reflexive: true,
            ..read()
        };

        assert!(read().takes_passive());
        assert!(!turned.takes_passive());
    }

    #[test]
    fn an_intransitive_verb_forms_no_passive_participle() {
        let standing = Verb {
            transitivity: Transitivity::Intransitive,
            ..read()
        };

        assert!(!standing.takes_passive());
    }
}
