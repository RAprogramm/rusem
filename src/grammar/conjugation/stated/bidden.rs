// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The imperative a stated index builds.
//!
//! Built on the present stem, with two classes standing aside: class 11
//! writes the full grade `-ей` its present hides — `бить, бью, бей` — and
//! class 13 builds on the infinitive stem the present has cut — `давать,
//! даю, давай`. Which ending closes a consonant stem is the scheme's to say:
//! `b` and `c` stress the imperative's ending and write `-и` — `пиши`,
//! `учи`, `толкни` — while `a` keeps the stem's stress and writes `-й`, `-ь`
//! or `-и` by what closes the stem, the same three-way [`imperative`]
//! states. The numerals ② and ③ then bend the cell the dictionary's way:
//! `плюнуть 3a(2)` writes `плюнь`, `вылезти 7a(3)` writes `вылези` in the
//! singular and `вылезьте` in the plural.

mod labial;

use super::stem;
use crate::grammar::{
    Number,
    conjugation::{
        imperative::{self, Ending},
        index::{VerbIndex, circled::Reach, kind::Kind, scheme::Scheme},
        stems
    }
};

/// One cell of the imperative, or nothing where the index alone does not
/// state it.
pub(super) fn written(infinitive: &str, index: VerbIndex, number: Number) -> Option<String> {
    if matches!(index.present, Scheme::CPrime | Scheme::CDouble) {
        return None;
    }
    let (one, many) = forms(infinitive, index)?;

    Some(match number {
        Number::Singular => one,
        Number::Plural => many
    })
}

/// Both cells at once, because ③ parts them: the singular's `-и` does not
/// carry into a plural written `-ьте`.
fn forms(infinitive: &str, index: VerbIndex) -> Option<(String, String)> {
    if matches!(index.kind, Kind::Eleven) {
        return eleventh(infinitive, index);
    }

    let held = base(infinitive, index)?;
    if held.ends_with('й') {
        return Some(pair(held));
    }
    if held.chars().last().is_some_and(crate::alphabet::is_vowel) {
        let mut grown = held;
        grown.push('й');
        return Some(pair(grown));
    }

    Some(closed(infinitive, held, index))
}

/// The stem the imperative is built on.
///
/// The present stem, except in class 13, where the class table builds the
/// cell on the infinitive stem the present has cut: `давать` commands
/// `давай`, not the `дай` its present stem would write — that form belongs
/// to `дать`.
fn base(infinitive: &str, index: VerbIndex) -> Option<String> {
    if matches!(index.kind, Kind::Thirteen) {
        if index.signs.ringed || index.signs.fleeting {
            return None;
        }
        return stems::cut(infinitive);
    }

    stem::present(infinitive, index)
}

/// The class-11 imperative, which is not built on the present stem at all.
///
/// The class table writes the full grade `-ей` where the present has `-ь-`:
/// `бей`, `пей`, `лей`. The starred indexes of the class keep it too —
/// `вбить 11*b` conjugates `вобью` but commands `вбей` — so the fleeting
/// vowel that refuses the present does not refuse this cell.
fn eleventh(infinitive: &str, index: VerbIndex) -> Option<(String, String)> {
    if index.signs.ringed {
        return None;
    }

    infinitive
        .strip_suffix("ить")
        .map(|head| pair(String::from(head) + "ей"))
}

/// The ending a consonant stem takes, the scheme's stress and the numerals
/// bending it — or the soft sign [`labial`] states for its root.
fn closed(infinitive: &str, held: String, index: VerbIndex) -> (String, String) {
    if matches!(index.circled.imperative_split, Some(Reach::Whole)) {
        let one = held.clone() + Ending::Vowel.written();
        let many = held + Ending::SoftSign.written() + imperative::PLURAL;
        return (one, many);
    }

    let stressed = matches!(index.present, Scheme::B | Scheme::C);
    let mut ending = imperative::of(&held, stressed);
    if matches!(ending, Ending::Vowel)
        && (matches!(index.circled.imperative_soft, Some(Reach::Whole))
            || labial::soft(infinitive, index))
    {
        ending = Ending::SoftSign;
    }
    let stem = if matches!(ending, Ending::SoftSign) {
        labial::shed(held, index)
    } else {
        held
    };

    pair(stem + ending.written())
}

