// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The circled numerals of a verb index, each an instruction about a cell.
//!
//! Zaliznyak prints them as circled digits, and a source that cannot print a
//! circle writes parentheses: `просить` is `4c`, `заменить` is `4c(7)`. A
//! numeral in single parentheses replaces the regular form; the same numeral
//! boxed — written in doubled parentheses — says both forms live, the way
//! `разделить 4b((7))` keeps `разде́ленный` beside `разделённый`. That is the
//! same single-or-either reading the declension index gives its numerals, so
//! the reach is the same fact and is stated once, there.
//!
//! Four of the nine reach the finite forms — the past stress of `(1)`, the
//! imperatives of `(2)` and `(3)`, the masculine past of `(5)` — and the
//! other five touch only participles and gerunds.

use core::{iter::Peekable, str::Chars};

/// How firmly a numeral holds its cell: stated by the declension index and
/// read the same way here, single parentheses replacing the regular form and
/// doubled ones keeping both.
pub use crate::grammar::declension::index::circled::Reach;

/// The circled numerals one verb index carries.
///
/// Each field is one numeral, absent when the index does not write it, and
/// an index may carry several at once — `гаснуть` is `3°a((5))(6)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circled {
    /// ①: the past stress retreats onto the prefix everywhere but the
    /// feminine — `умереть 9b/c(1)`: `у́мер, умерла́, у́мерло`.
    pub past_prefixed:     Option<Reach>,
    /// ②: the imperative takes `-ь(те)` or `-й(те)` where the class default
    /// gives `-и(те)` — `плюнуть 3a(2)`: `плю́нь`.
    pub imperative_soft:   Option<Reach>,
    /// ③: the imperative splits, `-и` in the singular and `-ьте` in the
    /// plural — `вылезти 7a(3)`: `вы́лези, вы́лезьте`.
    pub imperative_split:  Option<Reach>,
    /// ④: the present active participle stresses one syllable earlier than
    /// the scheme says — `любить 4c(4)`: `лю́бящий` against `нося́щий`.
    pub acting_retracted:  Option<Reach>,
    /// ⑤: the masculine past of a `3°` verb keeps `-ну-` — `гаснуть
    /// 3°a((5))(6)`: `га́снул` beside `га́с`.
    pub masculine_kept:    Option<Reach>,
    /// ⑥: the past active participle and gerund of a `3°` verb keep `-ну-` —
    /// `достигнуть 3°a((6))`: `дости́гнувший` beside `дости́гший`.
    pub bygone_kept:       Option<Reach>,
    /// ⑦: the past passive participle keeps the ending stress, `-ённый`,
    /// `-а́нный` — `заменить 4c(7)`: `заменённый`.
    pub passive_stressed:  Option<Reach>,
    /// ⑧: the past passive participle retreats to the third syllable from
    /// the end, unstressed `-енный` — `вкроить 4b(8)`: `вкро́енный`.
    pub passive_retracted: Option<Reach>,
    /// ⑨: the perfective past gerund is written `-я́`, `-а́` on the future
    /// stem — `внести 7b/b(9)`: `внеся́` beside `внёсши`.
    pub gerund_future:     Option<Reach>
}

impl Circled {
    /// A record with no numerals, which is what most indexes carry.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            past_prefixed:     None,
            imperative_soft:   None,
            imperative_split:  None,
            acting_retracted:  None,
            masculine_kept:    None,
            bygone_kept:       None,
            passive_stressed:  None,
            passive_retracted: None,
            gerund_future:     None
        }
    }
}

/// Reads one parenthesized numeral, the opening parenthesis already taken,
/// and reports whether what stood there was one.
pub(super) fn numeral(letters: &mut Peekable<Chars<'_>>, circled: &mut Circled) -> bool {
    let doubled = letters.next_if_eq(&'(').is_some();
    let Some(digit) = letters.next() else {
        return false;
    };
    if letters.next_if_eq(&')').is_none() {
        return false;
    }
    if doubled && letters.next_if_eq(&')').is_none() {
        return false;
    }

    let reach = if doubled { Reach::Either } else { Reach::Whole };
    let held = match digit {
        '1' => &mut circled.past_prefixed,
        '2' => &mut circled.imperative_soft,
        '3' => &mut circled.imperative_split,
        '4' => &mut circled.acting_retracted,
        '5' => &mut circled.masculine_kept,
        '6' => &mut circled.bygone_kept,
        '7' => &mut circled.passive_stressed,
        '8' => &mut circled.passive_retracted,
        '9' => &mut circled.gerund_future,
        _ => return false
    };
    *held = Some(reach);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(written: &str) -> Option<Circled> {
        let mut letters = written.chars().peekable();
        letters.next();
        let mut held = Circled::none();
        numeral(&mut letters, &mut held).then_some(held)
    }

    #[test]
    fn every_numeral_is_read_into_its_own_cell() {
        assert_eq!(
            one("(1)").and_then(|held| held.past_prefixed),
            Some(Reach::Whole)
        );
        assert_eq!(
            one("(5)").and_then(|held| held.masculine_kept),
            Some(Reach::Whole)
        );
        assert_eq!(
            one("(9)").and_then(|held| held.gerund_future),
            Some(Reach::Whole)
        );
    }

    #[test]
    fn a_doubled_numeral_reaches_either_form() {
        assert_eq!(
            one("((7))").and_then(|held| held.passive_stressed),
            Some(Reach::Either)
        );
        assert_eq!(
            one("((3))").and_then(|held| held.imperative_split),
            Some(Reach::Either)
        );
    }

    #[test]
    fn a_malformed_numeral_is_refused() {
        assert_eq!(one("(0)"), None);
        assert_eq!(one("(1"), None);
        assert_eq!(one("((5)"), None);
    }
}
