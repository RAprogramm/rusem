// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 72, пункт 1. Мягкий знак в конце слова.
//!
//! Буква ь пишется для обозначения мягкости согласной, кроме ч и щ, в конце
//! слова: `пить`, `темь`, `конь`.
//!
//! Это самый простой из пунктов параграфа: за согласной ничего не стоит, и
//! обозначить её мягкость больше нечем.

use super::{After, handed_over, stands, with_sign, without_sign};
use crate::rules::{Citation, Findings, Found, Scope, scope};

/// Where this point is written.
pub const CITES: Citation = Citation::point(72, 1);

/// What this point is about.
pub const SCOPE: Scope = scope::ANY;

/// What the point says when the sign is missing.
const SAYS: &str = "мягкость согласной в конце слова обозначается мягким знаком";

/// What it says when the sign stands after a hard consonant.
const SAYS_NOT: &str = "после твёрдой согласной мягкий знак не пишется";

/// What the point finds.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::soft_sign::{After, end_of_word::found};
///
/// assert!(found("конь", 'н', true, After::End, 2).is_empty());
///
/// let held = found("кон", 'н', true, After::End, 2);
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].instead, "конь");
/// ```
#[must_use]
pub fn found(word: &str, written: char, soft: bool, after: After, at: usize) -> Findings {
    let mut held = Findings::new();
    if !matches!(after, After::End) || handed_over(written) {
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
    fn a_soft_consonant_closing_a_word_takes_the_sign() {
        assert!(found("конь", 'н', true, After::End, 2).is_empty());

        let held = found("кон", 'н', true, After::End, 2);
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "конь");
    }

    #[test]
    fn a_hard_consonant_closing_a_word_takes_none() {
        assert!(found("стол", 'л', false, After::End, 3).is_empty());

        let held = found("столь", 'л', false, After::End, 3);
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "стол");
    }

    #[test]
    fn the_consonants_of_the_next_paragraph_are_left_to_it() {
        assert!(found("ноч", 'ч', true, After::End, 2).is_empty());
        assert!(found("вещ", 'щ', true, After::End, 2).is_empty());
    }

    #[test]
    fn a_consonant_standing_before_something_is_another_point() {
        assert!(found("косьба", 'с', true, After::Hard, 2).is_empty());
    }
}
