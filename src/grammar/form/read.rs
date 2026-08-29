// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Reading an adapter's tag into the form it describes.
//!
//! This is the direction the engine goes: an analyzer answers in a flat tag of
//! optional categories, and everything above wants a form that states what it
//! can state and nothing else.
//!
//! The reading refuses as well as translates. A tag that puts a verb in the
//! genitive, or a past tense in the first person, describes nothing in Russian
//! and comes back as [`None`]. That is the whole point of the exercise: the
//! flat tag lets those be written, and this is where they stop.

use crate::grammar::{
    GrammarTag, Mood, Number, PartOfSpeech, Tense,
    form::{
        Adjectival, Agreed, Bare, Counted, Form,
        verb::{Participle, VerbForm}
    }
};

/// Reads a tag into the form it describes, or refuses it.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Case, Gender, GrammarTag, Number, PartOfSpeech, form::read};
///
/// let noun = GrammarTag {
///     part_of_speech: Some(PartOfSpeech::Noun),
///     case: Some(Case::Genitive),
///     number: Some(Number::Singular),
///     gender: Some(Gender::Masculine),
///     ..GrammarTag::EMPTY
/// };
/// assert!(read::form(&noun).is_some());
///
/// let impossible = GrammarTag {
///     part_of_speech: Some(PartOfSpeech::Verb),
///     case: Some(Case::Genitive),
///     ..GrammarTag::EMPTY
/// };
/// assert!(read::form(&impossible).is_none());
/// ```
#[must_use]
pub fn form(tag: &GrammarTag) -> Option<Form> {
    let part = tag.part_of_speech?;

    match part {
        PartOfSpeech::Noun => agreed(tag).map(Form::Noun),
        PartOfSpeech::Numeral => counted(tag).map(Form::Numeral),
        PartOfSpeech::Pronoun => agreed(tag).map(Form::Pronoun),
        PartOfSpeech::Adjective => agreed(tag).map(|held| Form::Adjective(Adjectival::Full(held))),
        PartOfSpeech::ShortAdjective => {
            bare(tag).map(|held| Form::Adjective(Adjectival::Short(held)))
        }
        PartOfSpeech::Comparative => Some(Form::Adjective(Adjectival::Compared)),
        PartOfSpeech::Adverb => Some(Form::Adverb),
        PartOfSpeech::Predicative => Some(Form::Predicative),
        PartOfSpeech::Preposition => Some(Form::Preposition),
        PartOfSpeech::Conjunction => Some(Form::Conjunction),
        PartOfSpeech::Particle => Some(Form::Particle),
        PartOfSpeech::Interjection => Some(Form::Interjection),
        PartOfSpeech::Infinitive
        | PartOfSpeech::Verb
        | PartOfSpeech::Participle
        | PartOfSpeech::ShortParticiple
        | PartOfSpeech::AdverbialParticiple => verbal(tag, part).map(Form::Verb)
    }
}

/// A numeral, which states a number only when it has one.
///
/// `три` and `пять` state a case and stop there, and a reading that says so is
/// right rather than incomplete. `один` agrees, and is read as an adjective
/// would be.
fn counted(tag: &GrammarTag) -> Option<Counted> {
    if tag.number.is_none() {
        return Some(Counted::Cardinal(tag.case?));
    }

    agreed(tag).map(Counted::Agreeing)
}

/// Case and number, with gender where the singular states one.
///
/// A plural that names a gender is not refused: analyzers state one where the
/// language does not, and the gender is simply dropped rather than argued
/// with. A singular without a gender is refused, because there is no cell of
/// the paradigm it could be.
fn agreed(tag: &GrammarTag) -> Option<Agreed> {
    let case = tag.case?;

    match tag.number? {
        Number::Plural => Some(Agreed::Plural {
            case
        }),
        Number::Singular => Some(Agreed::Singular {
            case,
            gender: tag.gender?
        })
    }
}

/// Number, with gender where the singular states one, and no case.
///
/// A case here is refused rather than dropped: a short form standing in a case
/// is not a short form, and a tag that says so is describing something else.
fn bare(tag: &GrammarTag) -> Option<Bare> {
    if tag.case.is_some() {
        return None;
    }

    match tag.number? {
        Number::Plural => Some(Bare::Plural),
        Number::Singular => Some(Bare::Singular(tag.gender?))
    }
}

/// One form of a verb.
fn verbal(tag: &GrammarTag, part: PartOfSpeech) -> Option<VerbForm> {
    match part {
        PartOfSpeech::Infinitive => infinitive(tag),
        PartOfSpeech::Participle => Some(VerbForm::Participle {
            voice: tag.voice?,
            tense: tag.tense?,
            form:  Participle::Full(agreed(tag)?)
        }),
        PartOfSpeech::ShortParticiple => Some(VerbForm::Participle {
            voice: tag.voice?,
            tense: tag.tense?,
            form:  Participle::Short(bare(tag)?)
        }),
        PartOfSpeech::AdverbialParticiple => adverbial(tag),
        _ => finite(tag)
    }
}

