// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What a predicate takes from its subject.
//!
//! Not agreement, though it looks like it. An adjective repeats every category
//! its noun has; a predicate repeats only what its own tense lets it state,
//! and the two tenses state different things. The present states person and
//! number and has no gender at all — `читает` is the same word whoever reads.
//! The past states gender and number and has no person, because it descends
//! from a participle: `читал` is masculine and says nothing about whether the
//! speaker or someone else did the reading.
//!
//! So a subject and a predicate are compared in the categories the predicate
//! actually has, and the ones it has not are not a disagreement. A noun as
//! subject is of the third person — that is what makes `стол стоит` right and
//! `стол стою` wrong. A pronoun does not carry its person in its form, so a
//! pronoun subject is judged on number alone here, and the person of `я` is
//! settled where it is written down: in the closed class of pronouns.

use crate::grammar::{
    Gender, Person,
    dependency::agreeing,
    form::{Bare, Form, verb::VerbForm}
};

/// Reports whether a predicate may be said of a subject.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Case, Gender, Number, Person,
///     dependency::predicate::said_of,
///     form::{Agreed, Bare, Form, verb::VerbForm}
/// };
///
/// let table = Form::Noun(Agreed::Singular {
///     case:   Case::Nominative,
///     gender: Gender::Masculine
/// });
/// let stands = Form::Verb(VerbForm::Present {
///     person: Person::Third,
///     number: Number::Singular
/// });
/// let stand = Form::Verb(VerbForm::Present {
///     person: Person::First,
///     number: Number::Singular
/// });
///
/// assert!(said_of(table, stands));
/// assert!(!said_of(table, stand));
/// assert!(said_of(
///     table,
///     Form::Verb(VerbForm::Past(Bare::Singular(Gender::Masculine)))
/// ));
/// ```
#[must_use]
pub fn said_of(subject: Form, predicate: Form) -> bool {
    let Some(held) = agreeing(subject) else {
        return false;
    };
    let Form::Verb(verb) = predicate else {
        return false;
    };

    match verb {
        VerbForm::Present {
            person,
            number
        } => {
            number == held.number() && person_of(subject).is_none_or(|speaking| person == speaking)
        }
        VerbForm::Past(said) => said.number() == held.number() && genders(said, held.gender()),
        _ => false
    }
}

/// The person a subject speaks in, when its form says.
///
/// A noun says it by being a noun: it names what is spoken about, not who
/// speaks, and that is the third person. A pronoun does not say it — `я` and
/// `он` fill the same cell of the same shape, and which person is meant is in
/// the word rather than the form — so the answer is nothing, and a predicate
/// is then judged on number alone.
#[must_use]
pub const fn person_of(subject: Form) -> Option<Person> {
    match subject {
        Form::Noun(_) => Some(Person::Third),
        _ => None
    }
}

/// Reports whether a past predicate agrees in gender with its subject.
///
/// The plural has no gender in Russian, so a plural predicate agrees with any
/// subject it agrees with in number.
const fn genders(said: Bare, subject: Option<Gender>) -> bool {
    match (said, subject) {
        (Bare::Plural, _) => true,
        (Bare::Singular(held), Some(stated)) => matches!(
            (held, stated),
            (Gender::Masculine, Gender::Masculine)
                | (Gender::Feminine, Gender::Feminine)
                | (Gender::Neuter, Gender::Neuter)
        ),
        (Bare::Singular(_), None) => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{
        Case, Number,
        form::{Agreed, Bare}
    };

    fn subject(gender: Gender, number: Number) -> Form {
        Form::Noun(match number {
            Number::Singular => Agreed::Singular {
                case: Case::Nominative,
                gender
            },
            Number::Plural => Agreed::Plural {
                case: Case::Nominative
            }
        })
    }

    fn present(person: Person, number: Number) -> Form {
        Form::Verb(VerbForm::Present {
            person,
            number
        })
    }

    #[test]
    fn a_noun_takes_a_predicate_of_the_third_person() {
        let table = subject(Gender::Masculine, Number::Singular);

        assert!(said_of(table, present(Person::Third, Number::Singular)));
        assert!(!said_of(table, present(Person::First, Number::Singular)));
    }

    #[test]
    fn a_predicate_of_another_number_is_not_said_of_it() {
        let table = subject(Gender::Masculine, Number::Singular);

        assert!(!said_of(table, present(Person::Third, Number::Plural)));
    }

    #[test]
    fn the_past_agrees_in_gender_and_says_nothing_of_person() {
        let book = subject(Gender::Feminine, Number::Singular);

        assert!(said_of(
            book,
            Form::Verb(VerbForm::Past(Bare::Singular(Gender::Feminine)))
        ));
        assert!(!said_of(
            book,
            Form::Verb(VerbForm::Past(Bare::Singular(Gender::Masculine)))
        ));
    }

    #[test]
    fn a_plural_subject_takes_a_plural_past_of_any_gender() {
        let many = subject(Gender::Masculine, Number::Plural);

        assert!(said_of(many, Form::Verb(VerbForm::Past(Bare::Plural))));
        assert!(!said_of(
            many,
            Form::Verb(VerbForm::Past(Bare::Singular(Gender::Masculine)))
        ));
    }

    #[test]
    fn a_pronoun_subject_is_judged_on_number_alone() {
        let held = Form::Pronoun(Agreed::Singular {
            case:   Case::Nominative,
            gender: Gender::Masculine
        });

        assert_eq!(person_of(held), None);
        assert!(said_of(held, present(Person::First, Number::Singular)));
        assert!(!said_of(held, present(Person::First, Number::Plural)));
    }
}
