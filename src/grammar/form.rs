// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What one written form states, and only what it can state.
//!
//! [`super::tag::GrammarTag`] holds every category at once and leaves each one
//! optional, which is what an adapter needs: an analyzer answers with what it
//! could work out and stays silent about the rest. It is not what the language
//! is. A verb has no case, a plural states no gender, an infinitive has no
//! person — and a tag of optional fields lets all three be written down.
//!
//! Here a form is a sum: each variant carries the categories that form has and
//! no others, so the combinations the language does not make cannot be built.
//! Adapters keep speaking in tags; everything reasoning about Russian speaks in
//! forms, and [`Form::tag`] is the one place the two meet.

pub mod read;
pub mod verb;

use super::{Animacy, Case, Gender, GrammarTag, Number, PartOfSpeech};

/// Case and number, with gender where the language states one.
///
/// The plural has no gender in Russian: `красные` is the same word for every
/// gender, and a type that let a gender be written beside it would be stating
/// a distinction the language does not draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Agreed {
    /// One, in a case and of a gender.
    Singular {
        /// The case the form stands in.
        case:   Case,
        /// The gender the form agrees in.
        gender: Gender
    },
    /// More than one, in a case.
    Plural {
        /// The case the form stands in.
        case: Case
    }
}

impl Agreed {
    /// The case the form stands in.
    #[must_use]
    pub const fn case(self) -> Case {
        match self {
            Self::Singular {
                case, ..
            }
            | Self::Plural {
                case
            } => case
        }
    }

    /// The gender the form agrees in, absent in the plural.
    #[must_use]
    pub const fn gender(self) -> Option<Gender> {
        match self {
            Self::Singular {
                gender, ..
            } => Some(gender),
            Self::Plural {
                ..
            } => None
        }
    }

    /// How many the form states.
    #[must_use]
    pub const fn number(self) -> Number {
        match self {
            Self::Singular {
                ..
            } => Number::Singular,
            Self::Plural {
                ..
            } => Number::Plural
        }
    }
}

/// Number, with gender where the language states one, and no case at all.
///
/// This is the shape of every predicative form: the short adjective, the short
/// participle and the past tense, which is a short participle by descent. They
/// state who they are said of and never what case they stand in, because they
/// stand in none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Bare {
    /// One, of a gender.
    Singular(Gender),
    /// More than one.
    Plural
}

impl Bare {
    /// The gender the form agrees in, absent in the plural.
    #[must_use]
    pub const fn gender(self) -> Option<Gender> {
        match self {
            Self::Singular(held) => Some(held),
            Self::Plural => None
        }
    }

    /// How many the form states.
    #[must_use]
    pub const fn number(self) -> Number {
        match self {
            Self::Singular(_) => Number::Singular,
            Self::Plural => Number::Plural
        }
    }
}

/// How a numeral stands.
///
/// A cardinal from `три` up has a case and no number of its own: `три`, `трём`,
/// `тремя` are the same word counting any number of things. `один`, `оба` and
/// the ordinals agree like adjectives and have both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Counted {
    /// A cardinal that states a case and nothing else.
    Cardinal(Case),
    /// A numeral that agrees like an adjective.
    Agreeing(Agreed)
}

impl Counted {
    /// The case the numeral stands in.
    #[must_use]
    pub const fn case(self) -> Case {
        match self {
            Self::Cardinal(held) => held,
            Self::Agreeing(held) => held.case()
        }
    }

    /// How many the numeral states, absent in a bare cardinal.
    #[must_use]
    pub const fn number(self) -> Option<Number> {
        match self {
            Self::Cardinal(_) => None,
            Self::Agreeing(held) => Some(held.number())
        }
    }

    /// The gender the numeral agrees in, absent in a bare cardinal.
    #[must_use]
    pub const fn gender(self) -> Option<Gender> {
        match self {
            Self::Cardinal(_) => None,
            Self::Agreeing(held) => held.gender()
        }
    }
}

/// How an adjective stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Adjectival {
    /// Declined, before or after the noun it describes: `красная`.
    Full(Agreed),
    /// Predicative, said of the noun rather than attached to it: `красна`.
    Short(Bare),
    /// Compared, and therefore not declined at all: `краснее`.
    Compared
}

/// One written form of a Russian word, stating what that form can state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Form {
    /// A noun: a case and a number. Gender and animacy belong to the word, not
    /// to the form, and are stated in [`crate::lexis::lexeme::Lexeme`].
    Noun(Agreed),
    /// An adjective, a participle used as one, or an ordinal numeral.
    Adjective(Adjectival),
    /// A verb, in any of its forms.
    Verb(verb::VerbForm),
    /// A numeral, which counts like nothing else in the language.
    Numeral(Counted),
    /// A pronoun that stands in for a noun.
    Pronoun(Agreed),
    /// An adverb, which states nothing.
    Adverb,
    /// A preposition, which states nothing and governs a case.
    Preposition,
    /// A conjunction, which states nothing.
    Conjunction,
    /// A particle, which states nothing.
    Particle,
    /// An interjection, which states nothing.
    Interjection
}

impl Form {
    /// The part of speech this form belongs to.
    #[must_use]
    pub const fn part_of_speech(self) -> PartOfSpeech {
        match self {
            Self::Noun(_) => PartOfSpeech::Noun,
            Self::Adjective(Adjectival::Short(_)) => PartOfSpeech::ShortAdjective,
            Self::Adjective(Adjectival::Compared) => PartOfSpeech::Comparative,
            Self::Adjective(Adjectival::Full(_)) => PartOfSpeech::Adjective,
            Self::Verb(held) => held.part_of_speech(),
            Self::Numeral(_) => PartOfSpeech::Numeral,
            Self::Pronoun(_) => PartOfSpeech::Pronoun,
            Self::Adverb => PartOfSpeech::Adverb,
            Self::Preposition => PartOfSpeech::Preposition,
            Self::Conjunction => PartOfSpeech::Conjunction,
            Self::Particle => PartOfSpeech::Particle,
            Self::Interjection => PartOfSpeech::Interjection
        }
    }

