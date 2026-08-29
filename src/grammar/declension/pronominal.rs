// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The declension of the pronouns, which is neither the noun's nor the
//! adjective's.
//!
//! Before this module the engine measured `тот` against the adjectival table
//! and got `тым` for the instrumental. It reads `тем` now, and `чем`, and
//! `всём`, and every other cell an adjective cannot write.
//!
//! # What a pronoun is, to a paradigm
//!
//! A stem and two facts. `тот` is `т` declining the older way on a hard stem;
//! `весь` is `вс` declining the older way on a soft one; `мой` is `мо`,
//! newer and soft. [`endings`] holds what those two facts decide, and this
//! module holds the pronouns they are true of.
//!
//! `один` is here because it declines this way and not as a numeral: `одного`,
//! `одному`, `одним` are the endings of `этот`. Its feminine and neuter
//! dictionary forms are listed beside it, since a caller meeting `одна` has no
//! reason to know it should ask about `один`.
//!
//! The nominative singular masculine is the one cell no paradigm writes.
//! `тот`, `весь`, `мой`, `чей`, `кто` each say it their own way — `весь` even
//! grows a vowel the other cells do not have. So it is kept beside the stem,
//! as the dictionary form it is.

pub mod endings;

use self::endings::Pattern;
use crate::grammar::{
    Animacy, Case, Gender, Number,
    declension::{agreed::Cell, spelling},
    stem::Stem
};

/// A pronoun that declines, as the paradigm needs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Declining {
    /// The dictionary form: `тот`, `весь`, `мой`.
    pub dictionary: &'static str,
    /// What every other cell is built on: `т`, `вс`, `мо`.
    pub stem:       &'static str,
    /// Which of the two patterns it declines by.
    pub pattern:    Pattern,
    /// Whether the stem is hard or soft.
    pub shape:      Stem,
    /// Whether the ending carries the stress.
    ///
    /// § 4 writes `е` for an unstressed `о` or `ё` after a sibilant, so `наш`
    /// says `нашем` and `что` says `чём`. Nothing but the stress parts them.
    pub stressed:   bool
}

/// The pronouns that decline this way.
///
/// Every one of them is a dictionary form, a stem and two facts, and from
/// those the paradigm writes the rest. Eleven entries stand for some two
/// hundred and sixty written forms.
///
/// `кто` and `что` state a gender they do not have: they are masculine to
/// every verb that agrees with them — `кто пришёл` — and have no plural at
/// all, which [`cells`] answers for by never finding one.
const DECLINING: &[Declining] = &[
    Declining {
        dictionary: "тот",
        stem:       "т",
        pattern:    Pattern::Older,
        shape:      Stem::Hard,
        stressed:   false
    },
    Declining {
        dictionary: "этот",
        stem:       "эт",
        pattern:    Pattern::Newer,
        shape:      Stem::Hard,
        stressed:   false
    },
    Declining {
        dictionary: "весь",
        stem:       "вс",
        pattern:    Pattern::Older,
        shape:      Stem::Soft,
        stressed:   true
    },
    Declining {
        dictionary: "сей",
        stem:       "с",
        pattern:    Pattern::Newer,
        shape:      Stem::Soft,
        stressed:   true
    },
    Declining {
        dictionary: "мой",
        stem:       "мо",
        pattern:    Pattern::Newer,
        shape:      Stem::Soft,
        stressed:   true
    },
    Declining {
        dictionary: "твой",
        stem:       "тво",
        pattern:    Pattern::Newer,
        shape:      Stem::Soft,
        stressed:   true
    },
    Declining {
        dictionary: "свой",
        stem:       "сво",
        pattern:    Pattern::Newer,
        shape:      Stem::Soft,
        stressed:   true
    },
    Declining {
        dictionary: "чей",
        stem:       "чь",
        pattern:    Pattern::Newer,
        shape:      Stem::Soft,
        stressed:   true
    },
    Declining {
        dictionary: "наш",
        stem:       "наш",
        pattern:    Pattern::Newer,
        shape:      Stem::Soft,
        stressed:   false
    },
    Declining {
        dictionary: "ваш",
        stem:       "ваш",
        pattern:    Pattern::Newer,
        shape:      Stem::Soft,
        stressed:   false
    },
    Declining {
        dictionary: "один",
        stem:       "одн",
        pattern:    Pattern::Newer,
        shape:      Stem::Hard,
        stressed:   true
    },
    Declining {
        dictionary: "одна",
        stem:       "одн",
        pattern:    Pattern::Newer,
        shape:      Stem::Hard,
        stressed:   true
    },
    Declining {
        dictionary: "одно",
        stem:       "одн",
        pattern:    Pattern::Newer,
        shape:      Stem::Hard,
        stressed:   true
    },
    Declining {
        dictionary: "сам",
        stem:       "сам",
        pattern:    Pattern::Newer,
        shape:      Stem::Hard,
        stressed:   false
    },
    Declining {
        dictionary: "кто",
        stem:       "к",
        pattern:    Pattern::Older,
        shape:      Stem::Hard,
        stressed:   true
    },
    Declining {
        dictionary: "что",
        stem:       "ч",
        pattern:    Pattern::Older,
        shape:      Stem::Soft,
        stressed:   true
    }
];

