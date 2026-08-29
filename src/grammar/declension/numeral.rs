// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The declension of the numerals, which the engine did not have.
//!
//! `пять столов` and `пяти столов` were the same word to it, because nothing
//! read `пяти` back to `пять`. A whole closed class stood outside the case
//! system.
//!
//! # Three ways a numeral declines
//!
//! Most of them are a stem and a pattern: `пять` is `пят` in the third
//! declension, `двое` is `дво` in the adjectival plural. [`paradigm`] holds the
//! endings.
//!
//! Six of them remake the word instead — `два` gives `двух`, `сорок` gives
//! `сорока` — so [`whole`] states their cells as they are said.
//!
//! The rest are **two numerals grown together**, and both halves decline.
//! `пятьдесят` is `пять` and `десять`: the genitive is `пяти` and `десяти`
//! written as one word.
//!
//! § 82 п. 1 says which words those are, and says it by the ending rather than
//! by naming them: a cardinal whose last element is `-десят`, `-ста` or `-сот`
//! is written solid in every case. § 73 says the same from the other side —
//! the soft sign stands in the middle of `пятьдесят` because both halves
//! inflect. So eleven numerals are read off their own endings and cost
//! nothing. `двести` alone is not, and the paragraph does not pretend it is.

pub mod paradigm;
pub mod whole;

use self::paradigm::Paradigm;
use crate::grammar::{Animacy, Case, closed};

/// A numeral that declines by a stem and a pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Declining<'a> {
    /// What the endings are put on: `пят`, `дво`.
    pub stem:     &'a str,
    /// Which pattern the endings come from.
    pub paradigm: Paradigm
}

/// The numerals whose stem the letters do not give.
///
/// One word, and the grammars name it: the `е` of `восемь` runs out under
/// every ending — `восьми`, `восемью` — and nothing in the writing says it
/// will. Whether a vowel is fleeting is a fact about the word, not about the
/// letters, so the word is named and the rest are read.
const FLEETING: &[(&str, &str)] = &[("восемь", "восьм")];

/// The endings § 82 names a grown numeral by, with the numeral each half is.
///
/// `пятьдесят`, `пятидесяти`, `триста`, `трёхсот`, `семьсот`, `семисот` — the
/// paragraph lists the endings and says the word is written solid in every
/// case. § 73 says the same words from the other side: both halves inflect,
/// which is why the soft sign stands in the middle of `пятьдесят`.
///
/// So a grown numeral is known by how it ends, not by being on a list. What
/// stands in front of the ending is the other half, and it is a numeral in its
/// own right.
const GROWN: &[(&str, &str)] = &[("десят", "десять"), ("сот", "сто"), ("ста", "сто")];

/// The grown numeral the endings do not reach.
///
/// `двести` ends in neither `-ста` nor `-сот`: it is `две` and an old dual of
/// `сто` that survives in this one word. § 82 lists it among the solid ones
/// and does not pretend it follows the pattern, and neither does this.
const WORN: &[(&str, &str, &str)] = &[("двести", "два", "сто")];

/// The two halves a grown numeral is made of, when it is one.
///
/// # Examples
///
/// ```
/// use rusem::grammar::declension::numeral::halves;
///
/// assert_eq!(halves("пятьдесят"), Some(("пять", "десять")));
/// assert_eq!(halves("семьсот"), Some(("семь", "сто")));
/// assert_eq!(halves("триста"), Some(("три", "сто")));
/// assert_eq!(halves("двести"), Some(("два", "сто")));
/// assert_eq!(halves("пять"), None);
/// ```
#[must_use]
pub fn halves(dictionary: &str) -> Option<(&str, &'static str)> {
    if let Some((_, first, second)) = WORN
        .iter()
        .find(|(word, _, _)| word.eq_ignore_ascii_case(dictionary) || *word == dictionary)
    {
        return Some((first, second));
    }

    GROWN.iter().find_map(|(ending, second)| {
        let first = dictionary.strip_suffix(ending)?;

        (declines(first) || whole::cells_of(first).is_some()).then_some((first, *second))
    })
}

