// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! How a Russian verb changes, stated as a system rather than looked up.
//!
//! A verb states person and number in the present and the future, gender and
//! number in the past, and person in the imperative. Which endings it takes is
//! settled by its conjugation, and § 44 of the 1956 code settles the
//! conjugation from the infinitive — by its last letters for the general case,
//! and by the closed lists it names against them.
//!
//! § 44 reads only verbs with unstressed personal endings; that is the whole
//! reach of the rule. A verb that stresses its endings shows its conjugation
//! in them — `несёшь` beside `кричишь` — and its infinitive does not tell it:
//! `спать` conjugates `спишь` where `читать` conjugates `читаешь`. Such a verb
//! is settled here only where the code itself states it, as its additional
//! rule states `спать, спишь`; the rest are the dictionary's to state, and the
//! classes in [`class`] refuse to build forms for the ones they are told of.
//!
//! The paradigm runs both ways. [`endings`] builds a form from a cell;
//! [`reading`] takes a written form and names the cells it could have come
//! from, which is the direction a reader goes.

pub mod class;
pub mod endings;
pub mod imperative;
pub mod index;
pub mod inflect;
pub mod participle;
pub mod past;
pub mod reading;
pub mod reflexive;
pub mod stated;
pub mod stems;

/// The set of personal endings a verb takes in the present and the future.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Conjugation {
    /// The first conjugation: `-ешь`, `-ет`, `-ут`.
    First,
    /// The second conjugation: `-ишь`, `-ит`, `-ат`.
    Second,
    /// A verb that takes endings from both sets, as `хотеть` and `бежать` do.
    Mixed,
    /// A verb whose endings belong to no set, as `есть` and `дать`.
    Irregular
}

/// The verbs in `-ить` that nonetheless take the first conjugation.
const FIRST_IN_IT: &[&str] = &["брить", "стелить", "зиждиться", "почить"];

/// The verbs in `-еть` that take the second conjugation.
///
/// The six § 44 names, no more: `вертеть`, `видеть`, `зависеть`, `обидеть`,
/// `смотреть`, `терпеть`. `ненавидеть` needs no entry, because it is `видеть`
/// under prefixes and the paragraph's additional rule already reads a prefixed
/// verb by the verb inside it.
const SECOND_IN_ET: &[&str] = &[
    "смотреть",
    "видеть",
    "обидеть",
    "терпеть",
    "вертеть",
    "зависеть"
];

/// The verbs in `-ать` that take the second conjugation.
///
/// The first four are § 44's own list. `спать` stands beside them because the
/// paragraph's additional rule states it in as many words — `выспаться,
/// выспишься (ср. спать, спишь) — II спряжения` — reaching past the unstressed
/// endings the numbered points confine themselves to.
const SECOND_IN_AT: &[&str] = &["гнать", "держать", "дышать", "слышать", "спать"];

/// The verbs that draw endings from both sets.
const MIXED: &[&str] = &["хотеть", "бежать", "чтить"];

/// The verbs that stand outside both sets.
const IRREGULAR: &[&str] = &["есть", "дать"];

/// Names the conjugation § 44 reads off this infinitive.
///
/// The tests are ordered as the language orders them: the closed lists first,
/// because they exist precisely to overrule the ending, and the ending after.
/// A prefix does not change a conjugation — `посмотреть` conjugates as
/// `смотреть` — so every list is matched against the tail of the infinitive.
///
/// The answer carries § 44's own scope. For a verb with unstressed personal
/// endings, or one the code states by name, this is the verb's conjugation.
/// For a verb that stresses its endings the infinitive does not carry the
/// fact, and what comes back is the paragraph's general case — the reading
/// the shape would get, not a statement about the verb; [`class::of`] holds
/// the verbs known to be misread so, and answers [`class::Class::Listed`] for
/// them so that no form is built on the misreading.
///
/// # Examples
///
/// ```
/// use rusem::grammar::conjugation::{Conjugation, of};
///
/// assert_eq!(of("читать"), Conjugation::First);
/// assert_eq!(of("говорить"), Conjugation::Second);
/// assert_eq!(of("посмотреть"), Conjugation::Second);
/// assert_eq!(of("брить"), Conjugation::First);
/// assert_eq!(of("хотеть"), Conjugation::Mixed);
/// ```
#[must_use]
pub fn of(infinitive: &str) -> Conjugation {
    if listed(infinitive, IRREGULAR) {
        return Conjugation::Irregular;
    }
    if listed(infinitive, MIXED) {
        return Conjugation::Mixed;
    }
    if listed(infinitive, FIRST_IN_IT) {
        return Conjugation::First;
    }
    if listed(infinitive, SECOND_IN_ET) || listed(infinitive, SECOND_IN_AT) {
        return Conjugation::Second;
    }
    if infinitive.ends_with("ить") {
        return Conjugation::Second;
    }

    Conjugation::First
}

