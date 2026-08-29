// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The numerals whose stem changes under them.
//!
//! `два` gives `двух` and `двумя`; `сорок` gives `сорока` and nothing else.
//! Neither is a stem with endings on it — the word is remade — so their cells
//! are stated as they are said.
//!
//! The numerals with two forms between them — `сорок`, `сто`, `полтора` — are
//! not stated at all: [`oblique`] reads their one oblique form off the word.
//!
//! `оба` is here for the same reason: it says `обоих`, not `обих`, so the stem
//! it declines on is not the stem it is written with.
//!
//! Everything else in the class declines by a pattern, and everything above a
//! hundred is built out of what is here.

use crate::grammar::{Animacy, Case};

/// The cells of a numeral that states them whole.
///
/// One string to a case, in the order the grammars name them, and the
/// accusative resolved by animacy where the word tells the two apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cells {
    /// Nominative.
    pub nominative:    &'static str,
    /// Genitive, which the accusative repeats for a living being.
    pub genitive:      &'static str,
    /// Dative.
    pub dative:        &'static str,
    /// Instrumental.
    pub instrumental:  &'static str,
    /// Prepositional.
    pub prepositional: &'static str
}

impl Cells {
    /// The form this numeral is written in, in the case asked for.
    #[must_use]
    pub const fn of(&self, case: Case, animacy: Animacy) -> &'static str {
        match case.merged() {
            Case::Genitive => self.genitive,
            Case::Dative => self.dative,
            Case::Instrumental => self.instrumental,
            Case::Prepositional => self.prepositional,
            Case::Accusative => match animacy {
                Animacy::Animate => self.genitive,
                Animacy::Inanimate => self.nominative
            },
            _ => self.nominative
        }
    }
}

/// Every numeral that states its cells whole, by its dictionary form.
const WHOLE: &[(&str, Cells)] = &[
    (
        "два",
        Cells {
            nominative:    "два",
            genitive:      "двух",
            dative:        "двум",
            instrumental:  "двумя",
            prepositional: "двух"
        }
    ),
    (
        "две",
        Cells {
            nominative:    "две",
            genitive:      "двух",
            dative:        "двум",
            instrumental:  "двумя",
            prepositional: "двух"
        }
    ),
    (
        "три",
        Cells {
            nominative:    "три",
            genitive:      "трёх",
            dative:        "трём",
            instrumental:  "тремя",
            prepositional: "трёх"
        }
    ),
    (
        "четыре",
        Cells {
            nominative:    "четыре",
            genitive:      "четырёх",
            dative:        "четырём",
            instrumental:  "четырьмя",
            prepositional: "четырёх"
        }
    ),
    (
        "оба",
        Cells {
            nominative:    "оба",
            genitive:      "обоих",
            dative:        "обоим",
            instrumental:  "обоими",
            prepositional: "обоих"
        }
    ),
    (
        "обе",
        Cells {
            nominative:    "обе",
            genitive:      "обеих",
            dative:        "обеим",
            instrumental:  "обеими",
            prepositional: "обеих"
        }
    )
];

/// The one form a twofold numeral says in every cell but the first.
///
/// `сорок`, `девяносто` and `сто` say the word with `а` for its last letter:
/// `сорока`, `девяноста`, `ста`. `полтора` says the same shape on a stem that
/// grows — `пол-` becomes `полу-` — so `полтора` and `полторы` both give
/// `полутора`, and `полтораста` gives `полутораста`.
///
/// Nothing is stored: both are read off the word.
///
/// # Examples
///
/// ```
/// use rusem::grammar::declension::numeral::whole::oblique;
///
/// assert_eq!(oblique("сорок").as_deref(), Some("сорока"));
/// assert_eq!(oblique("сто").as_deref(), Some("ста"));
/// assert_eq!(oblique("полторы").as_deref(), Some("полутора"));
/// assert_eq!(oblique("пять"), None);
/// ```
#[must_use]
pub fn oblique(dictionary: &str) -> Option<String> {
    let held = dictionary.to_lowercase();

    if let Some(rest) = held.strip_prefix("полтор") {
        return match rest {
            "а" | "ы" => Some(String::from("полутора")),
            "аста" => Some(String::from("полутораста")),
            _ => None
        };
    }
    if !TWOFOLD.contains(&held.as_str()) {
        return None;
    }

    let stem = held.strip_suffix('о').unwrap_or(&held);

    Some(String::from(stem) + "а")
}