/// The pattern and stem a dictionary form declines by, when it has one.
///
/// Neither is stated anywhere. The class is [`closed::numeral`], the pattern
/// follows from how the word ends, and the stem is the word without that
/// letter: a numeral in a soft sign takes the third declension — `пять` is
/// `пят` — and one in a vowel is a collective taking the adjectival plural,
/// `двое` on `дво`.
///
/// # Examples
///
/// ```
/// use rusem::grammar::declension::numeral::declining;
///
/// assert_eq!(declining("пять").expect("a numeral").stem, "пят");
/// assert_eq!(declining("двое").expect("a numeral").stem, "дво");
/// assert_eq!(declining("восемь").expect("a numeral").stem, "восьм");
/// assert!(declining("стол").is_none());
/// ```
#[must_use]
pub fn declining(dictionary: &str) -> Option<Declining<'_>> {
    let paradigm = pattern_of(dictionary)?;
    let stem = FLEETING
        .iter()
        .find(|(word, _)| word.eq_ignore_ascii_case(dictionary) || *word == dictionary)
        .map_or_else(|| shortened(dictionary), |(_, stem)| *stem);

    Some(Declining {
        stem,
        paradigm
    })
}

/// The pattern a numeral declines by, read off how the word ends.
fn pattern_of(dictionary: &str) -> Option<Paradigm> {
    let held = dictionary.to_lowercase();
    if whole::cells_of(&held).is_some() {
        return None;
    }

    let last = held.chars().last()?;
    if last == 'ь' && closed::numeral::CARDINAL.contains(&held.as_str()) {
        return Some(Paradigm::Third);
    }
    if closed::numeral::COLLECTIVE.contains(&held.as_str()) {
        return Some(Paradigm::Collective);
    }

    None
}

/// The word without its last letter, which is the stem the endings go on.
fn shortened(dictionary: &str) -> &str {
    dictionary
        .char_indices()
        .next_back()
        .map_or(dictionary, |(at, _)| &dictionary[..at])
}

/// The form a numeral is written in, in the case asked for.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Animacy, Case, declension::numeral::written};
///
/// assert_eq!(
///     written("пять", Case::Instrumental, Animacy::Inanimate).as_deref(),
///     Some("пятью")
/// );
/// assert_eq!(
///     written("два", Case::Genitive, Animacy::Inanimate).as_deref(),
///     Some("двух")
/// );
/// assert_eq!(
///     written("сорок", Case::Dative, Animacy::Inanimate).as_deref(),
///     Some("сорока")
/// );
/// assert_eq!(
///     written("пятьдесят", Case::Genitive, Animacy::Inanimate).as_deref(),
///     Some("пятидесяти")
/// );
/// assert_eq!(
///     written("двести", Case::Dative, Animacy::Inanimate).as_deref(),
///     Some("двумстам")
/// );
/// ```
#[must_use]
pub fn written(dictionary: &str, case: Case, animacy: Animacy) -> Option<String> {
    let held = dictionary.to_lowercase();

    if let Some(cells) = whole::cells_of(&held) {
        return Some(String::from(cells.of(case, animacy)));
    }
    if let Some(oblique) = whole::oblique(&held) {
        return Some(match case.merged() {
            Case::Nominative | Case::Vocative | Case::Accusative => held,
            _ => oblique
        });
    }
    if let Some(numeral) = declining(&held) {
        return numeral
            .paradigm
            .ending(case, animacy)
            .map(|ending| String::from(numeral.stem) + ending);
    }

    grown(&held, case, animacy)
}

/// The form a numeral grown out of two halves is written in.
///
/// The nominative and the inanimate accusative are the word as it stands: § 82
/// writes it solid, and nothing about it is built. Every other case is the two
/// halves declined and written together.
fn grown(held: &str, case: Case, animacy: Animacy) -> Option<String> {
    let (first, second) = halves(held)?;
    if matches!(case.merged(), Case::Nominative | Case::Vocative)
        || (case.merged() == Case::Accusative && animacy == Animacy::Inanimate)
    {
        return Some(String::from(held));
    }

    let ahead = written(first, case, Animacy::Inanimate)?;
    let behind = if second == "сто" {
        String::from(hundreds(case))
    } else {
        written(second, case, Animacy::Inanimate)?
    };

    Some(ahead + &behind)
}

