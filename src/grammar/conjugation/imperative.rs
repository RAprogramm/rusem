// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The imperative, which is built from the present stem and not the infinitive.
//!
//! `читать` gives `читай` because its present stem is `чита-`; `писать` gives
//! `пиши` because its present stem is `пиш-`. A rule reaching for the
//! infinitive would get both wrong, so this asks for the stem the present
//! tense is built on.
//!
//! Three endings, and which one is taken is settled by what the stem ends
//! with and where the stress falls.

/// The ending the imperative takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
    /// `-й`, after a vowel: `читай`, `дай`, `стой`.
    Glide,
    /// `-и`, when the ending carries the stress or two consonants close the
    /// stem: `иди`, `неси`, `помни`.
    Vowel,
    /// `-ь`, after a single consonant with the stress on the stem: `брось`,
    /// `готовь`, `встань`.
    SoftSign
}

impl Ending {
    /// The ending appended to address more than one, or one politely.
    pub const PLURAL: &str = "те";

    /// How the ending is written.
    #[must_use]
    pub const fn written(self) -> &'static str {
        match self {
            Self::Glide => "й",
            Self::Vowel => "и",
            Self::SoftSign => "ь"
        }
    }
}

/// The glide a present stem may already end in.
///
/// `читай-` is the present stem of `читать` with the imperative's own ending
/// written in it, so such a stem settles the cell as surely as a vowel does.
const GLIDE: char = 'й';

/// Reports whether the stem alone settles the ending.
///
/// A stem in a vowel takes the glide however the stress falls, a stem that
/// ends in the glide has the ending in it already, and a stem closed by two
/// consonants takes the vowel however the stress falls. Between them stands
/// the single consonant, where `неси` and `брось` part ways on the stress
/// alone — and a caller that does not know the stress must not write that cell
/// at all.
///
/// # Examples
///
/// ```
/// use rusem::grammar::conjugation::imperative::settled;
///
/// assert!(settled("чита"));
/// assert!(settled("читай"));
/// assert!(settled("помн"));
/// assert!(!settled("нес"));
/// ```
#[must_use]
pub fn settled(present_stem: &str) -> bool {
    let Some(last) = present_stem.chars().last() else {
        return false;
    };

    crate::alphabet::is_vowel(last) || last == GLIDE || closed_by_two_consonants(present_stem)
}

/// The ending the imperative of a verb takes.
///
/// The stress is asked for because it is what separates `неси` from `брось`:
/// both stems end in a consonant, and only the place of the stress says which
/// ending follows. It is the stress of the first person singular of the
/// present, which is where Russian settles this.
///
/// # Examples
///
/// ```
/// use rusem::grammar::conjugation::imperative::{Ending, of};
///
/// assert_eq!(of("чита", false), Ending::Glide);
/// assert_eq!(of("нес", true), Ending::Vowel);
/// assert_eq!(of("брос", false), Ending::SoftSign);
/// assert_eq!(of("помн", false), Ending::Vowel);
/// ```
#[must_use]
pub fn of(present_stem: &str, ending_stressed: bool) -> Ending {
    let Some(last) = present_stem.chars().last() else {
        return Ending::Vowel;
    };

    if crate::alphabet::is_vowel(last) {
        return Ending::Glide;
    }
    if ending_stressed || closed_by_two_consonants(present_stem) {
        return Ending::Vowel;
    }

    Ending::SoftSign
}

/// Reports whether the stem ends in two consonants running.
///
/// Two consonants leave no room for a soft sign to be heard, so the stem takes
/// the full vowel however the stress falls: `помни`, not `помнь`. A stem that
/// already ends in a soft sign has its softness written and is not counted
/// among them.
fn closed_by_two_consonants(stem: &str) -> bool {
    let mut held = stem.chars().rev();
    let (Some(last), Some(before)) = (held.next(), held.next()) else {
        return false;
    };

    last != 'ь' && !crate::alphabet::is_vowel(last) && !crate::alphabet::is_vowel(before)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stem_in_a_vowel_takes_the_glide() {
        assert_eq!(of("чита", false), Ending::Glide);
        assert_eq!(of("да", false), Ending::Glide);
        assert_eq!(of("сто", true), Ending::Glide);
    }

    #[test]
    fn a_stressed_ending_takes_the_vowel() {
        assert_eq!(of("нес", true), Ending::Vowel);
        assert_eq!(of("ид", true), Ending::Vowel);
    }

    #[test]
    fn two_consonants_take_the_vowel_whatever_the_stress() {
        assert_eq!(of("помн", false), Ending::Vowel);
        assert_eq!(of("крикн", false), Ending::Vowel);
    }

    #[test]
    fn one_consonant_with_the_stress_on_the_stem_takes_the_soft_sign() {
        assert_eq!(of("брос", false), Ending::SoftSign);
        assert_eq!(of("готов", false), Ending::SoftSign);
    }

    #[test]
    fn a_single_consonant_after_a_vowel_takes_the_soft_sign() {
        assert_eq!(of("встан", false), Ending::SoftSign);
    }

    #[test]
    fn the_endings_are_written_as_stated() {
        assert_eq!(Ending::Glide.written(), "й");
        assert_eq!(Ending::Vowel.written(), "и");
        assert_eq!(Ending::SoftSign.written(), "ь");
    }
}
