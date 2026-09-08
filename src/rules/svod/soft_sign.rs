// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 72. Буква ь пишется для обозначения мягкости согласной.
//!
//! Кроме ч и щ (см. § 75), в конце слова — `пить`, `темь`, `конь` — и в
//! середине слова перед твёрдой согласной: `молотьба`, `просьба`, `нянька`,
//! `меньше`.
//!
//! Для обозначения мягкости согласной, стоящей перед другой мягкой согласной,
//! ь пишется в следующих случаях:
//!
//! 1. Если при изменении слова вторая мягкая согласная становится твёрдой, а
//!    первая согласная сохраняет свою мягкость: `няньки` (нянька), `свадьбе`
//!    (свадьба), `восьми` (восьмой).
//! 2. Для обозначения мягкости `л`: `сельдь`, `льстить`, `мельче`, `пальчик`.
//!
//! Во всех прочих случаях перед мягкими согласными, в том числе перед ч и щ,
//! буква ь не пишется: `кости`, `ранний`, `нянчить`, `кончик`, `каменщик`.
//!
//! Примечание. Между двумя мягкими `л` буква ь не пишется: `иллюзия`,
//! `гулливый`.
//!
//! # The statements, one to a file
//!
//! The paragraph says five different things, and each of them is its own
//! rule: it holds in its own case and it is broken on its own. Writing them
//! as one function would make a breach of one statement indistinguishable
//! from a breach of another, and a reader could not be told which half of
//! § 72 he had broken.
//!
//! The source numbers only the two soft-before-soft cases as пункт 1 and
//! пункт 2; the other three statements are unnumbered prose and are cited as
//! the paragraph whole, told apart by what each says.
//!
//! | Cites | Module | Says |
//! | --- | --- | --- |
//! | § 72 | [`end_of_word`] | a soft consonant closing a word takes the sign |
//! | § 72 | [`before_hard`] | a soft consonant before a hard one takes it |
//! | § 72, п. 1 | [`before_hardening`] | before a soft consonant that hardens as the word changes |
//! | § 72, п. 2 | [`before_l`] | to write the softness of `л` |
//! | § 72 | [`not_before_soft`] | nowhere else before a soft consonant |
//!
//! # What every rule is asked
//!
//! The same three things: the word, which consonant of it is being judged, and
//! what stands after that consonant. Whether the consonant is soft at all is
//! asked too, because the letters do not always show it — and a rule that is
//! not told cannot judge and says nothing.

pub mod before_hard;
pub mod before_hardening;
pub mod before_l;
pub mod end_of_word;
pub mod not_before_soft;

use crate::{
    alphabet::{Letter, consonant::Hardness},
    rules::{Citation, Findings}
};

/// § 72 as a rule: what it cites, the sign it writes and the consonant it
/// names.
///
/// Holds the paragraph-level facts together so callers name one owner for
/// them. The judging functions stay free; the points live in their own
/// modules.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::soft_sign::Rule;
///
/// assert_eq!(Rule::CITES.paragraph, 72);
/// assert_eq!(Rule::SIGN, 'ь');
/// assert_eq!(Rule::NAMED, 'л');
/// ```
pub struct Rule;

impl Rule {
    /// Where the paragraph is written.
    pub const CITES: Citation = Citation::whole(72);

    /// The letter the paragraph is about.
    pub const SIGN: char = 'ь';

    /// The consonant the second point names.
    pub const NAMED: char = 'л';
}

/// What stands after the consonant being judged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum After {
    /// Nothing: the consonant closes the word.
    End,
    /// A hard consonant.
    Hard,
    /// A soft consonant that stays soft when the word changes.
    Soft(char),
    /// A soft consonant that hardens when the word changes: `няньки` beside
    /// `нянька`.
    Hardening
}

/// Everything § 72 finds about one consonant of a word.
///
/// Each rule is asked in turn, and every one that is broken is reported with
/// what it says. They do not overlap: a consonant standing at the end of a
/// word is not standing before anything, and a rule about what follows has
/// nothing to say about it.
#[must_use]
pub fn found(word: &str, written: char, soft: bool, after: After, at: usize) -> Findings {
    let mut held = Findings::new();

    held.extend(end_of_word::found(word, written, soft, after, at));
    held.extend(before_hard::found(word, written, soft, after, at));
    held.extend(before_hardening::found(word, written, soft, after, at));
    held.extend(before_l::found(word, written, soft, after, at));
    held.extend(not_before_soft::found(word, written, after, at));

    held
}

