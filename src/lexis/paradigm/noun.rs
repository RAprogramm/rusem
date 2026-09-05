// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The paradigm of a noun, built from what the noun is.
//!
//! Everything the table needs is already stated: the gender and the animacy
//! belong to the word, the pattern is read off the index where a dictionary
//! states one and worked out from the gender and the dictionary form where
//! none does, the endings follow from the pattern, and the alphabet bends
//! them. Nothing is looked up word by word, and no noun is named here.
//!
//! The animacy is why the accusative comes out right. A noun that names
//! something alive repeats its genitive there and one that does not repeats
//! its nominative, and the word states which it is — so the cell is filled by
//! the word rather than guessed at from the sentence it turns up in.

use crate::{
    grammar::{
        Case, Number,
        declension::{Declension, adjective, noun as declined, stated},
        form::{Agreed, Form}
    },
    lexis::{
        lexeme::{Noun, Road},
        paradigm::Paradigm
    },
    morphology::WordForm
};

/// Builds every cell of a noun's paradigm.
///
/// An indeclinable noun gets its twelve cells too, each spelled the way the
/// dictionary spells it: `пальто` has all its forms and one spelling, which is
/// what indeclinable means.
///
/// # Examples
///
/// ```
/// use rusem::{
///     grammar::{
///         Animacy, Case, Gender,
///         declension::Declension,
///         form::{Agreed, Form}
///     },
///     lexis::{Noun, paradigm::noun},
///     morphology::WordForm
/// };
///
/// let table = noun::of(
///     &WordForm::parse("стол")?,
///     Noun {
///         gender:     Gender::Masculine,
///         animacy:    Animacy::Inanimate,
///         declension: Declension::Second,
///         index:      None
///     }
/// );
/// let cell = Form::Noun(Agreed::Singular {
///     case:   Case::Genitive,
///     gender: Gender::Masculine
/// });
///
/// assert_eq!(
///     table.fills(cell).first().map(WordForm::as_str),
///     Some("стола")
/// );
/// # Ok::<(), rusem::error::CoreError>(())
/// ```
#[must_use]
pub fn of(lemma: &WordForm, held: Noun) -> Paradigm {
    let mut cells = Vec::new();

    for number in [Number::Singular, Number::Plural] {
        for case in Case::STATED {
            if let Some(written) = spelling(lemma, held, case, number) {
                cells.push((cell(case, number, held), super::spelled(&written)));
            }
        }
    }

    Paradigm {
        lemma: lemma.clone(),
        cells
    }
}

/// The cell of the table a case and a number name.
const fn cell(case: Case, number: Number, held: Noun) -> Form {
    Form::Noun(match number {
        Number::Singular => Agreed::Singular {
            case,
            gender: held.gender
        },
        Number::Plural => Agreed::Plural {
            case
        }
    })
}

