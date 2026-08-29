// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The predicatives, which head a clause with no subject in it.
//!
//! `надо идти`, `холодно`, `жаль его`. These words look like adverbs and
//! behave like verbs: they head the clause, they take a tense with `было` and
//! `будет`, and the one who acts stands in the dative rather than the
//! nominative — `мне надо`, never `я надо`.
//!
//! That is why a checker must know them. A gate looking for the subject of a
//! clause would find none here and report a headless sentence, when the
//! sentence is right and simply has no subject to find.
//!
//! The class is closed: no new word joins it.

/// The predicatives that state a need or a leave.
pub const MODAL: &[&str] = &[
    "должно",
    "можно",
    "надлежит",
    "надо",
    "нельзя",
    "необходимо",
    "нужно",
    "пора",
    "следует"
];

/// The predicatives that state how it is.
pub const STATE: &[&str] = &[
    "видно",
    "грустно",
    "душно",
    "жаль",
    "жарко",
    "лень",
    "плохо",
    "светло",
    "скучно",
    "слышно",
    "стыдно",
    "темно",
    "тихо",
    "холодно",
    "хорошо",
    "шумно"
];

/// The predicatives that state whether there is any.
///
/// `нет` is the whole of the negative existential — `нет времени` — and takes
/// the genitive where `есть` takes the nominative.
pub const EXISTENTIAL: &[&str] = &["есть", "нет", "нету"];

/// Every predicative, whatever it states.
const ALL: &[&[&str]] = &[MODAL, STATE, EXISTENTIAL];

/// Reports whether a written word can head a clause with no subject.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::predicative::is_predicative;
///
/// assert!(is_predicative("надо"));
/// assert!(is_predicative("Холодно"));
/// assert!(!is_predicative("стол"));
/// ```
#[must_use]
pub fn is_predicative(written: &str) -> bool {
    let held = written.to_lowercase();

    ALL.iter().any(|kind| kind.contains(&held.as_str()))
}

/// Reports whether the one who acts stands in the dative.
///
/// A modal predicative puts them there — `мне надо` — and so does a state one
/// said of a person: `мне холодно`. An existential does not: it has no actor
/// at all.
#[must_use]
pub fn takes_a_dative_actor(written: &str) -> bool {
    let held = written.to_lowercase();

    MODAL.contains(&held.as_str()) || STATE.contains(&held.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_is_sorted_and_holds_no_word_twice() {
        for kind in ALL {
            let mut held = kind.to_vec();
            held.sort_unstable();
            held.dedup();

            assert_eq!(held.len(), kind.len(), "a predicative is listed twice");
        }
    }

    #[test]
    fn a_predicative_is_named_as_one() {
        assert!(is_predicative("надо"));
        assert!(is_predicative("нельзя"));
        assert!(is_predicative("жаль"));
        assert!(is_predicative("НЕТ"));
    }

    #[test]
    fn a_word_that_is_no_predicative_is_refused() {
        assert!(!is_predicative("стол"));
        assert!(!is_predicative(""));
    }

    #[test]
    fn a_modal_or_a_state_puts_the_actor_in_the_dative() {
        assert!(takes_a_dative_actor("надо"));
        assert!(takes_a_dative_actor("холодно"));
    }

    #[test]
    fn an_existential_has_no_actor_to_put_anywhere() {
        assert!(!takes_a_dative_actor("нет"));
        assert!(!takes_a_dative_actor("есть"));
        assert!(!takes_a_dative_actor("стол"));
    }

    #[test]
    fn every_predicative_is_russian_and_small() {
        for kind in ALL {
            for held in *kind {
                assert!(
                    held.chars().all(crate::alphabet::is_letter),
                    "{held} is no Russian word"
                );
                assert_eq!(*held, held.to_lowercase());
            }
        }
    }
}
