// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The past a stated index builds: the infinitive's stem, bent where the
//! index bends it.
//!
//! The past and the infinitive share a stem, so for most classes the cut is
//! the stem and the four endings follow — `читать, читал, читала`. The index
//! bends that three ways. A `3°` class drops `-ну-` — `сохнуть, сох` — and
//! puts it back in the masculine where ⑤ says so — `гаснул`. Class 9 writes
//! its stem `-ёр` — `тереть, тёр` — with the `е` kept wherever the scheme or
//! ① moves the stress off it: `умерла́` by the scheme's feminine, `у́мер` by
//! the numeral's prefix. And a masculine under past scheme `b` has a zero
//! ending, so the stress stands on the last syllable of the stem and writes
//! its `е` as `ё`: `везти 7b/b` puts `вёз, везла́`.

use crate::grammar::{
    Gender,
    conjugation::{
        index::{VerbIndex, circled::Reach, kind::Kind, scheme::Scheme},
        past, stems
    },
    form::Bare
};

/// One cell of the past, or nothing where the index alone does not state it.
pub(super) fn written(infinitive: &str, index: VerbIndex, cell: Bare) -> Option<String> {
    let stem = of(infinitive, index, cell)?;
    let table = past::table(&stem);
    let ending = table.of(cell.gender().unwrap_or(Gender::Masculine), cell.number());

    Some(stem + ending)
}

/// The past stem of one cell.
///
/// The cut serves every class whose infinitive shows the stem whole — which
/// for the past includes `6°` and class 14, whose hidden stems are a fact of
/// the present alone: `звать, звал`, `жать, жал`. What it does not serve is
/// refused: class 8 hides its velar — `печь` holds `пёк` — and `-сти` hides
/// whether a dental drops — `вести` holds `вёл` against `нести`'s `нёс` —
/// so only the `-зти`, `-зть` shapes of class 7 derive.
fn of(infinitive: &str, index: VerbIndex, cell: Bare) -> Option<String> {
    if index.signs.ringed && !matches!(index.kind, Kind::Three | Kind::Six) {
        return None;
    }
    if matches!(index.kind, Kind::Nine) {
        return ninth(infinitive, index, cell);
    }

    let stem = match index.kind {
        Kind::Eight | Kind::Isolated => None,
        Kind::Seven => stems::cut(infinitive).filter(|held| held.ends_with('з')),
        Kind::Three if index.signs.ringed => return third(infinitive, index, cell),
        _ => stems::cut(infinitive)
    }?;

    Some(yoed(stem, index, cell))
}

/// The `3°` stem: `-ну-` dropped, and kept in the masculine ⑤ names.
///
/// The numeral doubled keeps both forms alive — `гаснуть 3°a((5))(6)` prints
/// `га́снул` beside `га́с` — and the class's own dropped form stays right, so
/// that is the one written, the same way the declension writes its doubled
/// numerals.
fn third(infinitive: &str, index: VerbIndex, cell: Bare) -> Option<String> {
    let full = stems::cut(infinitive)?;
    let bare = full.strip_suffix("ну").map(String::from)?;
    if masculine(cell) && matches!(index.circled.masculine_kept, Some(Reach::Whole)) {
        return Some(full);
    }

    Some(bare)
}

/// The class-9 stem: `-ёр`, with the `е` kept where the stress has left it.
///
/// `тереть 9b` holds the stress on the stem in all four cells and writes
/// `тёр, тёрла`. The scheme moves it onto the ending — the feminine of `c`,
/// everything but the masculine of `b` — and ① moves it onto the prefix
/// everywhere, `умереть 9b/c(1)` writing `у́мер, умерла́, у́мерло`; in either
/// case the syllable is unstressed and its letter is `е`.
fn ninth(infinitive: &str, index: VerbIndex, cell: Bare) -> Option<String> {
    let head = infinitive.strip_suffix("ереть")?;
    let gone =
        matches!(index.circled.past_prefixed, Some(Reach::Whole)) || on_ending(index.past, cell);

    Some(String::from(head) + if gone { "ер" } else { "ёр" })
}

/// The stem with its last `е` written `ё` where the masculine's zero ending
/// leaves the stress on it.
///
/// Under past scheme `b` every ending is stressed; the masculine has none,
/// so its stress stands on the last syllable of the stem, and a stem that
/// ends in a consonant writes that syllable's `е` as `ё`: `вёз` against
/// `везла́`. A stem ending in a vowel keeps its spelling — the `л` follows
/// and the stressed vowel is the final one, written as it stands.
fn yoed(stem: String, index: VerbIndex, cell: Bare) -> String {
    let closed = stem
        .chars()
        .last()
        .is_some_and(|last| !crate::alphabet::is_vowel(last));
    if !closed || !masculine(cell) || !matches!(index.past, Scheme::B) {
        return stem;
    }
    let Some(at) = stem.chars().rev().position(|held| held == 'е') else {
        return stem;
    };

    let place = stem.chars().count() - 1 - at;
    stem.chars()
        .enumerate()
        .map(|(here, held)| if here == place { 'ё' } else { held })
        .collect()
}

