// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 90, пункт 1. Частица `ни` с местоимением пишется слитно.
//!
//! `никто`, `ничто`, `никакой`, `ничей`. Наречия — `никогда`, `нигде` — пункт
//! не называет: они перечислены в пункте 2 и записаны в [`super::adverbs`].
//!
//! Предлог между частицей и словом разводит их обратно: `ни у кого` пишется в
//! три слова, и это уже не тот случай, о котором говорит пункт.
//!
//! Что считается вопросительным местоимением, решает [`asking::PRONOUNS`]:
//! слияние и построение — один и тот же факт, увиденный с двух сторон.
//! `нисколько` пункт 2 называет наречием, и хотя `сколько` стоит среди
//! местоимений, слово уходит туда, куда его записал источник.

use super::PARTICLE;
use crate::{
    grammar::closed::asking,
    rules::{Citation, Findings, Found, Scope, scope}
};

/// Where this point is written.
pub const CITES: Citation = Citation::point(90, 1);

/// What this point is about.
pub const SCOPE: Scope = scope::ANY;

/// What the point says when it is broken.
const SAYS: &str = "частица ни пишется с местоимением слитно";

/// What the point finds in a run of characters.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::ni_together::pronouns::found;
///
/// let held = found("ни кто");
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].instead, "никто");
///
/// assert!(found("никто").is_empty());
/// assert!(found("ни у кого").is_empty());
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
    if particle != PARTICLE || !asking::PRONOUNS.contains(&rest) {
        return held;
    }

    let joined = std::format!("{particle}{rest}");
    if super::adverbs::names(&joined) {
        return held;
    }

    held.push(Found::new(CITES, particle.chars().count(), SAYS, joined));
    held
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_point_is_cited() {
        assert_eq!(CITES.paragraph, 90);
        assert_eq!(CITES.point, 1);
    }

    #[test]
    fn the_particle_written_off_its_word_is_found_and_joined() {
        let held = found("ни кто");

        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "никто");
    }

    #[test]
    fn a_word_already_written_solid_is_left_alone() {
        assert!(found("никто").is_empty());
    }

    #[test]
    fn a_preposition_between_them_is_another_case() {
        assert!(found("ни у кого").is_empty());
    }

    #[test]
    fn the_other_particle_is_the_other_paragraph() {
        assert!(found("не кто").is_empty());
    }

    #[test]
    fn an_adverb_is_the_other_point() {
        assert!(found("ни когда").is_empty());
        assert!(found("ни где").is_empty());
    }

    #[test]
    fn the_word_the_source_calls_an_adverb_is_left_to_the_other_point() {
        assert!(found("ни сколько").is_empty());
    }

    #[test]
    fn two_words_that_are_no_such_pair_are_left_alone() {
        assert!(found("на столе").is_empty());
        assert!(found("стол то").is_empty());
        assert!(found("").is_empty());
    }
}