/// What is written in one cell.
///
/// Which road a declining noun goes by — the stated index or the worked-out
/// pattern — is not decided here: [`Noun::road`] states it once, for this
/// writer and for the reader alike, because the two paths must not be
/// blended. The derivation cannot hear the stress and would quietly
/// contradict the dictionary where they differ — `конём` against `конем`,
/// `платка` against `платока`.
///
/// Two things neither road can read off the spelling are taken from the word.
/// Indeclinability is one: `окно` declines and `пальто` does not, and nothing
/// but the word knows which. The adjectival declension is the other:
/// `мороженое` was an adjective and still takes adjectival endings, and
/// `окно` beside it takes nominal ones though both end in `-о`.
fn spelling(lemma: &WordForm, held: Noun, case: Case, number: Number) -> Option<String> {
    if matches!(held.declension, Declension::Indeclinable) {
        return Some(String::from(lemma.as_str()));
    }
    if matches!(held.declension, Declension::Adjectival) {
        return adjective::written(lemma.as_str(), case, number, held.gender, held.animacy);
    }

    match held.road() {
        Road::Stated(index) => stated::written(
            lemma.as_str(),
            held.gender,
            held.animacy,
            index,
            case,
            number
        ),
        Road::Derived => declined::written(lemma.as_str(), held.gender, held.animacy, case, number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Animacy, Gender, declension::index};

    fn form(written: &str) -> WordForm {
        WordForm::parse(written).unwrap_or_else(|_| unreachable!("a written Russian word"))
    }

    fn word(gender: Gender, animacy: Animacy, declension: Declension) -> Noun {
        Noun {
            gender,
            animacy,
            declension,
            index: None
        }
    }

    fn listed(gender: Gender, animacy: Animacy, written: &str) -> Noun {
        Noun {
            gender,
            animacy,
            declension: Declension::Second,
            index: Some(index::read(written).unwrap_or_else(|| unreachable!("a stated index")))
        }
    }

    fn cell_of(table: &Paradigm, case: Case, number: Number, gender: Gender) -> Option<String> {
        let held = Form::Noun(match number {
            Number::Singular => Agreed::Singular {
                case,
                gender
            },
            Number::Plural => Agreed::Plural {
                case
            }
        });

        table
            .fills(held)
            .first()
            .map(|written| written.as_str().to_owned())
    }

    #[test]
    fn a_noun_states_a_cell_for_every_case_of_both_numbers() {
        let table = of(
            &form("стол"),
            word(Gender::Masculine, Animacy::Inanimate, Declension::Second)
        );

        assert_eq!(table.len(), 12);
    }

    #[test]
    fn an_inanimate_noun_repeats_its_nominative_in_the_accusative() {
        let table = of(
            &form("стол"),
            word(Gender::Masculine, Animacy::Inanimate, Declension::Second)
        );

        let nominative = cell_of(
            &table,
            Case::Nominative,
            Number::Singular,
            Gender::Masculine
        );
        let accusative = cell_of(
            &table,
            Case::Accusative,
            Number::Singular,
            Gender::Masculine
        );

        assert_eq!(nominative.as_deref(), Some("стол"));
        assert_eq!(accusative, nominative);
    }

    #[test]
    fn an_animate_noun_repeats_its_genitive_in_the_accusative() {
        let table = of(
            &form("студент"),
            word(Gender::Masculine, Animacy::Animate, Declension::Second)
        );

        let genitive = cell_of(&table, Case::Genitive, Number::Plural, Gender::Masculine);
        let accusative = cell_of(&table, Case::Accusative, Number::Plural, Gender::Masculine);

        assert_eq!(genitive.as_deref(), Some("студентов"));
        assert_eq!(accusative, genitive);
    }

    #[test]
    fn the_first_declension_declines_on_its_own_stem() {
        let table = of(
            &form("книга"),
            word(Gender::Feminine, Animacy::Inanimate, Declension::First)
        );

        assert_eq!(
            cell_of(&table, Case::Genitive, Number::Singular, Gender::Feminine).as_deref(),
            Some("книги")
        );
        assert_eq!(
            cell_of(&table, Case::Dative, Number::Singular, Gender::Feminine).as_deref(),
            Some("книге")
        );
    }

    #[test]
    fn a_mixed_noun_grows_its_stem_outside_the_nominative() {
        let table = of(
            &form("время"),
            word(Gender::Neuter, Animacy::Inanimate, Declension::Mixed)
        );

        assert_eq!(
            cell_of(&table, Case::Nominative, Number::Singular, Gender::Neuter).as_deref(),
            Some("время")
        );
        assert_eq!(
            cell_of(&table, Case::Genitive, Number::Singular, Gender::Neuter).as_deref(),
            Some("времени")
        );
    }

    #[test]
    fn an_indeclinable_noun_has_all_its_cells_and_one_spelling() {
        let table = of(
            &form("пальто"),
            word(Gender::Neuter, Animacy::Inanimate, Declension::Indeclinable)
        );

        assert_eq!(table.len(), 12);
        assert_eq!(table.spellings().len(), 1);
    }

    #[test]
    fn a_noun_that_was_an_adjective_keeps_the_adjectival_endings() {
        let table = of(
            &form("мороженое"),
            word(Gender::Neuter, Animacy::Inanimate, Declension::Adjectival)
        );

        assert_eq!(
            cell_of(&table, Case::Genitive, Number::Singular, Gender::Neuter).as_deref(),
            Some("мороженого")
        );
        assert_eq!(
            cell_of(&table, Case::Instrumental, Number::Plural, Gender::Neuter).as_deref(),
            Some("морожеными")
        );
    }

    #[test]
    fn an_adjectival_noun_of_the_feminine_declines_as_a_feminine_adjective() {
        let table = of(
            &form("столовая"),
            word(Gender::Feminine, Animacy::Inanimate, Declension::Adjectival)
        );

        assert_eq!(
            cell_of(&table, Case::Genitive, Number::Singular, Gender::Feminine).as_deref(),
            Some("столовой")
        );
        assert_eq!(
            cell_of(&table, Case::Accusative, Number::Singular, Gender::Feminine).as_deref(),
            Some("столовую")
        );
    }

    #[test]
    fn a_written_form_reads_back_into_the_cells_it_fills() {
        let table = of(
            &form("стол"),
            word(Gender::Masculine, Animacy::Inanimate, Declension::Second)
        );

        let cells = table.cells_of(&form("столы"));

        assert!(cells.contains(&Form::Noun(Agreed::Plural {
            case: Case::Nominative
        })));
        assert!(cells.contains(&Form::Noun(Agreed::Plural {
            case: Case::Accusative
        })));
    }

    #[test]
    fn a_stated_index_keeps_the_stress_on_the_ending() {
        let table = of(
            &form("конь"),
            listed(Gender::Masculine, Animacy::Animate, "2b")
        );

        assert_eq!(
            cell_of(
                &table,
                Case::Instrumental,
                Number::Singular,
                Gender::Masculine
            )
            .as_deref(),
            Some("конём")
        );
    }

    #[test]
    fn a_stated_index_writes_the_nominative_the_dictionary_spells() {
        let table = of(
            &form("лицо"),
            listed(Gender::Neuter, Animacy::Inanimate, "5d")
        );

        assert_eq!(
            cell_of(&table, Case::Nominative, Number::Singular, Gender::Neuter).as_deref(),
            Some("лицо")
        );
    }

    #[test]
    fn a_stated_index_drops_the_fleeting_vowel() {
        let table = of(
            &form("платок"),
            listed(Gender::Masculine, Animacy::Inanimate, "3*b")
        );

        assert_eq!(
            cell_of(&table, Case::Genitive, Number::Singular, Gender::Masculine).as_deref(),
            Some("платка")
        );
    }
}
