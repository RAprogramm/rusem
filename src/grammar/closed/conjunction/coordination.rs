// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What a coordinating conjunction does to the two equals it joins.
//!
//! [`super::COORDINATING`] says a word joins equals. It does not say how, and
//! a checker needs to know: `и` adds, `но` opposes, `или` offers a choice, and
//! a rule written for one of the three is wrong for the other two.
//!
//! `а то`, `не то` and `то ли` are written in two words and are not here. The
//! lists hold what a tokenizer hands over one token at a time.

/// What the joining does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Sense {
    /// `и`, `да`, `тоже`: the second is added to the first.
    Connective,
    /// `но`, `зато`, `однако`: the second stands against the first.
    Adversative,
    /// `или`, `либо`: one of the two, not both.
    Disjunctive,
    /// `притом`, `причём`: the second is said on top of the first.
    Attaching
}

/// The conjunctions that add.
pub const CONNECTIVE: &[&str] = &["и", "также", "тоже"];

/// The conjunctions that oppose.
pub const ADVERSATIVE: &[&str] = &["а", "же", "зато", "но", "однако"];

/// The conjunctions that offer a choice.
pub const DISJUNCTIVE: &[&str] = &["или", "либо", "ни"];

/// The conjunctions that say the second on top of the first.
pub const ATTACHING: &[&str] = &["притом", "причём"];

/// The conjunctions that both add and oppose.
///
/// `да` alone is in two senses at once: `хлеб да соль` adds and `мал да удал`
/// opposes. Nothing in the word tells which, and [`senses`] answers both.
pub const AMBIGUOUS: &[(&str, &[Sense])] = &[("да", &[Sense::Connective, Sense::Adversative])];

/// Every sense, with the conjunctions of one sense only.
const PLAIN: &[(Sense, &[&str])] = &[
    (Sense::Connective, CONNECTIVE),
    (Sense::Adversative, ADVERSATIVE),
    (Sense::Disjunctive, DISJUNCTIVE),
    (Sense::Attaching, ATTACHING)
];

/// The senses a coordinating conjunction joins in.
///
/// A list, because `да` joins in two and only the sentence tells which.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::conjunction::coordination::{Sense, senses};
///
/// assert_eq!(senses("и"), vec![Sense::Connective]);
/// assert_eq!(senses("но"), vec![Sense::Adversative]);
/// assert_eq!(senses("да").len(), 2);
/// assert!(senses("стол").is_empty());
/// ```
#[must_use]
pub fn senses(written: &str) -> Vec<Sense> {
    let held = written.to_lowercase();

    if let Some((_, senses)) = AMBIGUOUS.iter().find(|(word, _)| *word == held) {
        return (*senses).to_vec();
    }

    PLAIN
        .iter()
        .filter(|(_, words)| words.contains(&held.as_str()))
        .map(|(sense, _)| *sense)
        .collect()
}

/// Reports whether a coordinating conjunction can join in a sense.
#[must_use]
pub fn joins_in(written: &str, sense: Sense) -> bool {
    senses(written).contains(&sense)
}

/// Reports whether a conjunction sets the second equal against the first.
///
/// `он пришёл, но ушёл` opposes and `он пришёл и ушёл` does not, which is what
/// a gate on a contradicted predicate asks before it reports anything.
#[must_use]
pub fn opposes(written: &str) -> bool {
    joins_in(written, Sense::Adversative)
}

#[cfg(test)]
mod tests {
    use super::{super::COORDINATING, *};

    #[test]
    fn every_coordinating_conjunction_joins_in_a_sense() {
        for held in COORDINATING {
            assert!(!senses(held).is_empty(), "{held} joins in no stated sense");
        }
    }

    #[test]
    fn every_sense_holds_only_coordinating_conjunctions() {
        for (_, words) in PLAIN {
            for held in *words {
                assert!(
                    COORDINATING.contains(held),
                    "{held} is no coordinating conjunction"
                );
            }
        }
        for (held, _) in AMBIGUOUS {
            assert!(
                COORDINATING.contains(held),
                "{held} is no coordinating conjunction"
            );
        }
    }

    #[test]
    fn no_conjunction_of_one_sense_is_listed_under_two() {
        let mut every: Vec<&str> = PLAIN
            .iter()
            .flat_map(|(_, words)| *words)
            .copied()
            .collect();
        let counted = every.len();
        every.sort_unstable();
        every.dedup();

        assert_eq!(every.len(), counted, "a conjunction is listed twice");
    }

    #[test]
    fn a_conjunction_of_one_sense_names_that_sense() {
        assert_eq!(senses("и"), std::vec![Sense::Connective]);
        assert_eq!(senses("но"), std::vec![Sense::Adversative]);
        assert_eq!(senses("или"), std::vec![Sense::Disjunctive]);
        assert_eq!(senses("притом"), std::vec![Sense::Attaching]);
    }

    #[test]
    fn the_conjunction_of_two_senses_names_both() {
        let held = senses("да");

        assert!(held.contains(&Sense::Connective));
        assert!(held.contains(&Sense::Adversative));
    }

    #[test]
    fn a_word_that_joins_nothing_joins_in_no_sense() {
        assert!(senses("стол").is_empty());
        assert!(!joins_in("что", Sense::Connective));
        assert!(senses("").is_empty());
    }

    #[test]
    fn only_an_adversative_opposes() {
        assert!(opposes("но"));
        assert!(opposes("ОДНАКО"));
        assert!(opposes("да"));
        assert!(!opposes("и"));
        assert!(!opposes("или"));
    }
}
