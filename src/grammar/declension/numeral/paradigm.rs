// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The patterns a numeral declines by.
//!
//! Two of them, and between them they cover every numeral whose stem stands
//! still: five to thirty take the third declension of the nouns, the
//! collectives take the adjectival plural. Nothing here is a list of forms —
//! each pattern is the endings, and the numeral brings the stem.

use crate::grammar::{Animacy, Case};

/// How a numeral declines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Paradigm {
    /// `пять`, `десять`, `тридцать`: the third declension of the nouns —
    /// `пяти`, `пятью`. Everything from five to twenty and thirty.
    Third,
    /// `двое`, `трое`: the plural of the adjectival declension.
    Collective
}

/// The endings of the third declension, which is what five to thirty take.
///
/// `пять` is `пят` and a soft sign; the rest of the paradigm is the same stem
/// and these.
const THIRD: [(Case, &str); 6] = [
    (Case::Nominative, "ь"),
    (Case::Genitive, "и"),
    (Case::Dative, "и"),
    (Case::Accusative, "ь"),
    (Case::Instrumental, "ью"),
    (Case::Prepositional, "и")
];

/// The endings a collective numeral takes, which are the adjectival plural.
const COLLECTIVE: [(Case, &str); 6] = [
    (Case::Nominative, "е"),
    (Case::Genitive, "их"),
    (Case::Dative, "им"),
    (Case::Accusative, "е"),
    (Case::Instrumental, "ими"),
    (Case::Prepositional, "их")
];

impl Paradigm {
    /// The ending this pattern puts on a stem in the case asked for, or
    /// nothing when the pattern states no cell for it.
    ///
    /// The vocative is the one case that reaches the tables and finds no row:
    /// [`Case::merged`] leaves it standing, the patterns do not state it, and
    /// an answer invented here would hand the bare stem out as though `пят`
    /// were a form. The second cases are answered — the partitive by the
    /// genitive and the locative by the prepositional, which is what merging
    /// does.
    ///
    /// A numeral whose stem changes under it — `два`, `сорок`, `оба` — declines
    /// by no pattern at all, and is answered by
    /// [`super::whole`] instead.
    #[must_use]
    pub fn ending(self, case: Case, animacy: Animacy) -> Option<&'static str> {
        let table = match self {
            Self::Third => &THIRD,
            Self::Collective => &COLLECTIVE
        };
        let wanted = pointed(case, animacy);

        table
            .iter()
            .find(|(held, _)| *held == wanted)
            .map(|(_, ending)| *ending)
    }
}

/// The case a cell is written in, once the accusative has been resolved.
///
/// The accusative of a numeral counting living beings repeats the genitive —
/// `вижу двоих` — and repeats the nominative otherwise: `вижу двое суток`.
const fn pointed(case: Case, animacy: Animacy) -> Case {
    match (case.merged(), animacy) {
        (Case::Accusative, Animacy::Animate) => Case::Genitive,
        (held, _) => held
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_third_declension_writes_the_paradigm_of_five() {
        let written: Vec<String> = [
            Case::Nominative,
            Case::Genitive,
            Case::Dative,
            Case::Accusative,
            Case::Instrumental,
            Case::Prepositional
        ]
        .iter()
        .map(|case| {
            std::format!(
                "пят{}",
                Paradigm::Third
                    .ending(*case, Animacy::Inanimate)
                    .expect("a stated cell")
            )
        })
        .collect();

        assert_eq!(
            written,
            std::vec!["пять", "пяти", "пяти", "пять", "пятью", "пяти"]
        );
    }

    #[test]
    fn the_collective_pattern_writes_the_paradigm_of_dvoe() {
        let written: Vec<String> = [Case::Nominative, Case::Genitive, Case::Dative]
            .iter()
            .map(|case| {
                std::format!(
                    "дво{}",
                    Paradigm::Collective
                        .ending(*case, Animacy::Inanimate)
                        .expect("a stated cell")
                )
            })
            .collect();

        assert_eq!(written, std::vec!["двое", "двоих", "двоим"]);
    }

    #[test]
    fn a_numeral_counting_living_beings_points_at_them_in_the_genitive() {
        assert_eq!(
            Paradigm::Collective.ending(Case::Accusative, Animacy::Animate),
            Some("их")
        );
        assert_eq!(
            Paradigm::Collective.ending(Case::Accusative, Animacy::Inanimate),
            Some("е")
        );
    }

    #[test]
    fn every_pattern_states_an_ending_for_every_case_but_the_vocative() {
        for held in [Paradigm::Third, Paradigm::Collective] {
            for case in [
                Case::Nominative,
                Case::Genitive,
                Case::Dative,
                Case::Accusative,
                Case::Instrumental,
                Case::Prepositional,
                Case::Partitive,
                Case::Locative
            ] {
                assert!(
                    held.ending(case, Animacy::Inanimate)
                        .is_some_and(|ending| !ending.is_empty()),
                    "{held:?} {case:?}"
                );
            }

            assert_eq!(held.ending(Case::Vocative, Animacy::Inanimate), None);
        }
    }
}