/// The pronoun a dictionary form names, when it declines this way.
///
/// # Examples
///
/// ```
/// use rusem::grammar::declension::pronominal::declining;
///
/// assert_eq!(declining("тот").expect("a pronoun").stem, "т");
/// assert_eq!(declining("весь").expect("a pronoun").stem, "вс");
/// assert!(declining("стол").is_none());
/// ```
#[must_use]
pub fn declining(dictionary: &str) -> Option<Declining> {
    let held = dictionary.to_lowercase();

    DECLINING
        .iter()
        .find(|pronoun| pronoun.dictionary == held)
        .copied()
}

/// The form a cell of a pronoun's paradigm is written with.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Animacy, Case, Gender, Number, declension::pronominal::written};
///
/// assert_eq!(
///     written(
///         "тот",
///         Gender::Masculine,
///         Number::Singular,
///         Case::Instrumental,
///         Animacy::Inanimate
///     ),
///     Some(String::from("тем"))
/// );
/// assert_eq!(
///     written(
///         "весь",
///         Gender::Neuter,
///         Number::Singular,
///         Case::Prepositional,
///         Animacy::Inanimate
///     ),
///     Some(String::from("всём"))
/// );
/// ```
#[must_use]
pub fn written(
    dictionary: &str,
    gender: Gender,
    number: Number,
    case: Case,
    animacy: Animacy
) -> Option<String> {
    let held = declining(dictionary)?;
    if case == Case::Nominative && number == Number::Singular && gender == Gender::Masculine {
        return Some(String::from(held.dictionary));
    }

    let ending = endings::of(held.pattern, held.shape, gender, number, case, animacy);

    Some(String::from(held.stem) + &spelling::fitted(held.stem, ending, held.stressed))
}

