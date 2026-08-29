// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The ending a noun takes, with everything the bare table does not know.
//!
//! The table beside this one is keyed by pattern, stem and gender, and that is
//! not quite enough. Three things bend it, and all three are facts about the
//! written word rather than about the pattern:
//!
//! a stem that ends in the glide takes `и` where the table says `е` — `о
//! гении`, `в армии`, `о собрании`, § 33; the eleven mixed nouns grow `-ен-`
//! before every ending but the nominative; and a sibilant refuses the vowels
//! the table would otherwise put after it.

use crate::grammar::{
    Animacy, Case, Gender, Number,
    declension::{self, Declension, endings, spelling}
};

/// The ending a noun takes in the cell asked for.
///
/// An indeclinable noun takes none, and the answer is [`None`] rather than an
/// empty string: `пальто` has no ending, which is not the same as having one
/// that is written with nothing.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Animacy, Case, Gender, Number, declension::noun::ending};
///
/// let held = ending(
///     "армия",
///     Gender::Feminine,
///     Animacy::Inanimate,
///     Case::Prepositional,
///     Number::Singular
/// );
/// assert_eq!(held, Some("и"));
///
/// let plain = ending(
///     "вода",
///     Gender::Feminine,
///     Animacy::Inanimate,
///     Case::Prepositional,
///     Number::Singular
/// );
/// assert_eq!(plain, Some("е"));
/// ```
#[must_use]
pub fn ending(
    nominative: &str,
    gender: Gender,
    animacy: Animacy,
    case: Case,
    number: Number
) -> Option<&'static str> {
    let pattern = declension::of(nominative, gender);
    let shape = declension::stem(nominative);
    let table = endings::table(pattern, shape, gender, number)?;
    let held = table.of(case, animacy);

    if number == Number::Singular && declension::on_glide(nominative) && softened(case, pattern) {
        return Some("и");
    }

    Some(held)
}

/// Reports whether a cell is one the glide softens.
///
/// The prepositional always: `о гении`, `о собрании`. The dative and the
/// genitive of the first declension too, because there the table would put `е`
/// and `и` respectively and the word says `и` for both: `к армии`, `от армии`.
const fn softened(case: Case, pattern: Declension) -> bool {
    match case {
        Case::Prepositional | Case::Locative => true,
        Case::Dative | Case::Genitive => matches!(pattern, Declension::First),
        _ => false
    }
}

/// The stem a noun's endings are added to.
///
/// A mixed noun in `-мя` grows before every ending but the nominative and the
/// accusative, and the growth belongs to the stem rather than to the ending:
/// `время` gives `времен-` and then `-и`, `-ем`.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Case, Gender, Number, declension::noun::stem};
///
/// assert_eq!(
///     stem("время", Gender::Neuter, Case::Genitive, Number::Singular).as_deref(),
///     Some("времен")
/// );
/// assert_eq!(
///     stem("время", Gender::Neuter, Case::Genitive, Number::Plural).as_deref(),
///     Some("врем")
/// );
/// assert_eq!(
///     stem("время", Gender::Neuter, Case::Nominative, Number::Singular).as_deref(),
///     Some("врем")
/// );
/// ```
#[must_use]
pub fn stem(nominative: &str, gender: Gender, case: Case, number: Number) -> Option<String> {
    let pattern = declension::of(nominative, gender);
    let bare = bare_stem(nominative, pattern)?;

    if !matches!(pattern, Declension::Mixed) || !grows(case, number) {
        return Some(bare);
    }

    Some(bare + declension::GROWTH)
}

/// Reports whether a mixed noun grows its stem in this cell.
///
/// Everywhere but the nominative and the accusative, which keep the bare stem,
/// and the genitive plural, whose ending carries the growth in itself:
/// `времён` is `врем-` and `-ён`, and growing the stem as well would write it
/// twice.
const fn grows(case: Case, number: Number) -> bool {
    if matches!(case.merged(), Case::Nominative | Case::Accusative) {
        return false;
    }

    !matches!((case.merged(), number), (Case::Genitive, Number::Plural))
}

