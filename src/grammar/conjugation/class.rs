// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The classes a Russian verb belongs to.
//!
//! A verb has two stems, not one. `читать` is built on `чита-` in the past and
//! `читай-` in the present; `писать` on `писа-` and `пиш-`; `печь` on `пек-`
//! and `печ-`. No rule over the infinitive alone finds the second stem, which
//! is why an engine that knows only the infinitive has to be told every form.
//!
//! What does find it is the class. Russian verbs fall into five productive
//! classes — the ones a new verb joins — and a dozen or so unproductive groups
//! that are closed and have to be listed. The class states how the present
//! stem is got from the infinitive, whether the stem swaps a consonant, and
//! which set of personal endings follows.
//!
//! With the class in hand the whole paradigm follows from the infinitive, and
//! the exceptions stop being exceptions: they become the unproductive groups,
//! which is what they are.

use crate::grammar::conjugation::Conjugation;

/// How a verb builds its present stem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Class {
    /// `читать` — `читаj-`. The infinitive stem takes a glide.
    Glided,
    /// `рисовать` — `рису-`. `-ова-` and `-ева-` give way to `-у-` and `-ю-`.
    Suffixed,
    /// `крикнуть` — `крикн-`. The `-у-` of the infinitive drops.
    Dropped,
    /// `любить` — `люб-`. The `-и-` drops and the stem may swap or take a
    /// letter in the first person alone.
    Bare,
    /// `писать` — `пиш-`. The `-а-` drops and the stem swaps a consonant
    /// throughout.
    Swapped,
    /// `нести` — `нес-`, `печь` — `пек-`. The stem is a consonant, and a velar
    /// at the end of it swaps before every front vowel.
    Consonantal,
    /// `тереть` — `тр-`, `мыть` — `моj-`. Closed groups whose stem is not got
    /// from the infinitive by any rule and has to be listed.
    Listed
}

impl Class {
    /// Which set of personal endings the class takes.
    ///
    /// Four of the seven classes take the first conjugation whatever else they
    /// do. Only the bare class takes the second, and only the listed class has
    /// no answer of its own: `бежать` and `есть` are in it, and they are told
    /// their endings rather than deriving them.
    #[must_use]
    pub const fn conjugation(self) -> Option<Conjugation> {
        match self {
            Self::Glided | Self::Suffixed | Self::Dropped | Self::Swapped | Self::Consonantal => {
                Some(Conjugation::First)
            }
            Self::Bare => Some(Conjugation::Second),
            Self::Listed => None
        }
    }

    /// Reports whether the class is one a new Russian verb can join.
    ///
    /// Five are: every verb coined in the last century took one of them. The
    /// other two are closed, and a verb is in them only because it always was.
    #[must_use]
    pub const fn is_productive(self) -> bool {
        matches!(
            self,
            Self::Glided | Self::Suffixed | Self::Dropped | Self::Bare
        )
    }
}

/// The verbs in `-ать` whose stem swaps rather than taking a glide.
const SWAPPING: &[&str] = &[
    "писать",
    "плясать",
    "искать",
    "казать",
    "резать",
    "вязать",
    "мазать",
    "низать",
    "пахать",
    "махать",
    "плакать",
    "скакать",
    "прятать",
    "шептать",
    "клеветать",
    "роптать",
    "хлестать",
    "свистать",
    "блистать",
    "трепетать",
    "лепетать",
    "бормотать",
    "хохотать",
    "щекотать",
    "грохотать",
    "топтать",
    "колебать",
    "дремать",
    "сыпать",
    "щипать",
    "капать",
    "глодать",
    "страдать",
    "рыскать",
    "полоскать",
    "тесать",
    "чесать",
    "брызгать",
    "двигать"
];

/// The verbs whose stem no rule finds.
const UNRULY: &[&str] = &[
    "быть",
    "есть",
    "дать",
    "идти",
    "ехать",
    "хотеть",
    "бежать",
    "чтить",
    "мочь",
    "лечь",
    "сечь",
    "жечь",
    "тереть",
    "переть",
    "мереть",
    "брать",
    "звать",
    "жрать",
    "драть",
    "стлать",
    "слать",
    "мыть",
    "выть",
    "крыть",
    "рыть",
    "ныть",
    "пить",
    "бить",
    "вить",
    "лить",
    "шить",
    "жить",
    "плыть",
    "слыть",
    "давать",
    "вставать",
    "узнавать",
    "жать",
    "мять",
    "начать",
    "взять",
    "петь",
    "деть",
    "клясть",
    "красть",
    "расти",
    "сесть",
    "класть"
];

