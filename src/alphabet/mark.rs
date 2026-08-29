// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The marks written over a letter, which are not letters themselves.
//!
//! Russian is written without stress and marked where a reader would otherwise
//! stumble: a dictionary, a textbook, a line of verse. The mark stands after
//! the vowel it belongs to and is a character of its own — `во́да` is five
//! characters and four letters.
//!
//! Nothing above should be discovering that on its own. A word arriving from a
//! dictionary or a scanned page carries marks, and a caller asking for its
//! letters must not be told it is not a Russian word.
//!
//! Two marks are used. The acute carries the main stress, the grave a
//! secondary one in a compound: `вы́сокока́чественный`.

/// A mark written over a letter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Mark {
    /// The acute, which carries the main stress: `во́да`.
    Acute,
    /// The grave, which carries a secondary stress in a compound:
    /// `вы́сокока́чественный`.
    Grave
}

impl Mark {
    /// The mark a character is, or nothing when it is not one.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::alphabet::Mark;
    ///
    /// assert_eq!(Mark::of('\u{0301}'), Some(Mark::Acute));
    /// assert_eq!(Mark::of('а'), None);
    /// ```
    #[must_use]
    #[inline]
    pub const fn of(letter: char) -> Option<Self> {
        Some(match letter {
            '\u{0301}' => Self::Acute,
            '\u{0300}' => Self::Grave,
            _ => return None
        })
    }

    /// How the mark is written.
    #[must_use]
    #[inline]
    pub const fn written(self) -> char {
        match self {
            Self::Acute => '\u{0301}',
            Self::Grave => '\u{0300}'
        }
    }

    /// Reports whether the mark carries the main stress.
    #[must_use]
    #[inline]
    pub const fn is_main(self) -> bool {
        matches!(self, Self::Acute)
    }
}

/// Reports whether a character is a mark.
#[must_use]
#[inline]
pub const fn is_mark(letter: char) -> bool {
    Mark::of(letter).is_some()
}

/// Reports whether a written word carries any mark.
#[must_use]
#[inline]
pub fn is_marked(written: &str) -> bool {
    written.chars().any(is_mark)
}

/// The word with every mark taken off.
///
/// This is what a caller does before looking a word up: a dictionary is
/// written without marks, and `во́да` and `вода` are the same word.
///
/// # Examples
///
/// ```
/// use rusem::alphabet::mark::bare;
///
/// assert_eq!(bare("во\u{0301}да"), "вода");
/// assert_eq!(bare("вода"), "вода");
/// ```
#[must_use]
#[inline]
pub fn bare(written: &str) -> String {
    written.chars().filter(|held| !is_mark(*held)).collect()
}

/// Where the main stress is marked, counted in vowels from the first.
///
/// The mark stands after the vowel it belongs to, so the vowels are counted up
/// to it. A word carrying more than one acute is answered with the last, the
/// way a compound is stressed. A word carrying none answers nothing.
///
/// # Examples
///
/// ```
/// use rusem::alphabet::mark::marked_vowel;
///
/// assert_eq!(marked_vowel("вода\u{0301}"), Some(1));
/// assert_eq!(marked_vowel("за\u{0301}мок"), Some(0));
/// assert_eq!(marked_vowel("вода"), None);
/// ```
#[must_use]
pub fn marked_vowel(written: &str) -> Option<usize> {
    let mut vowel = 0_usize;
    let mut at = None;

    for held in written.chars() {
        if super::is_vowel(held) {
            vowel += 1;
            continue;
        }
        if Mark::of(held).is_some_and(Mark::is_main) {
            at = vowel.checked_sub(1);
        }
    }

    at
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_marks_read_back_as_themselves() {
        for held in [Mark::Acute, Mark::Grave] {
            assert_eq!(Mark::of(held.written()), Some(held));
            assert!(is_mark(held.written()));
        }
    }

    #[test]
    fn a_letter_is_no_mark_and_a_mark_is_no_letter() {
        for held in super::super::Letter::ALL {
            assert!(!is_mark(held.written()));
        }
        assert_eq!(super::super::Letter::of(Mark::Acute.written()), None);
        assert_eq!(Mark::of('а'), None);
    }

    #[test]
    fn only_the_acute_carries_the_main_stress() {
        assert!(Mark::Acute.is_main());
        assert!(!Mark::Grave.is_main());
    }

    #[test]
    fn a_marked_word_is_named_as_marked() {
        assert!(is_marked("во\u{0301}да"));
        assert!(is_marked("вы\u{0301}сокока\u{0300}чественный"));
        assert!(!is_marked("вода"));
        assert!(!is_marked(""));
    }

    #[test]
    fn the_marks_come_off_and_nothing_else_does() {
        assert_eq!(bare("во\u{0301}да"), "вода");
        assert_eq!(bare("вода"), "вода");
        assert_eq!(
            bare("вы\u{0301}сокока\u{0300}чественный"),
            "высококачественный"
        );
        assert_eq!(bare("во-да"), "во-да");
    }

    #[test]
    fn the_marked_vowel_is_counted_in_vowels() {
        assert_eq!(marked_vowel("вода\u{0301}"), Some(1));
        assert_eq!(marked_vowel("за\u{0301}мок"), Some(0));
        assert_eq!(marked_vowel("замо\u{0301}к"), Some(1));
        assert_eq!(marked_vowel("вода"), None);
    }

    #[test]
    fn a_compound_is_stressed_on_its_last_acute() {
        assert_eq!(marked_vowel("во\u{0301}да\u{0301}"), Some(1));
    }

    #[test]
    fn a_grave_is_not_the_main_stress() {
        assert_eq!(marked_vowel("вы\u{0300}сока"), None);
    }

    #[test]
    fn a_mark_standing_before_any_vowel_is_answered_with_nothing() {
        assert_eq!(marked_vowel("\u{0301}вода"), None);
    }
}
