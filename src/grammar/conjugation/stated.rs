// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Conjugating a verb by the index a dictionary states for it.
//!
//! The modules beside this one work a verb's class out from its infinitive,
//! which is what has to be done for a word no dictionary holds. Here the
//! class is not worked out: it is read off [`super::index`], the way
//! Zaliznyak's dictionary and the Russian Wiktionary state it, and the
//! finite forms follow — the present and simple future, the past, the
//! imperative. `толкнуть 3b` writes `толкнёшь` where `тянуть 3c` writes
//! `тянешь`, and nothing but the index letter parts them.
//!
//! What the index alone does not state is not invented. The class table
//! itself says class 8 needs the dictionary's note for its velar, class 14
//! for its nasal, `-сти` for its dental, `6°` and the starred indexes for
//! their vanishing vowel; an index marked `^` overrides cells no rule
//! states, and an isolated verb has no class at all. Every such cell comes
//! back [`None`], because the dictionary that stated the index states those
//! forms itself, and writing anything else would put words in its mouth.
//!
//! The index states the paradigm, not the census of its cells: `победить
//! 4b` derives a first person the dictionaries decline to print. Whether a
//! cell is in use is the entry's own fact, stated beside the index, never
//! derived here.

pub mod bidden;
pub mod gone;
pub mod reading;
pub mod stem;

use crate::grammar::{
    Number, Person,
    conjugation::{
        Conjugation,
        class::Class,
        endings,
        index::{VerbIndex, kind::Kind, scheme::Scheme},
        inflect, stems
    },
    declension::spelling,
    form::verb::VerbForm,
    stem::Stem
};

/// Writes one finite form of a verb whose index a dictionary states.
///
/// A reflexive verb is the plain verb inside it with the particle after
/// every ending, so its infinitive is unwrapped, the plain form written, and
/// the particle put back — the same way [`inflect::written`] treats it.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Gender, Number, Person,
///     conjugation::{index, stated::written},
///     form::{Bare, verb::VerbForm}
/// };
///
/// let second = VerbForm::Present {
///     person: Person::Second,
///     number: Number::Singular
/// };
/// let pushed = index::read("3b").expect("a stated index");
/// assert_eq!(
///     written("толкнуть", pushed, second, false).as_deref(),
///     Some("толкнёшь")
/// );
///
/// let pulled = index::read("3c").expect("a stated index");
/// assert_eq!(
///     written("тянуть", pulled, second, false).as_deref(),
///     Some("тянешь")
/// );
///
/// let carried = index::read("7b/b").expect("a stated index");
/// let he = VerbForm::Past(Bare::Singular(Gender::Masculine));
/// assert_eq!(written("везти", carried, he, false).as_deref(), Some("вёз"));
/// ```
#[must_use]
pub fn written(
    infinitive: &str,
    index: VerbIndex,
    form: VerbForm,
    reflexive: bool
) -> Option<String> {
    if matches!(form, VerbForm::Infinitive) {
        return Some(String::from(infinitive));
    }
    if reflexive {
        let plain = super::reflexive::bare(infinitive)?;
        return written(&plain, index, form, false).map(|held| super::reflexive::attached(&held));
    }
    if index.noted || matches!(index.kind, Kind::Isolated) {
        return None;
    }

    match form {
        VerbForm::Present {
            person,
            number
        } => present(infinitive, index, person, number),
        VerbForm::Past(held) => gone::written(infinitive, index, held),
        VerbForm::Imperative(number) => bidden::written(infinitive, index, number),
        _ => None
    }
}

