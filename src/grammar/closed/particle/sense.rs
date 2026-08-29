// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What a particle does to the word it stands beside.
//!
//! [`super::PARTICLES`] says a word is a particle, which is to say it names
//! nothing and adds a shade to something else. The shade is the whole of what
//! a particle is, so a class that does not state it states almost nothing.
//!
//! [`super::FORMATIVE`] is apart from these. A formative particle builds a
//! verb form and is a piece of the predicate rather than a shade on it — the
//! grammars set it against the modal ones for that reason, and so does this
//! module.
//!
//! Several particles carry more than one shade. `просто` limits in `просто
//! устал` and intensifies in `просто прелесть`; `точно` affirms and compares.
//! [`senses`] answers a list.

/// What shade the particle adds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Sense {
    /// `вот`, `вон`: here is the thing.
    Pointing,
    /// `именно`, `как раз`: this one and no other.
    Clarifying,
    /// `лишь`, `только`: this one and nothing besides.
    Limiting,
    /// `даже`, `же`, `ведь`: this one, and note it.
    Intensifying,
    /// `не`, `ни`: not this one.
    Denying,
    /// `да`: yes, this one.
    Affirming,
    /// `ли`, `разве`, `неужели`: is it this one?
    Asking,
    /// `будто`, `словно`: as if this one.
    Comparing,
    /// `якобы`, `мол`: this one, so they say.
    Retelling
}

/// Every shade, with the particles that carry it.
///
/// A particle stands under every shade it carries, so the lists overlap on
/// purpose.
const SENSES: &[(Sense, &[&str])] = &[
    (Sense::Pointing, &["вон", "вот"]),
    (Sense::Clarifying, &["именно", "просто", "чуть"]),
    (Sense::Limiting, &["лишь", "только", "хоть"]),
    (Sense::Intensifying, &["ведь", "даже", "же", "просто", "уж"]),
    (Sense::Denying, &["не", "ни"]),
    (Sense::Affirming, &["да"]),
    (Sense::Asking, &["ли", "ль", "неужели", "разве"]),
    (Sense::Comparing, &["будто", "как", "словно"]),
    (Sense::Retelling, &["мол", "якобы"])
];

/// The shades a written particle carries.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::particle::sense::{Sense, senses};
///
/// assert_eq!(senses("вот"), vec![Sense::Pointing]);
/// assert_eq!(senses("не"), vec![Sense::Denying]);
/// assert!(senses("просто").len() > 1);
/// assert!(senses("стол").is_empty());
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

/// Reports whether a particle can carry a shade.
#[must_use]
pub fn carries(written: &str, sense: Sense) -> bool {
    senses(written).contains(&sense)
}

/// Reports whether a particle turns the phrase it stands in into a question.
///
/// `разве он пришёл` asks and `ведь он пришёл` does not, which is what a gate
/// on a missing question mark asks before it reports one.
#[must_use]
pub fn asks(written: &str) -> bool {
    carries(written, Sense::Asking)
}

#[cfg(test)]
mod tests {
    use super::{
        super::{FORMATIVE, PARTICLES},
        *
    };

    #[test]
    fn every_listed_particle_is_a_particle() {
        for (sense, words) in SENSES {
            for held in *words {
                assert!(
                    PARTICLES.contains(held),
                    "{held} is under {sense:?} and is no particle"
                );
            }
        }
    }

    #[test]
    fn every_particle_that_is_not_formative_carries_a_shade() {
        for held in PARTICLES {
            if FORMATIVE.contains(held) {
                continue;
            }

            assert!(!senses(held).is_empty(), "{held} carries no stated shade");
        }
    }

    #[test]
    fn no_formative_particle_carries_a_shade() {
        for held in FORMATIVE {
            assert!(
                senses(held).is_empty(),
                "{held} builds a form and adds no shade"
            );
        }
    }

    #[test]
    fn every_shade_is_sorted_and_holds_no_word_twice() {
        for (sense, words) in SENSES {
            let mut held = words.to_vec();
            held.sort_unstable();
            held.dedup();

            assert_eq!(held.len(), words.len(), "{sense:?} holds a word twice");
        }
    }

    #[test]
    fn a_particle_of_one_shade_names_it() {
        assert_eq!(senses("вот"), std::vec![Sense::Pointing]);
        assert_eq!(senses("не"), std::vec![Sense::Denying]);
        assert_eq!(senses("РАЗВЕ"), std::vec![Sense::Asking]);
    }

    #[test]
    fn every_shade_of_the_enum_has_a_carrier() {
        for sense in [
            Sense::Pointing,
            Sense::Clarifying,
            Sense::Limiting,
            Sense::Intensifying,
            Sense::Denying,
            Sense::Affirming,
            Sense::Asking,
            Sense::Comparing,
            Sense::Retelling
        ] {
            assert!(
                SENSES.iter().any(|(held, _)| *held == sense),
                "{sense:?} is a shade no particle carries"
            );
        }
    }

    #[test]
    fn the_affirming_and_the_retelling_shades_are_carried() {
        assert_eq!(senses("да"), std::vec![Sense::Affirming]);
        assert!(carries("мол", Sense::Retelling));
        assert!(carries("якобы", Sense::Retelling));
    }

    #[test]
    fn a_particle_of_two_shades_names_both() {
        let held = senses("просто");

        assert!(held.contains(&Sense::Clarifying));
        assert!(held.contains(&Sense::Intensifying));
    }

    #[test]
    fn a_word_that_is_no_particle_carries_no_shade() {
        assert!(senses("стол").is_empty());
        assert!(!carries("и", Sense::Denying));
        assert!(senses("").is_empty());
    }

    #[test]
    fn only_an_asking_particle_asks() {
        assert!(asks("ли"));
        assert!(asks("неужели"));
        assert!(!asks("ведь"));
        assert!(!asks("не"));
    }
}
