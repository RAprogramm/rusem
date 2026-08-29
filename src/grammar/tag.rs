// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Every category a form realizes at once, and what agreement between two of
//! them means.

use super::{
    aspect::{Aspect, Transitivity, Voice},
    case::Case,
    gender::{Animacy, Gender},
    number::Number,
    part::PartOfSpeech,
    tense::{Mood, Person, Tense}
};

/// A full grammatical description of one word form.
///
/// Absent categories are absent, not defaulted: a verb has no case, an
/// infinitive has no person, and the checker must treat "not applicable" and
/// "not yet known" the same way — as no constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GrammarTag {
    /// Part of speech, absent when the analyzer could not commit to one.
    pub part_of_speech: Option<PartOfSpeech>,
    /// Case of a nominal form.
    pub case:           Option<Case>,
    /// Number of a nominal or verbal form.
    pub number:         Option<Number>,
    /// Gender of a nominal form or a past-tense verb.
    pub gender:         Option<Gender>,
    /// Animacy of a nominal.
    pub animacy:        Option<Animacy>,
    /// Aspect of a verbal form.
    pub aspect:         Option<Aspect>,
    /// Tense of a verbal form.
    pub tense:          Option<Tense>,
    /// Mood of a verbal form.
    pub mood:           Option<Mood>,
    /// Person of a verbal form.
    pub person:         Option<Person>,
    /// Transitivity of a verb.
    pub transitivity:   Option<Transitivity>,
    /// Voice of a participle.
    pub voice:          Option<Voice>
}

impl GrammarTag {
    /// An empty description, constraining nothing.
    pub const EMPTY: Self = Self {
        part_of_speech: None,
        case:           None,
        number:         None,
        gender:         None,
        animacy:        None,
        aspect:         None,
        tense:          None,
        mood:           None,
        person:         None,
        transitivity:   None,
        voice:          None
    };

    /// Builds an empty description with a part of speech set.
    #[must_use]
    pub const fn of(part_of_speech: PartOfSpeech) -> Self {
        Self {
            part_of_speech: Some(part_of_speech),
            ..Self::EMPTY
        }
    }

    /// Reports whether two forms may agree in the categories both of them
    /// state.
    ///
    /// A category is checked only when both sides carry it. An adjective with
    /// no gender in the plural does not disagree with a feminine noun; it
    /// simply says nothing about gender.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::grammar::{Case, Gender, GrammarTag, Number, PartOfSpeech};
    ///
    /// let noun = GrammarTag {
    ///     gender: Some(Gender::Feminine),
    ///     number: Some(Number::Singular),
    ///     case: Some(Case::Nominative),
    ///     ..GrammarTag::of(PartOfSpeech::Noun)
    /// };
    /// let fitting = GrammarTag {
    ///     gender: Some(Gender::Feminine),
    ///     number: Some(Number::Singular),
    ///     case: Some(Case::Nominative),
    ///     ..GrammarTag::of(PartOfSpeech::Adjective)
    /// };
    /// let clashing = GrammarTag {
    ///     gender: Some(Gender::Masculine),
    ///     ..fitting
    /// };
    ///
    /// assert!(noun.agrees_with(&fitting));
    /// assert!(!noun.agrees_with(&clashing));
    /// ```
    #[must_use]
    pub fn agrees_with(&self, other: &Self) -> bool {
        fn compatible<T: PartialEq>(left: Option<T>, right: Option<T>) -> bool {
            match (left, right) {
                (Some(left), Some(right)) => left == right,
                _ => true
            }
        }

        compatible(self.case.map(Case::merged), other.case.map(Case::merged))
            && compatible(self.number, other.number)
            && compatible(self.gender, other.gender)
            && compatible(self.person, other.person)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_words_are_looked_up() {
        assert!(PartOfSpeech::Verb.is_content());
        assert!(!PartOfSpeech::Conjunction.is_content());
    }

    #[test]
    fn predicates_head_frames() {
        assert!(PartOfSpeech::Verb.is_predicative());
        assert!(!PartOfSpeech::Noun.is_predicative());
    }

    #[test]
    fn unstated_categories_do_not_clash() {
        let noun = GrammarTag {
            gender: Some(Gender::Feminine),
            ..GrammarTag::of(PartOfSpeech::Noun)
        };
        let adjective = GrammarTag::of(PartOfSpeech::Adjective);

        assert!(noun.agrees_with(&adjective));
    }

    #[test]
    fn stated_categories_must_match() {
        let singular = GrammarTag {
            number: Some(Number::Singular),
            ..GrammarTag::EMPTY
        };
        let plural = GrammarTag {
            number: Some(Number::Plural),
            ..GrammarTag::EMPTY
        };

        assert!(!singular.agrees_with(&plural));
    }
}