/// One cell of the present or the simple future.
///
/// The scheme letter settles the vowel of the endings: `b` stresses them
/// all, so the first conjugation writes `ё` — `толкнёшь, живёшь, даёшь` —
/// while `a` and `c` leave the four middle cells on the stem and write `е` —
/// `тянешь, пишешь, колешь`. The one scheme outside that statement is the
/// `c′` the sources write only on `хотеть`'s overridden table, and it is
/// refused rather than guessed.
///
/// Classes 4 and 5 are the second conjugation, and their first person alone
/// takes the swap or the `л` that [`stems::first_person`] states — `прошу,
/// просишь`. Class 10 takes first-conjugation endings written soft, `-олю,
/// -олет` in the class table's own spelling — `колю, колешь, колют`.
fn present(infinitive: &str, index: VerbIndex, person: Person, number: Number) -> Option<String> {
    if matches!(index.present, Scheme::CPrime | Scheme::CDouble) {
        return None;
    }

    let base = stem::present(infinitive, index)?;
    let opening = matches!((person, number), (Person::First, Number::Singular));
    let held = if opening && second(index.kind) {
        stems::first_person(&base, Class::Bare)
    } else {
        base
    };

    let conjugation = if second(index.kind) {
        Conjugation::Second
    } else {
        Conjugation::First
    };
    let shape = if matches!(index.kind, Kind::Ten) {
        Stem::Soft
    } else {
        inflect::shape(&held, conjugation)
    };
    let stressed = matches!(index.present, Scheme::B);
    let table = endings::table(conjugation, shape, stressed)?;

    Some(inflect::joined(
        &held,
        &spelling::fitted(&held, table.of(person, number), stressed)
    ))
}