/// The form the second half of a hundreds numeral takes.
///
/// `сто` in the plural, which it has nowhere else in the language: `сот`,
/// `стам`, `стами`, `стах`. The nominative is not here because a grown numeral
/// says its own — `двести`, not `двасто` — and never reaches this.
const fn hundreds(case: Case) -> &'static str {
    match case.merged() {
        Case::Dative => "стам",
        Case::Instrumental => "стами",
        Case::Prepositional => "стах",
        _ => "сот"
    }
}

/// Every case a numeral could have been written in.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Animacy, Case, declension::numeral::cases};
///
/// assert!(cases("пяти", "пять", Animacy::Inanimate).contains(&Case::Genitive));
/// assert!(cases("двухсот", "двести", Animacy::Inanimate).contains(&Case::Genitive));
/// assert!(cases("стол", "пять", Animacy::Inanimate).is_empty());
/// ```
#[must_use]
pub fn cases(word: &str, dictionary: &str, animacy: Animacy) -> Vec<Case> {
    let held = word.to_lowercase();

    Case::STATED
        .into_iter()
        .filter(|case| written(dictionary, *case, animacy).as_deref() == Some(&held))
        .collect()
}

/// Reports whether a dictionary form is a numeral this module declines.
#[must_use]
pub fn declines(dictionary: &str) -> bool {
    let held = dictionary.to_lowercase();

    whole::cells_of(&held).is_some()
        || whole::is_twofold(&held)
        || declining(&held).is_some()
        || halves(&held).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every numeral the module declines, by its dictionary form.
    fn every() -> Vec<String> {
        closed::numeral::CARDINAL
            .iter()
            .chain(closed::numeral::COLLECTIVE)
            .chain(closed::numeral::FRACTIONAL)
            .map(|held| (*held).to_owned())
            .filter(|held| declines(held))
            .collect()
    }

    /// The whole paradigm of a numeral, written out.
    fn paradigm_of(dictionary: &str) -> Vec<String> {
        Case::STATED
            .iter()
            .map(|case| written(dictionary, *case, Animacy::Inanimate).unwrap_or_default())
            .collect()
    }

    #[test]
    fn five_declines_as_a_noun_of_the_third_declension() {
        assert_eq!(
            paradigm_of("пять"),
            std::vec!["пять", "пяти", "пяти", "пять", "пятью", "пяти"]
        );
    }

    #[test]
    fn the_vowel_of_eight_runs_out_under_the_endings() {
        assert_eq!(
            written("восемь", Case::Genitive, Animacy::Inanimate).as_deref(),
            Some("восьми")
        );
    }

    #[test]
    fn a_collective_declines_as_an_adjective_in_the_plural() {
        assert_eq!(
            paradigm_of("двое"),
            std::vec!["двое", "двоих", "двоим", "двое", "двоими", "двоих"]
        );
    }

    #[test]
    fn both_declines_as_a_pronoun_in_the_plural() {
        assert_eq!(
            paradigm_of("оба"),
            std::vec!["оба", "обоих", "обоим", "оба", "обоими", "обоих"]
        );
        assert_eq!(
            written("обе", Case::Genitive, Animacy::Inanimate).as_deref(),
            Some("обеих")
        );
    }

    #[test]
    fn the_tens_decline_in_both_halves() {
        assert_eq!(
            paradigm_of("пятьдесят"),
            std::vec![
                "пятьдесят",
                "пятидесяти",
                "пятидесяти",
                "пятьдесят",
                "пятьюдесятью",
                "пятидесяти"
            ]
        );
        assert_eq!(
            written("восемьдесят", Case::Genitive, Animacy::Inanimate).as_deref(),
            Some("восьмидесяти")
        );
    }

    #[test]
    fn the_hundreds_decline_in_both_halves() {
        assert_eq!(
            paradigm_of("двести"),
            std::vec![
                "двести",
                "двухсот",
                "двумстам",
                "двести",
                "двумястами",
                "двухстах"
            ]
        );
        assert_eq!(
            written("пятьсот", Case::Genitive, Animacy::Inanimate).as_deref(),
            Some("пятисот")
        );
        assert_eq!(
            written("четыреста", Case::Instrumental, Animacy::Inanimate).as_deref(),
            Some("четырьмястами")
        );
        assert_eq!(
            written("девятьсот", Case::Dative, Animacy::Inanimate).as_deref(),
            Some("девятистам")
        );
    }

    #[test]
    fn a_numeral_that_remakes_itself_states_its_cells_whole() {
        assert_eq!(
            paradigm_of("два"),
            std::vec!["два", "двух", "двум", "два", "двумя", "двух"]
        );
        assert_eq!(
            paradigm_of("сорок"),
            std::vec!["сорок", "сорока", "сорока", "сорок", "сорока", "сорока"]
        );
    }

    #[test]
    fn a_written_form_is_read_back_to_the_case_that_wrote_it() {
        assert!(cases("пяти", "пять", Animacy::Inanimate).contains(&Case::Genitive));
        assert!(cases("двумя", "два", Animacy::Inanimate).contains(&Case::Instrumental));
        assert!(cases("двухсот", "двести", Animacy::Inanimate).contains(&Case::Genitive));
        assert!(cases("сорока", "сорок", Animacy::Inanimate).contains(&Case::Dative));
    }

    #[test]
    fn every_cell_of_every_numeral_is_read_back_to_itself() {
        for dictionary in every() {
            for case in Case::STATED {
                let word = written(&dictionary, case, Animacy::Inanimate)
                    .expect("a numeral that declines writes every cell");

                assert!(
                    !cases(&word, &dictionary, Animacy::Inanimate).is_empty(),
                    "{word} is written and not read back"
                );
            }
        }
    }

    #[test]
    fn every_half_of_every_grown_numeral_is_itself_a_numeral_that_declines() {
        for grown in every().iter().filter(|held| halves(held).is_some()) {
            let (first, second) = halves(grown).expect("it grew out of two");

            assert!(
                declines(first),
                "{grown} grew out of {first}, which declines by nothing"
            );
            assert!(
                declines(second),
                "{grown} grew out of {second}, which declines by nothing"
            );
        }
    }

    #[test]
    fn a_numeral_counting_living_beings_points_at_them_in_the_genitive() {
        assert_eq!(
            written("два", Case::Accusative, Animacy::Animate).as_deref(),
            Some("двух")
        );
        assert_eq!(
            written("двое", Case::Accusative, Animacy::Animate).as_deref(),
            Some("двоих")
        );
    }

    #[test]
    fn every_numeral_of_the_closed_class_declines_somewhere() {
        for held in closed::numeral::CARDINAL
            .iter()
            .chain(closed::numeral::COLLECTIVE)
            .chain(closed::numeral::FRACTIONAL)
        {
            let elsewhere = crate::grammar::declension::pronominal::declining(held).is_some()
                || matches!(*held, "тысяча" | "миллион" | "миллиард");

            assert!(
                declines(held) || elsewhere,
                "{held} is a numeral and declines nowhere at all"
            );
        }
    }

    #[test]
    fn a_word_that_is_no_numeral_declines_by_nothing() {
        assert!(written("стол", Case::Genitive, Animacy::Inanimate).is_none());
        assert!(cases("стол", "пять", Animacy::Inanimate).is_empty());
        assert!(!declines("стол"));
        assert!(!declines(""));
    }

    #[test]
    fn every_numeral_this_module_holds_says_it_declines() {
        for dictionary in every() {
            assert!(declines(&dictionary), "{dictionary}");
        }
        assert!(declines("сорок"));
        assert!(declines("два"));
    }

    #[test]
    fn the_case_a_word_is_written_in_does_not_matter() {
        assert_eq!(
            written("ПЯТЬ", Case::Genitive, Animacy::Inanimate).as_deref(),
            Some("пяти")
        );
        assert!(cases("ПЯТИ", "пять", Animacy::Inanimate).contains(&Case::Genitive));
    }
}
