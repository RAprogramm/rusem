// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The comparative of an adjective: one ending on one stem.
//!
//! School grammar states the productive form outright: the stem takes `-ее` —
//! `новее`, `красивее`, `свежее` — and it neither declines nor agrees, which
//! is why the cell it fills carries no categories. A stem in a back consonant
//! takes `-е` instead and swaps the consonant the way it swaps everywhere
//! before a front vowel — `громче`, `тише`, `строже` — and the swap is the
//! velar table already stated for the conjugation,
//! [`crate::morphemics::alternation`], asked there rather than restated.
//!
//! What the core cannot derive stays unstated, and much of what the grammar
//! calls the comparative on `-е` is exactly that. A dental closing the stem
//! may swap the way the present stem swaps it — `моложе`, `богаче` — or take
//! `-ее` the way `желтее` does, and no letter says which. A `к` after a
//! dental or a vowel is the suffix `-к-`/`-ок-`, which the comparative cuts
//! out while bending what stood before it — `у́же`, `ни́же`, `вы́ше`, `ши́ре`,
//! `коро́че` — and after `л` the swap writes a soft sign of its own:
//! `мельче`. The grammar further lists a closed handful built on `-ше`,
//! `-же`, `-ле`, and the suppletive ones on another stem entirely. All of
//! these need the dictionary, so all of them are refused rather than handed
//! a fabricated ending.
//!
//! The lemma's `ё` is not kept: the comparative does not hold the dictionary
//! form's stress — `лёгкий` says `легче` — so the derived stem writes `е`,
//! the letter § 10 of the 1956 code admits everywhere. A relational
//! adjective does not compare at all, and the colloquial `-ей` beside `-ее`
//! hangs on a stress the dictionary form does not state and is not written.

use super::{parted, relational, undotted};
use crate::{
    alphabet::{Letter, is_consonant},
    morphemics::alternation::{Kind, swapped}
};

/// The adjectives whose comparative school grammar lists on another stem.
///
/// `хороший` — `лучше`, `плохой` — `хуже`, `большой` — `больше`, `малый` and
/// `маленький` — `меньше`. The list is the source's enumeration, not a
/// tuning: these are the suppletive comparatives the school course names.
const SUPPLETIVE: &[&str] = &["хороший", "плохой", "большой", "малый", "маленький"];

/// The adjectives whose comparative the grammar builds on `-ше`, `-же`, `-ле`.
///
/// The academic grammar states them as a closed list — `дальше`, `дольше`,
/// `тоньше`, `старше`, `раньше`, `глубже`, `дешевле` (Русская грамматика
/// 1980) — and nothing in the letters parts `тонкий` from `звонкий`, whose
/// `звонче` the velar swap does write. The list is the source's enumeration,
/// not a tuning, and each word on it is refused rather than handed an ending.
const SUFFIXED: &[&str] = &[
    "далёкий",
    "долгий",
    "тонкий",
    "старый",
    "ранний",
    "глубокий",
    "дешёвый"
];

/// The comparative written out, when the core can derive it.
///
/// # Examples
///
/// ```
/// use rusem::grammar::declension::adjective::compared;
///
/// assert_eq!(compared::written("новый").as_deref(), Some("новее"));
/// assert_eq!(compared::written("громкий").as_deref(), Some("громче"));
/// assert_eq!(compared::written("тихий").as_deref(), Some("тише"));
/// assert_eq!(compared::written("узкий"), None);
/// assert_eq!(compared::written("хороший"), None);
/// ```
#[must_use]
pub fn written(dictionary: &str) -> Option<String> {
    if SUPPLETIVE.contains(&dictionary) || SUFFIXED.contains(&dictionary) {
        return None;
    }
    let (base, _) = parted(dictionary)?;
    if relational(&base) {
        return None;
    }
    let base = undotted(&base);
    let last = base.chars().last()?;

    if alternating(last) {
        return None;
    }
    if is_back(last) {
        return backed(&base, last);
    }

    Some(base + "ее")
}

