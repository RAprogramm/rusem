// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 88, пункт 5. Частица `не` с вопросительным словом
//! пишется слитно.
//!
//! `некто`, `нечто`, `некогда`, `негде`, `некуда`, `неоткуда`.
//!
//! Предлог между частицей и словом разводит их обратно: `не за что` пишется в
//! три слова, и это уже не тот случай, о котором говорит пункт.
//!
//! Что считается вопросительным словом, решает [`asking::asks`]: слияние и
//! построение — один и тот же факт, увиденный с двух сторон.

use super::PARTICLE;
use crate::{
    grammar::closed::asking,
    rules::{Citation, Findings, Found, Scope, scope}
};

/// Where this point is written.
pub const CITES: Citation = Citation::point(88, 5);

/// What this point is about.
pub const SCOPE: Scope = scope::ANY;

/// What the point says when it is broken.
const SAYS: &str = "частица не пишется с вопросительным словом слитно";

/// What the point finds in a run of characters.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::ne_together::pronouns::found;
///
/// let held = found("не кто");
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].instead, "некто");
///
/// assert!(found("некто").is_empty());
/// assert!(found("не за что").is_empty());
/// assert!(found("стол").is_empty());
/// ```
#[must_use]
pub fn found(written: &str) -> Findings {
    let mut held = Findings::new();
    let lowered = written.to_lowercase();
    let mut said = lowered.split_whitespace();
    let (Some(particle), Some(rest), None) = (said.next(), said.next(), said.next()) else {
        return held;
    };
    if particle != PARTICLE || !asking::asks(rest) {
        return held;
    }

    held.push(Found::new(
        CITES,
        particle.chars().count(),
        SAYS,
        std::format!("{particle}{rest}")
    ));
    held
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_point_is_cited() {
        assert_eq!(CITES.paragraph, 88);
        assert_eq!(CITES.point, 5);
    }

    #[test]
    fn the_particle_written_off_its_word_is_found_and_joined() {
        let held = found("не кто");

        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "некто");
    }

    #[test]
    fn a_word_already_written_solid_is_left_alone() {
        assert!(found("некто").is_empty());
    }

    #[test]
    fn a_preposition_between_them_is_another_case() {
        assert!(found("не за что").is_empty());
    }

    #[test]
    fn the_other_particle_is_the_other_paragraph() {
        assert!(found("ни кто").is_empty());
    }

    #[test]
    fn two_words_that_are_no_such_pair_are_left_alone() {
        assert!(found("на столе").is_empty());
        assert!(found("стол то").is_empty());
        assert!(found("").is_empty());
    }
}
