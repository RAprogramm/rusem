// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Where a preposition came from, which decides how it is written.
//!
//! A **primitive** preposition was never anything else. `в`, `на`, `к` have no
//! history inside Russian: they are as old as the cases they govern, they
//! govern more than one of them, and nothing about them is spelled by rule.
//!
//! A **derived** one is a word of another class that stopped being one. It
//! keeps the shape it had — `вокруг` is an adverb, `вследствие` a noun in a
//! case, `благодаря` an adverbial participle — and that shape is what a
//! spelling rule reads. `вследствие ошибки` is a preposition written as one
//! word with `е`; `в следствии по делу` is a noun in the prepositional. Only
//! knowing which it came from tells the two apart.
//!
//! The division is therefore not a taxonomy for its own sake: it names the
//! prepositions a spelling gate must look at, and leaves out the ones it never
//! needs to.

/// What a preposition was before it became one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Origin {
    /// `в`, `на`, `к`: never anything else.
    Primitive,
    /// `вокруг`, `мимо`, `после`: an adverb.
    Adverbial,
    /// `вместо`, `вследствие`, `в течение`: a noun in a case.
    Nominal,
    /// `благодаря`, `несмотря на`: an adverbial participle.
    Verbal
}

/// The prepositions that were never anything else.
pub const PRIMITIVE: &[&str] = &[
    "без",
    "безо",
    "в",
    "во",
    "для",
    "до",
    "за",
    "из",
    "из-за",
    "из-под",
    "изо",
    "к",
    "ко",
    "между",
    "на",
    "над",
    "надо",
    "о",
    "об",
    "обо",
    "от",
    "ото",
    "перед",
    "передо",
    "по",
    "под",
    "подо",
    "при",
    "про",
    "ради",
    "с",
    "сквозь",
    "со",
    "у",
    "через"
];

/// The prepositions that were adverbs.
pub const ADVERBIAL: &[&str] = &[
    "близ",
    "вблизи",
    "вдоль",
    "вне",
    "внутри",
    "возле",
    "вокруг",
    "вопреки",
    "впереди",
    "кроме",
    "мимо",
    "навстречу",
    "наперекор",
    "напротив",
    "наряду с",
    "около",
    "позади",
    "помимо",
    "после",
    "посреди",
    "против",
    "сверх",
    "сзади",
    "согласно",
    "среди"
];

/// The prepositions that were nouns in a case.
///
/// The one-word ones and the compounds alike: `вследствие` and `в течение`
/// stand or fall by the same rule, and both are a noun that stopped being one.
pub const NOMINAL: &[&str] = &[
    "в отличие от",
    "в продолжение",
    "в связи с",
    "в силу",
    "в течение",
    "в ходе",
    "вместо",
    "во время",
    "вследствие",
    "за счёт",
    "на протяжении",
    "по мере",
    "по поводу",
    "по причине"
];

/// The prepositions that were adverbial participles.
pub const VERBAL: &[&str] = &["благодаря", "несмотря на"];

/// Every origin, with the prepositions of it.
const ORIGINS: &[(Origin, &[&str])] = &[
    (Origin::Primitive, PRIMITIVE),
    (Origin::Adverbial, ADVERBIAL),
    (Origin::Nominal, NOMINAL),
    (Origin::Verbal, VERBAL)
];

/// Where a written preposition came from, or nothing when it is none.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::preposition::origin::{Origin, origin};
///
/// assert_eq!(origin("в"), Some(Origin::Primitive));
/// assert_eq!(origin("вокруг"), Some(Origin::Adverbial));
/// assert_eq!(origin("вследствие"), Some(Origin::Nominal));
/// assert_eq!(origin("стол"), None);
/// ```
#[must_use]
pub fn origin(written: &str) -> Option<Origin> {
    let held = written.to_lowercase();

    ORIGINS
        .iter()
        .find(|(_, words)| words.contains(&held.as_str()))
        .map(|(origin, _)| *origin)
}

/// Reports whether a preposition was a word of another class.
///
/// Everything but the primitive ones. A spelling gate reads these and skips
/// the rest, because only a derived preposition can be confused with the word
/// it came from.
#[must_use]
pub fn is_derived(written: &str) -> bool {
    matches!(
        origin(written),
        Some(Origin::Adverbial | Origin::Nominal | Origin::Verbal)
    )
}

#[cfg(test)]
mod tests {
    use super::{
        super::{COMPOUND, PREPOSITIONS},
        *
    };

    #[test]
    fn every_preposition_has_an_origin() {
        for held in PREPOSITIONS {
            assert!(origin(held).is_some(), "{held} came from nowhere");
        }
        for (held, _) in COMPOUND {
            assert!(origin(held).is_some(), "{held} came from nowhere");
        }
    }

    #[test]
    fn every_stated_origin_holds_a_preposition() {
        for (named, words) in ORIGINS {
            for held in *words {
                assert!(
                    PREPOSITIONS.contains(held) || COMPOUND.iter().any(|(word, _)| word == held),
                    "{held} is under {named:?} and is no preposition"
                );
            }
        }
    }

    #[test]
    fn no_preposition_came_from_two_places() {
        let mut every: Vec<&str> = ORIGINS
            .iter()
            .flat_map(|(_, words)| *words)
            .copied()
            .collect();
        let counted = every.len();
        every.sort_unstable();
        every.dedup();

        assert_eq!(every.len(), counted, "a preposition is listed twice");
    }

    #[test]
    fn a_preposition_names_where_it_came_from() {
        assert_eq!(origin("в"), Some(Origin::Primitive));
        assert_eq!(origin("ВОКРУГ"), Some(Origin::Adverbial));
        assert_eq!(origin("в течение"), Some(Origin::Nominal));
        assert_eq!(origin("благодаря"), Some(Origin::Verbal));
    }

    #[test]
    fn a_word_that_is_no_preposition_came_from_nowhere() {
        assert_eq!(origin("стол"), None);
        assert!(!is_derived("стол"));
        assert!(origin("").is_none());
    }

    #[test]
    fn everything_but_a_primitive_is_derived() {
        assert!(is_derived("вокруг"));
        assert!(is_derived("вследствие"));
        assert!(is_derived("несмотря на"));
        assert!(!is_derived("в"));
        assert!(!is_derived("через"));
    }
}
