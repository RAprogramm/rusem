// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 72, пункт 1. Мягкий знак перед мягкой согласной, которая твердеет.
//!
//! Для обозначения мягкости согласной, стоящей перед другой мягкой согласной,
//! ь пишется, если при изменении слова вторая мягкая согласная становится
//! твёрдой, а первая согласная сохраняет свою мягкость: `няньки` (нянька),
//! `свадьбе` (свадьба), `восьми` (восьмой).
//!
//! Точка опоры этого пункта — не буквы, а другое слово: `няньки` пишется со
//! знаком потому, что есть `нянька`, где за `н` стоит твёрдая. Кто спрашивает
//! пункт, тот и знает это; сам он по написанному этого не выведет.

use super::{After, handed_over, stands, with_sign, without_sign};
use crate::rules::{Citation, Findings, Found, Scope, scope};

/// Where this point is written.
///
/// The first of the two numbered points of § 72 — both belong to the
/// soft-before-soft case.
pub const CITES: Citation = Citation::point(72, 1);

/// What this point is about.
pub const SCOPE: Scope = scope::ANY;

/// What the point says when the sign is missing.
const SAYS: &str =
    "мягкость согласной обозначается знаком, если при изменении слова следующая твердеет";

/// What it says when the sign stands after a hard consonant.
const SAYS_NOT: &str = "после твёрдой согласной мягкий знак не пишется";

/// What the point finds.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::soft_sign::{After, before_hardening::found};
///
/// assert!(found("няньки", 'н', true, After::Hardening, 2).is_empty());
///
/// let held = found("нянки", 'н', true, After::Hardening, 2);
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].instead, "няньки");
/// ```
#[must_use]
pub fn found(word: &str, written: char, soft: bool, after: After, at: usize) -> Findings {
    let mut held = Findings::new();
    if !matches!(after, After::Hardening) || handed_over(written) {
        return held;
    }

    let written_sign = stands(word, at);
    if written_sign == soft {
        return held;
    }

    let (says, instead) = if soft {
        (SAYS, with_sign(word, at))
    } else {
        (SAYS_NOT, without_sign(word, at))
    };

    held.push(Found::new(CITES, at + 1, says, instead));
    held
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_point_is_cited() {
        assert_eq!(CITES.paragraph, 72);
        assert_eq!(CITES.point, 1);
    }

    #[test]
    fn a_consonant_before_one_that_hardens_takes_the_sign() {
        assert!(found("няньки", 'н', true, After::Hardening, 2).is_empty());

        let held = found("нянки", 'н', true, After::Hardening, 2);
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "няньки");
    }

    #[test]
    fn a_soft_consonant_that_stays_soft_is_another_point() {
        assert!(found("кости", 'с', true, After::Soft('т'), 2).is_empty());
    }
}
