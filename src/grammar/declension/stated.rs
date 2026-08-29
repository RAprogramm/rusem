// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Declining a noun by the index a dictionary states for it.
//!
//! The module beside this one works the pattern out from the gender and the
//! dictionary form, which is what has to be done for a word no dictionary
//! holds. Here the pattern is not worked out: it is read off the index, which
//! says what the stem ends in and where the stress falls, and the endings
//! follow from those two and the gender.
//!
//! That is the difference between `конем` and `конём`, between `лице` and
//! `лицо`, between `мужев` and `мужей`. None of them can be settled without
//! the stress or without knowing that the stem ends in a sibilant rather than
//! merely in a soft consonant, and the index says both.

pub mod ending;
pub mod fleeting;
pub mod reading;
pub mod stem;
pub mod yo;

use crate::grammar::{
    Animacy, Case, Gender, Number,
    declension::{
        index::{
            Index,
            falls::{self, Falls}
        },
        spelling
    }
};

/// The stem with its `ё` written `е` where the stress has left it.
///
/// `ё` is a stressed letter and nothing else in Russian: `жёлудь` keeps it
/// while the stress is on the stem and writes `желудей` when the scheme moves
/// the stress onto the ending. A cell with no ending at all keeps it, because
/// there is nowhere else for the stress to have gone: `кочерёг`, `сестёр`.
fn unstressed(base: &str, stressed: bool) -> String {
    if !stressed {
        return String::from(base);
    }

    base.replace('ё', "е")
}

/// Reports whether a noun declines by the first paradigm.
///
/// By its dictionary form and not by its gender: `мужчина` and `слуга` are
/// masculine and decline like `книга`, and `дядя` does too. Every noun written
/// with `-а` or `-я` belongs here, whoever it names.
#[must_use]
pub fn opens(lemma: &str) -> bool {
    matches!(lemma.chars().last(), Some('а' | 'я'))
}