/// The comparative of a stem in a back consonant, where the swap is safe.
///
/// The swap stands when the back consonant is the stem's own — `громче`,
/// `тише`, `строже`, `дороже` — and it is the stem's own except where `к`
/// follows a vowel or a dental, spelling the suffix `-к-`/`-ок-` that the
/// comparative cuts — `у́же`, `вы́ше` — or follows `л` and softens it:
/// `мельче`. Those stems are refused, because the cut and the softening are
/// the dictionary's facts, not the letters'.
fn backed(base: &str, last: char) -> Option<String> {
    let mut held: String = base.chars().take(base.chars().count() - 1).collect();
    if last == 'к' {
        let before = held.chars().last()?;
        if !is_consonant(before) || alternating(before) || before == 'л' {
            return None;
        }
    }

    held.push(swapped(last, Kind::Velar));
    held.push('е');
    Some(held)
}

/// Reports whether the comparative may claim this letter's own swap.
///
/// The dentals swap in the comparative the way they swap in the present stem
/// — `моложе`, `богаче`, `про́ще` — yet `желтее` takes the productive ending
/// on the same final letter. Which of the two a stem does is a dictionary
/// fact, so a stem closed by one of these answers nothing.
const fn alternating(letter: char) -> bool {
    matches!(letter, 'д' | 'т' | 'з' | 'с')
}

/// Reports whether a letter is a back consonant.
const fn is_back(letter: char) -> bool {
    matches!(Letter::of(letter), Some(Letter::Consonant(held)) if held.is_back())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_productive_ending_goes_on_the_bare_stem() {
        assert_eq!(written("новый").as_deref(), Some("новее"));
        assert_eq!(written("важный").as_deref(), Some("важнее"));
        assert_eq!(written("свежий").as_deref(), Some("свежее"));
    }

    #[test]
    fn a_back_consonant_swaps_and_takes_the_short_ending() {
        assert_eq!(written("громкий").as_deref(), Some("громче"));
        assert_eq!(written("тихий").as_deref(), Some("тише"));
        assert_eq!(written("строгий").as_deref(), Some("строже"));
        assert_eq!(written("дорогой").as_deref(), Some("дороже"));
    }

    #[test]
    fn a_suppletive_comparative_is_refused_not_fabricated() {
        for held in ["хороший", "плохой", "большой", "малый", "маленький"]
        {
            assert_eq!(written(held), None, "{held}");
        }
    }

    #[test]
    fn a_dental_stem_is_refused_not_handed_the_productive_ending() {
        for held in [
            "молодой",
            "простой",
            "частый",
            "чистый",
            "богатый",
            "толстый",
            "твёрдый"
        ] {
            assert_eq!(written(held), None, "{held}");
        }
    }

    #[test]
    fn a_suffix_k_the_comparative_cuts_is_refused() {
        for held in [
            "узкий",
            "низкий",
            "высокий",
            "широкий",
            "короткий",
            "сладкий",
            "редкий",
            "мелкий"
        ] {
            assert_eq!(written(held), None, "{held}");
        }
    }

    #[test]
    fn a_comparative_the_grammar_lists_on_another_suffix_is_refused() {
        for held in [
            "далёкий",
            "долгий",
            "тонкий",
            "старый",
            "ранний",
            "глубокий",
            "дешёвый"
        ] {
            assert_eq!(written(held), None, "{held}");
        }
    }

    #[test]
    fn the_lemmas_yo_is_written_e_off_its_stress() {
        assert_eq!(written("зелёный").as_deref(), Some("зеленее"));
        assert_eq!(written("весёлый").as_deref(), Some("веселее"));
        assert_eq!(written("лёгкий").as_deref(), Some("легче"));
        assert_eq!(written("тяжёлый").as_deref(), Some("тяжелее"));
    }

    #[test]
    fn a_relational_adjective_does_not_compare() {
        assert_eq!(written("русский"), None);
    }
}
