// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Writing out a form of a verb, given its infinitive and the cell wanted.
//!
//! Everything the paradigm knows meets here: the class says which stem, the
//! stem says which endings, and the alphabet bends the ending to the stem it
//! lands on. `писать` and the third person plural come out as `пишут` and not
//! `пишют`, because `ю` does not stand after a sibilant.
//!
//! A verb of the listed class comes back as [`None`] in every cell but the
//! infinitive: its stems are told, not derived. The present cells state a
//! shape rather than an aspect — a perfective verb writes its simple future
//! there, `прочитают` beside `читают`, as
//! [`crate::grammar::form::verb::VerbForm::Present`] says of the cell — and
//! which of the two a word means is the word's aspect, stated on the word
//! rather than read off the infinitive here.
//!
//! A reflexive verb is the plain verb inside it with the particle after every
//! ending, so the plain verb is written out and the particle put back —
//! `учиться` writes `учился`, not the `учил` its inside writes alone.

use crate::grammar::{
    Number, Person,
    conjugation::{Conjugation, class, endings, imperative, past, reflexive, stems},
    form::{Bare, verb::VerbForm},
    stem::{self, Stem}
};

/// Writes out one form of a verb.
///
/// The stress is asked for because the first conjugation writes `е` where the
/// ending is unstressed and `ё` where it is stressed, and no rule finds which.
/// A caller that does not know passes `false` and gets the unstressed
/// spelling, which is the commoner one.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Gender, Number, Person,
///     conjugation::inflect::written,
///     form::{Bare, verb::VerbForm}
/// };
///
/// let third = VerbForm::Present {
///     person: Person::Third,
///     number: Number::Plural
/// };
/// assert_eq!(written("читать", third, false).as_deref(), Some("читают"));
/// assert_eq!(written("писать", third, false).as_deref(), Some("пишут"));
/// assert_eq!(written("любить", third, false).as_deref(), Some("любят"));
///
/// let she = VerbForm::Past(Bare::Singular(Gender::Feminine));
/// assert_eq!(written("читать", she, false).as_deref(), Some("читала"));
/// ```
#[must_use]
pub fn written(infinitive: &str, form: VerbForm, stressed: bool) -> Option<String> {
    if matches!(form, VerbForm::Infinitive) {
        return Some(String::from(infinitive));
    }
    if let Some(plain) = reflexive::bare(infinitive) {
        return written(&plain, form, stressed).map(|held| reflexive::attached(&held));
    }

    match form {
        VerbForm::Present {
            person,
            number
        } => present(infinitive, person, number, stressed),
        VerbForm::Past(held) => gone(infinitive, held),
        VerbForm::Imperative(number) => bidden(infinitive, number, stressed),
        _ => None
    }
}

/// One cell of the imperative.
///
/// Built on the present stem, like the imperative always is, and the plural is
/// the singular with `-те` after it: `читай`, `читайте`. Nothing else parts
/// them, which is why the language writes one ending and adds to it rather
/// than keeping two tables.
///
/// A present stem ending in the glide has the imperative's ending written in
/// it already: `читай-` is the stem and the form at once, and adding to it
/// would spell the glide twice.
fn bidden(infinitive: &str, number: Number, stressed: bool) -> Option<String> {
    let class = class::of(infinitive);
    let stem = stems::present(infinitive, class)?;
    let one = if stem.ends_with(GLIDE) {
        stem
    } else {
        let ending = imperative::of(&stem, stressed).written();
        joined(&stem, &fitted(&stem, ending, stressed))
    };

    Some(match number {
        Number::Singular => one,
        Number::Plural => one + imperative::PLURAL
    })
}

/// One cell of the present.
fn present(infinitive: &str, person: Person, number: Number, stressed: bool) -> Option<String> {
    let class = class::of(infinitive);
    let conjugation = class.conjugation()?;
    let base = stems::present(infinitive, class)?;
    let stem = if matches!((person, number), (Person::First, Number::Singular)) {
        stems::first_person(&base, class)
    } else {
        base
    };

    let table = endings::table(conjugation, shape(&stem, conjugation), stressed)?;

    Some(joined(
        &stem,
        &fitted(&stem, table.of(person, number), stressed)
    ))
}

