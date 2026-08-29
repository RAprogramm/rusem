// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The particles, and the two of them the checker asks about most.
//!
//! A particle carries no meaning of its own and changes the meaning of what it
//! stands beside. `не` denies, `ли` questions, `бы` supposes, `же` insists.
//!
//! Two subsets are named because rules keep asking for them. The **denying**
//! particles are what a rule on double negation counts. The **enclitics** lean
//! on the word before them and cannot open a phrase — `ли` at the start of a
//! sentence is not a Russian sentence.

//! What shade each particle adds is stated in [`sense`], apart from the
//! formative ones, which build a verb form rather than shade a word.

pub mod sense;

/// The particles.
pub const PARTICLES: &[&str] = &[
    "б",
    "будто",
    "бы",
    "ведь",
    "вон",
    "вот",
    "давай",
    "давайте",
    "даже",
    "же",
    "именно",
    "как",
    "ли",
    "лишь",
    "ль",
    "не",
    "неужели",
    "ни",
    "просто",
    "пусть",
    "пускай",
    "разве",
    "словно",
    "только",
    "уж",
    "хоть",
    "чуть"
];

/// The particles that build a form rather than change a meaning.
///
/// `бы` makes the subjunctive out of the past, `пусть` and `давай` make an
/// imperative out of the present. They are part of the verb form and not
/// words standing beside it, which is why a gate counting words in a predicate
/// must not count them.
pub const FORMATIVE: &[&str] = &["б", "бы", "давай", "давайте", "пускай", "пусть"];

/// The particles that deny.
///
/// `не` denies what follows it; `ни` denies it again and is what a rule on
/// double negation counts alongside the first.
pub const DENYING: &[&str] = &["не", "ни"];

/// The particles that lean on the word before them.
///
/// They cannot open a phrase: nothing precedes them to lean on.
pub const ENCLITIC: &[&str] = &["б", "бы", "же", "ли", "ль"];

/// Reports whether a written word is a particle.
#[must_use]
pub fn is_particle(written: &str) -> bool {
    PARTICLES.contains(&written.to_lowercase().as_str())
}

/// Reports whether a written word denies.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::particle::denies;
///
/// assert!(denies("не"));
/// assert!(denies("ни"));
/// assert!(!denies("же"));
/// ```
#[must_use]
pub fn denies(written: &str) -> bool {
    DENYING.contains(&written.to_lowercase().as_str())
}

/// Reports whether a written word builds a form of a verb.
#[must_use]
pub fn forms(written: &str) -> bool {
    FORMATIVE.contains(&written.to_lowercase().as_str())
}

/// Reports whether a written word leans on what stands before it.
#[must_use]
pub fn leans(written: &str) -> bool {
    ENCLITIC.contains(&written.to_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_particle_is_named_as_one() {
        assert!(is_particle("же"));
        assert!(is_particle("Не"));
        assert!(!is_particle("стол"));
    }

    #[test]
    fn every_denying_particle_and_every_enclitic_is_a_particle() {
        for held in DENYING.iter().chain(ENCLITIC) {
            assert!(is_particle(held), "{held} is no particle");
        }
    }

    #[test]
    fn a_denying_particle_is_named_as_one() {
        assert!(denies("не"));
        assert!(denies("ни"));
        assert!(!denies("же"));
    }

    #[test]
    fn an_enclitic_is_named_as_one() {
        assert!(leans("ли"));
        assert!(leans("бы"));
        assert!(!leans("не"));
    }

    #[test]
    fn a_formative_particle_is_a_particle() {
        for held in FORMATIVE {
            assert!(is_particle(held), "{held} is no particle");
        }
    }

    #[test]
    fn a_formative_particle_is_named_as_one() {
        assert!(forms("бы"));
        assert!(forms("пусть"));
        assert!(!forms("же"));
    }

    #[test]
    fn nothing_both_denies_and_leans() {
        assert!(!DENYING.iter().any(|held| leans(held)));
    }
}
