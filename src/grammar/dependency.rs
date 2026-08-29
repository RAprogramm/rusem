// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What one word takes from another.
//!
//! A form of a Russian word rarely stands on its own account. An adjective is
//! in the genitive because the noun it describes is; a verb is plural because
//! its subject is; a noun is in the instrumental because the preposition
//! before it hands out that case and no other. None of those categories is the
//! word's own, and the word cannot be judged alone — which is why a checker
//! that reads one word at a time cannot see any of these mistakes, and why
//! this is stated here rather than left to whoever compares two tags.
//!
//! Three dependencies and no more, because Russian has three ways of making
//! one word answer to another: agreement, where the dependent repeats the
//! head's categories; predication, where the predicate repeats the subject's
//! and no others; and government, where the head demands a case and takes no
//! interest in anything else.
//!
//! Each is asked of two forms and answers whether the language allows them
//! standing so. What it never does is choose which word is the head: that is
//! the sentence's business, and a dependency asked the wrong way round would
//! answer about a sentence nobody wrote.

pub mod governed;
pub mod predicate;

use crate::grammar::{
    Case, Number,
    form::{
        Adjectival, Agreed, Form,
        verb::{Participle, VerbForm}
    }
};

/// Reports whether a dependent word agrees with its head.
///
/// Agreement is repetition: the case, the number and — where the language
/// tells genders apart, which is the singular — the gender. A word that states
/// none of them agrees with anything, because it says nothing to disagree
/// with.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Case, Gender,
///     dependency::agrees,
///     form::{Adjectival, Agreed, Form}
/// };
///
/// let noun = Form::Noun(Agreed::Singular {
///     case:   Case::Genitive,
///     gender: Gender::Feminine
/// });
/// let fitting = Form::Adjective(Adjectival::Full(Agreed::Singular {
///     case:   Case::Genitive,
///     gender: Gender::Feminine
/// }));
/// let clashing = Form::Adjective(Adjectival::Full(Agreed::Singular {
///     case:   Case::Nominative,
///     gender: Gender::Feminine
/// }));
///
/// assert!(agrees(noun, fitting));
/// assert!(!agrees(noun, clashing));
/// ```
#[must_use]
pub fn agrees(head: Form, dependent: Form) -> bool {
    let (Some(above), Some(under)) = (agreeing(head), agreeing(dependent)) else {
        return false;
    };

    same(above, under)
}

/// Reports whether a form is one that carries case, number and gender
/// together.
///
/// A noun carries them because they are its own; an adjective, a participle
/// and a pronoun carry them because they are the noun's. Everything else — a
/// verb, an adverb, a preposition — has nothing to agree with or by.
#[must_use]
pub const fn agreeing(form: Form) -> Option<Agreed> {
    match form {
        Form::Noun(held)
        | Form::Pronoun(held)
        | Form::Adjective(Adjectival::Full(held))
        | Form::Verb(VerbForm::Participle {
            form: Participle::Full(held),
            ..
        }) => Some(held),
        _ => None
    }
}

/// Reports whether two agreeing forms state the same categories.
///
/// The two accusatives and the two genitives Russian keeps inside its cases
/// are the same case for this purpose: `в нашем лесу` is the second locative
/// of the noun and the ordinary prepositional of the adjective, and calling
/// them different would call the phrase a mistake.
fn same(head: Agreed, dependent: Agreed) -> bool {
    if head.case().merged() != dependent.case().merged() {
        return false;
    }
    if head.number() != dependent.number() {
        return false;
    }
    if matches!(head.number(), Number::Plural) {
        return true;
    }

    head.gender() == dependent.gender()
}

/// The case a word stands in, when its form states one.
#[must_use]
pub fn case_of(form: Form) -> Option<Case> {
    form.case().map(Case::merged)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Gender, Person, form::verb::VerbForm};

    fn noun(case: Case, gender: Gender) -> Form {
        Form::Noun(Agreed::Singular {
            case,
            gender
        })
    }

    fn attribute(case: Case, gender: Gender) -> Form {
        Form::Adjective(Adjectival::Full(Agreed::Singular {
            case,
            gender
        }))
    }

    #[test]
    fn a_dependent_in_the_head_s_cell_agrees() {
        assert!(agrees(
            noun(Case::Dative, Gender::Neuter),
            attribute(Case::Dative, Gender::Neuter)
        ));
    }

    #[test]
    fn a_dependent_in_another_case_does_not() {
        assert!(!agrees(
            noun(Case::Dative, Gender::Neuter),
            attribute(Case::Genitive, Gender::Neuter)
        ));
    }

    #[test]
    fn a_dependent_in_another_gender_does_not() {
        assert!(!agrees(
            noun(Case::Dative, Gender::Neuter),
            attribute(Case::Dative, Gender::Feminine)
        ));
    }

    #[test]
    fn a_dependent_in_another_number_does_not() {
        let plural = Form::Adjective(Adjectival::Full(Agreed::Plural {
            case: Case::Dative
        }));

        assert!(!agrees(noun(Case::Dative, Gender::Neuter), plural));
    }

    #[test]
    fn the_plural_agrees_without_a_gender_to_agree_in() {
        let head = Form::Noun(Agreed::Plural {
            case: Case::Instrumental
        });
        let dependent = Form::Adjective(Adjectival::Full(Agreed::Plural {
            case: Case::Instrumental
        }));

        assert!(agrees(head, dependent));
    }

    #[test]
    fn the_second_prepositional_agrees_with_the_first() {
        assert!(agrees(
            noun(Case::Locative, Gender::Masculine),
            attribute(Case::Prepositional, Gender::Masculine)
        ));
    }

    #[test]
    fn a_word_that_states_no_case_agrees_with_nothing() {
        let verb = Form::Verb(VerbForm::Present {
            person: Person::Third,
            number: Number::Singular
        });

        assert!(!agrees(noun(Case::Nominative, Gender::Masculine), verb));
        assert!(!agrees(
            verb,
            attribute(Case::Nominative, Gender::Masculine)
        ));
    }
}