/// One cell of the past.
///
/// Built on the stem the class vouches for. A verb whose class does not —
/// the consonantal and the listed — comes back as [`None`] rather than
/// spelled on a stem the infinitive never showed: `мочь` refuses instead of
/// writing `мол` for `мог`.
fn gone(infinitive: &str, held: Bare) -> Option<String> {
    let stem = stems::past(infinitive, class::of(infinitive))?;
    let table = past::table(&stem);
    let ending = table.of(
        held.gender().unwrap_or(crate::grammar::Gender::Masculine),
        held.number()
    );

    Some(stem + ending)
}

/// How the stem ends, for the purpose of the ending that follows it.
///
/// A verb stem is asked this differently from a noun stem. A sibilant refuses
/// the soft vowel wherever it stands. A vowel or a glide takes it. And the
/// whole second conjugation takes it — `любят`, `водят`, `строят` — because
/// its endings are soft by their `и`, whatever consonant precedes them; the
/// `л` that grows in `люблю` is not what decides.
fn shape(stem: &str, conjugation: Conjugation) -> Stem {
    let Some(last) = stem.chars().last() else {
        return Stem::Hard;
    };

    if stem::is_sibilant(last) {
        return Stem::Hard;
    }
    if crate::alphabet::is_vowel(last) || last == GLIDE {
        return Stem::Soft;
    }
    if matches!(conjugation, Conjugation::Second) {
        return Stem::Soft;
    }

    Stem::of(last)
}

/// The glide a present stem may end in.
const GLIDE: char = 'й';

/// The letter a glide and a following hard vowel are written as together.
const IOTATED: &[(char, char)] = &[('у', 'ю'), ('а', 'я'), ('э', 'е'), ('о', 'ё')];

/// The letters that already carry the glide in them.
const CARRIES_GLIDE: &[char] = &['ю', 'я', 'е', 'ё', 'и'];

/// The stem and the ending, written as one word.
///
/// A stem ending in the glide does not keep it before a vowel: `читай-` and
/// `-ю` are written `читаю`, because the letter `ю` carries the glide in
/// itself. A hard vowel after the glide is written as the soft letter that
/// stands for both. The glide survives only before a consonant, which is where
/// nothing carries it — `читайте`.
fn joined(stem: &str, ending: &str) -> String {
    let (Some(last), Some(first)) = (stem.chars().last(), ending.chars().next()) else {
        return String::from(stem) + ending;
    };
    if last != GLIDE {
        return String::from(stem) + ending;
    }

    let merged = if CARRIES_GLIDE.contains(&first) {
        first
    } else if let Some((_, held)) = IOTATED.iter().find(|(bare, _)| *bare == first) {
        *held
    } else {
        return String::from(stem) + ending;
    };

    let mut held: String = stem.chars().take(stem.chars().count() - 1).collect();
    held.push(merged);
    held.extend(ending.chars().skip(1));
    held
}