/// The prefixes a Russian verb takes.
///
/// The scan below takes the first match, so a prefix that opens with another
/// must stand before it: `недо-` before `не-`, `подо-` before `под-`, `со-`
/// before `с-`. Nothing else about the order matters, and the test beside
/// the list holds that invariant against future insertions.
const PREFIXES: &[&str] = &[
    "недо", "обез", "пред", "разо", "рас", "роз", "рос", "надо", "подо", "пере", "пре", "при",
    "про", "раз", "изо", "ото", "обо", "воз", "вос", "взо", "вне", "над", "низ", "нис", "под",
    "без", "бес", "вз", "вс", "вы", "до", "за", "из", "ис", "на", "не", "об", "от", "по", "со",
    "в", "о", "с", "у"
];

/// Reports whether the infinitive is one of the verbs in a list, bare or
/// under prefixes.
///
/// A tail match alone is not enough: `любить` ends with `бить` and is not a
/// prefixed `бить`, because `лю` is not a prefix. So what stands before the
/// tail is required to be prefixes and nothing else, stripped one at a time
/// because Russian stacks them — `понавыдумывать` carries three.
pub(crate) fn listed(infinitive: &str, table: &[&str]) -> bool {
    table.iter().any(|held| {
        infinitive
            .strip_suffix(held)
            .is_some_and(|head| all_prefixes(head, opens_iotated(held)))
    })
}

/// Reports whether a verb opens with a letter § 70 guards with the hard sign.
fn opens_iotated(verb: &str) -> bool {
    verb.chars()
        .next()
        .is_some_and(|first| matches!(first, 'е' | 'ё' | 'ю' | 'я'))
}

/// Reports whether what stands before a verb is prefixes and nothing else.
///
/// A prefix ending in a consonant that meets `е`, `ё`, `ю` or `я` is written
/// with a hard sign after it — `съесть`, `объехать`, § 70 — and the sign
/// belongs to the prefix rather than to the verb, so it is stripped along with
/// it. The sign is not optional there, so its absence proves the consonant
/// belongs to the root: `сесть` is not `с` and `есть`, because that word is
/// written `съесть`.
///
/// The scan terminates because no prefix is empty — the test beside the list
/// holds that — so every strip shortens what is left.
fn all_prefixes(head: &str, iotated: bool) -> bool {
    let mut left = match head.strip_suffix('ъ') {
        Some(bare) => bare,
        None if iotated
            && head
                .chars()
                .last()
                .is_some_and(|last| !crate::alphabet::is_vowel(last)) =>
        {
            return false;
        }
        None => head
    };

    while !left.is_empty() {
        let Some(shorter) = PREFIXES.iter().find_map(|held| left.strip_prefix(held)) else {
            return false;
        };
        left = shorter;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ending_settles_the_general_case() {
        assert_eq!(of("читать"), Conjugation::First);
        assert_eq!(of("говорить"), Conjugation::Second);
    }

    #[test]
    fn the_closed_lists_overrule_the_ending() {
        assert_eq!(of("брить"), Conjugation::First);
        assert_eq!(of("смотреть"), Conjugation::Second);
        assert_eq!(of("держать"), Conjugation::Second);
    }

    #[test]
    fn a_prefix_does_not_change_a_conjugation() {
        assert_eq!(of("посмотреть"), Conjugation::Second);
        assert_eq!(of("побрить"), Conjugation::First);
        assert_eq!(of("прочитать"), Conjugation::First);
    }

    #[test]
    fn the_verbs_outside_both_sets_are_named() {
        assert_eq!(of("хотеть"), Conjugation::Mixed);
        assert_eq!(of("бежать"), Conjugation::Mixed);
        assert_eq!(of("есть"), Conjugation::Irregular);
        assert_eq!(of("съесть"), Conjugation::Irregular);
    }

    #[test]
    fn the_code_states_the_stressed_verb_it_names() {
        assert_eq!(of("спать"), Conjugation::Second);
        assert_eq!(of("поспать"), Conjugation::Second);
    }

    #[test]
    fn a_prefixed_hater_is_read_by_the_verb_inside_it() {
        assert_eq!(of("ненавидеть"), Conjugation::Second);
    }

    #[test]
    fn a_missing_hard_sign_refuses_the_prefix_reading() {
        assert_eq!(of("сесть"), Conjugation::First);
        assert_eq!(of("поесть"), Conjugation::Irregular);
        assert_eq!(of("надоесть"), Conjugation::Irregular);
    }

    #[test]
    fn no_prefix_stands_after_one_that_opens_it() {
        for (at, held) in PREFIXES.iter().enumerate() {
            assert!(!held.is_empty(), "an empty prefix would strip nothing");
            for earlier in &PREFIXES[..at] {
                assert!(
                    !held.starts_with(earlier),
                    "{held} would never be reached past {earlier}"
                );
            }
        }
    }
}
