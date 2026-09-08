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
//! гении`, `в армии`, `о собрании`, § 33; the ten mixed nouns in `-мя` grow
//! `-ен-` where their paradigm asks for it; and a sibilant refuses the vowels
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
    if held == "ён" && GROWN_WITH_YA.contains(&nominative) {
        return Some("ян");
    }

    Some(held)
}

/// The two mixed nouns whose genitive plural growth is written with `я`.
///
/// Zaliznyak's dictionary marks `семя` and `стремя` apart from the other
/// eight nouns in `-мя`: `семян` and `стремян` against `времён`, `имён`,
/// `знамён`. The departure is this one ending, so it is replaced wherever the
/// paradigm reads it — the genitive plural and the cells that repeat it —
/// and the dictionary enumerates exactly these two words.
const GROWN_WITH_YA: &[&str] = &["семя", "стремя"];

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
/// A mixed noun in `-мя` grows where its paradigm asks for it, and the growth
/// belongs to the stem rather than to the ending: `время` gives `времен-` and
/// then `-и`, `-ем`. The animacy is asked for because the plural accusative
/// has no row of its own, and which row it repeats decides whether the stem
/// is grown. `путь` is mixed and grows nothing.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Animacy, Case, Gender, Number, declension::noun::stem};
///
/// let one = stem(
///     "время",
///     Gender::Neuter,
///     Animacy::Inanimate,
///     Case::Genitive,
///     Number::Singular
/// );
/// assert_eq!(one.as_deref(), Some("времен"));
///
/// let bare = stem(
///     "время",
///     Gender::Neuter,
///     Animacy::Inanimate,
///     Case::Genitive,
///     Number::Plural
/// );
/// assert_eq!(bare.as_deref(), Some("врем"));
///
/// let grown = stem(
///     "время",
///     Gender::Neuter,
///     Animacy::Inanimate,
///     Case::Nominative,
///     Number::Plural
/// );
/// assert_eq!(grown.as_deref(), Some("времен"));
/// ```
#[must_use]
pub fn stem(
    nominative: &str,
    gender: Gender,
    animacy: Animacy,
    case: Case,
    number: Number
) -> Option<String> {
    let pattern = declension::of(nominative, gender);
    let bare = bare_stem(nominative, pattern)?;

    if !matches!(pattern, Declension::Mixed)
        || !nominative.ends_with("мя")
        || !grows(case, number, animacy)
    {
        return Some(bare);
    }

    Some(bare + declension::Declension::GROWTH)
}

/// Reports whether a mixed noun in `-мя` grows its stem in this cell.
///
/// The singular keeps the bare stem in the nominative and the accusative,
/// whose endings the paradigm states on it — `время` — and the vocative reads
/// the nominative's row. The plural grows throughout — `времена`, `временам`
/// — except the genitive, whose ending carries the growth in itself: `времён`
/// is `врем-` and `-ён`, and growing the stem as well would write it twice.
/// The plural accusative states no row of its own, so it grows as the row it
/// repeats does: the nominative's for a thing, the genitive's for a living
/// being.
const fn grows(case: Case, number: Number, animacy: Animacy) -> bool {
    match number {
        Number::Singular => !matches!(
            case.merged(),
            Case::Nominative | Case::Vocative | Case::Accusative
        ),
        Number::Plural => match case.merged() {
            Case::Genitive => false,
            Case::Accusative => matches!(animacy, Animacy::Inanimate),
            _ => true
        }
    }
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
    let base = stem(nominative, gender, animacy, case, number)?;
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

    fn many(nominative: &str, gender: Gender, case: Case) -> Option<String> {
        written(nominative, gender, Animacy::Inanimate, case, Number::Plural)
    }

    #[test]
    fn a_mixed_noun_grows_through_the_whole_plural() {
        assert_eq!(
            many("время", Gender::Neuter, Case::Nominative).as_deref(),
            Some("времена")
        );
        assert_eq!(
            many("время", Gender::Neuter, Case::Accusative).as_deref(),
            Some("времена")
        );
        assert_eq!(
            many("имя", Gender::Neuter, Case::Nominative).as_deref(),
            Some("имена")
        );
    }

    #[test]
    fn the_plural_obliques_of_a_mixed_noun_are_hard() {
        assert_eq!(
            many("время", Gender::Neuter, Case::Dative).as_deref(),
            Some("временам")
        );
        assert_eq!(
            many("время", Gender::Neuter, Case::Instrumental).as_deref(),
            Some("временами")
        );
        assert_eq!(
            many("время", Gender::Neuter, Case::Prepositional).as_deref(),
            Some("временах")
        );
    }

    #[test]
    fn the_two_sown_nouns_write_their_genitive_plural_with_ya() {
        assert_eq!(
            many("семя", Gender::Neuter, Case::Genitive).as_deref(),
            Some("семян")
        );
        assert_eq!(
            many("стремя", Gender::Neuter, Case::Genitive).as_deref(),
            Some("стремян")
        );
        assert_eq!(
            many("время", Gender::Neuter, Case::Genitive).as_deref(),
            Some("времён")
        );
    }

    #[test]
    fn the_way_declines_by_the_third_declension_with_its_own_instrumental() {
        assert_eq!(
            one("путь", Gender::Masculine, Case::Nominative).as_deref(),
            Some("путь")
        );
        assert_eq!(
            one("путь", Gender::Masculine, Case::Genitive).as_deref(),
            Some("пути")
        );
        assert_eq!(
            one("путь", Gender::Masculine, Case::Instrumental).as_deref(),
            Some("путём")
        );
        assert_eq!(
            many("путь", Gender::Masculine, Case::Nominative).as_deref(),
            Some("пути")
        );
        assert_eq!(
            many("путь", Gender::Masculine, Case::Genitive).as_deref(),
            Some("путей")
        );
        assert_eq!(
            many("путь", Gender::Masculine, Case::Dative).as_deref(),
            Some("путям")
        );
    }
}
