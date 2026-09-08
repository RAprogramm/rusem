// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The interjections, which say a feeling and name nothing.
//!
//! An interjection is not a part of the sentence and stands outside its
//! grammar: it agrees with nothing, governs nothing, and is parted from what
//! follows by a comma or an exclamation mark (§ 155). That is the whole of
//! what a checker needs from it — and the reason it must be recognised, since
//! a word that names nothing has no sense to look up and no case to agree in.
//!
//! The grammars give three kinds and set a fourth beside them, and [`kind`]
//! names which. An
//! **emotional** one says a feeling — `увы`, `ура`. A **commanding** one calls
//! or drives — `эй`, `брысь`. A **courteous** one carries a formula whole —
//! `спасибо`, `здравствуйте` — and is an interjection because the word no
//! longer means what its parts mean.
//!
//! The fourth, **sound-naming**, is not an interjection to every grammar: a
//! word like `бац` imitates rather than expresses. It is listed here because
//! the code of 1956 punctuates it the same way and a checker meets it in the
//! same place.

/// Which kind an interjection is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Kind {
    /// `увы`, `ура`: a feeling said outright.
    Feeling,
    /// `эй`, `брысь`: a call or a drive.
    Calling,
    /// `спасибо`: a formula carried whole.
    Courtesy,
    /// `бац`: a sound named rather than a feeling said.
    Sounding
}

impl Kind {
    /// The interjections of feeling.
    ///
    /// `о` is here because § 157 names it one. It is written the same as the
    /// preposition `о`, and nothing but the sentence tells the two apart: the
    /// preposition governs a case and the interjection governs nothing.
    pub const FEELING: &[&str] = &[
        "ай",
        "ах",
        "ба",
        "батюшки",
        "боже",
        "браво",
        "брр",
        "господи",
        "о",
        "ого",
        "ой",
        "ох",
        "тьфу",
        "увы",
        "ура",
        "уф",
        "фи",
        "фу",
        "ха",
        "хм",
        "эх"
    ];

    /// The interjections that call, answer or drive.
    pub const CALLING: &[&str] = &[
        "ага",
        "айда",
        "алло",
        "ау",
        "брысь",
        "кыш",
        "марш",
        "ну",
        "стоп",
        "тсс",
        "цыц",
        "чур",
        "эге",
        "эй"
    ];

    /// The interjections of courtesy, which carry a formula whole.
    ///
    /// `извините`, `простите` and `прощайте` are not here. Each is also the
    /// imperative of a live verb — `простите меня` commands — and no list tells
    /// the two apart. The three below have no such reading left.
    pub const COURTESY: &[&str] = &["здравствуйте", "пожалуйста", "спасибо"];

    /// The words that name a sound rather than say a feeling.
    pub const SOUNDING: &[&str] = &[
        "бабах",
        "бац",
        "бух",
        "дзинь",
        "кап",
        "хлоп",
        "хрусть",
        "чмок",
        "шасть",
        "шлёп"
    ];
}

/// Every kind, with the words in it.
const ALL: &[(Kind, &[&str])] = &[
    (Kind::Feeling, Kind::FEELING),
    (Kind::Calling, Kind::CALLING),
    (Kind::Courtesy, Kind::COURTESY),
    (Kind::Sounding, Kind::SOUNDING)
];

/// The kind a written interjection is, or nothing when it is none.
///
/// One kind at most: no word here belongs to two.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::interjection::{Kind, kind};
///
/// assert_eq!(kind("увы"), Some(Kind::Feeling));
/// assert_eq!(kind("эй"), Some(Kind::Calling));
/// assert_eq!(kind("бац"), Some(Kind::Sounding));
/// assert_eq!(kind("стол"), None);
/// ```
#[must_use]
pub fn kind(written: &str) -> Option<Kind> {
    let held = written.to_lowercase();

    ALL.iter()
        .find(|(_, words)| words.contains(&held.as_str()))
        .map(|(kind, _)| *kind)
}

/// Reports whether a written word imitates a sound rather than says a feeling.
///
/// The one kind the grammars set apart: `бац` is not an interjection to every
/// grammar, and a caller that follows the narrower reading asks this.
#[must_use]
pub fn imitates(written: &str) -> bool {
    kind(written) == Some(Kind::Sounding)
}

/// Reports whether a written word is an interjection.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::interjection::is_interjection;
///
/// assert!(is_interjection("увы"));
/// assert!(is_interjection("Эй"));
/// assert!(!is_interjection("стол"));
/// ```
#[must_use]
pub fn is_interjection(written: &str) -> bool {
    kind(written).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_is_sorted_and_holds_no_word_twice() {
        let mut every: Vec<&str> = ALL.iter().flat_map(|(_, words)| *words).copied().collect();
        let counted = every.len();
        every.sort_unstable();
        every.dedup();

        assert_eq!(every.len(), counted, "an interjection is listed twice");
    }

    #[test]
    fn an_interjection_is_named_as_one() {
        assert!(is_interjection("ах"));
        assert!(is_interjection("эй"));
        assert!(is_interjection("бац"));
        assert!(is_interjection("УВЫ"));
    }

    #[test]
    fn every_listed_interjection_names_its_kind() {
        for (named, words) in ALL {
            for held in *words {
                assert_eq!(kind(held), Some(*named), "{held}");
            }
        }
    }

    #[test]
    fn only_a_sound_naming_word_imitates() {
        assert!(imitates("бац"));
        assert!(imitates("ХЛОП"));
        assert!(!imitates("увы"));
        assert!(!imitates("стол"));
    }

    #[test]
    fn a_word_that_names_something_is_no_interjection() {
        assert!(!is_interjection("стол"));
        assert!(!is_interjection(""));
    }

    #[test]
    fn every_interjection_is_russian_and_small() {
        for (_, words) in ALL {
            for held in *words {
                assert!(
                    held.chars().all(crate::alphabet::is_letter),
                    "{held} is no Russian word"
                );
                assert_eq!(*held, held.to_lowercase());
            }
        }
    }
}
