// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Fitting an ending to the stem it attaches to.
//!
//! The paradigm states an ending in one shape, and the alphabet then bends it.
//! `ы` does not stand after a sibilant or a back consonant and becomes `и`
//! (§ 1, § 2); unstressed `о` does not stand after a sibilant or `ц` and
//! becomes `е` (§ 4, § 18), and neither does the `ё` the soft endings are
//! stated with — `нашем` against the stressed `о чём`. Nor do `я` and `ю`
//! stand there: a stem in `ц` takes the soft endings by its shape and writes
//! them hard, which is why `солнце` gives `солнца` and not `солнця` (§ 1,
//! § 3). A paradigm that spelled these out cell by cell would be stating the
//! same rules dozens of times over.
//!
//! The zero ending of the genitive plural is bent the other way: it leaves two
//! consonants at the end of the word, and Russian parts them with a vowel it
//! does not write anywhere else in the paradigm.

use crate::alphabet::{Consonant, Letter, is_consonant};

/// Fits an ending to the stem, and says how the pair is written.
///
/// Whether the ending carries the stress has to be told, because that is what
/// separates `отцом` from `месяцем`. A reader who does not know the stress asks
/// for both answers and keeps them both.
#[must_use]
pub fn fitted(stem: &str, ending: &str, ending_stressed: bool) -> String {
    let Some(last) = stem.chars().last() else {
        return ending.to_owned();
    };
    let Some(first) = ending.chars().next() else {
        return ending.to_owned();
    };

    let bent = match first {
        'ы' if bars_yi(last) => 'и',
        'о' | 'ё' if !ending_stressed && bars_unstressed_o(last) => 'е',
        'я' if bars_soft_vowels(last) => 'а',
        'ю' if bars_soft_vowels(last) => 'у',
        held => held
    };

    let mut written = String::with_capacity(ending.len());
    written.push(bent);
    written.extend(ending.chars().skip(1));
    written
}

/// Reports whether a consonant refuses ы after it.
pub use crate::alphabet::bars_yi;

/// Reports whether a consonant refuses `я` and `ю` after it.
///
/// The sibilants by § 1 and `ц` by § 3: `свеча` and not `свечя`, `солнца` and
/// not `солнця`. The stem may well be soft — `солнц-` takes the soft endings
/// because the dictionary form ends in `е` — and the letters are written hard
/// all the same.
const fn bars_soft_vowels(letter: char) -> bool {
    match Letter::of(letter) {
        Some(Letter::Consonant(held)) => held.is_sibilant() || matches!(held, Consonant::Tse),
        _ => false
    }
}

/// Reports whether a consonant refuses an unstressed о after it.
///
/// The sibilants and `ц`, by § 4 and § 18: `хорошего` against the stressed
/// `большого`. A writer who cannot tell whether the vowel is stressed cannot
/// pick its letter after these consonants, and asks here before writing.
#[must_use]
pub const fn bars_unstressed_o(letter: char) -> bool {
    match Letter::of(letter) {
        Some(Letter::Consonant(held)) => held.is_sibilant() || matches!(held, Consonant::Tse),
        _ => false
    }
}

/// Parts the last two consonants of a stem with the vowel Russian inserts
/// there when an ending would otherwise be absent.
///
/// The vowel is `о` where one of the two consonants is a back one — `окно`
/// gives `окон`, `кукла` gives `кукол`, `сумка` gives `сумок` — and `е`
/// everywhere else: `сосна` gives `сосен`, `письмо` gives `писем`, `овца`
/// gives `овец`. Under the stress that `е` is written `ё` — `кочерга` gives
/// `кочерёг` — and after a sibilant § 18 writes it `о`: `кишка` gives
/// `кишок`, `княжна` gives `княжон`.
#[must_use]
pub fn parted(stem: &str, stressed: bool) -> String {
    let letters: Vec<char> = stem.chars().collect();
    let count = letters.len();
    if count < 2 {
        return stem.to_owned();
    }

    let (Some(&last), Some(&before)) = (letters.get(count - 1), letters.get(count - 2)) else {
        return stem.to_owned();
    };
    if !is_consonant(last) || !is_consonant(before) {
        return stem.to_owned();
    }

    let vowel = parting(before, last, stressed);
    let kept = if before == GLIDE {
        count - 2
    } else {
        count - 1
    };

    let mut written: String = letters.iter().take(kept).collect();
    written.push(vowel);
    written.push(last);
    written
}

/// The glide a stem may end in before the parting vowel.
///
/// It does not survive it: `майка` parts as `маек` and `чайка` as `чаек`,
/// because the vowel written there carries the glide in itself.
const GLIDE: char = 'й';

/// The vowel that parts two consonants.
const fn parting(before: char, last: char, stressed: bool) -> char {
    if softens(last) || before == GLIDE {
        return 'е';
    }
    if softens(before) {
        if stressed {
            return 'о';
        }

        return 'е';
    }
    if backs(before) {
        return 'о';
    }
    if stressed {
        return 'ё';
    }
    if backs(last) {
        return 'о';
    }

    'е'
}

/// Reports whether a consonant is a sibilant or `ц`.
///
/// Standing after the parting vowel it makes the vowel `е` whatever the
/// stress: `овца` gives `ове́ц`, `кольцо` gives `коле́ц`, `сердце` gives
/// `серде́ц`. Standing before it, § 18 lets the stress decide the letter of
/// the vowel it governs: `е` off the stress — `бабочек`, `кошек` — and `о`
/// under it — `кишо́к`, `княжо́н`.
const fn softens(letter: char) -> bool {
    match Letter::of(letter) {
        Some(Letter::Consonant(held)) => held.is_sibilant() || matches!(held, Consonant::Tse),
        _ => false
    }
}

const fn backs(letter: char) -> bool {
    matches!(
        Letter::of(letter),
        Some(Letter::Consonant(
            Consonant::Ka | Consonant::Ge | Consonant::Ha
        ))
    )
}

/// Drops the vowel that parts the last two consonants of a stem.
///
/// This is [`parted`] read backwards. A noun that shows the fluent vowel in
/// its dictionary form loses it as soon as an ending arrives, and a reader
/// starting from the dictionary form has to take it out again to recognize
/// what the ending was attached to.
#[must_use]
pub fn merged(stem: &str) -> Option<String> {
    let letters: Vec<char> = stem.chars().collect();
    let count = letters.len();
    let last = *letters.get(count.checked_sub(1)?)?;
    let vowel = *letters.get(count.checked_sub(2)?)?;
    let before = *letters.get(count.checked_sub(3)?)?;

    if !is_consonant(last) || !is_consonant(before) || !matches!(vowel, 'о' | 'е' | 'ё') {
        return None;
    }

    let mut written: String = letters.iter().take(count - 2).collect();
    written.push(last);
    Some(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stressed_parting_vowel_after_a_sibilant_is_written_o() {
        assert_eq!(parted("кишк", true), "кишок");
        assert_eq!(parted("княжн", true), "княжон");
    }

    #[test]
    fn an_unstressed_parting_vowel_after_a_sibilant_stays_e() {
        assert_eq!(parted("кошк", false), "кошек");
        assert_eq!(parted("бабочк", false), "бабочек");
    }

    #[test]
    fn a_parting_vowel_before_a_sibilant_is_e_whatever_the_stress() {
        assert_eq!(parted("овц", true), "овец");
        assert_eq!(parted("овц", false), "овец");
    }

    #[test]
    fn a_stressed_parting_vowel_elsewhere_is_written_yo() {
        assert_eq!(parted("кочерг", true), "кочерёг");
    }
}
