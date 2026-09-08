// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 157. Междометия отделяются запятой.
//!
//! `Эй, поберегись!`, `Увы, он не придёт`. The interjection stands outside the
//! sentence, and the comma is what says so.
//!
//! # Where the exception comes from, and why it is a refusal
//!
//! The paragraph's примечание says that some of the same words are not
//! interjections at all but particles, and those take no comma: `о` when it
//! opens an address — `О поле, поле` — and `ну`, `ах`, `ох` when they carry
//! усилительный оттенок — `Ох ты гой еси`, `Ну и денёк!`.
//!
//! What separates the particle from the interjection is the use, not the
//! writing. The paragraph's own examples show the same word both ways: `Ох,
//! уж эти мне ребята!` keeps its comma while `Ох ты гой еси` takes none, and
//! `о` opens an address in `О поле` but exclaims in `О, вы были ребенок
//! резвый`. A pair of written words does not state which use it holds, so for
//! the words the note names the rule answers nothing rather than either way.
//! Every other interjection is outside the note and keeps the main rule's
//! comma: `Эй, поберегись!`, `Увы, он не придёт`.

use crate::{
    grammar::closed::interjection,
    rules::{Citation, Findings, Found, Scope, scope, svod::hyphen_compound::interjections}
};

/// § 157 as a rule.
///
/// Holds where the paragraph is written and what it is about.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::interjection_comma::Rule;
///
/// assert_eq!(Rule::CITES.paragraph, 157);
/// ```
pub struct Rule;

impl Rule {
    /// Where this rule is written.
    pub const CITES: Citation = Citation::whole(157);

    /// What this rule is about.
    pub const SCOPE: Scope = scope::Scope::ANY;
}

/// The words the примечание of § 157 says may be particles instead.
///
/// The note enumerates them: `о`, употребляемая при обращении, and `ну`,
/// `ах`, `ох`, употребляемые для выражения усилительного оттенка. Its
/// `и т. п.` leaves the class open, but nothing in the writing derives a
/// further member, so the rule keeps to the words the source itself names.
const PARTICLES_TOO: &[&str] = &["о", "ну", "ах", "ох"];

/// Reports whether a written word is an interjection this paragraph speaks of.
///
/// Two ways of knowing, and the paragraph needs both. The words it names are
/// stated in [`interjection`]; the ones it does not are reached through § 86,
/// which says how an interjection is built and so recognises one never listed
/// anywhere.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::interjection_comma::speaks_of;
///
/// assert!(speaks_of("увы"));
/// assert!(speaks_of("бла-бла-бла"));
/// assert!(!speaks_of("стол"));
/// ```
#[must_use]
pub fn speaks_of(written: &str) -> bool {
    interjection::is_interjection(written) || interjections::interjects(written)
}

/// Reports whether the interjection is parted from what follows by a comma.
///
/// [`None`] when the paragraph gives the core no answer: the word is no
/// interjection, or it is one of the words the примечание says may be a
/// particle — there the comma follows the use, which the written pair does
/// not state, so the rule refuses rather than guesses either way.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::interjection_comma::parted_from;
///
/// assert_eq!(parted_from("увы", Some("он")), Some(true));
/// assert_eq!(parted_from("ах", Some("ты")), None);
/// assert_eq!(parted_from("стол", Some("стоит")), None);
/// ```
#[must_use]
pub fn parted_from(written: &str, next: Option<&str>) -> Option<bool> {
    if !speaks_of(written) {
        return None;
    }
    let Some(following) = next else {
        return Some(false);
    };
    if unsettled(written, following) {
        return None;
    }

    Some(true)
}

/// Reports whether the pair is one the примечание leaves undecidable.
///
/// A word the note names may be the interjection — comma — or the particle —
/// none — and only its use in the sentence tells them apart. With a word
/// following, both readings are open, and the letters do not choose.
fn unsettled(written: &str, following: &str) -> bool {
    PARTICLES_TOO.contains(&written.to_lowercase().as_str()) && !following.is_empty()
}

/// What the paragraph says when it is broken.
const SAYS: &str = "междометие отделяется запятой от того, что стоит за ним";

/// What the paragraph finds in an interjection and what follows it.
///
/// The finding is the pair written with the comma the paragraph requires. A
/// word that is no interjection is outside the paragraph, and a word the
/// примечание says may be a particle is refused rather than judged — in
/// either case nothing is found.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::interjection_comma::found;
///
/// let held = found("увы", Some("он"));
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].instead, "увы, он");
///
/// assert!(found("ах", Some("ты")).is_empty());
/// assert!(found("стол", Some("стоит")).is_empty());
/// ```
#[must_use]
pub fn found(written: &str, next: Option<&str>) -> Findings {
    let mut held = Findings::new();
    let (Some(following), Some(true)) = (next, parted_from(written, next)) else {
        return held;
    };

    held.push(Found::new(
        Rule::CITES,
        written.chars().count(),
        SAYS,
        std::format!("{written}, {following}")
    ));
    held
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_paragraph_is_cited() {
        assert_eq!(Rule::CITES.paragraph, 157);
        assert!(Rule::CITES.is_stated());
    }

    #[test]
    fn every_interjection_the_paragraph_names_is_spoken_of() {
        for held in ["эй", "ах", "о", "ох", "эх", "ну", "увы"] {
            assert!(
                speaks_of(held),
                "the paragraph names {held} and does not know it"
            );
        }
    }

    #[test]
    fn an_interjection_never_listed_is_reached_through_its_shape() {
        assert!(speaks_of("бла-бла-бла"));
        assert!(speaks_of("тук-тук"));
    }

    #[test]
    fn a_word_that_is_no_interjection_is_outside_the_paragraph() {
        assert_eq!(parted_from("стол", Some("стоит")), None);
        assert_eq!(parted_from("", None), None);
    }

    #[test]
    fn an_interjection_is_parted_by_a_comma() {
        assert_eq!(parted_from("увы", Some("он")), Some(true));
        assert_eq!(parted_from("эй", Some("поберегись")), Some(true));
        assert_eq!(parted_from("эй", Some("вы")), Some(true));
    }

    #[test]
    fn a_word_the_note_names_is_not_judged_before_another_word() {
        assert_eq!(parted_from("ах", Some("ты")), None);
        assert_eq!(parted_from("ну", Some("и")), None);
        assert_eq!(parted_from("ох", Some("ты")), None);
        assert_eq!(parted_from("ох", Some("уж")), None);
        assert_eq!(parted_from("о", Some("поле")), None);
        assert_eq!(parted_from("о", Some("как")), None);
    }

    #[test]
    fn another_interjection_before_an_address_keeps_its_comma() {
        assert_eq!(parted_from("эй", Some("друзья")), Some(true));
    }

    #[test]
    fn an_interjection_with_nothing_after_it_parts_from_nothing() {
        assert_eq!(parted_from("увы", None), Some(false));
        assert_eq!(parted_from("ох", None), Some(false));
    }

    #[test]
    fn the_case_a_word_is_written_in_does_not_matter() {
        assert_eq!(parted_from("АХ", Some("ТЫ")), None);
        assert_eq!(parted_from("Увы", Some("он")), Some(true));
    }
}
