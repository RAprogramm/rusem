// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 157. Междометия отделяются запятой.
//!
//! `Эй, поберегись!`, `Увы, он не придёт`. The interjection stands outside the
//! sentence, and the comma is what says so.
//!
//! # Where the exceptions come from
//!
//! The paragraph excepts two cases, and neither is a list of words. Both are
//! read off what stands next to the interjection.
//!
//! `О поле, поле` takes no comma after `о`, because `о` there opens an
//! address and the address is what it opens onto. The same `о` in `О, как
//! хорошо` takes one. The word did not change; what followed it did.
//!
//! `Ах ты!`, `Ну и денёк!`, `Ох ты гой еси` take none either, because the
//! interjection and the word after it are one exclamation rather than two
//! things needing parting. Again the following word decides: `Ах, как жаль`
//! takes its comma.
//!
//! So the rule reads the pair, not the word. That is why it can be asked about
//! an interjection it has never met — the shape of the exception does not
//! depend on which interjection stands in it.

use crate::{
    grammar::closed::interjection,
    rules::{Citation, Findings, Found, Scope, scope, svod::hyphen_compound::interjections}
};

/// Where this rule is written.
pub const CITES: Citation = Citation::whole(157);

/// What this rule is about.
pub const SCOPE: Scope = scope::ANY;

/// The words that make an exclamation of the interjection before them.
///
/// `Ах ты!`, `Ну и денёк!`, `Ох ты гой еси`. Each is a particle or a pronoun
/// that leans on what comes before it and cannot open anything of its own, so
/// nothing stands between it and the interjection for a comma to divide.
const LEANING: &[&str] = &["ты", "и", "же", "уж", "вы"];

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
/// [`None`] when the word is no interjection, so that a caller can tell a word
/// the paragraph says nothing about from one it excepts.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::interjection_comma::parted_from;
///
/// assert_eq!(parted_from("увы", Some("он")), Some(true));
/// assert_eq!(parted_from("ах", Some("ты")), Some(false));
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

    Some(!leans(following) && !addresses(written, following))
}

/// Reports whether a word makes one exclamation of the interjection before it.
fn leans(following: &str) -> bool {
    LEANING.contains(&following.to_lowercase().as_str())
}

/// Reports whether the interjection opens an address rather than standing
/// alone.
///
/// Only `о` does this, and the paragraph says so: `О поле, поле`, `О други`.
/// The other interjections are parted from an address like anything else —
/// `Эй, друзья` keeps its comma.
fn addresses(written: &str, following: &str) -> bool {
    written.to_lowercase() == "о" && !following.is_empty()
}

/// What the paragraph says when it is broken.
const SAYS: &str = "междометие отделяется запятой от того, что стоит за ним";

/// What the paragraph finds in an interjection and what follows it.
///
/// The finding is the pair written with the comma the paragraph requires. A
/// word that is no interjection is outside the paragraph and nothing is found.
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
        CITES,
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
        assert_eq!(CITES.paragraph, 157);
        assert!(CITES.is_stated());
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
        assert_eq!(parted_from("ах", Some("как")), Some(true));
    }

    #[test]
    fn a_leaning_word_makes_one_exclamation_and_takes_no_comma() {
        assert_eq!(parted_from("ах", Some("ты")), Some(false));
        assert_eq!(parted_from("ну", Some("и")), Some(false));
        assert_eq!(parted_from("ох", Some("ты")), Some(false));
    }

    #[test]
    fn the_o_of_an_address_takes_no_comma() {
        assert_eq!(parted_from("о", Some("поле")), Some(false));
        assert_eq!(parted_from("о", Some("други")), Some(false));
    }

    #[test]
    fn another_interjection_before_an_address_keeps_its_comma() {
        assert_eq!(parted_from("эй", Some("друзья")), Some(true));
    }

    #[test]
    fn an_interjection_with_nothing_after_it_parts_from_nothing() {
        assert_eq!(parted_from("увы", None), Some(false));
    }

    #[test]
    fn the_case_a_word_is_written_in_does_not_matter() {
        assert_eq!(parted_from("АХ", Some("ТЫ")), Some(false));
        assert_eq!(parted_from("Увы", Some("он")), Some(true));
    }
}