/// The ending as the alphabet writes it after this stem.
///
/// The same bending the nominal endings get, and the same statement of it: `у`
/// and `а` stand where `ю` and `я` would after a sibilant or `ц` — `пишут`,
/// `держат`, `танцуют` — and a second copy of that rule here is how `танцюют`
/// gets written.
fn fitted(stem: &str, ending: &str, stressed: bool) -> String {
    crate::grammar::declension::spelling::fitted(stem, ending, stressed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::Gender;

    #[test]
    fn the_imperative_is_built_on_the_present_stem() {
        let one = VerbForm::Imperative(Number::Singular);
        let many = VerbForm::Imperative(Number::Plural);

        assert_eq!(written("читать", one, false).as_deref(), Some("читай"));
        assert_eq!(written("читать", many, false).as_deref(), Some("читайте"));
        assert_eq!(written("писать", one, true).as_deref(), Some("пиши"));
    }

    fn cell(infinitive: &str, person: Person, number: Number) -> Option<String> {
        written(
            infinitive,
            VerbForm::Present {
                person,
                number
            },
            false
        )
    }

    #[test]
    fn a_glided_verb_is_written_out() {
        assert_eq!(
            cell("читать", Person::First, Number::Singular).as_deref(),
            Some("читаю")
        );
        assert_eq!(
            cell("читать", Person::Third, Number::Plural).as_deref(),
            Some("читают")
        );
    }

    #[test]
    fn a_sibilant_refuses_the_soft_vowel_after_it() {
        assert_eq!(
            cell("писать", Person::Third, Number::Plural).as_deref(),
            Some("пишут")
        );
        assert_eq!(
            cell("держать", Person::Third, Number::Plural).as_deref(),
            Some("держат")
        );
        assert_eq!(
            cell("писать", Person::First, Number::Singular).as_deref(),
            Some("пишу")
        );
    }

    #[test]
    fn the_first_person_of_a_bare_verb_swaps_or_grows() {
        assert_eq!(
            cell("любить", Person::First, Number::Singular).as_deref(),
            Some("люблю")
        );
        assert_eq!(
            cell("водить", Person::First, Number::Singular).as_deref(),
            Some("вожу")
        );
        assert_eq!(
            cell("любить", Person::Second, Number::Singular).as_deref(),
            Some("любишь")
        );
    }

    #[test]
    fn a_stem_in_a_vowel_takes_the_soft_ending() {
        assert_eq!(
            cell("строить", Person::First, Number::Singular).as_deref(),
            Some("строю")
        );
        assert_eq!(
            cell("строить", Person::Third, Number::Plural).as_deref(),
            Some("строят")
        );
    }

    #[test]
    fn the_past_is_written_out() {
        let she = VerbForm::Past(Bare::Singular(Gender::Feminine));
        let they = VerbForm::Past(Bare::Plural);

        assert_eq!(written("читать", she, false).as_deref(), Some("читала"));
        assert_eq!(written("читать", they, false).as_deref(), Some("читали"));
    }

    #[test]
    fn a_listed_verb_is_not_written_out() {
        assert_eq!(cell("тереть", Person::First, Number::Singular), None);
        assert_eq!(cell("мыть", Person::Third, Number::Plural), None);
        assert_eq!(cell("брить", Person::Second, Number::Singular), None);
        assert_eq!(cell("гнать", Person::First, Number::Singular), None);
        assert_eq!(cell("сидеть", Person::First, Number::Singular), None);
        assert_eq!(cell("кричать", Person::First, Number::Singular), None);
    }

    #[test]
    fn a_hidden_past_is_refused_rather_than_invented() {
        let he = VerbForm::Past(Bare::Singular(Gender::Masculine));

        assert_eq!(written("мочь", he, false), None);
        assert_eq!(written("печь", he, false), None);
        assert_eq!(written("идти", he, false), None);
        assert_eq!(written("сесть", he, false), None);
        assert_eq!(written("тереть", he, false), None);
        assert_eq!(written("красть", he, false), None);
    }

    #[test]
    fn the_verb_the_code_states_by_name_is_written_out() {
        assert_eq!(
            cell("спать", Person::First, Number::Singular).as_deref(),
            Some("сплю")
        );
        assert_eq!(
            cell("спать", Person::Second, Number::Singular).as_deref(),
            Some("спишь")
        );
        assert_eq!(
            cell("спать", Person::Third, Number::Plural).as_deref(),
            Some("спят")
        );
    }

    #[test]
    fn a_reflexive_verb_carries_its_particle_in_every_cell() {
        let he = VerbForm::Past(Bare::Singular(Gender::Masculine));
        let she = VerbForm::Past(Bare::Singular(Gender::Feminine));

        assert_eq!(written("учиться", he, false).as_deref(), Some("учился"));
        assert_eq!(written("учиться", she, false).as_deref(), Some("училась"));
        assert_eq!(
            written(
                "учиться",
                VerbForm::Present {
                    person: Person::Third,
                    number: Number::Singular
                },
                false
            )
            .as_deref(),
            Some("учится")
        );
        assert_eq!(
            written("учиться", VerbForm::Infinitive, false).as_deref(),
            Some("учиться")
        );
    }
}
