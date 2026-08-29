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

use crate::{
    grammar::{
        Case, Gender, Number, PartOfSpeech,
        form::{Agreed, Form}
    },
    phonetics::stress::Stressed,
    rules::{Citation, Findings, Found, Scope, found::spelled}
};

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

/// What the paragraph says when it is broken.
const SAYS: &str = "в неударяемом окончании пишется и только после и, иначе е";

/// The letter the ending takes after `и`.
const GLIDE: char = 'и';

/// The letter it takes after anything else.
const OTHERWISE: char = 'е';

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
/// nothing can be judged.
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
    if !SCOPE.admits(form) || outside(form) || !stress.is_settled() {
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

    let wanted = if before == GLIDE { GLIDE } else { OTHERWISE };
    if letters.get(at) == Some(&wanted) {
        return held;
    }

    held.push(Found::new(CITES, at, SAYS, spelled(written, at, wanted)));
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
        assert_eq!(CITES.paragraph, 40);
        assert!(CITES.is_stated());
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
    fn an_ending_after_anything_else_is_written_with_e() {
        let stress = Stressed::settled(Stress::On(0));
        let form = noun(Case::Prepositional, Gender::Neuter);

        assert!(found("платье", form, &stress).is_empty());

        let held = found("платьи", form, &stress);
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "платье");
    }

    #[test]
    fn a_dative_of_another_gender_is_outside_the_paragraph() {
        let stress = Stressed::settled(Stress::On(0));

        assert!(found("гению", noun(Case::Dative, Gender::Masculine), &stress).is_empty());

        let held = found("деревни", noun(Case::Dative, Gender::Feminine), &stress);
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "деревне");
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