/// Reports whether the scheme stresses this cell's ending.
///
/// Scheme `a` stresses none; `b` all but the masculine, whose ending is
/// zero; `c` and its primed variants the feminine alone — the doubled cells
/// of `c′` and `c″` keep the stem's form alive, and that is the one written.
const fn on_ending(past: Scheme, cell: Bare) -> bool {
    match past {
        Scheme::A => false,
        Scheme::B => !masculine(cell),
        Scheme::C | Scheme::CPrime | Scheme::CDouble => {
            matches!(cell, Bare::Singular(Gender::Feminine))
        }
    }
}

/// Reports whether the cell is the masculine singular, common gender read
/// with it the way the past endings read it.
const fn masculine(cell: Bare) -> bool {
    matches!(cell, Bare::Singular(Gender::Masculine | Gender::Common))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::conjugation::index;

    fn cell(infinitive: &str, written: &str, held: Bare) -> Option<String> {
        let stated = index::read(written).unwrap_or_else(|| unreachable!("a stated index"));
        super::written(infinitive, stated, held)
    }

    fn four(infinitive: &str, written: &str) -> [Option<String>; 4] {
        [
            cell(infinitive, written, Bare::Singular(Gender::Masculine)),
            cell(infinitive, written, Bare::Singular(Gender::Feminine)),
            cell(infinitive, written, Bare::Singular(Gender::Neuter)),
            cell(infinitive, written, Bare::Plural)
        ]
    }

    fn spelled(infinitive: &str, written: &str, wanted: [&str; 4]) {
        let held = four(infinitive, written);
        let found: Vec<&str> = held
            .iter()
            .map(|one| one.as_deref().unwrap_or(""))
            .collect();
        assert_eq!(found, wanted, "{infinitive} {written}");
    }

    #[test]
    fn a_vowel_stem_takes_the_four_endings() {
        spelled("делать", "1a", ["делал", "делала", "делало", "делали"]);
        spelled(
            "толкнуть",
            "3b",
            ["толкнул", "толкнула", "толкнуло", "толкнули"]
        );
        spelled("спать", "5b/c", ["спал", "спала", "спало", "спали"]);
        spelled("молоть", "10c", ["молол", "молола", "мололо", "мололи"]);
        spelled("давать", "13b", ["давал", "давала", "давало", "давали"]);
    }

    #[test]
    fn the_hidden_present_does_not_hide_the_past() {
        spelled("звать", "6°b/c", ["звал", "звала", "звало", "звали"]);
        spelled("жать", "14b", ["жал", "жала", "жало", "жали"]);
        spelled(
            "начать",
            "14b/c(1)",
            ["начал", "начала", "начало", "начали"]
        );
        spelled("взять", "14b/c'", ["взял", "взяла", "взяло", "взяли"]);
        spelled("жить", "16b/c", ["жил", "жила", "жило", "жили"]);
        spelled("пить", "11b/c", ["пил", "пила", "пило", "пили"]);
        spelled("вбить", "11*b", ["вбил", "вбила", "вбило", "вбили"]);
    }

    #[test]
    fn a_ringed_third_class_drops_what_the_masculine_may_keep() {
        spelled("ввязнуть", "3°a", ["ввяз", "ввязла", "ввязло", "ввязли"]);
        spelled("гаснуть", "3°a((5))(6)", ["гас", "гасла", "гасло", "гасли"]);
        assert_eq!(
            cell("сохнуть", "3°a(5)", Bare::Singular(Gender::Masculine)).as_deref(),
            Some("сохнул")
        );
        assert_eq!(
            cell("сохнуть", "3°a(5)", Bare::Singular(Gender::Feminine)).as_deref(),
            Some("сохла")
        );
    }

    #[test]
    fn the_masculine_under_b_writes_its_yo_and_drops_its_l() {
        spelled("везти", "7b/b", ["вёз", "везла", "везло", "везли"]);
        spelled("лезть", "7a", ["лез", "лезла", "лезло", "лезли"]);
    }

    #[test]
    fn the_ninth_class_keeps_e_where_the_stress_has_left_it() {
        spelled("тереть", "9b", ["тёр", "тёрла", "тёрло", "тёрли"]);
        spelled("втереть", "9*b", ["втёр", "втёрла", "втёрло", "втёрли"]);
        spelled("умереть", "9b/c(1)", ["умер", "умерла", "умерло", "умерли"]);
    }

    #[test]
    fn what_the_index_does_not_state_is_refused() {
        assert_eq!(cell("печь", "8b/b", Bare::Plural), None);
        assert_eq!(cell("нести", "7b/b", Bare::Plural), None);
        assert_eq!(cell("вести", "7b/b", Bare::Plural), None);
        assert_eq!(cell("есть", "^b", Bare::Plural), None);
    }

    #[test]
    fn the_common_gender_is_read_as_the_masculine() {
        assert_eq!(
            cell("читать", "1a", Bare::Plural).as_deref(),
            Some("читали")
        );
        assert_eq!(
            cell("читать", "1a", Bare::Singular(Gender::Common)).as_deref(),
            Some("читал")
        );
    }
}
