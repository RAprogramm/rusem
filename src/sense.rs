// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Dictionary senses.
//!
//! A sense is the unit the engine reasons about. It carries a definition
//! written by a lexicographer, the register that definition belongs to, and the
//! semantic class the sense denotes. The class is what the selectional gate
//! reads: `пить` wants a liquid in its object slot, and `пить кирпич` fails not
//! because the phrase is rare but because a brick is an artifact.

use std::{string::String, vec::Vec};

use crate::{
    grammar::{Animacy, PartOfSpeech},
    id::{LemmaId, SenseId}
};

/// The broad ontological type a sense denotes.
///
/// The inventory is deliberately small. It is not an attempt at a universal
/// ontology — fine distinctions live in the hypernym graph, where they can be
/// stated by a source. These classes are the coarse filter that runs before the
/// graph is touched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum SemanticClass {
    /// A human being.
    Person,
    /// An animal.
    Animal,
    /// A plant.
    Plant,
    /// A made object.
    Artifact,
    /// A substance or material.
    Substance,
    /// A liquid substance.
    Liquid,
    /// Food or drink.
    Food,
    /// A natural object that was not made.
    NaturalObject,
    /// A part of a body.
    BodyPart,
    /// A place or a region of space.
    Place,
    /// A point or stretch of time.
    Time,
    /// An organization or an institution.
    Organization,
    /// An event that happens.
    Event,
    /// An action performed by an agent.
    Action,
    /// A state that holds.
    State,
    /// A property of something.
    Property,
    /// A quantity or a measure.
    Quantity,
    /// Information, a text or a sign.
    Information,
    /// A feeling.
    Emotion,
    /// An abstraction with no other class fitting it.
    Abstraction
}

impl SemanticClass {
    /// Reports whether the class denotes something that can act on its own.
    ///
    /// Agent slots read this: an agent that is neither a person, an animal nor
    /// an organization is a mismatch unless the reading is figurative.
    #[must_use]
    pub const fn is_agentive(self) -> bool {
        matches!(self, Self::Person | Self::Animal | Self::Organization)
    }

    /// Returns the animacy the class implies, when the question applies.
    ///
    /// Animacy is asked of fillers, not of every sense: a slot that demands an
    /// animate participant is answered by a person or an animal, refused by a
    /// stone, and left unanswered by an abstraction — and an unanswered
    /// question fails the restriction rather than passing it.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::{grammar::Animacy, sense::SemanticClass};
    ///
    /// assert_eq!(SemanticClass::Person.animacy(), Some(Animacy::Animate));
    /// assert_eq!(SemanticClass::Liquid.animacy(), Some(Animacy::Inanimate));
    /// assert_eq!(SemanticClass::Emotion.animacy(), None);
    /// ```
    #[must_use]
    pub const fn animacy(self) -> Option<Animacy> {
        match self {
            Self::Person | Self::Animal => Some(Animacy::Animate),
            Self::Plant
            | Self::Artifact
            | Self::Substance
            | Self::Liquid
            | Self::Food
            | Self::NaturalObject
            | Self::BodyPart
            | Self::Place => Some(Animacy::Inanimate),
            _ => None
        }
    }

    /// Reports whether the class denotes something physical.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(
            self,
            Self::Person
                | Self::Animal
                | Self::Plant
                | Self::Artifact
                | Self::Substance
                | Self::Liquid
                | Self::Food
                | Self::NaturalObject
                | Self::BodyPart
        )
    }
}

/// Where on the time line a sense points.
///
/// A handful of words place the sentence in time all by themselves: `вчера`
/// names a day already gone, `завтра` a day not yet come, `сейчас` the moment
/// of speech. A predicate conjugated against that placement is a sentence at
/// war with itself, which is what the time gate reads this fact for.
///
/// The orientation is a property of a sense, not of a surface form, because
/// the same placement is carried by every part of speech — `вчерашний` points
/// where `вчера` points — and because a dictionary states it in the
/// definition, where a build can learn it rather than a checker enumerate it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum TimeOrientation {
    /// Before the moment of speech: `вчера`, `накануне`, `минувший`.
    Past,
    /// The moment of speech itself: `сейчас`, `теперь`, `нынешний`.
    Present,
    /// After the moment of speech: `завтра`, `послезавтра`, `грядущий`.
    Future
}

/// How a sense came to point where it points.
///
/// The engine's rule is that every answer is traceable to a source, and this
/// fact keeps the rule: a gate that refuses a sentence over an orientation
/// must know how good the orientation is. A stated or a traced one is the
/// dictionary speaking; a learned one is a well-checked guess, good enough to
/// carry as data and not good enough, on its own, to refuse a sentence over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum TimeEvidence {
    /// Stated outright in the build's anchor table.
    Stated,
    /// Traced through the dictionary itself: a synonym line, a derivation, or
    /// a definition that is a cross-reference to an oriented word.
    Traced,
    /// Learned from the definition by the trained rules.
    Learned
}

