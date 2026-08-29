// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 72, пункт 2. Мягкий знак для обозначения мягкости `л`.
//!
//! Для обозначения мягкости согласной, стоящей перед другой мягкой согласной,
//! ь пишется для обозначения мягкости `л`: `сельдь`, `льстить`, `мельче`,
//! `пальчик`.
//!
//! Примечание к параграфу: между двумя мягкими `л` буква ь не пишется —
//! `иллюзия`, `гулливый`, — и это единственный случай, когда пункт молчит о
//! своей же букве.

use super::{After, NAMED, stands, with_sign};
use crate::rules::{Citation, Findings, Found, Scope, scope};

/// Where this point is written.
///
/// The second of the two numbered points of § 72 — both belong to the
/// soft-before-soft case.
pub const CITES: Citation = Citation::point(72, 2);

/// What this point is about.
pub const SCOPE: Scope = scope::ANY;

/// What the point says when the sign is missing.
const SAYS: &str = "мягкость л обозначается мягким знаком";

/// Reports whether the point writes the sign between two consonants.
///
/// The first must be the letter the point names, soft — which the caller
/// vouches for, since the letters do not show it — and the second must not be
/// the same letter, by the note.
#[must_use]
pub const fn keeps(written: char, next: char) -> bool {
    written == NAMED && next != NAMED
}

/// What the point finds.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::soft_sign::{After, before_l::found};
///
/// assert!(found("сельдь", 'л', true, After::Soft('д'), 2).is_empty());
///
/// let held = found("селдь", 'л', true, After::Soft('д'), 2);
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].instead, "сельдь");
///
/// assert!(found("иллюзия", 'л', true, After::Soft('л'), 1).is_empty());
/// ```
#[must_use]
pub fn found(word: &str, written: char, soft: bool, after: After, at: usize) -> Findings {
    let mut held = Findings::new();
    let After::Soft(next) = after else {
        return held;
    };
    if !keeps(written, next) || !soft {
        return held;
    }
    if stands(word, at) {
        return held;
    }

    held.push(Found::new(CITES, at + 1, SAYS, with_sign(word, at)));
    held
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_point_is_cited() {
        assert_eq!(CITES.paragraph, 72);
        assert_eq!(CITES.point, 2);
    }

    #[test]
    fn the_letter_the_point_names_takes_the_sign_before_a_soft_consonant() {
        assert!(found("сельдь", 'л', true, After::Soft('д'), 2).is_empty());

        let held = found("селдь", 'л', true, After::Soft('д'), 2);
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "сельдь");
    }

    #[test]
    fn between_two_soft_l_the_note_keeps_the_point_silent() {
        assert!(found("иллюзия", 'л', true, After::Soft('л'), 1).is_empty());
        assert!(found("илльюзия", 'л', true, After::Soft('л'), 2).is_empty());
    }

    #[test]
    fn any_other_consonant_is_not_this_point() {
        assert!(found("кости", 'с', true, After::Soft('т'), 2).is_empty());
    }
}