/// Names the class a verb belongs to, given its infinitive.
///
/// The closed lists come first, because they exist to overrule the ending, and
/// the ending decides everything else. A prefix does not change a class, so
/// every list is matched against the tail of the infinitive.
///
/// # Examples
///
/// ```
/// use rusem::grammar::conjugation::class::{Class, of};
///
/// assert_eq!(of("читать"), Class::Glided);
/// assert_eq!(of("рисовать"), Class::Suffixed);
/// assert_eq!(of("крикнуть"), Class::Dropped);
/// assert_eq!(of("любить"), Class::Bare);
/// assert_eq!(of("написать"), Class::Swapped);
/// assert_eq!(of("нести"), Class::Consonantal);
/// assert_eq!(of("тереть"), Class::Listed);
/// ```
#[must_use]
pub fn of(infinitive: &str) -> Class {
    if super::listed(infinitive, UNRULY) {
        return Class::Listed;
    }
    if super::listed(infinitive, SWAPPING) {
        return Class::Swapped;
    }
    if infinitive.ends_with("овать") || infinitive.ends_with("евать") {
        return Class::Suffixed;
    }
    if infinitive.ends_with("нуть") {
        return Class::Dropped;
    }
    if infinitive.ends_with("ить") {
        return Class::Bare;
    }
    if infinitive.ends_with("ти") || infinitive.ends_with("чь") {
        return Class::Consonantal;
    }
    if infinitive.ends_with("ать") || infinitive.ends_with("ять") || infinitive.ends_with("еть")
    {
        return second_or_glided(infinitive);
    }

    Class::Listed
}

/// The verbs in `-еть` and `-ать` that conjugate bare rather than glided.
///
/// These are § 44 of the code of 1956: the six verbs in `-еть` and the four in
/// `-ать` that take the second conjugation. They lose their vowel like a verb
/// in `-ить` does, so they are in the bare class and not in the glided one.
fn second_or_glided(infinitive: &str) -> Class {
    if matches!(super::of(infinitive), Conjugation::Second) {
        return Class::Bare;
    }

    Class::Glided
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_five_productive_classes_are_found_by_the_ending() {
        assert_eq!(of("читать"), Class::Glided);
        assert_eq!(of("уметь"), Class::Glided);
        assert_eq!(of("рисовать"), Class::Suffixed);
        assert_eq!(of("воевать"), Class::Suffixed);
        assert_eq!(of("крикнуть"), Class::Dropped);
        assert_eq!(of("любить"), Class::Bare);
    }

    #[test]
    fn a_swapping_verb_is_named_rather_than_derived() {
        assert_eq!(of("писать"), Class::Swapped);
        assert_eq!(of("искать"), Class::Swapped);
        assert_eq!(of("читать"), Class::Glided);
    }

    #[test]
    fn a_consonant_stem_is_found_by_its_infinitive() {
        assert_eq!(of("нести"), Class::Consonantal);
        assert_eq!(of("везти"), Class::Consonantal);
        assert_eq!(of("печь"), Class::Consonantal);
    }

    #[test]
    fn the_verbs_of_paragraph_forty_four_conjugate_bare() {
        assert_eq!(of("смотреть"), Class::Bare);
        assert_eq!(of("держать"), Class::Bare);
        assert_eq!(of("слышать"), Class::Bare);
    }

    #[test]
    fn a_prefix_does_not_change_a_class() {
        assert_eq!(of("написать"), Class::Swapped);
        assert_eq!(of("прочитать"), Class::Glided);
        assert_eq!(of("посмотреть"), Class::Bare);
        assert_eq!(of("принести"), Class::Consonantal);
    }

    #[test]
    fn a_verb_no_rule_reaches_is_listed() {
        assert_eq!(of("тереть"), Class::Listed);
        assert_eq!(of("мыть"), Class::Listed);
        assert_eq!(of("давать"), Class::Listed);
        assert_eq!(of("съесть"), Class::Listed);
    }

    #[test]
    fn every_class_but_the_listed_one_names_its_endings() {
        assert_eq!(Class::Glided.conjugation(), Some(Conjugation::First));
        assert_eq!(Class::Bare.conjugation(), Some(Conjugation::Second));
        assert_eq!(Class::Listed.conjugation(), None);
    }

    #[test]
    fn the_closed_classes_are_named_as_closed() {
        assert!(Class::Glided.is_productive());
        assert!(!Class::Swapped.is_productive());
        assert!(!Class::Listed.is_productive());
    }
}