/// Reports whether the class takes the second conjugation.
///
/// The class table splits exactly there: 4 and 5 conjugate `-ишь, -ит`, and
/// every other class `-ешь, -ет`.
const fn second(kind: Kind) -> bool {
    matches!(kind, Kind::Four | Kind::Five)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Gender, conjugation::index, form::Bare};

    fn cell(infinitive: &str, written: &str, person: Person, number: Number) -> Option<String> {
        let held = index::read(written).unwrap_or_else(|| unreachable!("a stated index"));
        super::written(
            infinitive,
            held,
            VerbForm::Present {
                person,
                number
            },
            false
        )
    }

    fn six(infinitive: &str, written: &str, wanted: [&str; 6]) {
        let held = [
            cell(infinitive, written, Person::First, Number::Singular),
            cell(infinitive, written, Person::Second, Number::Singular),
            cell(infinitive, written, Person::Third, Number::Singular),
            cell(infinitive, written, Person::First, Number::Plural),
            cell(infinitive, written, Person::Second, Number::Plural),
            cell(infinitive, written, Person::Third, Number::Plural)
        ];
        let found: Vec<&str> = held
            .iter()
            .map(|one| one.as_deref().unwrap_or(""))
            .collect();
        assert_eq!(found, wanted, "{infinitive} {written}");
    }

    #[test]
    fn the_first_class_glides_and_the_second_trades_its_suffix() {
        six(
            "читать",
            "1a",
            ["читаю", "читаешь", "читает", "читаем", "читаете", "читают"]
        );
        six(
            "рисовать",
            "2a",
            ["рисую", "рисуешь", "рисует", "рисуем", "рисуете", "рисуют"]
        );
        six(
            "ковать",
            "2b",
            ["кую", "куёшь", "куёт", "куём", "куёте", "куют"]
        );
    }

    #[test]
    fn the_scheme_letter_parts_yo_from_e() {
        six(
            "толкнуть",
            "3b",
            [
                "толкну",
                "толкнёшь",
                "толкнёт",
                "толкнём",
                "толкнёте",
                "толкнут"
            ]
        );
        six(
            "тянуть",
            "3c",
            ["тяну", "тянешь", "тянет", "тянем", "тянете", "тянут"]
        );
    }

    #[test]
    fn the_second_conjugation_swaps_its_first_person_alone() {
        six(
            "ставить",
            "4a",
            ["ставлю", "ставишь", "ставит", "ставим", "ставите", "ставят"]
        );
        six(
            "говорить",
            "4b",
            [
                "говорю",
                "говоришь",
                "говорит",
                "говорим",
                "говорите",
                "говорят"
            ]
        );
        six(
            "просить",
            "4c",
            ["прошу", "просишь", "просит", "просим", "просите", "просят"]
        );
        six(
            "любить",
            "4c(4)",
            ["люблю", "любишь", "любит", "любим", "любите", "любят"]
        );
        six(
            "слышать",
            "5a",
            ["слышу", "слышишь", "слышит", "слышим", "слышите", "слышат"]
        );
        six(
            "смотреть",
            "5c",
            [
                "смотрю",
                "смотришь",
                "смотрит",
                "смотрим",
                "смотрите",
                "смотрят"
            ]
        );
        six(
            "спать",
            "5b/c",
            ["сплю", "спишь", "спит", "спим", "спите", "спят"]
        );
        six(
            "держать",
            "5b",
            ["держу", "держишь", "держит", "держим", "держите", "держат"]
        );
    }

    #[test]
    fn the_sixth_class_swaps_throughout() {
        six(
            "плакать",
            "6a",
            ["плачу", "плачешь", "плачет", "плачем", "плачете", "плачут"]
        );
        six(
            "писать",
            "6c",
            ["пишу", "пишешь", "пишет", "пишем", "пишете", "пишут"]
        );
    }

    #[test]
    fn the_consonant_stems_conjugate_where_the_spelling_shows_them() {
        six(
            "лезть",
            "7a",
            ["лезу", "лезешь", "лезет", "лезем", "лезете", "лезут"]
        );
        six(
            "везти",
            "7b/b",
            ["везу", "везёшь", "везёт", "везём", "везёте", "везут"]
        );
        six(
            "тереть",
            "9b",
            ["тру", "трёшь", "трёт", "трём", "трёте", "трут"]
        );
        six(
            "умереть",
            "9b/c(1)",
            ["умру", "умрёшь", "умрёт", "умрём", "умрёте", "умрут"]
        );
    }

    #[test]
    fn the_tenth_class_writes_its_endings_soft() {
        six(
            "колоть",
            "10c",
            ["колю", "колешь", "колет", "колем", "колете", "колют"]
        );
        six(
            "молоть",
            "10c",
            ["мелю", "мелешь", "мелет", "мелем", "мелете", "мелют"]
        );
    }

    #[test]
    fn the_glide_stems_write_their_vowel_into_the_letter() {
        six(
            "бить",
            "11b",
            ["бью", "бьёшь", "бьёт", "бьём", "бьёте", "бьют"]
        );
        six(
            "мыть",
            "12a",
            ["мою", "моешь", "моет", "моем", "моете", "моют"]
        );
        six(
            "петь",
            "12b",
            ["пою", "поёшь", "поёт", "поём", "поёте", "поют"]
        );
        six(
            "гнить",
            "12b/c",
            ["гнию", "гниёшь", "гниёт", "гниём", "гниёте", "гниют"]
        );
        six(
            "давать",
            "13b",
            ["даю", "даёшь", "даёт", "даём", "даёте", "дают"]
        );
    }

    #[test]
    fn the_growing_classes_grow_their_letter() {
        six(
            "стать",
            "15a",
            ["стану", "станешь", "станет", "станем", "станете", "станут"]
        );
        six(
            "жить",
            "16b/c",
            ["живу", "живёшь", "живёт", "живём", "живёте", "живут"]
        );
        six(
            "выжить",
            "16a",
            [
                "выживу",
                "выживешь",
                "выживет",
                "выживем",
                "выживете",
                "выживут"
            ]
        );
    }

    #[test]
    fn what_the_index_does_not_state_is_refused() {
        assert_eq!(cell("печь", "8b/b", Person::First, Number::Singular), None);
        assert_eq!(cell("жать", "14b", Person::First, Number::Singular), None);
        assert_eq!(
            cell("звать", "6°b/c", Person::First, Number::Singular),
            None
        );
        assert_eq!(cell("нести", "7b/b", Person::First, Number::Singular), None);
        assert_eq!(cell("вбить", "11*b", Person::First, Number::Singular), None);
        assert_eq!(cell("дать", "^b/c'", Person::First, Number::Singular), None);
        assert_eq!(
            cell("лгать", "6°b/c^", Person::First, Number::Singular),
            None
        );
        assert_eq!(cell("сыпать", "6a^", Person::First, Number::Singular), None);
        assert_eq!(
            cell("хотеть", "5c'⌧^", Person::First, Number::Singular),
            None
        );
    }

    #[test]
    fn the_x_touches_no_finite_form() {
        assert_eq!(
            cell("длить", "4bX", Person::First, Number::Singular).as_deref(),
            Some("длю")
        );
        assert_eq!(
            cell("длить", "4bX", Person::Second, Number::Singular).as_deref(),
            Some("длишь")
        );
    }

    #[test]
    fn a_reflexive_verb_carries_its_particle_in_every_cell() {
        let held = index::read("6b").unwrap_or_else(|| unreachable!("a stated index"));
        let first = VerbForm::Present {
            person: Person::First,
            number: Number::Singular
        };
        let second = VerbForm::Present {
            person: Person::Second,
            number: Number::Singular
        };
        let they = VerbForm::Present {
            person: Person::Third,
            number: Number::Plural
        };

        assert_eq!(
            written("смеяться", held, first, true).as_deref(),
            Some("смеюсь")
        );
        assert_eq!(
            written("смеяться", held, second, true).as_deref(),
            Some("смеёшься")
        );
        assert_eq!(
            written("смеяться", held, they, true).as_deref(),
            Some("смеются")
        );
        assert_eq!(
            written(
                "смеяться",
                held,
                VerbForm::Imperative(Number::Singular),
                true
            )
            .as_deref(),
            Some("смейся")
        );
        assert_eq!(
            written(
                "смеяться",
                held,
                VerbForm::Past(Bare::Singular(Gender::Feminine)),
                true
            )
            .as_deref(),
            Some("смеялась")
        );
        assert_eq!(
            written("смеяться", held, VerbForm::Infinitive, true).as_deref(),
            Some("смеяться")
        );
    }

    #[test]
    fn a_reflexive_imperative_bends_by_its_numeral() {
        let held = index::read("6a(2)").unwrap_or_else(|| unreachable!("a stated index"));
        assert_eq!(
            written(
                "высыпаться",
                held,
                VerbForm::Imperative(Number::Singular),
                true
            )
            .as_deref(),
            Some("высыпься")
        );

        let taught = index::read("4b").unwrap_or_else(|| unreachable!("a stated index"));
        assert_eq!(
            written(
                "учиться",
                taught,
                VerbForm::Imperative(Number::Plural),
                true
            )
            .as_deref(),
            Some("учитесь")
        );
    }

    #[test]
    fn a_reflexive_past_writes_its_particle_after_the_ending() {
        let held = index::read("14c/c\"").unwrap_or_else(|| unreachable!("a stated index"));
        assert_eq!(
            written(
                "подняться",
                held,
                VerbForm::Past(Bare::Singular(Gender::Masculine)),
                true
            )
            .as_deref(),
            Some("поднялся")
        );
        assert_eq!(
            written(
                "подняться",
                held,
                VerbForm::Past(Bare::Singular(Gender::Feminine)),
                true
            )
            .as_deref(),
            Some("поднялась")
        );
    }

    #[test]
    fn the_participles_are_not_this_modules_to_state() {
        use crate::grammar::{Tense, Voice, form::verb::Participle};

        let held = index::read("1a").unwrap_or_else(|| unreachable!("a stated index"));
        let form = VerbForm::Participle {
            voice: Voice::Active,
            tense: Tense::Present,
            form:  Participle::Short(Bare::Plural)
        };
        assert_eq!(written("читать", held, form, false), None);
        assert_eq!(
            written("читать", held, VerbForm::Adverbial(Tense::Present), false),
            None
        );
    }
}