/// The plural beside the singular, which is the singular with `-те` after
/// it.
fn pair(one: String) -> (String, String) {
    let many = one.clone() + imperative::PLURAL;
    (one, many)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::conjugation::index;

    fn one(infinitive: &str, written: &str) -> Option<String> {
        bid(infinitive, written, Number::Singular)
    }

    fn bid(infinitive: &str, written: &str, number: Number) -> Option<String> {
        let held = index::read(written).unwrap_or_else(|| unreachable!("a stated index"));
        super::written(infinitive, held, number)
    }

    #[test]
    fn a_vowel_stem_takes_the_glide_whatever_the_scheme() {
        assert_eq!(one("читать", "1a").as_deref(), Some("читай"));
        assert_eq!(
            bid("читать", "1a", Number::Plural).as_deref(),
            Some("читайте")
        );
        assert_eq!(one("рисовать", "2a").as_deref(), Some("рисуй"));
        assert_eq!(one("ковать", "2b").as_deref(), Some("куй"));
        assert_eq!(one("мыть", "12a").as_deref(), Some("мой"));
        assert_eq!(one("петь", "12b").as_deref(), Some("пой"));
        assert_eq!(one("брить", "12a").as_deref(), Some("брей"));
    }

    #[test]
    fn a_stressed_ending_writes_the_vowel() {
        assert_eq!(one("толкнуть", "3b").as_deref(), Some("толкни"));
        assert_eq!(one("тянуть", "3c").as_deref(), Some("тяни"));
        assert_eq!(one("говорить", "4b").as_deref(), Some("говори"));
        assert_eq!(one("просить", "4c").as_deref(), Some("проси"));
        assert_eq!(one("гореть", "5b").as_deref(), Some("гори"));
        assert_eq!(one("смотреть", "5c").as_deref(), Some("смотри"));
        assert_eq!(one("писать", "6c").as_deref(), Some("пиши"));
        assert_eq!(one("везти", "7b/b").as_deref(), Some("вези"));
        assert_eq!(one("жить", "16b/c").as_deref(), Some("живи"));
        assert_eq!(one("колоть", "10c").as_deref(), Some("коли"));
        assert_eq!(one("молоть", "10c").as_deref(), Some("мели"));
    }

    #[test]
    fn a_stem_stressed_consonant_is_closed_by_its_own_shape() {
        assert_eq!(one("ставить", "4a").as_deref(), Some("ставь"));
        assert_eq!(one("плакать", "6a").as_deref(), Some("плачь"));
        assert_eq!(one("лезть", "7a").as_deref(), Some("лезь"));
        assert_eq!(one("стать", "15a").as_deref(), Some("стань"));
        assert_eq!(one("прыгнуть", "3a").as_deref(), Some("прыгни"));
    }

    #[test]
    fn the_ending_stressed_schemes_write_the_vowel_over_a_cluster() {
        assert_eq!(one("тереть", "9b").as_deref(), Some("три"));
        assert_eq!(one("умереть", "9b/c(1)").as_deref(), Some("умри"));
        assert_eq!(one("спать", "5b/c").as_deref(), Some("спи"));
    }

    #[test]
    fn the_stated_labial_root_sheds_its_letter_and_the_rest_keep_it() {
        assert_eq!(one("насыпать", "6a").as_deref(), Some("насыпь"));
        assert_eq!(
            bid("насыпать", "6a", Number::Plural).as_deref(),
            Some("насыпьте")
        );
        assert_eq!(one("колебать", "6a").as_deref(), Some("колебли"));
        assert_eq!(one("дремать", "6c").as_deref(), Some("дремли"));
        assert_eq!(one("трепать", "6c").as_deref(), Some("трепли"));
    }

    #[test]
    fn the_second_numeral_softens_the_vowel_the_default_writes() {
        assert_eq!(one("плюнуть", "3a(2)").as_deref(), Some("плюнь"));
        assert_eq!(one("вынуть", "3a(2)").as_deref(), Some("вынь"));
        assert_eq!(one("выстроить", "4a(2)").as_deref(), Some("выстрой"));
        assert_eq!(one("высыпать", "6a(2)").as_deref(), Some("высыпь"));
        assert_eq!(one("выдвинуть", "3a((3))").as_deref(), Some("выдвинь"));
    }

    #[test]
    fn the_third_numeral_parts_the_singular_from_the_plural() {
        assert_eq!(one("вылезти", "7a(3)").as_deref(), Some("вылези"));
        assert_eq!(
            bid("вылезти", "7a(3)", Number::Plural).as_deref(),
            Some("вылезьте")
        );
        assert_eq!(one("сахарить", "4a(3)").as_deref(), Some("сахари"));
        assert_eq!(
            bid("сахарить", "4a(3)", Number::Plural).as_deref(),
            Some("сахарьте")
        );
    }

    #[test]
    fn the_eleventh_and_thirteenth_classes_build_their_own_cell() {
        assert_eq!(one("бить", "11b").as_deref(), Some("бей"));
        assert_eq!(one("пить", "11b/c").as_deref(), Some("пей"));
        assert_eq!(one("вбить", "11*b").as_deref(), Some("вбей"));
        assert_eq!(one("давать", "13b").as_deref(), Some("давай"));
        assert_eq!(
            bid("давать", "13b", Number::Plural).as_deref(),
            Some("давайте")
        );
    }

    #[test]
    fn what_the_index_does_not_state_is_refused() {
        assert_eq!(one("печь", "8b/b"), None);
        assert_eq!(one("жать", "14b"), None);
        assert_eq!(one("звать", "6°b/c"), None);
        assert_eq!(one("втереть", "9*b"), None);
    }
}