    /// The case this form stands in, absent when it stands in none.
    #[must_use]
    pub const fn case(self) -> Option<Case> {
        match self {
            Self::Noun(held) | Self::Pronoun(held) | Self::Adjective(Adjectival::Full(held)) => {
                Some(held.case())
            }
            Self::Numeral(held) => Some(held.case()),
            Self::Verb(held) => held.case(),
            _ => None
        }
    }

    /// How many this form states, absent when it states none.
    #[must_use]
    pub const fn number(self) -> Option<Number> {
        match self {
            Self::Noun(held) | Self::Pronoun(held) | Self::Adjective(Adjectival::Full(held)) => {
                Some(held.number())
            }
            Self::Numeral(held) => held.number(),
            Self::Adjective(Adjectival::Short(held)) => Some(held.number()),
            Self::Verb(held) => held.number(),
            _ => None
        }
    }

    /// The gender this form agrees in, absent when it states none.
    #[must_use]
    pub const fn gender(self) -> Option<Gender> {
        match self {
            Self::Noun(held) | Self::Pronoun(held) | Self::Adjective(Adjectival::Full(held)) => {
                held.gender()
            }
            Self::Numeral(held) => held.gender(),
            Self::Adjective(Adjectival::Short(held)) => held.gender(),
            Self::Verb(held) => held.gender(),
            _ => None
        }
    }

    /// This form as the tag an adapter speaks in.
    ///
    /// The categories a form does not state come out absent, which is what a
    /// tag means by absent. The categories that belong to the word rather than
    /// the form — gender and animacy of a noun, aspect of a verb — are not
    /// here to be given, and the caller that has the word fills them in.
    #[must_use]
    pub const fn tag(self) -> GrammarTag {
        GrammarTag {
            part_of_speech: Some(self.part_of_speech()),
            case:           self.case(),
            number:         self.number(),
            gender:         self.gender(),
            animacy:        None,
            aspect:         None,
            tense:          self.tense(),
            mood:           self.mood(),
            person:         self.person(),
            transitivity:   None,
            voice:          self.voice()
        }
    }

    /// The tense this form states, absent outside the verb.
    #[must_use]
    const fn tense(self) -> Option<super::Tense> {
        match self {
            Self::Verb(held) => held.tense(),
            _ => None
        }
    }

    /// The mood this form states, absent outside the verb.
    #[must_use]
    const fn mood(self) -> Option<super::Mood> {
        match self {
            Self::Verb(held) => held.mood(),
            _ => None
        }
    }

    /// The person this form states, absent outside the finite verb.
    #[must_use]
    const fn person(self) -> Option<super::Person> {
        match self {
            Self::Verb(held) => held.person(),
            _ => None
        }
    }

    /// The voice this form states, absent outside the participle.
    #[must_use]
    const fn voice(self) -> Option<super::Voice> {
        match self {
            Self::Verb(held) => held.voice(),
            _ => None
        }
    }
}

/// Reports whether an animacy can stand with a case at all.
///
/// Animacy is only ever visible in the accusative, where it decides whether
/// the form is written as the nominative or as the genitive. Elsewhere it is
/// a fact about the word and says nothing about the form.
#[must_use]
pub const fn animacy_shows(case: Case, animacy: Animacy) -> bool {
    matches!(case, Case::Accusative) && matches!(animacy, Animacy::Animate | Animacy::Inanimate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plural_states_no_gender() {
        let held = Agreed::Plural {
            case: Case::Nominative
        };

        assert_eq!(held.gender(), None);
        assert_eq!(held.number(), Number::Plural);
    }

    #[test]
    fn a_noun_states_a_case_and_a_verb_does_not() {
        let noun = Form::Noun(Agreed::Singular {
            case:   Case::Genitive,
            gender: Gender::Masculine
        });

        assert_eq!(noun.case(), Some(Case::Genitive));
        assert_eq!(Form::Verb(verb::VerbForm::Infinitive).case(), None);
    }

    #[test]
    fn a_short_adjective_states_no_case() {
        let held = Form::Adjective(Adjectival::Short(Bare::Singular(Gender::Feminine)));

        assert_eq!(held.case(), None);
        assert_eq!(held.gender(), Some(Gender::Feminine));
        assert_eq!(held.part_of_speech(), PartOfSpeech::ShortAdjective);
    }

    #[test]
    fn a_compared_adjective_states_nothing_but_itself() {
        let held = Form::Adjective(Adjectival::Compared);

        assert_eq!(held.case(), None);
        assert_eq!(held.number(), None);
        assert_eq!(held.gender(), None);
    }

    #[test]
    fn a_form_becomes_the_tag_an_adapter_speaks_in() {
        let held = Form::Noun(Agreed::Singular {
            case:   Case::Dative,
            gender: Gender::Neuter
        })
        .tag();

        assert_eq!(held.part_of_speech, Some(PartOfSpeech::Noun));
        assert_eq!(held.case, Some(Case::Dative));
        assert_eq!(held.person, None);
    }

    #[test]
    fn animacy_is_only_visible_in_the_accusative() {
        assert!(animacy_shows(Case::Accusative, Animacy::Animate));
        assert!(!animacy_shows(Case::Nominative, Animacy::Animate));
    }
}
