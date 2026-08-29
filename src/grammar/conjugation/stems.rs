// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The two stems of a verb, got from the infinitive by its class.
//!
//! The past is built on one stem and the present on another, and the class
//! says how the second is got from the first. `читать` cuts to `чита-` and
//! adds a glide; `рисовать` cuts `-ова-` and puts `-у-`; `любить` drops the
//! `-и-` and leaves the stem bare; `писать` drops the `-а-` and swaps the
//! consonant under it.
//!
//! One cell of the present is built apart from the rest: the first person
//! singular of the bare class, where the stem swaps or takes an `л` and the
//! other five cells do not. `люблю` beside `любишь`, `вожу` beside `водишь`.
//!
//! A verb of the listed class has no answer here. Its stems are facts about
//! that verb, not consequences of any rule, and they are told rather than
//! derived.

use crate::{
    grammar::{conjugation::class::Class, declension::spelling},
    morphemics::alternation::{self, Kind}
};

/// The glide a stem takes when the class adds one rather than cutting.
const GLIDE: char = 'й';

/// The stem the past tense is built on.
///
/// It is the infinitive without its `-ть`, `-ти` or `-чь`, which is what the
/// past has always been built on: `читать` gives `чита-` and `читал`, `нести`
/// gives `нес-` and `нёс`.
///
/// # Examples
///
/// ```
/// use rusem::grammar::conjugation::stems::past;
///
/// assert_eq!(past("читать").as_deref(), Some("чита"));
/// assert_eq!(past("нести").as_deref(), Some("нес"));
/// ```
#[must_use]
pub fn past(infinitive: &str) -> Option<String> {
    for held in ["ться", "ть", "тись", "ти", "чься", "чь"] {
        if let Some(stem) = infinitive.strip_suffix(held) {
            return Some(String::from(stem));
        }
    }

    None
}

/// The stem the present tense is built on.
///
/// The class settles it. A verb of the listed class comes back as [`None`]:
/// there is no rule to apply, and inventing one would be worse than saying so.
///
/// # Examples
///
/// ```
/// use rusem::grammar::conjugation::{class, stems::present};
///
/// assert_eq!(
///     present("читать", class::of("читать")).as_deref(),
///     Some("читай")
/// );
/// assert_eq!(
///     present("рисовать", class::of("рисовать")).as_deref(),
///     Some("рису")
/// );
/// assert_eq!(
///     present("крикнуть", class::of("крикнуть")).as_deref(),
///     Some("крикн")
/// );
/// assert_eq!(
///     present("любить", class::of("любить")).as_deref(),
///     Some("люб")
/// );
/// assert_eq!(
///     present("писать", class::of("писать")).as_deref(),
///     Some("пиш")
/// );
/// assert_eq!(present("тереть", class::of("тереть")), None);
/// ```
#[must_use]
pub fn present(infinitive: &str, class: Class) -> Option<String> {
    let stem = past(infinitive)?;

    match class {
        Class::Glided => Some(stem + &GLIDE.to_string()),
        Class::Suffixed => suffixed(&stem),
        Class::Dropped => stem.strip_suffix('у').map(String::from),
        Class::Bare => cut_vowel(&stem),
        Class::Swapped => cut_vowel(&stem).map(|held| swap_last(&held)),
        Class::Consonantal => Some(stem),
        Class::Listed => None
    }
}

/// The present stem of the first person singular, where the bare class differs
/// from its own other five cells.
///
/// `любить` conjugates on `люб-` and puts `люблю`; `водить` conjugates on
/// `вод-` and puts `вожу`. Every other class puts the same stem here as
/// everywhere else.
///
/// # Examples
///
/// ```
/// use rusem::grammar::conjugation::{class, stems::first_person};
///
/// assert_eq!(first_person("люб", class::of("любить")).as_str(), "любл");
/// assert_eq!(first_person("вод", class::of("водить")).as_str(), "вож");
/// assert_eq!(first_person("читай", class::of("читать")).as_str(), "читай");
/// ```
#[must_use]
pub fn first_person(present_stem: &str, class: Class) -> String {
    if !matches!(class, Class::Bare) {
        return String::from(present_stem);
    }
    let Some(last) = present_stem.chars().last() else {
        return String::from(present_stem);
    };

    if alternation::takes_epenthesis(last) {
        let mut held = String::from(present_stem);
        held.push(alternation::EPENTHESIS);
        return held;
    }

    swap_last(present_stem)
}