/// Reads a written pronoun back into every cell that could have written it.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Animacy, Case, declension::pronominal::cells};
///
/// let held = cells("тем", "тот", Animacy::Inanimate);
/// assert!(held.iter().any(|cell| cell.case == Case::Instrumental));
///
/// let asked = cells("чем", "что", Animacy::Inanimate);
/// assert!(asked.iter().any(|cell| cell.case == Case::Instrumental));
/// ```
#[must_use]
pub fn cells(word: &str, dictionary: &str, animacy: Animacy) -> Vec<Cell> {
    let held = word.to_lowercase();
    let mut found = Vec::new();

    for number in [Number::Singular, Number::Plural] {
        for gender in Gender::STATED {
            if number == Number::Plural && gender != Gender::Masculine {
                continue;
            }
            for case in Case::STATED {
                if written(dictionary, gender, number, case, animacy).as_deref() != Some(&held) {
                    continue;
                }
                found.push(Cell {
                    case,
                    number,
                    gender: (number == Number::Singular).then_some(gender)
                });
            }
        }
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pronoun_that_declines_this_way_names_its_stem() {
        for held in DECLINING {
            assert_eq!(
                declining(held.dictionary).map(|found| found.stem),
                Some(held.stem)
            );
        }
    }

    #[test]
    fn a_word_that_declines_another_way_is_not_here() {
        assert!(declining("стол").is_none());
        assert!(declining("красный").is_none());
        assert!(declining("").is_none());
    }

    #[test]
    fn the_instrumental_an_adjective_could_not_write_is_written() {
        for (dictionary, form) in [
            ("тот", "тем"),
            ("этот", "этим"),
            ("весь", "всем"),
            ("мой", "моим"),
            ("чей", "чьим"),
            ("кто", "кем"),
            ("что", "чем"),
            ("сам", "самим")
        ] {
            assert_eq!(
                written(
                    dictionary,
                    Gender::Masculine,
                    Number::Singular,
                    Case::Instrumental,
                    Animacy::Inanimate
                )
                .as_deref(),
                Some(form),
                "{dictionary}"
            );
        }
    }

    #[test]
    fn a_sibilant_stem_writes_e_where_the_stress_does_not_fall() {
        assert_eq!(
            written(
                "наш",
                Gender::Masculine,
                Number::Singular,
                Case::Prepositional,
                Animacy::Inanimate
            )
            .as_deref(),
            Some("нашем")
        );
        assert_eq!(
            written(
                "ваш",
                Gender::Neuter,
                Number::Singular,
                Case::Nominative,
                Animacy::Inanimate
            )
            .as_deref(),
            Some("ваше")
        );
    }

    #[test]
    fn the_prepositional_of_a_soft_stem_keeps_its_soft_vowel() {
        assert_eq!(
            written(
                "весь",
                Gender::Masculine,
                Number::Singular,
                Case::Prepositional,
                Animacy::Inanimate
            )
            .as_deref(),
            Some("всём")
        );
        assert_eq!(
            written(
                "что",
                Gender::Masculine,
                Number::Singular,
                Case::Prepositional,
                Animacy::Inanimate
            )
            .as_deref(),
            Some("чём")
        );
    }

    #[test]
    fn one_declines_as_a_pronoun_and_not_as_a_numeral() {
        assert_eq!(
            written(
                "один",
                Gender::Masculine,
                Number::Singular,
                Case::Genitive,
                Animacy::Inanimate
            )
            .as_deref(),
            Some("одного")
        );
        assert_eq!(
            written(
                "одна",
                Gender::Feminine,
                Number::Singular,
                Case::Accusative,
                Animacy::Inanimate
            )
            .as_deref(),
            Some("одну")
        );
    }

    #[test]
    fn the_nominative_masculine_is_the_dictionary_form_itself() {
        for held in DECLINING {
            assert_eq!(
                written(
                    held.dictionary,
                    Gender::Masculine,
                    Number::Singular,
                    Case::Nominative,
                    Animacy::Inanimate
                )
                .as_deref(),
                Some(held.dictionary),
                "{}",
                held.dictionary
            );
        }
    }

    #[test]
    fn a_written_form_is_read_back_to_the_case_that_wrote_it() {
        for (word, dictionary, case) in [
            ("тем", "тот", Case::Instrumental),
            ("того", "тот", Case::Genitive),
            ("тому", "тот", Case::Dative),
            ("том", "тот", Case::Prepositional),
            ("чем", "что", Case::Instrumental),
            ("чём", "что", Case::Prepositional),
            ("всём", "весь", Case::Prepositional),
            ("моими", "мой", Case::Instrumental)
        ] {
            let found = cells(word, dictionary, Animacy::Inanimate);

            assert!(
                found.iter().any(|cell| cell.case == case),
                "{word} from {dictionary} is no {case:?}: {found:?}"
            );
        }
    }

    #[test]
    fn the_plural_states_no_gender_and_the_singular_states_one() {
        let plural = cells("теми", "тот", Animacy::Inanimate);
        let singular = cells("тому", "тот", Animacy::Inanimate);

        assert!(plural.iter().all(|cell| cell.gender.is_none()));
        assert!(singular.iter().all(|cell| cell.gender.is_some()));
    }

    #[test]
    fn a_form_of_another_word_is_read_back_to_nothing() {
        assert!(cells("стола", "тот", Animacy::Inanimate).is_empty());
        assert!(cells("тем", "стол", Animacy::Inanimate).is_empty());
        assert!(cells("", "тот", Animacy::Inanimate).is_empty());
    }

    #[test]
    fn every_cell_of_every_pronoun_is_read_back_to_itself() {
        for held in DECLINING {
            for number in [Number::Singular, Number::Plural] {
                for gender in Gender::STATED {
                    for case in Case::STATED {
                        let word =
                            written(held.dictionary, gender, number, case, Animacy::Inanimate)
                                .expect("a pronoun that declines writes every cell");
                        let found = cells(&word, held.dictionary, Animacy::Inanimate);

                        assert!(!found.is_empty(), "{word} is written and not read back");
                    }
                }
            }
        }
    }

    #[test]
    fn a_living_being_is_pointed_at_in_the_genitive() {
        assert_eq!(
            written(
                "тот",
                Gender::Masculine,
                Number::Singular,
                Case::Accusative,
                Animacy::Animate
            )
            .as_deref(),
            Some("того")
        );
        assert_eq!(
            written(
                "тот",
                Gender::Masculine,
                Number::Singular,
                Case::Accusative,
                Animacy::Inanimate
            )
            .as_deref(),
            Some("т")
        );
    }

    #[test]
    fn the_case_a_word_is_written_in_does_not_matter() {
        assert_eq!(declining("ТОТ").map(|held| held.stem), Some("т"));
        assert!(!cells("ТЕМ", "тот", Animacy::Inanimate).is_empty());
    }
}
