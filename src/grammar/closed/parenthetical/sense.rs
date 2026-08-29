// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What an aside says about the sentence it stands beside.
//!
//! [`super::needs_commas`] answers the punctuation question and stops there.
//! This module answers the other one: an aside is the speaker stepping out of
//! the sentence to say something about it, and what they say falls into six
//! kinds the grammars agree on.
//!
//! It matters beyond style. Two asides of the same kind in one sentence are a
//! repetition — `конечно, безусловно, он придёт` says sureness twice — and two
//! of opposite kinds are a contradiction: `конечно, кажется, он придёт` is sure
//! and unsure at once. Neither is visible to a rule that knows only that both
//! words take commas.
//!
//! Only the words of [`super::ALWAYS`] and [`super::EITHER`] are here.
//! [`super::NEVER`] holds words that are no asides at all, and an aside they
//! are not has nothing to say about the sentence.

/// What the aside says.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Sense {
    /// `конечно`, `безусловно`: the speaker is sure of it.
    Certainty,
    /// `кажется`, `возможно`, `наверное`: the speaker is not.
    Doubt,
    /// `по-моему`, `говорят`: who it is known from.
    Source,
    /// `во-первых`, `итак`, `следовательно`: where it stands among the rest.
    Order,
    /// `впрочем`, `наоборот`, `однако`: how it turns against what came before.
    Turn,
    /// `например`, `кстати`: how it is being put.
    Manner
}

/// Every sense, with the asides that carry it.
const SENSES: &[(Sense, &[&str])] = &[
    (
        Sense::Certainty,
        &[
            "безусловно",
            "бесспорно",
            "действительно",
            "конечно",
            "разумеется"
        ]
    ),
    (
        Sense::Doubt,
        &[
            "видимо",
            "возможно",
            "кажется",
            "наверное",
            "пожалуй",
            "по-видимому"
        ]
    ),
    (Sense::Source, &["бывало", "верно", "правда"]),
    (
        Sense::Order,
        &[
            "в-пятых",
            "в-третьих",
            "в-четвёртых",
            "во-вторых",
            "во-первых",
            "итак",
            "наконец",
            "следовательно"
        ]
    ),
    (
        Sense::Turn,
        &["вообще", "впрочем", "наоборот", "напротив", "однако"]
    ),
    (
        Sense::Manner,
        &["естественно", "кстати", "например", "значит"]
    )
];

/// The senses a written aside carries.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::parenthetical::sense::{Sense, senses};
///
/// assert_eq!(senses("конечно"), vec![Sense::Certainty]);
/// assert_eq!(senses("кажется"), vec![Sense::Doubt]);
/// assert!(senses("вдруг").is_empty());
/// ```
#[must_use]
pub fn senses(written: &str) -> Vec<Sense> {
    let held = written.to_lowercase();

    SENSES
        .iter()
        .filter(|(_, words)| words.contains(&held.as_str()))
        .map(|(sense, _)| *sense)
        .collect()
}

/// Reports whether an aside carries a sense.
#[must_use]
pub fn carries(written: &str, sense: Sense) -> bool {
    senses(written).contains(&sense)
}

/// Reports whether two asides contradict one another.
///
/// Sureness against doubt and nothing else: `конечно, кажется, он придёт`
/// claims both at once, while two asides of the same sense merely repeat.
#[must_use]
pub fn contradict(one: &str, other: &str) -> bool {
    let (first, second) = (senses(one), senses(other));

    (first.contains(&Sense::Certainty) && second.contains(&Sense::Doubt))
        || (first.contains(&Sense::Doubt) && second.contains(&Sense::Certainty))
}

#[cfg(test)]
mod tests {
    use super::{
        super::{ALWAYS, EITHER, NEVER},
        *
    };

    #[test]
    fn every_aside_says_something() {
        for held in [ALWAYS, EITHER].concat() {
            assert!(
                !senses(held).is_empty(),
                "{held} is an aside that says nothing"
            );
        }
    }

    #[test]
    fn every_listed_word_is_an_aside() {
        for (sense, words) in SENSES {
            for held in *words {
                assert!(
                    ALWAYS.contains(held) || EITHER.contains(held),
                    "{held} is under {sense:?} and is no aside"
                );
            }
        }
    }

    #[test]
    fn a_word_that_is_never_an_aside_says_nothing() {
        for held in NEVER {
            assert!(
                senses(held).is_empty(),
                "{held} is no aside and says something"
            );
        }
    }

    #[test]
    fn no_aside_is_listed_under_two_senses() {
        let mut every: Vec<&str> = SENSES
            .iter()
            .flat_map(|(_, words)| *words)
            .copied()
            .collect();
        let counted = every.len();
        every.sort_unstable();
        every.dedup();

        assert_eq!(every.len(), counted, "an aside is listed twice");
    }

    #[test]
    fn an_aside_names_what_it_says() {
        assert_eq!(senses("конечно"), std::vec![Sense::Certainty]);
        assert_eq!(senses("КАЖЕТСЯ"), std::vec![Sense::Doubt]);
        assert_eq!(senses("итак"), std::vec![Sense::Order]);
        assert!(carries("однако", Sense::Turn));
    }

    #[test]
    fn a_word_that_is_no_aside_carries_nothing() {
        assert!(senses("стол").is_empty());
        assert!(!carries("вдруг", Sense::Certainty));
        assert!(senses("").is_empty());
    }

    #[test]
    fn sureness_and_doubt_contradict_and_nothing_else_does() {
        assert!(contradict("конечно", "кажется"));
        assert!(contradict("возможно", "безусловно"));
        assert!(!contradict("конечно", "разумеется"));
        assert!(!contradict("итак", "кстати"));
        assert!(!contradict("стол", "конечно"));
    }
}
