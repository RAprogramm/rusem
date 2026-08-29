// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! How a Russian verb changes, stated as a system rather than looked up.
//!
//! A verb states person and number in the present and the future, gender and
//! number in the past, and person in the imperative. Which endings it takes is
//! settled by its conjugation, and the conjugation is settled by the
//! infinitive — by its last letters for the general case, and by a closed list
//! of verbs for everything the general case gets wrong.
//!
//! The paradigm runs both ways. [`endings`] builds a form from a cell;
//! [`reading`] takes a written form and names the cells it could have come
//! from, which is the direction a reader goes.

pub mod class;
pub mod endings;
pub mod imperative;
pub mod inflect;
pub mod participle;
pub mod past;
pub mod reading;
pub mod reflexive;
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
const SECOND_IN_ET: &[&str] = &[
    "смотреть",
    "видеть",
    "ненавидеть",
    "обидеть",
    "терпеть",
    "вертеть",
    "зависеть"
];

/// The verbs in `-ать` that take the second conjugation.
const SECOND_IN_AT: &[&str] = &["гнать", "держать", "дышать", "слышать"];

/// The verbs that draw endings from both sets.
const MIXED: &[&str] = &["хотеть", "бежать", "чтить"];

/// The verbs that stand outside both sets.
const IRREGULAR: &[&str] = &["есть", "дать"];

/// Names the conjugation a verb belongs to, given its infinitive.
///
/// The tests are ordered as the language orders them: the closed lists first,
/// because they exist precisely to overrule the ending, and the ending after.
/// A prefix does not change a conjugation — `посмотреть` conjugates as
/// `смотреть` — so every list is matched against the tail of the infinitive.
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
    table
        .iter()
        .any(|held| infinitive.strip_suffix(held).is_some_and(all_prefixes))
}

/// Reports whether what stands before a verb is prefixes and nothing else.
///
/// A prefix that meets `е`, `ю` or `я` is written with a hard sign after it —
/// `съесть`, `объехать`, § 70 — and the sign belongs to the prefix rather than
/// to the verb, so it is stripped along with it.
fn all_prefixes(mut head: &str) -> bool {
    head = head.strip_suffix('ъ').unwrap_or(head);

    while !head.is_empty() {
        let Some(shorter) = PREFIXES
            .iter()
            .find_map(|held| head.strip_prefix(held))
            .filter(|shorter| shorter.len() < head.len())
        else {
            return false;
        };
        head = shorter;
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
    fn no_prefix_stands_after_one_that_opens_it() {
        for (at, held) in PREFIXES.iter().enumerate() {
            for earlier in &PREFIXES[..at] {
                assert!(
                    !held.starts_with(earlier),
                    "{held} would never be reached past {earlier}"
                );
            }
        }
    }
}
