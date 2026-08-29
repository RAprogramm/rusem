// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The numerals, which are a closed class however many numbers there are.
//!
//! A hundred and one is written with two words already listed here, and a
//! thousand and one with three. Nothing new joins the class: every number of
//! Russian is built from these and from nothing else.
//!
//! Three kinds behave differently and the difference matters to the checker.
//!
//! A **cardinal** counts and governs: `три стола` puts the noun in the
//! genitive singular, `пять столов` in the genitive plural, and only `один`
//! agrees like an adjective.
//!
//! A **collective** counts people and animals and refuses things: `двое
//! друзей` is right and `двое столов` is not.
//!
//! A **fractional** counts a part: `полтора часа` is one and a half of them.
//! Only three are written in one word — `полтора`, `полторы`, `полтораста`;
//! the rest are a cardinal and an ordinal side by side, `две третьих`, and a
//! tokenizer hands them over as two words.
//!
//! An **ordinal** does not count at all. It declines as an adjective and
//! agrees with its noun like one, which is why it is not in the government
//! table below.

use crate::grammar::{Case, Number};

/// The cardinals a number is built from.
pub const CARDINAL: &[&str] = &[
    "восемнадцать",
    "восемь",
    "восемьдесят",
    "восемьсот",
    "два",
    "двадцать",
    "две",
    "двенадцать",
    "девяносто",
    "девятнадцать",
    "девять",
    "девятьсот",
    "десять",
    "сорок",
    "сто",
    "тринадцать",
    "три",
    "тридцать",
    "триста",
    "тысяча",
    "четыре",
    "четыреста",
    "четырнадцать",
    "шестнадцать",
    "шесть",
    "шестьдесят",
    "шестьсот",
    "миллиард",
    "миллион",
    "ноль",
    "нуль",
    "один",
    "одна",
    "одно",
    "одиннадцать",
    "пятнадцать",
    "пять",
    "пятьдесят",
    "пятьсот",
    "семнадцать",
    "семь",
    "семьдесят",
    "семьсот"
];

/// The collectives, which count only what is animate.
pub const COLLECTIVE: &[&str] = &[
    "восьмеро",
    "двое",
    "девятеро",
    "десятеро",
    "оба",
    "обе",
    "пятеро",
    "семеро",
    "трое",
    "четверо",
    "шестеро"
];

/// The fractionals written in one word.
pub const FRACTIONAL: &[&str] = &["полтора", "полтораста", "полторы"];

/// The ordinals, in dictionary form.
///
/// They decline as adjectives and agree rather than govern, so they carry no
/// entry in the government table.
pub const ORDINAL: &[&str] = &[
    "восемнадцатый",
    "восьмидесятый",
    "восьмисотый",
    "восьмой",
    "второй",
    "двадцатый",
    "двенадцатый",
    "двухсотый",
    "девяностый",
    "девятисотый",
    "девятнадцатый",
    "девятый",
    "десятый",
    "миллиардный",
    "миллионный",
    "нулевой",
    "первый",
    "пятидесятый",
    "пятисотый",
    "пятнадцатый",
    "пятый",
    "седьмой",
    "семидесятый",
    "семисотый",
    "семнадцатый",
    "сороковой",
    "сотый",
    "тридцатый",
    "тринадцатый",
    "третий",
    "трёхсотый",
    "тысячный",
    "четвёртый",
    "четырёхсотый",
    "четырнадцатый",
    "шестидесятый",
    "шестисотый",
    "шестнадцатый",
    "шестой"
];

/// Which kind a numeral is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Kind {
    /// Counts and governs: `три`, `пять`.
    Cardinal,
    /// Counts the animate: `двое`, `трое`.
    Collective,
    /// Counts a part: `полтора`.
    Fractional,
    /// Names a place in an order and agrees: `первый`, `третий`.
    Ordinal
}

/// What a numeral does to the noun it counts, in the nominative.
///
/// `один` agrees, so it takes the case the phrase stands in and answers
/// [`None`]. Everything else governs, and the noun after it stands in the case
/// and number named here whatever the phrase does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Counts {
    /// The case the counted noun stands in.
    pub case:   Case,
    /// How many the counted noun states.
    pub number: Number
}

/// The kind a written numeral is, or nothing when it is no numeral.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::numeral::{Kind, kind};
///
/// assert_eq!(kind("три"), Some(Kind::Cardinal));
/// assert_eq!(kind("трое"), Some(Kind::Collective));
/// assert_eq!(kind("третий"), Some(Kind::Ordinal));
/// assert_eq!(kind("стол"), None);
/// ```
#[must_use]
pub fn kind(written: &str) -> Option<Kind> {
    let held = written.to_lowercase();

    if CARDINAL.contains(&held.as_str()) {
        return Some(Kind::Cardinal);
    }
    if COLLECTIVE.contains(&held.as_str()) {
        return Some(Kind::Collective);
    }
    if FRACTIONAL.contains(&held.as_str()) {
        return Some(Kind::Fractional);
    }
    if ORDINAL.contains(&held.as_str()) {
        return Some(Kind::Ordinal);
    }

    None
}

