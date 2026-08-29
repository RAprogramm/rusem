// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The case a preposition hands out, and whether a word took it.
//!
//! A preposition governs: it demands a case of what follows and says nothing
//! about the rest. `в` demands the accusative or the prepositional and does
//! not care about gender or number; `от` demands the genitive and nothing
//! else. Which cases a preposition hands out is stated with the prepositions
//! themselves, in the closed class, and this only asks whether the form that
//! arrived is standing in one of them.

use crate::grammar::{closed::preposition, dependency::case_of, form::Form};

/// Reports whether a word may stand after a preposition.
///
/// A form that states no case cannot be governed by a preposition, and the
/// answer is no rather than nothing: `в бежать` is not a phrase Russian has,
/// and the reason is exactly that the infinitive stands in no case.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Case, Gender,
///     dependency::governed::admits,
///     form::{Agreed, Form}
/// };
///
/// let prepositional = Form::Noun(Agreed::Singular {
///     case:   Case::Prepositional,
///     gender: Gender::Masculine
/// });
/// let genitive = Form::Noun(Agreed::Singular {
///     case:   Case::Genitive,
///     gender: Gender::Masculine
/// });
///
/// assert!(admits("в", prepositional));
/// assert!(!admits("в", genitive));
/// assert!(admits("от", genitive));
/// ```
#[must_use]
pub fn admits(written: &str, dependent: Form) -> bool {
    let Some(case) = case_of(dependent) else {
        return false;
    };

    preposition::admits(written, case)
}

/// The cases a preposition hands out.
#[must_use]
pub fn handed(written: &str) -> &'static [crate::grammar::Case] {
    preposition::governs(written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{
        Case, Gender, Number, Person,
        form::{Agreed, Form, verb::VerbForm}
    };

    fn noun(case: Case) -> Form {
        Form::Noun(Agreed::Singular {
            case,
            gender: Gender::Masculine
        })
    }

    #[test]
    fn a_preposition_admits_the_case_it_hands_out() {
        assert!(admits("в", noun(Case::Accusative)));
        assert!(admits("в", noun(Case::Prepositional)));
        assert!(admits("от", noun(Case::Genitive)));
    }

    #[test]
    fn a_preposition_refuses_a_case_it_does_not_hand_out() {
        assert!(!admits("от", noun(Case::Dative)));
        assert!(!admits("в", noun(Case::Instrumental)));
    }

    #[test]
    fn the_second_locative_is_the_prepositional_a_preposition_hands_out() {
        assert!(admits("в", noun(Case::Locative)));
    }

    #[test]
    fn a_word_standing_in_no_case_cannot_be_governed() {
        let verb = Form::Verb(VerbForm::Present {
            person: Person::Third,
            number: Number::Singular
        });

        assert!(!admits("в", verb));
        assert!(!admits("в", Form::Adverb));
    }

    #[test]
    fn a_preposition_hands_out_the_cases_the_closed_class_states() {
        assert!(!handed("в").is_empty());
        assert!(handed("стол").is_empty());
    }
}
