// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The affix tables the segmenter cuts by.
//!
//! The tables are authored here rather than imported, because no open Russian
//! morpheme dictionary carries a license the project can ship under. They are
//! written longest first, so a greedy scan finds `недо-` before `не-` and
//! `-тельн-` before `-н-`.
//!
//! A table entry is a claim about the language, not about any one word. The
//! segmenter never trusts a match on its own: a prefix comes off only when what
//! remains still looks like a word, and the result is graded accordingly.

/// Prefixes, ordered so that a longer one is tried before a shorter one it
/// starts with.
pub const PREFIXES: &[&str] = &[
    "противо",
    "сверх",
    "перед",
    "около",
    "между",
    "через",
    "полу",
    "пере",
    "пред",
    "недо",
    "анти",
    "контр",
    "супер",
    "ультра",
    "псевдо",
    "квази",
    "кибер",
    "нео",
    "обез",
    "обес",
    "разо",
    "рас",
    "раз",
    "роз",
    "рос",
    "без",
    "бес",
    "воз",
    "вос",
    "низ",
    "нис",
    "под",
    "над",
    "при",
    "про",
    "пре",
    "про",
    "из",
    "ис",
    "вз",
    "вс",
    "до",
    "за",
    "на",
    "не",
    "об",
    "от",
    "по",
    "со",
    "вы",
    "во",
    "в",
    "о",
    "у",
    "с"
];

/// Suffixes, ordered so that a longer one is tried before a shorter one it ends
/// with.
pub const SUFFIXES: &[&str] = &[
    "оват",
    "еват",
    "тельн",
    "енниц",
    "енник",
    "ательн",
    "ительн",
    "ость",
    "есть",
    "изм",
    "ист",
    "тель",
    "еник",
    "ниц",
    "ник",
    "чик",
    "щик",
    "щиц",
    "чиц",
    "льщик",
    "ени",
    "ани",
    "ств",
    "еств",
    "изн",
    "инк",
    "ушк",
    "юшк",
    "ишк",
    "оньк",
    "еньк",
    "отн",
    "ищ",
    "иц",
    "ец",
    "ач",
    "аж",
    "яг",
    "як",
    "ак",
    "ик",
    "ок",
    "ек",
    "лив",
    "чив",
    "аст",
    "ов",
    "ев",
    "ск",
    "ыва",
    "ива",
    "ова",
    "ева",
    "от",
    "ну",
    "н",
    "к",
    "л",
    "и",
    "а",
    "я",
    "е"
];

/// Inflectional endings, longest first.
///
/// The table is a filter, not a parser: the ending of a form is found from its
/// paradigm, and this list only rules out a tail that no Russian ending looks
/// like.
pub const ENDINGS: &[&str] = &[
    "ыми", "ими", "ого", "его", "ому", "ему", "ами", "ями", "ешь", "ишь", "ете", "ите", "ает",
    "ают", "ует", "уют", "ой", "ей", "ый", "ий", "ая", "яя", "ое", "ее", "ые", "ие", "ом", "ем",
    "ам", "ям", "ах", "ях", "ов", "ев", "ут", "ют", "ат", "ят", "ет", "ит", "ем", "им", "ть",
    "ти", "чь", "ла", "ло", "ли", "ья", "ью", "а", "я", "о", "е", "ы", "и", "у", "ю", "ь"
];

/// The suffixes only a verb is built with.
///
/// A stem vowel and the mark of the past tense belong to verbs and to nothing
/// else. Left in the common table they cut nouns apart: `учитель` came out as
/// `учит` and the past-tense `л`, which is not a word anybody has written.
pub const VERBAL_SUFFIXES: &[&str] = &["л", "ну", "ыва", "ива", "ова", "ева", "и", "а", "я", "е"];

/// Postfixes, which stand after the ending.
pub const POSTFIXES: &[&str] = &["ся", "сь", "то", "либо", "нибудь"];

/// The postfixes that turn a verb back on itself.
///
/// A subset of the postfixes above, and the one a gate on reflexive verbs
/// wants: `-то`, `-либо` and `-нибудь` are postfixes too and make a pronoun
/// indefinite rather than a verb reflexive.
pub const REFLEXIVE: &[&str] = &["ся", "сь"];

/// Linking vowels, which join two roots in a compound.
pub const INTERFIXES: &[&str] = &["о", "е"];

/// Reports whether a stretch of a word holds a vowel.
///
/// A morpheme cut that leaves a vowelless root is nearly always wrong, so the
/// segmenter uses this as its stopping rule.
///
/// # Examples
///
/// ```
/// use rusem::morphemics::affix::has_vowel;
///
/// assert!(has_vowel("стол"));
/// assert!(!has_vowel("стл"));
/// ```
#[must_use]
pub fn has_vowel(text: &str) -> bool {
    text.chars().any(crate::alphabet::is_vowel)
}

/// Returns the longest table entry the text starts with.
#[must_use]
pub fn leading<'t>(table: &[&'t str], text: &str) -> Option<&'t str> {
    table
        .iter()
        .filter(|candidate| text.starts_with(**candidate))
        .max_by_key(|candidate| candidate.chars().count())
        .copied()
}

/// Returns the longest table entry the text ends with.
#[must_use]
pub fn trailing<'t>(table: &[&'t str], text: &str) -> Option<&'t str> {
    table
        .iter()
        .filter(|candidate| text.ends_with(**candidate))
        .max_by_key(|candidate| candidate.chars().count())
        .copied()
}

/// Every table entry the text starts with, longest first.
///
/// [`leading`] answers the longest alone, which is right when a caller wants
/// one reading. A caller offering every cut wants every match: `записк` ends
/// in both `иск` and `к`, and taking only the longer one loses the right
/// answer when what it leaves behind is no root.
#[must_use]
pub fn all_leading<'t>(table: &[&'t str], text: &str) -> Vec<&'t str> {
    let mut held: Vec<&str> = table
        .iter()
        .filter(|candidate| text.starts_with(**candidate))
        .copied()
        .collect();
    held.sort_unstable_by_key(|candidate| core::cmp::Reverse(candidate.chars().count()));

    held
}

/// Every table entry the text ends with, longest first.
#[must_use]
pub fn all_trailing<'t>(table: &[&'t str], text: &str) -> Vec<&'t str> {
    let mut held: Vec<&str> = table
        .iter()
        .filter(|candidate| text.ends_with(**candidate))
        .copied()
        .collect();
    held.sort_unstable_by_key(|candidate| core::cmp::Reverse(candidate.chars().count()));

    held
}

#[cfg(test)]
mod extra {
    use super::{POSTFIXES, REFLEXIVE};

    #[test]
    fn every_reflexive_postfix_is_a_postfix() {
        assert!(REFLEXIVE.iter().all(|held| POSTFIXES.contains(held)));
        assert!(REFLEXIVE.len() < POSTFIXES.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_prefix_wins() {
        assert_eq!(leading(PREFIXES, "перестройка"), Some("пере"));
        assert_eq!(leading(PREFIXES, "недоделка"), Some("недо"));
    }

    #[test]
    fn longest_suffix_wins() {
        assert_eq!(trailing(SUFFIXES, "читатель"), Some("тель"));
        assert_eq!(trailing(SUFFIXES, "смелость"), Some("ость"));
    }

    #[test]
    fn a_word_without_a_listed_affix_matches_nothing() {
        assert_eq!(leading(PREFIXES, "лес"), None);
    }

    #[test]
    fn vowels_are_recognized() {
        assert!(has_vowel("мгла"));
        assert!(!has_vowel("вств"));
    }
}
