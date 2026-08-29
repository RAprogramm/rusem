// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The forms of a Russian verb, each stating what that form states.
//!
//! Six shapes, and no two of them state the same categories. The infinitive
//! states nothing. The present states person and number. The past states
//! gender and number and no person, because it descends from a participle
//! rather than from a conjugation. The imperative states number. The
//! participle declines and therefore states a case; the adverbial participle
//! declines nothing and states only a tense.
//!
//! Writing them as one shape with optional fields — which is what a tag does —
//! makes «past tense, first person» and «infinitive in the dative» writable.
//! Neither exists in Russian, and neither can be written here.

use crate::grammar::{
    Case, Gender, Mood, Number, PartOfSpeech, Person, Tense, Voice,
    form::{Agreed, Bare}
};

/// A participle, which is a verb that declines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Participle {
    /// Declined, standing with the noun it describes: `читающий`,
    /// `прочитанный`.
    Full(Agreed),
    /// Predicative, said of the noun: `прочитан`, `прочитана`.
    ///
    /// Only the passive participle has this form; an active one cannot be said
    /// of anything, and the type does not stop that — the voice beside it does.
    Short(Bare)
}

impl Participle {
    /// The case the participle stands in, absent in the short form.
    #[must_use]
    pub const fn case(self) -> Option<Case> {
        match self {
            Self::Full(held) => Some(held.case()),
            Self::Short(_) => None
        }
    }

    /// The gender the participle agrees in, absent in the plural.
    #[must_use]
    pub const fn gender(self) -> Option<Gender> {
        match self {
            Self::Full(held) => held.gender(),
            Self::Short(held) => held.gender()
        }
    }

    /// How many the participle states.
    #[must_use]
    pub const fn number(self) -> Number {
        match self {
            Self::Full(held) => held.number(),
            Self::Short(held) => held.number()
        }
    }
}

/// One form of a verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VerbForm {
    /// The dictionary form, which states nothing: `читать`.
    Infinitive,
    /// The present, or the simple future of a perfective verb: `читает`,
    /// `прочитает`. Person and number, and no gender.
    Present {
        /// Who acts.
        person: Person,
        /// How many act.
        number: Number
    },
    /// The past: `читал`, `читала`, `читали`. Gender and number, and no person.
    Past(Bare),
    /// The imperative: `читай`, `читайте`.
    Imperative(Number),
    /// A participle, with its voice and its tense.
    Participle {
        /// Whether the one described acts or is acted on.
        voice: Voice,
        /// When, relative to the sentence.
        tense: Tense,
        /// How the participle stands.
        form:  Participle
    },
    /// An adverbial participle, which declines nothing: `читая`, `прочитав`.
    Adverbial(Tense)
}

impl VerbForm {
    /// The part of speech this form is read as.
    #[must_use]
    pub const fn part_of_speech(self) -> PartOfSpeech {
        match self {
            Self::Infinitive => PartOfSpeech::Infinitive,
            Self::Present {
                ..
            }
            | Self::Past(_)
            | Self::Imperative(_) => PartOfSpeech::Verb,
            Self::Participle {
                form: Participle::Short(_),
                ..
            } => PartOfSpeech::ShortParticiple,
            Self::Participle {
                ..
            } => PartOfSpeech::Participle,
            Self::Adverbial(_) => PartOfSpeech::AdverbialParticiple
        }
    }

    /// The case this form stands in, which only a full participle does.
    #[must_use]
    pub const fn case(self) -> Option<Case> {
        match self {
            Self::Participle {
                form, ..
            } => form.case(),
            _ => None
        }
    }

    /// How many this form states, absent in the infinitive and the adverbial.
    #[must_use]
    pub const fn number(self) -> Option<Number> {
        match self {
            Self::Present {
                number, ..
            }
            | Self::Imperative(number) => Some(number),
            Self::Past(held) => Some(held.number()),
            Self::Participle {
                form, ..
            } => Some(form.number()),
            Self::Infinitive | Self::Adverbial(_) => None
        }
    }

    /// The gender this form agrees in, which only the past and the participle
    /// state, and only in the singular.
    #[must_use]
    pub const fn gender(self) -> Option<Gender> {
        match self {
            Self::Past(held) => held.gender(),
            Self::Participle {
                form, ..
            } => form.gender(),
            _ => None
        }
    }

    /// Who acts, which only the finite present states.
    ///
    /// The imperative addresses the second person and states no person of its
    /// own: `читайте` is the same form for one addressed politely and for many.
    #[must_use]
    pub const fn person(self) -> Option<Person> {
        match self {
            Self::Present {
                person, ..
            } => Some(person),
            _ => None
        }
    }

    /// When the form places what it states.
    #[must_use]
    pub const fn tense(self) -> Option<Tense> {
        match self {
            Self::Present {
                ..
            } => Some(Tense::Present),
            Self::Past(_) => Some(Tense::Past),
            Self::Participle {
                tense, ..
            }
            | Self::Adverbial(tense) => Some(tense),
            Self::Infinitive | Self::Imperative(_) => None
        }
    }

    /// The mood the form is in.
    #[must_use]
    pub const fn mood(self) -> Option<Mood> {
        match self {
            Self::Present {
                ..
            }
            | Self::Past(_) => Some(Mood::Indicative),
            Self::Imperative(_) => Some(Mood::Imperative),
            _ => None
        }
    }

    /// Whether the one described acts or is acted on, which only a participle
    /// states.
    #[must_use]
    pub const fn voice(self) -> Option<Voice> {
        match self {
            Self::Participle {
                voice, ..
            } => Some(voice),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_infinitive_states_nothing() {
        let held = VerbForm::Infinitive;

        assert_eq!(held.case(), None);
        assert_eq!(held.number(), None);
        assert_eq!(held.person(), None);
        assert_eq!(held.tense(), None);
    }

    #[test]
    fn the_present_states_person_and_no_gender() {
        let held = VerbForm::Present {
            person: Person::First,
            number: Number::Singular
        };

        assert_eq!(held.person(), Some(Person::First));
        assert_eq!(held.gender(), None);
        assert_eq!(held.tense(), Some(Tense::Present));
    }

    #[test]
    fn the_past_states_gender_and_no_person() {
        let held = VerbForm::Past(Bare::Singular(Gender::Feminine));

        assert_eq!(held.gender(), Some(Gender::Feminine));
        assert_eq!(held.person(), None);
        assert_eq!(held.tense(), Some(Tense::Past));
    }

    #[test]
    fn the_past_plural_states_neither() {
        let held = VerbForm::Past(Bare::Plural);

        assert_eq!(held.gender(), None);
        assert_eq!(held.person(), None);
        assert_eq!(held.number(), Some(Number::Plural));
    }

    #[test]
    fn only_the_full_participle_stands_in_a_case() {
        let full = VerbForm::Participle {
            voice: Voice::Active,
            tense: Tense::Present,
            form:  Participle::Full(Agreed::Plural {
                case: Case::Dative
            })
        };
        let short = VerbForm::Participle {
            voice: Voice::Passive,
            tense: Tense::Past,
            form:  Participle::Short(Bare::Plural)
        };

        assert_eq!(full.case(), Some(Case::Dative));
        assert_eq!(short.case(), None);
        assert_eq!(short.part_of_speech(), PartOfSpeech::ShortParticiple);
    }

    #[test]
    fn the_adverbial_states_only_a_tense() {
        let held = VerbForm::Adverbial(Tense::Past);

        assert_eq!(held.tense(), Some(Tense::Past));
        assert_eq!(held.number(), None);
        assert_eq!(held.gender(), None);
    }
}
