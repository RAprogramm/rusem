// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 90, пункт 2. Частица `ни` в наречиях пишется слитно.
//!
//! Пункт перечисляет их сам: `никогда`, `нигде`, `никуда`, `ниоткуда`,
//! `никак`, `нисколько`, `нимало`, `нипочём`, `ничуть` — и частицу `-нибудь`,
//! чей дефис записан в § 86.
//!
//! Список закрыт самим источником, поэтому здесь он допустим: слово вне
//! перечня — `низачем` — пункт не пишет, и правило о нём молчит.

use crate::rules::{Citation, Findings, Found, Scope, scope};

/// § 90, пункт 2, as a rule.
///
/// Holds where the point is written and what it is about.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::ni_together::adverbs::Rule;
///
/// assert_eq!(Rule::CITES.paragraph, 90);
/// assert_eq!(Rule::CITES.point, 2);
/// ```
pub struct Rule;

impl Rule {
    /// Where this point is written.
    pub const CITES: Citation = Citation::point(90, 2);

    /// What this point is about.
    pub const SCOPE: Scope = scope::Scope::ANY;
}

/// The adverbs the point enumerates.
///
/// The list is the source's own, from § 90 п. 2. The running text of the
/// code prints `нипочем` — it prints `е` for `ё` throughout — but its word
/// index states the word as `нипочём, наречие (§ 4, Б, п. 7; § 90, п. 2)`,
/// and the engine writes the letter the word carries.
const JOINED: &[&str] = &[
    "никогда",
    "нигде",
    "никуда",
    "ниоткуда",
    "никак",
    "нисколько",
    "нимало",
    "нипочём",
    "ничуть"
];

/// What the point says when it is broken.
const SAYS: &str = "частица ни в этих наречиях пишется слитно";

/// Reports whether a joined word is one the point enumerates.
#[must_use]
pub fn names(written: &str) -> bool {
    JOINED.contains(&written)
}

/// What the point finds in a run of characters.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::ni_together::adverbs::found;
///
/// let held = found("ни когда");
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].instead, "никогда");
///
/// assert!(found("никогда").is_empty());
/// assert!(found("ни зачем").is_empty());
/// ```
#[must_use]
pub fn found(written: &str) -> Findings {
    let mut held = Findings::new();
    let lowered = written.to_lowercase();
    let mut said = lowered.split_whitespace();
    let (Some(particle), Some(rest), None) = (said.next(), said.next(), said.next()) else {
        return held;
    };
    if particle != super::Rule::PARTICLE {
        return held;
    }

    let joined = std::format!("{particle}{rest}");
    if !names(&joined) {
        return held;
    }

    held.push(Found::new(
        Rule::CITES,
        particle.chars().count(),
        SAYS,
        joined
    ));
    held
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_point_is_cited() {
        assert_eq!(Rule::CITES.paragraph, 90);
        assert_eq!(Rule::CITES.point, 2);
    }

    #[test]
    fn the_particle_written_off_its_adverb_is_found_and_joined() {
        let held = found("ни когда");

        assert_eq!(held.len(), 1);
        assert_eq!(held[0].cites, Rule::CITES);
        assert_eq!(held[0].instead, "никогда");
    }

    #[test]
    fn every_adverb_the_point_names_is_reached() {
        for joined in JOINED {
            let parted = std::format!("ни {}", &joined[super::super::Rule::PARTICLE.len()..]);

            assert_eq!(found(&parted)[0].instead, *joined, "{joined}");
        }
    }

    #[test]
    fn a_word_already_written_solid_is_left_alone() {
        assert!(found("никогда").is_empty());
    }

    #[test]
    fn a_word_outside_the_list_is_not_the_point_s() {
        assert!(found("ни зачем").is_empty());
        assert!(found("ни почему").is_empty());
    }

    #[test]
    fn the_other_particle_is_the_other_paragraph() {
        assert!(found("не когда").is_empty());
    }
}