/// The dictionary form without the ending it carries.
fn bare_stem(nominative: &str, pattern: Declension) -> Option<String> {
    if matches!(pattern, Declension::Indeclinable) {
        return None;
    }

    let last = nominative.chars().last()?;
    if matches!(last, 'а' | 'я' | 'о' | 'е' | 'ё' | 'ь' | 'й') {
        return Some(
            nominative
                .chars()
                .take(nominative.chars().count() - 1)
                .collect()
        );
    }

    Some(String::from(nominative))
}

/// The noun written out in the cell asked for.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Animacy, Case, Gender, Number, declension::noun::written};
///
/// assert_eq!(
///     written(
///         "время",
///         Gender::Neuter,
///         Animacy::Inanimate,
///         Case::Genitive,
///         Number::Singular
///     )
///     .as_deref(),
///     Some("времени")
/// );
/// assert_eq!(
///     written(
///         "армия",
///         Gender::Feminine,
///         Animacy::Inanimate,
///         Case::Dative,
///         Number::Singular
///     )
///     .as_deref(),
///     Some("армии")
/// );
/// assert_eq!(
///     written(
///         "книга",
///         Gender::Feminine,
///         Animacy::Inanimate,
///         Case::Genitive,
///         Number::Singular
///     )
///     .as_deref(),
///     Some("книги")
/// );
/// ```
#[must_use]
pub fn written(
    nominative: &str,
    gender: Gender,
    animacy: Animacy,
    case: Case,
    number: Number
) -> Option<String> {
    let base = stem(nominative, gender, case, number)?;
    let held = ending(nominative, gender, animacy, case, number)?;

    Some(base.clone() + &spelling::fitted(&base, held, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(nominative: &str, gender: Gender, case: Case) -> Option<String> {
        written(
            nominative,
            gender,
            Animacy::Inanimate,
            case,
            Number::Singular
        )
    }

    #[test]
    fn a_stem_on_the_glide_takes_i_where_the_table_says_e() {
        assert_eq!(
            one("армия", Gender::Feminine, Case::Prepositional).as_deref(),
            Some("армии")
        );
        assert_eq!(
            one("армия", Gender::Feminine, Case::Dative).as_deref(),
            Some("армии")
        );
        assert_eq!(
            one("собрание", Gender::Neuter, Case::Prepositional).as_deref(),
            Some("собрании")
        );
    }

    #[test]
    fn an_ordinary_stem_keeps_the_e() {
        assert_eq!(
            one("вода", Gender::Feminine, Case::Prepositional).as_deref(),
            Some("воде")
        );
        assert_eq!(
            one("стол", Gender::Masculine, Case::Prepositional).as_deref(),
            Some("столе")
        );
    }

    #[test]
    fn a_mixed_noun_grows_before_its_endings() {
        assert_eq!(
            one("время", Gender::Neuter, Case::Genitive).as_deref(),
            Some("времени")
        );
        assert_eq!(
            one("время", Gender::Neuter, Case::Instrumental).as_deref(),
            Some("временем")
        );
        assert_eq!(
            one("имя", Gender::Neuter, Case::Dative).as_deref(),
            Some("имени")
        );
    }

    #[test]
    fn a_mixed_noun_does_not_grow_in_the_nominative() {
        assert_eq!(
            one("время", Gender::Neuter, Case::Nominative).as_deref(),
            Some("время")
        );
    }

    #[test]
    fn a_sibilant_refuses_the_vowel_the_table_would_put() {
        assert_eq!(
            one("книга", Gender::Feminine, Case::Genitive).as_deref(),
            Some("книги")
        );
        assert_eq!(
            one("душа", Gender::Feminine, Case::Genitive).as_deref(),
            Some("души")
        );
    }

    #[test]
    fn an_indeclinable_noun_takes_no_ending() {
        assert_eq!(one("пальто", Gender::Feminine, Case::Genitive), None);
    }
}
