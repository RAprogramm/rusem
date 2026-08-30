// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The class-6 labial in the imperative: the grown `л` before a vowel, the
//! bare labial before the soft sign.
//!
//! A labial of class 6 grows an `л` through its present — `сыплю, сыплешь` —
//! and the imperative keeps it wherever the ending is a vowel: `дремать`
//! commands `дремли`, `трепать` commands `трепли`, `колебать` commands
//! `колебли`. Where the cell is closed by the soft sign instead, the `л`
//! cannot stand, and the stem is the bare labial: `высыпаться 6a(2)`
//! commands `высыпься`. One root closes that way without any numeral saying
//! so, and it is stated here by name.

use crate::grammar::conjugation::{
    index::{VerbIndex, kind::Kind, scheme::Scheme},
    listed
};

/// The roots whose imperative Zaliznyak's dictionary states with the bare
/// labial and the soft sign.
///
/// `сыпать` is the one: his entry prints `повел. сыпь`, and the prefixed
/// family follows it — `насыпь, рассыпься, просыпьтесь`. Its stem-stressed
/// neighbour in the class keeps the `л` and writes the vowel — `колебать`
/// commands `колебли` — so the shedding is this root's own stated fact, not
/// the class's rule (Зализняк, «Грамматический словарь русского языка»,
/// статья «сыпать»).
const SOFT_ROOTS: &[&str] = &["сыпать"];

/// Reports whether the verb's imperative is stated with the soft sign in
/// place of the vowel the scheme would write.
///
/// True only for a stem-stressed class-6 verb on a root of [`SOFT_ROOTS`],
/// bare or under prefixes: the end-stressed schemes put the stress on the
/// ending itself, and a stressed ending is always the vowel — `трепать 6c`
/// commands `трепли`, whatever its labial.
pub(super) fn soft(infinitive: &str, index: VerbIndex) -> bool {
    matches!(index.kind, Kind::Six)
        && matches!(index.present, Scheme::A)
        && listed(infinitive, SOFT_ROOTS)
}

/// The stem without the `л` a class-6 labial grows, which stands only
/// before a vowel.
///
/// `сыпать` conjugates `сыплю, сыплешь` and keeps the `л` before the
/// imperative's `-и` — `дремать` commands `дремли` — but a soft sign closes
/// the bare labial: `высыпаться 6a(2)` commands `высыпься`, not `высыплься`,
/// and Zaliznyak's own table for `сыпать` prints `сыпь`.
pub(super) fn shed(held: String, index: VerbIndex) -> String {
    let mut letters = held.chars().rev();
    let grown = matches!(index.kind, Kind::Six)
        && letters.next() == Some(crate::morphemics::alternation::EPENTHESIS)
        && letters
            .next()
            .is_some_and(crate::morphemics::alternation::takes_epenthesis);
    if !grown {
        return held;
    }

    held.chars().take(held.chars().count() - 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::conjugation::index;

    fn read(written: &str) -> VerbIndex {
        index::read(written).unwrap_or_else(|| unreachable!("a stated index"))
    }

    #[test]
    fn the_stated_root_is_soft_bare_and_under_prefixes() {
        assert!(soft("сыпать", read("6a")));
        assert!(soft("насыпать", read("6a")));
        assert!(soft("рассыпать", read("6a")));
    }

    #[test]
    fn its_neighbours_and_its_end_stressed_kin_are_not() {
        assert!(!soft("колебать", read("6a")));
        assert!(!soft("зыбать", read("6a")));
        assert!(!soft("трепать", read("6c")));
        assert!(!soft("сыпать", read("6b")));
        assert!(!soft("сыпать", read("4a")));
        assert!(!soft("шептать", read("6c")));
    }

    #[test]
    fn only_the_grown_labial_sheds_its_letter() {
        assert_eq!(shed(String::from("сыпл"), read("6a")), "сып");
        assert_eq!(shed(String::from("колебл"), read("6a")), "колеб");
        assert_eq!(shed(String::from("стел"), read("6a")), "стел");
        assert_eq!(shed(String::from("сыпл"), read("4a")), "сыпл");
        assert_eq!(shed(String::from("пиш"), read("6a")), "пиш");
    }
}
