// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The past tense, which states gender and number and says nothing of person.
//!
//! The past of a Russian verb is not a conjugation at all: it descends from a
//! participle and agrees the way an adjective agrees. `он читал`, `она читала`,
//! `они читали` — the same four cells whatever the verb, and one of them is
//! bare.
//!
//! The bare cell is the whole difficulty. `читал` takes `л` and `нёс` does
//! not, because a stem ending in a consonant has nowhere to put it.

use crate::grammar::{Gender, Number};

/// The four cells of the past.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Endings {
    /// он
    pub masculine: &'static str,
    /// она
    pub feminine:  &'static str,
    /// оно
    pub neuter:    &'static str,
    /// они
    pub plural:    &'static str
}

/// The past of a stem that ends in a vowel: `читал`, `читала`, `читали`.
const AFTER_VOWEL: Endings = Endings {
    masculine: "л",
    feminine:  "ла",
    neuter:    "ло",
    plural:    "ли"
};

/// The past of a stem that ends in a consonant: `нёс`, `несла`, `несли`.
///
/// The masculine has no `л`: the stem ends where the ending would begin, and
/// Russian drops it rather than let two consonants close the word.
const AFTER_CONSONANT: Endings = Endings {
    masculine: "",
    feminine:  "ла",
    neuter:    "ло",
    plural:    "ли"
};

impl Endings {
    /// The ending for one gender and number.
    ///
    /// The plural states no gender, so a gender given with it is ignored
    /// rather than refused: `они читали` is the same form for everyone.
    #[must_use]
    pub const fn of(&self, gender: Gender, number: Number) -> &'static str {
        match number {
            Number::Plural => self.plural,
            Number::Singular => match gender {
                Gender::Masculine | Gender::Common => self.masculine,
                Gender::Feminine => self.feminine,
                Gender::Neuter => self.neuter
            }
        }
    }
}

/// The past endings a stem takes.
///
/// Only one question is asked of the stem: whether it ends in a vowel. That is
/// what the `л` of the masculine hangs on, and the other three cells do not
/// care.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Gender, Number, conjugation::past};
///
/// let read = past::table("чита");
/// assert_eq!(read.of(Gender::Masculine, Number::Singular), "л");
///
/// let carried = past::table("нёс");
/// assert_eq!(carried.of(Gender::Masculine, Number::Singular), "");
/// assert_eq!(carried.of(Gender::Feminine, Number::Singular), "ла");
/// ```
#[must_use]
pub fn table(stem: &str) -> Endings {
    match stem.chars().last() {
        Some(last) if crate::alphabet::is_vowel(last) => AFTER_VOWEL,
        _ => AFTER_CONSONANT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stem_in_a_vowel_takes_the_l() {
        let held = table("чита");
        assert_eq!(held.of(Gender::Masculine, Number::Singular), "л");
        assert_eq!(held.of(Gender::Feminine, Number::Singular), "ла");
        assert_eq!(held.of(Gender::Neuter, Number::Singular), "ло");
        assert_eq!(held.of(Gender::Masculine, Number::Plural), "ли");
    }

    #[test]
    fn a_stem_in_a_consonant_drops_it_in_the_masculine_alone() {
        let held = table("нёс");
        assert_eq!(held.of(Gender::Masculine, Number::Singular), "");
        assert_eq!(held.of(Gender::Feminine, Number::Singular), "ла");
        assert_eq!(held.of(Gender::Neuter, Number::Singular), "ло");
        assert_eq!(held.of(Gender::Masculine, Number::Plural), "ли");
    }

    #[test]
    fn the_plural_states_no_gender() {
        let held = table("чита");
        assert_eq!(
            held.of(Gender::Feminine, Number::Plural),
            held.of(Gender::Neuter, Number::Plural)
        );
    }

    #[test]
    fn a_common_gender_is_read_as_the_masculine() {
        let held = table("чита");
        assert_eq!(
            held.of(Gender::Common, Number::Singular),
            held.of(Gender::Masculine, Number::Singular)
        );
    }

    #[test]
    fn an_empty_stem_is_read_as_ending_in_a_consonant() {
        assert_eq!(table("").of(Gender::Masculine, Number::Singular), "");
    }
}
