// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 40. Безударное `и` в окончании пишется только после `и`.
//!
//! In a masculine or neuter noun in the prepositional, and in a feminine noun
//! in `-а`/`-я` in the dative and the prepositional singular, an unstressed
//! ending is written with `и` only when `и` precedes it — `о гении`, `в
//! отделении`, `к Марии` — and with `е` otherwise: `о клее`, `в платье`, `к
//! Марье`, `о счастье`.
//!
//! Three facts are needed and all three are in the domain: the case and number
//! from the form, the letter before the ending from the written word, and
//! whether the ending carries the stress. The last is why nothing could ask
//! this paragraph before: an unsettled stress makes the rule silent rather
//! than wrong, because `о ружьё` is stressed and takes neither answer.
//!
//! # Why only half the paragraph is judged
//!
//! The paragraph speaks of particular declensions: the feminine it admits is
//! the one in `-а`/`-я`, and its masculines and neuters are the ones whose
//! ending alternates between `е` and `и` at all. Neither fact is in the form
//! — [`Form`] states case, number and gender, not the nominative — and the
//! written word does not certify it either: `ночи` is a correct dative of a
//! feminine outside `-а`/`-я`, and `имени` a correct prepositional of a
//! neuter in `-мя`, yet demanding `е` after a letter other than `и` would
//! flag both. So the rule judges only the half the written word itself
//! certifies: after `и` the paragraph writes `и` in every declension that
//! reaches this ending — `о гении`, `к Марии`, `в отделении` — and a word
//! ending otherwise after `и` is reported. Where the letter before the ending
//! is not `и`, the demanded `е` rests on a declension fact the core does not
//! hold, and the rule stays silent rather than guesses.

use crate::{
    grammar::{
        Case, Gender, Number, PartOfSpeech,
        form::{Agreed, Form}
    },
    phonetics::stress::Stressed,
    rules::{Citation, Findings, Found, Scope, found::spelled}
};

/// The paragraph as a rule: what it cites and what it is about.
///
/// Holds the citation and the scope together so the engine can list the
/// paragraph alongside the others. The judging function [`found`] stays
/// free.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::unstressed_ending::Rule;
///
/// assert_eq!(Rule::CITES.paragraph, 40);
/// ```
pub struct Rule;

impl Rule {
    /// Where this rule is written.
    pub const CITES: Citation = Citation::whole(40);

    /// What this rule is about.
    ///
    /// Nouns in the singular, in the dative or the prepositional. The paragraph
    /// admits the dative only for the feminine, and that is asked in [`found`]
    /// rather than here: a scope filters on the part, the case and the number,
    /// and the gender is read off the form where the word states it.
    pub const SCOPE: Scope = Scope {
        parts:   &[PartOfSpeech::Noun],
        cases:   &[Case::Dative, Case::Prepositional],
        numbers: &[Number::Singular],
        needs:   crate::rules::scope::Needs {
            stress: true,
            parts:  false
        }
    };
}

/// What the paragraph says when it is broken.
const SAYS: &str = "в неударяемом окончании после и пишется и";

/// The letter the ending takes after `и`.
const GLIDE: char = 'и';

/// Reports whether the form is a dative the paragraph does not speak of.
///
/// § 40 admits the dative only for a feminine in `-а`/`-я` — `к Марии`
/// beside `к Марье` — because only there does the dative end in `е` or `и`
/// at all. A masculine or neuter dative ends in `у`/`ю`, so judging `гению`
/// by the letter before its ending would demand `гении` of a correctly
/// written word.
const fn outside(form: Form) -> bool {
    let Form::Noun(Agreed::Singular {
        case,
        gender
    }) = form
    else {
        return false;
    };

    matches!(case.merged(), Case::Dative) && !matches!(gender, Gender::Feminine)
}