/// Writes one cell of a noun that a dictionary states the index of.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Animacy, Case, Gender, Number,
///     declension::{index, stated::written}
/// };
///
/// let held = index::read("2b").expect("a stated index");
/// let one = written(
///     "конь",
///     Gender::Masculine,
///     Animacy::Animate,
///     held,
///     Case::Instrumental,
///     Number::Singular
/// );
/// assert_eq!(one.as_deref(), Some("конём"));
///
/// let tse = index::read("5d").expect("a stated index");
/// let two = written(
///     "лицо",
///     Gender::Neuter,
///     Animacy::Inanimate,
///     tse,
///     Case::Nominative,
///     Number::Singular
/// );
/// assert_eq!(two.as_deref(), Some("лицо"));
/// ```
#[must_use]
pub fn written(
    lemma: &str,
    gender: Gender,
    animacy: Animacy,
    index: Index,
    case: Case,
    number: Number
) -> Option<String> {
    let opens = opens(lemma);
    let base = stem::of(lemma, gender, index)?;
    let stressed = matches!(
        falls::on(index.accent, case, number, animacy),
        Falls::Ending
    );
    let parted = index.fleeting;
    let word = fleeting::Word {
        gender,
        kind: index.kind,
        animacy,
        opens
    };
    let held = ending::of(
        gender,
        index,
        case,
        number,
        animacy,
        ending::Shape {
            stressed,
            opens,
            parted
        }
    );
    let parting = parted && fleeting::parts(word, case, number);
    let base = if index.yo && yo::stands(stressed, held.is_empty(), parting) {
        yo::written(&base)
    } else {
        base
    };
    let base = if parted {
        fleeting::of(&base, word, case, number, stressed)
    } else {
        base
    };
    let base = unstressed(&base, stressed && !held.is_empty());

    Some(base.clone() + &spelling::fitted(&base, held, stressed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::declension::index;

    fn stated(written: &str) -> Index {
        index::read(written).unwrap_or_else(|| unreachable!("a stated index"))
    }

    fn cell(
        lemma: &str,
        gender: Gender,
        animacy: Animacy,
        held: &str,
        case: Case,
        number: Number
    ) -> String {
        written(lemma, gender, animacy, stated(held), case, number)
            .unwrap_or_else(|| unreachable!("a written cell"))
    }

    #[test]
    fn a_marked_stem_writes_yo_where_the_scheme_holds_the_stem() {
        for (case, number, form) in [
            (Case::Genitive, Number::Singular, "жены"),
            (Case::Accusative, Number::Singular, "жену"),
            (Case::Nominative, Number::Plural, "жёны"),
            (Case::Genitive, Number::Plural, "жён"),
            (Case::Dative, Number::Plural, "жёнам")
        ] {
            assert_eq!(
                cell(
                    "жена",
                    Gender::Feminine,
                    Animacy::Animate,
                    "1d, ё",
                    case,
                    number
                ),
                form
            );
        }
    }

    #[test]
    fn the_marked_plural_of_a_thing_carries_the_letter_too() {
        assert_eq!(
            cell(
                "звезда",
                Gender::Feminine,
                Animacy::Inanimate,
                "1d, ё",
                Case::Nominative,
                Number::Plural
            ),
            "звёзды"
        );
        assert_eq!(
            cell(
                "звезда",
                Gender::Feminine,
                Animacy::Inanimate,
                "1d, ё",
                Case::Genitive,
                Number::Plural
            ),
            "звёзд"
        );
    }

    #[test]
    fn a_marked_neuter_alternates_the_same_way() {
        for (case, number, form) in [
            (Case::Prepositional, Number::Singular, "колесе"),
            (Case::Nominative, Number::Plural, "колёса"),
            (Case::Genitive, Number::Plural, "колёс")
        ] {
            assert_eq!(
                cell(
                    "колесо",
                    Gender::Neuter,
                    Animacy::Inanimate,
                    "1d, ё",
                    case,
                    number
                ),
                form
            );
        }
    }

    #[test]
    fn a_lemma_written_with_yo_loses_it_off_the_stress() {
        for (case, number, form) in [
            (Case::Genitive, Number::Singular, "мёда"),
            (Case::Nominative, Number::Plural, "меды"),
            (Case::Genitive, Number::Plural, "медов")
        ] {
            assert_eq!(
                cell(
                    "мёд",
                    Gender::Masculine,
                    Animacy::Inanimate,
                    "1c, ё",
                    case,
                    number
                ),
                form
            );
        }

        for (case, number, form) in [
            (Case::Nominative, Number::Singular, "ёж"),
            (Case::Genitive, Number::Singular, "ежа"),
            (Case::Nominative, Number::Plural, "ежи"),
            (Case::Genitive, Number::Plural, "ежей")
        ] {
            assert_eq!(
                cell(
                    "ёж",
                    Gender::Masculine,
                    Animacy::Animate,
                    "4b, ё",
                    case,
                    number
                ),
                form
            );
        }
    }

    #[test]
    fn the_marked_letter_and_the_fleeting_vowel_share_a_paradigm() {
        for (case, number, form) in [
            (Case::Genitive, Number::Singular, "весны"),
            (Case::Nominative, Number::Plural, "вёсны"),
            (Case::Genitive, Number::Plural, "вёсен"),
            (Case::Dative, Number::Plural, "вёснам")
        ] {
            assert_eq!(
                cell(
                    "весна",
                    Gender::Feminine,
                    Animacy::Inanimate,
                    "1*d, ё",
                    case,
                    number
                ),
                form
            );
        }

        assert_eq!(
            cell(
                "десна",
                Gender::Feminine,
                Animacy::Inanimate,
                "1*d, ё",
                Case::Genitive,
                Number::Plural
            ),
            "дёсен"
        );
        assert_eq!(
            cell(
                "сестра",
                Gender::Feminine,
                Animacy::Animate,
                "1*d, ё",
                Case::Nominative,
                Number::Plural
            ),
            "сёстры"
        );
    }

    #[test]
    fn a_bare_cell_takes_the_letter_even_off_the_scheme() {
        assert_eq!(
            cell(
                "желна",
                Gender::Feminine,
                Animacy::Animate,
                "1b−, ё",
                Case::Genitive,
                Number::Plural
            ),
            "жёлн"
        );
    }

    #[test]
    fn the_first_numeral_reads_the_nominative_plural_off_the_other_row() {
        assert_eq!(
            cell(
                "дом",
                Gender::Masculine,
                Animacy::Inanimate,
                "1c(1)",
                Case::Nominative,
                Number::Plural
            ),
            "дома"
        );
        assert_eq!(
            cell(
                "дом",
                Gender::Masculine,
                Animacy::Inanimate,
                "1c(1)",
                Case::Genitive,
                Number::Plural
            ),
            "домов"
        );
        assert_eq!(
            cell(
                "снег",
                Gender::Masculine,
                Animacy::Inanimate,
                "3c(1)",
                Case::Nominative,
                Number::Plural
            ),
            "снега"
        );
        assert_eq!(
            cell(
                "яблоко",
                Gender::Neuter,
                Animacy::Inanimate,
                "3a(1)",
                Case::Nominative,
                Number::Plural
            ),
            "яблоки"
        );
        assert_eq!(
            cell(
                "яблоко",
                Gender::Neuter,
                Animacy::Inanimate,
                "3a(1)",
                Case::Genitive,
                Number::Plural
            ),
            "яблок"
        );
    }

    #[test]
    fn the_second_numeral_reads_the_genitive_plural_off_the_other_row() {
        assert_eq!(
            cell(
                "сапог",
                Gender::Masculine,
                Animacy::Inanimate,
                "3b(2)",
                Case::Genitive,
                Number::Plural
            ),
            "сапог"
        );
        assert_eq!(
            cell(
                "сапог",
                Gender::Masculine,
                Animacy::Inanimate,
                "3b(2)",
                Case::Nominative,
                Number::Plural
            ),
            "сапоги"
        );
    }

    #[test]
    fn two_numerals_take_their_two_cells_and_the_repeated_accusative() {
        for (case, form) in [
            (Case::Nominative, "глаза"),
            (Case::Genitive, "глаз"),
            (Case::Accusative, "глаза")
        ] {
            assert_eq!(
                cell(
                    "глаз",
                    Gender::Masculine,
                    Animacy::Inanimate,
                    "1c(1)(2)",
                    case,
                    Number::Plural
                ),
                form
            );
        }
    }

    #[test]
    fn the_third_numeral_trades_the_prepositional_vowel() {
        assert_eq!(
            cell(
                "Бабий",
                Gender::Masculine,
                Animacy::Animate,
                "7a(3)",
                Case::Prepositional,
                Number::Singular
            ),
            "Бабие"
        );
        assert_eq!(
            cell(
                "полоний",
                Gender::Masculine,
                Animacy::Inanimate,
                "7a",
                Case::Prepositional,
                Number::Singular
            ),
            "полонии"
        );
    }

    #[test]
    fn a_doubled_numeral_keeps_the_pattern_form() {
        assert_eq!(
            cell(
                "баклажан",
                Gender::Masculine,
                Animacy::Inanimate,
                "1a((2))",
                Case::Genitive,
                Number::Plural
            ),
            "баклажанов"
        );
        assert_eq!(
            cell(
                "жвало",
                Gender::Neuter,
                Animacy::Inanimate,
                "1a((1))",
                Case::Nominative,
                Number::Plural
            ),
            "жвала"
        );
    }
}