/// Reports whether a written word is a numeral of any kind.
#[must_use]
pub fn is_numeral(written: &str) -> bool {
    kind(written).is_some()
}

/// What the numeral does to the noun it counts, in the nominative.
///
/// `два`, `три`, `четыре` and `полтора` take the genitive singular — a relic
/// of the dual, which is why `два стола` and not `два столов`. Everything from
/// five up takes the genitive plural. `один` agrees and answers [`None`], and
/// so does anything that is no numeral.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Case, Number, closed::numeral::counts};
///
/// let few = counts("три").expect("three governs");
/// assert_eq!(few.case, Case::Genitive);
/// assert_eq!(few.number, Number::Singular);
///
/// let many = counts("пять").expect("five governs");
/// assert_eq!(many.number, Number::Plural);
///
/// assert_eq!(counts("один"), None, "one agrees rather than governs");
/// ```
#[must_use]
pub fn counts(written: &str) -> Option<Counts> {
    let held = written.to_lowercase();

    if matches!(held.as_str(), "один" | "одна" | "одно") {
        return None;
    }
    if !matches!(
        kind(written)?,
        Kind::Cardinal | Kind::Collective | Kind::Fractional
    ) {
        return None;
    }

    let number = if matches!(
        held.as_str(),
        "два" | "две" | "три" | "четыре" | "полтора" | "полторы" | "оба" | "обе"
    ) {
        Number::Singular
    } else {
        Number::Plural
    };

    Some(Counts {
        case: Case::Genitive,
        number
    })
}

/// Reports whether a numeral counts only what is animate.
///
/// `двое друзей` is right and `двое столов` is not, so a gate on a counted
/// phrase asks this before it asks anything else.
#[must_use]
pub fn counts_the_animate(written: &str) -> bool {
    matches!(kind(written), Some(Kind::Collective))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_class_is_sorted_and_holds_no_word_twice() {
        for class in [CARDINAL, COLLECTIVE, FRACTIONAL, ORDINAL] {
            let mut held = class.to_vec();
            held.sort_unstable();
            held.dedup();

            assert_eq!(held.len(), class.len(), "a numeral is listed twice");
        }
    }

    #[test]
    fn every_numeral_is_of_exactly_one_kind() {
        for class in [CARDINAL, COLLECTIVE, FRACTIONAL, ORDINAL] {
            for held in class {
                assert!(is_numeral(held), "{held} is no numeral");
            }
        }
        let mut every: Vec<&str> = [CARDINAL, COLLECTIVE, FRACTIONAL, ORDINAL].concat();
        let counted = every.len();
        every.sort_unstable();
        every.dedup();

        assert_eq!(every.len(), counted, "a numeral is of two kinds at once");
    }

    #[test]
    fn a_numeral_names_its_kind() {
        assert_eq!(kind("три"), Some(Kind::Cardinal));
        assert_eq!(kind("трое"), Some(Kind::Collective));
        assert_eq!(kind("третий"), Some(Kind::Ordinal));
        assert_eq!(kind("полтора"), Some(Kind::Fractional));
        assert_eq!(kind("Пять"), Some(Kind::Cardinal));
    }

    #[test]
    fn a_word_that_is_no_numeral_names_no_kind() {
        assert_eq!(kind("стол"), None);
        assert!(!is_numeral("читать"));
        assert!(!is_numeral(""));
    }

    #[test]
    fn the_small_numerals_take_the_genitive_singular() {
        for held in ["два", "две", "три", "четыре", "полтора", "оба"] {
            let counted = counts(held).expect("it governs");

            assert_eq!(counted.case, Case::Genitive, "{held}");
            assert_eq!(counted.number, Number::Singular, "{held}");
        }
    }

    #[test]
    fn everything_from_five_up_takes_the_genitive_plural() {
        for held in ["пять", "десять", "сто", "трое", "пятеро"] {
            let counted = counts(held).expect("it governs");

            assert_eq!(counted.number, Number::Plural, "{held}");
        }
    }

    #[test]
    fn one_agrees_rather_than_governs() {
        assert_eq!(counts("один"), None);
        assert_eq!(counts("одна"), None);
        assert_eq!(counts("одно"), None);
    }

    #[test]
    fn an_ordinal_governs_nothing() {
        assert_eq!(counts("первый"), None);
        assert_eq!(counts("третий"), None);
    }

    #[test]
    fn a_word_that_is_no_numeral_governs_nothing() {
        assert_eq!(counts("стол"), None);
        assert_eq!(counts(""), None);
    }

    #[test]
    fn a_collective_counts_only_the_animate() {
        assert!(counts_the_animate("двое"));
        assert!(counts_the_animate("оба"));
        assert!(!counts_the_animate("два"));
        assert!(!counts_the_animate("первый"));
    }
}
