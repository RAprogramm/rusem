// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The personal endings of the present and the simple future.
//!
//! Two sets, and the conjugation of the verb says which one it takes. Within a
//! set the first person singular and the third person plural bend to the stem
//! — `читаю` beside `несу`, `читают` beside `несут` — and the rest of the
//! first conjugation bends to the stress: `читаешь` beside `несёшь`.
//!
//! The past has no person and is stated beside this, in [`super::past`].

use crate::grammar::{Number, Person, conjugation::Conjugation, stem::Stem};

/// The six personal endings of one verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Endings {
    /// я
    pub first_singular:  &'static str,
    /// ты
    pub second_singular: &'static str,
    /// он
    pub third_singular:  &'static str,
    /// мы
    pub first_plural:    &'static str,
    /// вы
    pub second_plural:   &'static str,
    /// они
    pub third_plural:    &'static str
}

impl Endings {
    /// The ending for one person and number.
    #[must_use]
    pub const fn of(&self, person: Person, number: Number) -> &'static str {
        match (person, number) {
            (Person::First, Number::Singular) => self.first_singular,
            (Person::Second, Number::Singular) => self.second_singular,
            (Person::Third, Number::Singular) => self.third_singular,
            (Person::First, Number::Plural) => self.first_plural,
            (Person::Second, Number::Plural) => self.second_plural,
            (Person::Third, Number::Plural) => self.third_plural
        }
    }
}

/// The endings a verb takes, given its conjugation, its stem and its stress.
///
/// The stress is asked for because the first conjugation writes `е` where the
/// ending is unstressed and `ё` where it is stressed, and nothing in the
/// spelling of the stem tells which. A reader who does not know the stress
/// asks for both and keeps them both.
///
/// A verb that stands outside both sets has no table, and the answer is
/// [`None`]: `есть` and `дать` are stated form by form, not derived.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Number, Person,
///     conjugation::{Conjugation, endings},
///     stem::Stem
/// };
///
/// let first = endings::table(Conjugation::First, Stem::Hard, false)
///     .expect("the first conjugation has a table");
/// assert_eq!(first.of(Person::Second, Number::Singular), "ешь");
/// assert_eq!(first.of(Person::Third, Number::Plural), "ут");
///
/// let second = endings::table(Conjugation::Second, Stem::Soft, false)
///     .expect("the second conjugation has a table");
/// assert_eq!(second.of(Person::Third, Number::Plural), "ят");
/// ```
#[must_use]
pub const fn table(conjugation: Conjugation, stem: Stem, stressed: bool) -> Option<Endings> {
    match conjugation {
        Conjugation::First => Some(first(stem, stressed)),
        Conjugation::Second => Some(second(stem)),
        Conjugation::Mixed => Some(mixed(stem)),
        Conjugation::Irregular => None
    }
}

/// The first conjugation.
///
/// The stress moves `е` to `ё` in the four middle cells and nowhere else: the
/// first person singular and the third person plural have no `е` to move.
const fn first(stem: Stem, stressed: bool) -> Endings {
    let (single, third) = match stem {
        Stem::Soft => ("ю", "ют"),
        Stem::Hard => ("у", "ут")
    };
    let (second_singular, third_singular, first_plural, second_plural) = if stressed {
        ("ёшь", "ёт", "ём", "ёте")
    } else {
        ("ешь", "ет", "ем", "ете")
    };

    Endings {
        first_singular: single,
        second_singular,
        third_singular,
        first_plural,
        second_plural,
        third_plural: third
    }
}

/// The second conjugation, which the stress does not bend.
const fn second(stem: Stem) -> Endings {
    let (single, third) = match stem {
        Stem::Soft => ("ю", "ят"),
        Stem::Hard => ("у", "ат")
    };

    Endings {
        first_singular:  single,
        second_singular: "ишь",
        third_singular:  "ит",
        first_plural:    "им",
        second_plural:   "ите",
        third_plural:    third
    }
}

/// The verbs that draw from both sets.
///
/// `хотеть` takes the first conjugation in the singular and the second in the
/// plural; `бежать` takes the second everywhere but the third person plural.
/// The table below is the singular of the first and the plural of the second,
/// which is `хотеть`; `бежать` is read off the same table with its own third
/// person plural, and the reader that needs the difference asks for the verb.
const fn mixed(stem: Stem) -> Endings {
    let single = match stem {
        Stem::Soft => "ю",
        Stem::Hard => "у"
    };

    Endings {
        first_singular:  single,
        second_singular: "ешь",
        third_singular:  "ет",
        first_plural:    "им",
        second_plural:   "ите",
        third_plural:    "ят"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table_of(conjugation: Conjugation, stem: Stem, stressed: bool) -> Endings {
        table(conjugation, stem, stressed).expect("a table")
    }

    #[test]
    fn the_first_conjugation_writes_e_when_the_ending_is_unstressed() {
        let held = table_of(Conjugation::First, Stem::Hard, false);
        assert_eq!(held.of(Person::Second, Number::Singular), "ешь");
        assert_eq!(held.of(Person::First, Number::Plural), "ем");
    }

    #[test]
    fn the_first_conjugation_writes_jo_when_the_ending_is_stressed() {
        let held = table_of(Conjugation::First, Stem::Hard, true);
        assert_eq!(held.of(Person::Second, Number::Singular), "ёшь");
        assert_eq!(held.of(Person::Third, Number::Singular), "ёт");
    }

    #[test]
    fn the_stem_bends_the_first_person_and_the_third_plural() {
        let hard = table_of(Conjugation::First, Stem::Hard, false);
        let soft = table_of(Conjugation::First, Stem::Soft, false);
        assert_eq!(hard.of(Person::First, Number::Singular), "у");
        assert_eq!(soft.of(Person::First, Number::Singular), "ю");
        assert_eq!(hard.of(Person::Third, Number::Plural), "ут");
        assert_eq!(soft.of(Person::Third, Number::Plural), "ют");
    }

    #[test]
    fn the_second_conjugation_is_not_bent_by_the_stress() {
        let quiet = table_of(Conjugation::Second, Stem::Soft, false);
        let loud = table_of(Conjugation::Second, Stem::Soft, true);
        assert_eq!(quiet, loud);
        assert_eq!(quiet.of(Person::Third, Number::Plural), "ят");
    }

    #[test]
    fn a_verb_outside_both_sets_has_no_table() {
        assert!(table(Conjugation::Irregular, Stem::Hard, false).is_none());
    }
}
