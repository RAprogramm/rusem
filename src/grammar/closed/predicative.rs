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
//! # Two closed kinds, and one open one this module refuses
//!
//! The modal and the existential predicatives are closed: no new word states
//! a leave or an existence, and the lists below hold them whole. The **state**
//! predicatives are not. Any quality adverb in `-о` can head a subjectless
//! clause — `мне холодно`, `мне тепло`, `в комнате шумно`, and tomorrow's
//! adverb too — so the kind is as open as the adverbs are, and no grammar
//! enumerates it. A list here would be a sample passed off as the class, true
//! for the words someone happened to write down and false for `тепло` beside
//! `холодно`. So there is no such list: this module answers for the modal and
//! the existential ones and stays silent about state, and a caller that meets
//! `холодно` reads the clause rather than a list.

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

/// The predicatives that state whether there is any.
///
/// `нет` is the whole of the negative existential — `нет времени` — and takes
/// the genitive where `есть` takes the nominative.
pub const EXISTENTIAL: &[&str] = &["есть", "нет", "нету"];

/// Every predicative the module can list.
const ALL: &[&[&str]] = &[MODAL, EXISTENTIAL];

/// Reports whether a written word is a modal or an existential predicative.
///
/// The state predicatives — `холодно`, `жаль` — are an open kind and are not
/// listed, so they answer false here: false means the lists do not hold the
/// word, not that the word cannot head a clause.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::predicative::is_predicative;
///
/// assert!(is_predicative("надо"));
/// assert!(is_predicative("Нет"));
/// assert!(!is_predicative("стол"));
/// ```
#[must_use]
pub fn is_predicative(written: &str) -> bool {
    let held = written.to_lowercase();

    ALL.iter().any(|kind| kind.contains(&held.as_str()))
}

/// Reports whether the one who acts stands in the dative.
///
/// A modal predicative puts them there — `мне надо` — and an existential does
/// not: it has no actor at all. A state predicative would put them there too,
/// but the state kind is open and unlisted, so it is not answered here.
#[must_use]
pub fn takes_a_dative_actor(written: &str) -> bool {
    MODAL.contains(&written.to_lowercase().as_str())
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
        assert!(is_predicative("НЕТ"));
    }

    #[test]
    fn a_word_that_is_no_predicative_is_refused() {
        assert!(!is_predicative("стол"));
        assert!(!is_predicative(""));
    }

    #[test]
    fn the_open_state_kind_is_not_pretended_to_be_listed() {
        for held in ["холодно", "тепло", "жаль", "шумно"] {
            assert!(
                !is_predicative(held),
                "{held} is a state word and the open kind holds no list"
            );
            assert!(!takes_a_dative_actor(held), "{held}");
        }
    }

    #[test]
    fn a_modal_puts_the_actor_in_the_dative() {
        assert!(takes_a_dative_actor("надо"));
        assert!(takes_a_dative_actor("нельзя"));
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
