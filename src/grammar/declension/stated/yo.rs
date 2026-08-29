// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The `е` a marked stem trades for `ё` where the stress stands on it.
//!
//! `ё` is a stressed letter, so a stem that alternates writes it exactly in
//! the cells whose stress the scheme puts on the stem, and `е` in the cells
//! whose stress is on the ending: `жена́` against `жёны`, `мёд` against
//! `меды́`, `весна́` against `вёсны`. The dictionary marks the words that do
//! this with `, ё` after the index, because most stems do not — `стена́`
//! keeps its `е` in `сте́ны` — and the letter goes into the last syllable of
//! the stem, which is where every marked word of the evidence carries it:
//! `жён`, `звёзд`, `колёс`, `веретён`.
//!
//! A cell whose ending is nothing takes the stress onto the stem even when
//! the scheme says the ending, because there is nothing else to carry it, and
//! the letter follows: `желна́` is `1b−, ё` and its genitive plural is
//! `жёлн`. The one such cell that does not is the cell where a fleeting vowel
//! comes in stressed — there the stress stands on the vowel that was just put
//! back, not on the stem's own syllable, and the stem keeps its `е` while the
//! parting vowel itself is written `ё` where its own rule says so.

/// Reports whether this cell writes the marked stem's `ё`.
///
/// Yes wherever the stem carries the stress: the cells the scheme gives the
/// stem, and the cells whose ending is nothing — unless the nothing is about
/// to be parted by a stressed fleeting vowel, which then carries the stress
/// instead.
#[must_use]
pub const fn stands(stressed: bool, bare: bool, parting: bool) -> bool {
    if !stressed {
        return true;
    }

    bare && !parting
}

/// The stem with its last `е` written `ё`.
///
/// A stem already showing `ё` is left alone: the letter is where the
/// dictionary form put it — `мёд`, `ёж` — and the evidence holds no marked
/// stem with two syllables that alternate.
#[must_use]
pub fn written(stem: &str) -> String {
    if stem.contains('ё') {
        return String::from(stem);
    }
    let Some(at) = stem.rfind('е') else {
        return String::from(stem);
    };

    let mut held = String::from(stem);
    held.replace_range(at..at + 'е'.len_utf8(), "ё");
    held
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_last_e_of_the_stem_takes_the_diaeresis() {
        assert_eq!(written("жен"), "жён");
        assert_eq!(written("веретен"), "веретён");
    }

    #[test]
    fn a_stem_already_showing_the_letter_is_left_alone() {
        assert_eq!(written("мёд"), "мёд");
        assert_eq!(written("ёж"), "ёж");
    }

    #[test]
    fn a_stem_with_no_e_is_left_alone() {
        assert_eq!(written("стол"), "стол");
    }
}