/// The numerals whose whole paradigm is the word and the word ending in `а`.
///
/// Three of them and the grammars name no others: forty, ninety, a hundred.
/// What they say in the five oblique cells is read off the word rather than
/// stored beside it.
const TWOFOLD: &[&str] = &["сорок", "девяносто", "сто"];

/// Reports whether a numeral says one form in every case but the first.
#[must_use]
pub fn is_twofold(dictionary: &str) -> bool {
    oblique(dictionary).is_some()
}

/// The cells a dictionary form states, when it states them whole.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Animacy, Case, declension::numeral::whole::cells_of};
///
/// let held = cells_of("два").expect("два states its cells");
/// assert_eq!(held.of(Case::Instrumental, Animacy::Inanimate), "двумя");
/// assert_eq!(cells_of("пять"), None, "five declines by a pattern");
/// ```
#[must_use]
pub fn cells_of(dictionary: &str) -> Option<Cells> {
    let held = dictionary.to_lowercase();

    WHOLE
        .iter()
        .find(|(word, _)| *word == held)
        .map(|(_, cells)| *cells)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_three_and_four_are_each_declined_their_own_way() {
        for (dictionary, genitive, instrumental) in [
            ("два", "двух", "двумя"),
            ("три", "трёх", "тремя"),
            ("четыре", "четырёх", "четырьмя")
        ] {
            let held = cells_of(dictionary).expect("stated whole");

            assert_eq!(held.of(Case::Genitive, Animacy::Inanimate), genitive);
            assert_eq!(
                held.of(Case::Instrumental, Animacy::Inanimate),
                instrumental
            );
        }
    }

    #[test]
    fn the_twofold_numerals_say_one_word_read_off_themselves() {
        for (dictionary, rest) in [
            ("сорок", "сорока"),
            ("девяносто", "девяноста"),
            ("сто", "ста"),
            ("полтора", "полутора"),
            ("полторы", "полутора"),
            ("полтораста", "полутораста")
        ] {
            assert_eq!(oblique(dictionary).as_deref(), Some(rest), "{dictionary}");
            assert!(is_twofold(dictionary));
        }
    }

    #[test]
    fn a_numeral_that_is_not_twofold_says_no_oblique_of_its_own() {
        assert_eq!(oblique("пять"), None);
        assert_eq!(oblique("полтина"), None);
        assert_eq!(oblique(""), None);
        assert!(!is_twofold("два"));
    }

    #[test]
    fn the_feminine_two_parts_from_the_masculine_in_the_nominative_only() {
        let masculine = cells_of("два").expect("stated whole");
        let feminine = cells_of("две").expect("stated whole");

        assert_ne!(masculine.nominative, feminine.nominative);
        assert_eq!(masculine.genitive, feminine.genitive);
        assert_eq!(masculine.instrumental, feminine.instrumental);
    }

    #[test]
    fn a_numeral_counting_living_beings_points_at_them_in_the_genitive() {
        let held = cells_of("два").expect("stated whole");

        assert_eq!(held.of(Case::Accusative, Animacy::Animate), "двух");
        assert_eq!(held.of(Case::Accusative, Animacy::Inanimate), "два");
    }

    #[test]
    fn a_numeral_that_declines_by_a_pattern_is_not_here() {
        assert_eq!(cells_of("пять"), None);
        assert_eq!(cells_of("двое"), None);
        assert_eq!(cells_of("стол"), None);
        assert_eq!(cells_of(""), None);
    }

    #[test]
    fn the_case_a_word_is_written_in_does_not_matter() {
        assert!(cells_of("ДВА").is_some());
    }
}
