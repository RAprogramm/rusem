// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 72. Перед мягкой согласной знак не пишется.
//!
//! Во всех прочих случаях перед мягкими согласными, в том числе перед ч и щ,
//! буква ь не пишется: `кости`, `ранний`, `нянчить`, `кончик`, `каменщик`.
//!
//! Прочие — это все, кроме двух названных выше: кроме согласной, за которой
//! мягкая при изменении слова твердеет (пункт 1), и кроме `л`, чью мягкость
//! параграф пишет всегда (пункт 2). О мягкости самой согласной правило не
//! спрашивает — оно запрещает знак перед мягкой, какова бы ни была первая.
//! В источнике этот запрет стоит вне нумерации, поэтому ссылка идёт на
//! параграф целиком.

use super::{After, stands, without_sign};
use crate::rules::{Citation, Findings, Found, Scope, scope};

/// The point as a rule: what it cites and what it is about.
///
/// Holds the citation and the scope together so the engine can list the
/// point alongside the other paragraphs. The judging function [`found`]
/// stays free.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::soft_sign::not_before_soft::Rule;
///
/// assert_eq!(Rule::CITES.paragraph, 72);
/// ```
pub struct Rule;

impl Rule {
    /// Where this rule is written.
    ///
    /// The closing prohibition is unnumbered prose in § 72 — only the two
    /// soft-before-soft cases carry numbers — so the citation is the paragraph
    /// whole.
    pub const CITES: Citation = Citation::whole(72);

    /// What this point is about.
    pub const SCOPE: Scope = scope::Scope::ANY;
}

/// What the point says when it is broken.
const SAYS: &str = "перед мягкой согласной мягкий знак не пишется";

/// What the point finds.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::soft_sign::{After, not_before_soft::found};
///
/// assert!(found("кости", 'с', After::Soft('т'), 2).is_empty());
///
/// let held = found("косьти", 'с', After::Soft('т'), 2);
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].instead, "кости");
/// ```
#[must_use]
pub fn found(word: &str, written: char, after: After, at: usize) -> Findings {
    let mut held = Findings::new();
    let After::Soft(next) = after else {
        return held;
    };
    if super::before_l::keeps(written, next) {
        return held;
    }
    if !stands(word, at) {
        return held;
    }

    held.push(Found::new(
        Rule::CITES,
        at + 1,
        SAYS,
        without_sign(word, at)
    ));
    held
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_paragraph_is_cited_whole() {
        assert_eq!(Rule::CITES.paragraph, 72);
        assert_eq!(Rule::CITES.point, 0);
    }

    #[test]
    fn a_word_written_without_the_sign_is_left_alone() {
        assert!(found("кости", 'с', After::Soft('т'), 2).is_empty());
        assert!(found("нянчить", 'н', After::Soft('ч'), 2).is_empty());
    }

    #[test]
    fn a_sign_before_a_soft_consonant_is_found_and_taken_out() {
        let held = found("косьти", 'с', After::Soft('т'), 2);

        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "кости");
    }

    #[test]
    fn the_letter_of_the_l_point_is_left_to_it() {
        assert!(found("сельдь", 'л', After::Soft('д'), 2).is_empty());
    }

    #[test]
    fn between_two_soft_l_the_note_takes_the_sign_out() {
        let held = found("гульливый", 'л', After::Soft('л'), 2);

        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "гулливый");
    }
}
