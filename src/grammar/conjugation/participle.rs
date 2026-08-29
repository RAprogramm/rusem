// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The dictionary form of a participle, built from the verb it comes from.
//!
//! A participle is a verb that declines, and it declines like an adjective. So
//! the work here stops where the adjective's begins: what is built is the
//! masculine nominative — `читающий`, `читавший`, `читаемый`, `прочитанный` —
//! and the adjectival tables take it from there.
//!
//! Each of the four is built off a form the verb already has. The active
//! present is the third person plural without its `-т`; the active past is the
//! past stem; the passive present is the first person plural entire. The
//! passive past is built only where the suffix follows from the infinitive:
//! `-ать` and `-ять` take `-нн-`, and the rest — `-енн-` after `-ить`, `-т-`
//! after `-нуть` — turn on alternations the core does not yet write, so
//! nothing is returned for them.

use crate::grammar::{
    Number, Person, Tense, Voice,
    conjugation::{inflect, stems},
    declension::spelling,
    form::verb::VerbForm
};

/// The ending the masculine nominative of an adjectival word takes.
const NOMINATIVE: &str = "ый";

/// The suffix of the active past after a vowel.
const AFTER_VOWEL: &str = "вш";

/// The suffix of the active past after a consonant.
const AFTER_CONSONANT: &str = "ш";

/// The suffix of the active present.
const ACTING: &str = "щ";

/// The suffix of the passive past that the infinitive settles.
const DONE: &str = "нн";

/// The infinitive endings whose passive past takes the settled suffix.
const SETTLED: [&str; 2] = ["ать", "ять"];

/// The dictionary form of one participle of a verb.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Tense, Voice, conjugation::participle::dictionary};
///
/// let acting = dictionary("читать", Voice::Active, Tense::Present);
/// assert_eq!(acting.as_deref(), Some("читающий"));
///
/// let acted = dictionary("читать", Voice::Active, Tense::Past);
/// assert_eq!(acted.as_deref(), Some("читавший"));
///
/// let borne = dictionary("читать", Voice::Passive, Tense::Present);
/// assert_eq!(borne.as_deref(), Some("читаемый"));
///
/// let done = dictionary("прочитать", Voice::Passive, Tense::Past);
/// assert_eq!(done.as_deref(), Some("прочитанный"));
/// ```
#[must_use]
pub fn dictionary(infinitive: &str, voice: Voice, tense: Tense) -> Option<String> {
    let stem = stem(infinitive, voice, tense)?;
    let ending = spelling::fitted(&stem, NOMINATIVE, false);

    Some(stem + &ending)
}

/// The stem one participle declines on.
fn stem(infinitive: &str, voice: Voice, tense: Tense) -> Option<String> {
    match (voice, tense) {
        (Voice::Active, Tense::Present) => acting(infinitive),
        (Voice::Active, Tense::Past) => acted(infinitive),
        (Voice::Passive, Tense::Present) => borne(infinitive),
        (Voice::Passive, Tense::Past) => done(infinitive),
        _ => None
    }
}

/// The stem of the active present: the third person plural without its `-т`.
fn acting(infinitive: &str) -> Option<String> {
    let third = inflect::written(
        infinitive,
        VerbForm::Present {
            person: Person::Third,
            number: Number::Plural
        },
        false
    )?;
    let bare = third.strip_suffix('т')?;

    Some(String::from(bare) + ACTING)
}

/// The stem of the active past, whose suffix the last sound of the stem picks.
fn acted(infinitive: &str) -> Option<String> {
    let stem = stems::past(infinitive)?;
    let suffix = match stem.chars().last() {
        Some(last) if crate::alphabet::is_vowel(last) => AFTER_VOWEL,
        _ => AFTER_CONSONANT
    };

    Some(stem + suffix)
}

/// The stem of the passive present: the first person plural entire.
fn borne(infinitive: &str) -> Option<String> {
    inflect::written(
        infinitive,
        VerbForm::Present {
            person: Person::First,
            number: Number::Plural
        },
        false
    )
}

/// The stem of the passive past, where the infinitive settles the suffix.
fn done(infinitive: &str) -> Option<String> {
    if !SETTLED.iter().any(|held| infinitive.ends_with(held)) {
        return None;
    }

    Some(stems::past(infinitive)? + DONE)
}
