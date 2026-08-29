// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The verbs that join a subject to what it is said to be.
//!
//! `он стал врачом`, `она оказалась другом`. A copula carries almost no
//! meaning of its own: it joins, and what it joins to stands in the
//! instrumental. That is the one thing a checker needs, and it is why `он стал
//! врач` is wrong — two nominatives in one clause with nothing to tell them
//! apart.
//!
//! `быть` is here and behaves differently from the rest. It drops out
//! altogether in the present — `он врач`, with no verb at all — and admits the
//! nominative in the past: `он был врач` is old but not wrong. So it is named
//! apart, and a gate refusing a nominative must not refuse it over `быть`.

/// The copulas that require the instrumental.
///
/// `остаться`, `казаться`, `работать` and `служить` are not here. They take a
/// place as readily as a predicate — `остался дома`, `работает дома` — and a
/// gate cannot tell the two apart by case alone.
pub const JOINING: &[&str] = &[
    "делаться",
    "оказаться",
    "оказываться",
    "сделаться",
    "слыть",
    "становиться",
    "стать",
    "считаться",
    "числиться",
    "являться"
];

/// The verb of being, which joins without requiring a case.
pub const BEING: &[&str] = &["быть", "бывать"];

/// The forms `быть` takes in the future, which build the analytic future of
/// every imperfective verb: `буду читать`.
pub const WILL: &[&str] = &["буду", "будем", "будет", "будете", "будешь", "будут"];

/// The forms `быть` takes in the past.
pub const WAS: &[&str] = &["был", "была", "были", "было"];

/// Reports whether a written lemma joins a subject to what it is said to be.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::copula::joins;
///
/// assert!(joins("стать"));
/// assert!(joins("быть"));
/// assert!(!joins("читать"));
/// ```
#[must_use]
pub fn joins(lemma: &str) -> bool {
    let held = lemma.to_lowercase();

    JOINING.contains(&held.as_str()) || BEING.contains(&held.as_str())
}

/// Reports whether a written lemma requires the instrumental after it.
///
/// True of every copula but `быть`, which admits the nominative in the past
/// and drops out in the present.
#[must_use]
pub fn requires_the_instrumental(lemma: &str) -> bool {
    JOINING.contains(&lemma.to_lowercase().as_str())
}

/// Reports whether a written form is a form of `быть` in the future.
///
/// These build the analytic future — `буду читать` — so a gate counting verbs
/// in a predicate must read the pair as one.
#[must_use]
pub fn is_future_of_being(written: &str) -> bool {
    WILL.contains(&written.to_lowercase().as_str())
}

/// Reports whether a written form is a form of `быть` in the past.
#[must_use]
pub fn is_past_of_being(written: &str) -> bool {
    WAS.contains(&written.to_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_list_is_sorted_and_holds_no_word_twice() {
        for class in [JOINING, BEING, WILL, WAS] {
            let mut held = class.to_vec();
            held.sort_unstable();
            held.dedup();

            assert_eq!(held.len(), class.len(), "a copula is listed twice");
        }
    }

    #[test]
    fn a_copula_is_named_as_one() {
        assert!(joins("стать"));
        assert!(joins("являться"));
        assert!(joins("Быть"));
        assert!(!joins("читать"));
    }

    #[test]
    fn every_copula_but_being_requires_the_instrumental() {
        for held in JOINING {
            assert!(requires_the_instrumental(held), "{held}");
        }
        for held in BEING {
            assert!(
                !requires_the_instrumental(held),
                "{held} admits the nominative"
            );
        }
    }

    #[test]
    fn the_forms_of_being_are_named() {
        assert!(is_future_of_being("буду"));
        assert!(is_future_of_being("Будут"));
        assert!(!is_future_of_being("был"));

        assert!(is_past_of_being("была"));
        assert!(!is_past_of_being("буду"));
    }

    #[test]
    fn the_future_has_a_form_for_every_person_and_number() {
        assert_eq!(WILL.len(), 6);
    }

    #[test]
    fn the_past_has_a_form_for_every_gender_and_the_plural() {
        assert_eq!(WAS.len(), 4);
    }
}