/// Where a sense points on the time line, and on whose word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TimePlacement {
    /// Where the sense points.
    pub points:   TimeOrientation,
    /// How the engine knows.
    pub evidence: TimeEvidence
}

/// The stylistic layer a sense belongs to.
///
/// A model that knows the register can keep a register: it will not answer an
/// official question with a vernacular word, and it will not present an
/// obsolete sense as current usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Register {
    /// Unmarked, usable anywhere.
    #[default]
    Neutral,
    /// Colloquial.
    Colloquial,
    /// Vernacular, below the colloquial layer.
    Vernacular,
    /// Bookish.
    Bookish,
    /// Official or clerical.
    Official,
    /// Poetic or elevated.
    Poetic,
    /// Belonging to a trade or a science.
    Terminological,
    /// Regional.
    Dialectal,
    /// Out of current use.
    Obsolete,
    /// Slang.
    Slang,
    /// Coarse.
    Vulgar
}

impl Register {
    /// Reports whether the register is safe in a neutral answer.
    #[must_use]
    pub const fn is_unmarked(self) -> bool {
        matches!(self, Self::Neutral)
    }

    /// Reports whether the register stands outside ordinary written Russian.
    ///
    /// A colloquial, bookish, official, poetic or terminological sense is
    /// ordinary Russian wearing different clothes, and a phrase read through
    /// one deserves no doubt — half the nouns of a conversation carry the
    /// colloquial mark. An obsolete sense is ordinary too: `примус` names a
    /// dated thing, and a sentence about that thing is how the word lives on.
    /// Slang, coarse, dialectal and vernacular senses are different words
    /// sharing a spelling, and a phrase that holds together only through one
    /// is not the phrase anyone meant.
    #[must_use]
    pub const fn is_outlying(self) -> bool {
        matches!(
            self,
            Self::Vernacular | Self::Dialectal | Self::Slang | Self::Vulgar
        )
    }
}

/// One sense of one lemma.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sense {
    /// Identity of the sense.
    pub id:             SenseId,
    /// The lemma the sense belongs to.
    pub lemma:          LemmaId,
    /// Position of the sense in its dictionary entry, counted from one.
    pub index:          u16,
    /// Part of speech the sense is realized by.
    pub part_of_speech: PartOfSpeech,
    /// The definition as the source wrote it.
    pub definition:     String,
    /// The ontological type the sense denotes.
    pub class:          SemanticClass,
    /// The stylistic layer.
    pub register:       Register,
    /// Subject field the sense is restricted to, when it is.
    pub domain:         Option<String>,
    /// Whether the sense is a figurative extension of another sense of the same
    /// lemma.
    pub figurative:     bool,
    /// Usage examples quoted from the source.
    pub examples:       Vec<String>,
    /// Where on the time line the sense points, when it points anywhere.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub orientation:    Option<TimePlacement>
}

impl Sense {
    /// Reports whether the sense is a plain, current, literal reading.
    ///
    /// The checker prefers such senses; it accepts a marked one only when no
    /// plain reading survives, and says so in the verdict.
    #[must_use]
    pub const fn is_plain(&self) -> bool {
        !self.figurative && self.register.is_unmarked() && self.domain.is_none()
    }
}

/// A set expression whose meaning is not the sum of its parts.
///
/// Idioms are checked before the selectional gate: `бить баклуши` must not be
/// rejected for the object slot of `бить`, because the phrase is a unit and the
/// parts are not free.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Idiom {
    /// Identity of the idiom, held as a sense of its own.
    pub id:         SenseId,
    /// The lemmas the expression is written with, in order.
    pub lemmas:     Vec<LemmaId>,
    /// The definition as the source wrote it.
    pub definition: String,
    /// The stylistic layer.
    pub register:   Register
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sense() -> Sense {
        Sense {
            id:             SenseId::new(1).expect("non-zero"),
            lemma:          LemmaId::new(1).expect("non-zero"),
            index:          1,
            part_of_speech: PartOfSpeech::Noun,
            definition:     String::from("предмет мебели"),
            class:          SemanticClass::Artifact,
            register:       Register::Neutral,
            domain:         None,
            figurative:     false,
            examples:       Vec::new(),
            orientation:    None
        }
    }

    #[test]
    fn agentive_classes_can_act() {
        assert!(SemanticClass::Person.is_agentive());
        assert!(!SemanticClass::Artifact.is_agentive());
    }

    #[test]
    fn liquids_are_physical() {
        assert!(SemanticClass::Liquid.is_physical());
        assert!(!SemanticClass::Abstraction.is_physical());
    }

    #[test]
    fn plain_sense_has_no_marks() {
        assert!(sense().is_plain());
    }

    #[test]
    fn figurative_sense_is_not_plain() {
        let figurative = Sense {
            figurative: true,
            ..sense()
        };

        assert!(!figurative.is_plain());
    }

    #[test]
    fn terminological_sense_is_not_plain() {
        let term = Sense {
            register: Register::Terminological,
            domain: Some(String::from("мор.")),
            ..sense()
        };

        assert!(!term.is_plain());
    }
}