/// What the paragraph finds in a written noun.
///
/// It is about the unstressed ending, so a word whose stress is unknown and a
/// word stressed on the ending are outside it: nothing is found, because
/// nothing can be judged. Only the half after `и` is judged at all — the
/// other half needs the declension, which neither the form nor the writing
/// states — so a word like `ночи` or `платьи` is left alone rather than
/// guessed at.
///
/// # Examples
///
/// ```
/// use rusem::{
///     grammar::{
///         Case, Gender,
///         form::{Agreed, Form}
///     },
///     phonetics::stress::{Stress, Stressed},
///     rules::svod::unstressed_ending::found
/// };
///
/// let form = Form::Noun(Agreed::Singular {
///     case:   Case::Prepositional,
///     gender: Gender::Masculine
/// });
/// let stress = Stressed::settled(Stress::On(0));
///
/// assert!(found("гении", form, &stress).is_empty());
///
/// let held = found("гение", form, &stress);
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].instead, "гении");
/// ```
#[must_use]
pub fn found(written: &str, form: Form, stress: &Stressed) -> Findings {
    let mut held = Findings::new();
    if !Rule::SCOPE.admits(form) || outside(form) || !stress.is_settled() {
        return held;
    }
    if stress.ending_stressed(crate::phonetics::stress::vowels(written)) {
        return held;
    }

    let letters: std::vec::Vec<char> = written.chars().collect();
    let Some(at) = letters.len().checked_sub(1) else {
        return held;
    };
    let Some(before) = at.checked_sub(1).and_then(|one| letters.get(one)).copied() else {
        return held;
    };

    if before != GLIDE || letters.get(at) == Some(&GLIDE) {
        return held;
    }

    held.push(Found::new(
        Rule::CITES,
        at,
        SAYS,
        spelled(written, at, GLIDE)
    ));
    held
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        grammar::{Gender, form::Agreed},
        phonetics::stress::Stress
    };

    fn noun(case: Case, gender: Gender) -> Form {
        Form::Noun(Agreed::Singular {
            case,
            gender
        })
    }

    #[test]
    fn the_paragraph_is_cited() {
        assert_eq!(Rule::CITES.paragraph, 40);
        assert!(Rule::CITES.is_stated());
    }

    #[test]
    fn an_ending_after_i_is_written_with_i() {
        let stress = Stressed::settled(Stress::On(0));
        let form = noun(Case::Prepositional, Gender::Masculine);

        assert!(found("гении", form, &stress).is_empty());

        let held = found("гение", form, &stress);
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "гении");
    }

    #[test]
    fn an_ending_after_anything_else_needs_the_declension_and_is_not_judged() {
        let stress = Stressed::settled(Stress::On(0));

        assert!(found("платье", noun(Case::Prepositional, Gender::Neuter), &stress).is_empty());
        assert!(found("платьи", noun(Case::Prepositional, Gender::Neuter), &stress).is_empty());
        assert!(found("имени", noun(Case::Prepositional, Gender::Neuter), &stress).is_empty());
        assert!(found("ночи", noun(Case::Prepositional, Gender::Feminine), &stress).is_empty());
        assert!(found("ночи", noun(Case::Dative, Gender::Feminine), &stress).is_empty());
        assert!(found("деревни", noun(Case::Dative, Gender::Feminine), &stress).is_empty());
    }

    #[test]
    fn a_dative_of_another_gender_is_outside_the_paragraph() {
        let stress = Stressed::settled(Stress::On(0));

        assert!(found("гению", noun(Case::Dative, Gender::Masculine), &stress).is_empty());
    }

    #[test]
    fn a_feminine_dative_after_i_is_written_with_i() {
        let stress = Stressed::settled(Stress::On(0));

        assert!(found("марии", noun(Case::Dative, Gender::Feminine), &stress).is_empty());

        let held = found("марие", noun(Case::Dative, Gender::Feminine), &stress);
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "марии");
    }

    #[test]
    fn a_word_whose_stress_is_unknown_is_not_judged() {
        let form = noun(Case::Prepositional, Gender::Masculine);

        assert!(found("гение", form, &Stressed::unknown()).is_empty());
    }

    #[test]
    fn a_form_the_paragraph_does_not_speak_of_is_left_alone() {
        let stress = Stressed::settled(Stress::On(0));

        assert!(found("гение", Form::Adverb, &stress).is_empty());
    }
}