/// The infinitive, which states nothing and refuses anything stated.
const fn infinitive(tag: &GrammarTag) -> Option<VerbForm> {
    if tag.case.is_some() || tag.person.is_some() || tag.number.is_some() {
        return None;
    }

    Some(VerbForm::Infinitive)
}

/// The adverbial participle, which states a tense and nothing else.
fn adverbial(tag: &GrammarTag) -> Option<VerbForm> {
    if tag.case.is_some() || tag.person.is_some() {
        return None;
    }

    Some(VerbForm::Adverbial(tag.tense?))
}

/// A finite verb: the present, the past or the imperative.
///
/// A case is refused outright. A past tense with a person is refused too: the
/// past descends from a participle and has never stated one.
fn finite(tag: &GrammarTag) -> Option<VerbForm> {
    if tag.case.is_some() {
        return None;
    }
    if matches!(tag.mood, Some(Mood::Imperative)) {
        return Some(VerbForm::Imperative(tag.number?));
    }
    if matches!(tag.tense, Some(Tense::Past)) {
        return bare(tag).map(VerbForm::Past);
    }

    Some(VerbForm::Present {
        person: tag.person?,
        number: tag.number?
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Case, Gender, Person, Voice};

    fn tag(part: PartOfSpeech) -> GrammarTag {
        GrammarTag {
            part_of_speech: Some(part),
            ..GrammarTag::EMPTY
        }
    }

    #[test]
    fn a_noun_is_read_into_a_case_and_a_number() {
        let held = GrammarTag {
            case: Some(Case::Genitive),
            number: Some(Number::Singular),
            gender: Some(Gender::Masculine),
            ..tag(PartOfSpeech::Noun)
        };

        assert_eq!(form(&held).and_then(Form::case), Some(Case::Genitive));
    }

    #[test]
    fn a_verb_in_a_case_is_refused() {
        let held = GrammarTag {
            case: Some(Case::Genitive),
            person: Some(Person::Third),
            number: Some(Number::Singular),
            ..tag(PartOfSpeech::Verb)
        };

        assert_eq!(form(&held), None);
    }

    #[test]
    fn a_past_tense_in_a_person_is_refused() {
        let held = GrammarTag {
            tense: Some(Tense::Past),
            person: Some(Person::First),
            number: Some(Number::Singular),
            gender: Some(Gender::Masculine),
            ..tag(PartOfSpeech::Verb)
        };

        assert!(matches!(form(&held), Some(Form::Verb(VerbForm::Past(_)))));
        assert_eq!(
            form(&held).and_then(|read| read.tag().person),
            None,
            "the person is dropped rather than carried"
        );
    }

    #[test]
    fn an_infinitive_that_states_a_person_is_refused() {
        let held = GrammarTag {
            person: Some(Person::First),
            ..tag(PartOfSpeech::Infinitive)
        };

        assert_eq!(form(&held), None);
        assert_eq!(
            form(&tag(PartOfSpeech::Infinitive)),
            Some(Form::Verb(VerbForm::Infinitive))
        );
    }

    #[test]
    fn a_short_form_in_a_case_is_refused() {
        let held = GrammarTag {
            case: Some(Case::Nominative),
            number: Some(Number::Singular),
            gender: Some(Gender::Feminine),
            ..tag(PartOfSpeech::ShortAdjective)
        };

        assert_eq!(form(&held), None);
    }

    #[test]
    fn a_participle_carries_its_voice_and_its_case() {
        let held = GrammarTag {
            voice: Some(Voice::Passive),
            tense: Some(Tense::Past),
            case: Some(Case::Dative),
            number: Some(Number::Plural),
            ..tag(PartOfSpeech::Participle)
        };

        assert_eq!(form(&held).and_then(Form::case), Some(Case::Dative));
    }

    #[test]
    fn a_predicative_is_read_as_itself_and_not_as_an_adverb() {
        let held = form(&tag(PartOfSpeech::Predicative));

        assert_eq!(held, Some(Form::Predicative));
        assert_eq!(
            held.map(Form::part_of_speech),
            Some(PartOfSpeech::Predicative),
            "the part of speech survives the round trip"
        );
    }

    #[test]
    fn a_singular_without_a_gender_is_refused() {
        let held = GrammarTag {
            case: Some(Case::Nominative),
            number: Some(Number::Singular),
            ..tag(PartOfSpeech::Noun)
        };

        assert_eq!(form(&held), None);
    }
}