/// What § 72 finds in a written word, judged by the letters alone.
///
/// The letters show only part of the paragraph, and only that part is judged.
/// A sign standing before a consonant that is soft of itself — `ч`, `щ`, `й`
/// — breaks the closing prohibition unless the second point holds it:
/// `няньчить` is written `нянчить`, while `пальчик` keeps its sign for the
/// `л` the second point names. A sign between two `л` breaks the note:
/// `гульливый` is written `гулливый`.
///
/// A missing sign is never judged here, because whether one is due needs the
/// softness of the consonant, and the letters do not carry it: `кон` and
/// `конь` are both words. The rules take that softness as a fact from
/// whoever holds it — the paradigm does — and judge the rest.
#[must_use]
pub fn judged(word: &str) -> Findings {
    let letters: std::vec::Vec<char> = word.chars().collect();
    let mut held = Findings::new();

    for (at, letter) in letters.iter().enumerate() {
        if letters.get(at + 1) != Some(&Rule::SIGN) {
            continue;
        }
        if !crate::alphabet::is_consonant(*letter) || handed_over(*letter) {
            continue;
        }
        let Some(next) = letters.get(at + 2).copied() else {
            continue;
        };
        let between_two_l = *letter == Rule::NAMED && next == Rule::NAMED;
        if !surely_soft(next) && !between_two_l {
            continue;
        }

        held.extend(not_before_soft::found(word, *letter, After::Soft(next), at));
    }

    held
}

/// Reports whether a consonant is soft whatever the word does with it.
const fn surely_soft(written: char) -> bool {
    matches!(
        Letter::of(written),
        Some(Letter::Consonant(held)) if matches!(held.hardness(), Hardness::AlwaysSoft)
    )
}

/// Reports whether § 75 speaks of this consonant instead.
///
/// The four sibilants. `ч` and `щ` are soft and `ж` and `ш` hard whatever
/// follows, so a sign after any of them marks no softness — where it stands,
/// as in `рожь` or `ешь`, it is § 75 that put it there, and § 72 names the
/// hand-over outright.
#[must_use]
pub const fn handed_over(written: char) -> bool {
    matches!(
        Letter::of(written),
        Some(Letter::Consonant(held)) if held.is_sibilant()
    )
}

/// Reports whether the sign stands after a consonant in a word.
#[must_use]
pub fn stands(word: &str, at: usize) -> bool {
    word.chars().nth(at + 1) == Some(Rule::SIGN)
}

/// The word with the sign put in after a consonant.
#[must_use]
pub fn with_sign(word: &str, at: usize) -> std::string::String {
    let mut held: std::string::String = word.chars().take(at + 1).collect();
    held.push(Rule::SIGN);
    held.extend(word.chars().skip(at + 1));
    held
}

/// The word with the sign taken out from after a consonant.
#[must_use]
pub fn without_sign(word: &str, at: usize) -> std::string::String {
    crate::rules::found::without(word, at + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_paragraph_is_cited() {
        assert_eq!(Rule::CITES.paragraph, 72);
        assert!(Rule::CITES.is_stated());
    }

    #[test]
    fn a_sign_before_a_consonant_soft_of_itself_is_judged_off_the_letters() {
        let held = judged("няньчить");

        assert_eq!(held.len(), 1);
        assert_eq!(held[0].cites, not_before_soft::Rule::CITES);
        assert_eq!(held[0].instead, "нянчить");
    }

    #[test]
    fn the_sign_the_l_point_holds_is_left_standing() {
        assert!(judged("пальчик").is_empty());
        assert!(judged("мальчик").is_empty());
    }

    #[test]
    fn a_sign_between_two_l_is_judged_by_the_note() {
        let held = judged("гульливый");

        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "гулливый");
    }

    #[test]
    fn a_sign_the_letters_cannot_judge_is_left_alone() {
        for held in ["конь", "просьба", "нянька", "меньше", "сельдь", "письмо"]
        {
            assert!(judged(held).is_empty(), "{held}");
        }
    }

    #[test]
    fn a_sign_after_a_sibilant_is_the_next_paragraph_s() {
        for held in ["рожь", "мышь", "ешь", "ночью"] {
            assert!(judged(held).is_empty(), "{held}");
        }
    }

    #[test]
    fn a_dividing_sign_before_a_vowel_is_another_paragraph_s() {
        for held in ["вьюн", "семья", "ружьё"] {
            assert!(judged(held).is_empty(), "{held}");
        }
    }
}