/// The stem of a verb in `-овать` or `-евать`.
///
/// The suffix is not cut but replaced: `рисовать` puts `-у-` where `-ова-`
/// stood, `воевать` puts `-ю-` where `-ева-` stood.
///
/// The `-ю-` lands on whatever the stem ends in, and the alphabet bends it
/// there like anywhere else: after `ц` it is written `у`, which is why
/// `танцевать` conjugates on `танцу-` and not on `танцю-`.
fn suffixed(stem: &str) -> Option<String> {
    if let Some(head) = stem.strip_suffix("ова") {
        return Some(String::from(head) + "у");
    }

    stem.strip_suffix("ева")
        .map(|head| String::from(head) + &spelling::fitted(head, "ю", false))
}

/// The stem without the vowel the infinitive put before its ending.
fn cut_vowel(stem: &str) -> Option<String> {
    let last = stem.chars().last()?;
    if !crate::alphabet::is_vowel(last) {
        return Some(String::from(stem));
    }

    Some(stem.chars().take(stem.chars().count() - 1).collect())
}

/// The stem with its last consonant swapped for the one the present takes.
///
/// A cluster swaps whole and is tried first: `иск-` gives `ищ-`, which no
/// letter-by-letter rule would find.
fn swap_last(stem: &str) -> String {
    if let Some(held) = alternation::clustered(stem) {
        return held;
    }
    let Some(last) = stem.chars().last() else {
        return String::from(stem);
    };

    let mut held: String = stem.chars().take(stem.chars().count() - 1).collect();
    held.push(alternation::swapped(last, Kind::Present));
    held
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::conjugation::class;

    fn present_of(infinitive: &str) -> Option<String> {
        present(infinitive, class::of(infinitive))
    }

    #[test]
    fn the_past_stem_is_the_infinitive_without_its_ending() {
        assert_eq!(past("читать").as_deref(), Some("чита"));
        assert_eq!(past("нести").as_deref(), Some("нес"));
        assert_eq!(past("печь").as_deref(), Some("пе"));
        assert_eq!(past("умываться").as_deref(), Some("умыва"));
    }

    #[test]
    fn a_glided_verb_takes_a_glide() {
        assert_eq!(present_of("читать").as_deref(), Some("читай"));
        assert_eq!(present_of("гулять").as_deref(), Some("гуляй"));
        assert_eq!(present_of("уметь").as_deref(), Some("умей"));
    }

    #[test]
    fn a_suffixed_verb_replaces_its_suffix() {
        assert_eq!(present_of("рисовать").as_deref(), Some("рису"));
        assert_eq!(present_of("воевать").as_deref(), Some("вою"));
        assert_eq!(present_of("требовать").as_deref(), Some("требу"));
    }

    #[test]
    fn a_dropped_verb_loses_the_u_of_its_infinitive() {
        assert_eq!(present_of("крикнуть").as_deref(), Some("крикн"));
        assert_eq!(present_of("вернуть").as_deref(), Some("верн"));
    }

    #[test]
    fn a_bare_verb_loses_its_vowel_and_keeps_its_consonant() {
        assert_eq!(present_of("любить").as_deref(), Some("люб"));
        assert_eq!(present_of("водить").as_deref(), Some("вод"));
        assert_eq!(present_of("смотреть").as_deref(), Some("смотр"));
    }

    #[test]
    fn a_swapping_verb_swaps_throughout() {
        assert_eq!(present_of("писать").as_deref(), Some("пиш"));
        assert_eq!(present_of("искать").as_deref(), Some("ищ"));
        assert_eq!(present_of("плакать").as_deref(), Some("плач"));
    }

    #[test]
    fn a_consonant_stem_is_the_past_stem_itself() {
        assert_eq!(present_of("нести").as_deref(), Some("нес"));
        assert_eq!(present_of("везти").as_deref(), Some("вез"));
    }

    #[test]
    fn a_listed_verb_has_no_derived_stem() {
        assert_eq!(present_of("тереть"), None);
        assert_eq!(present_of("мыть"), None);
        assert_eq!(present_of("давать"), None);
    }

    #[test]
    fn a_labial_takes_a_letter_in_the_first_person_alone() {
        assert_eq!(first_person("люб", class::of("любить")), "любл");
        assert_eq!(first_person("сп", class::of("спать")), "сп");
        assert_eq!(first_person("готов", class::of("готовить")), "готовл");
    }

    #[test]
    fn a_swapping_consonant_swaps_in_the_first_person_alone() {
        assert_eq!(first_person("вод", class::of("водить")), "вож");
        assert_eq!(first_person("нос", class::of("носить")), "нош");
        assert_eq!(first_person("плат", class::of("платить")), "плач");
    }

    #[test]
    fn no_other_class_changes_its_stem_in_the_first_person() {
        assert_eq!(first_person("читай", class::of("читать")), "читай");
        assert_eq!(first_person("пиш", class::of("писать")), "пиш");
        assert_eq!(first_person("нес", class::of("нести")), "нес");
    }
}
