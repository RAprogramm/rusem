// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The marks an index carries after its accent letter.
//!
//! Zaliznyak's index does not stop at `1c`: after the letter come the circled
//! numerals, then the signs about hypothetical forms, and last of all `ё`,
//! written off the index proper with a comma — `1c(1)`, `1a−`, `4b, ё`. The
//! dictionary source states them in that order and this reader takes them in
//! any, because the data it is read from was typed by many hands.
//!
//! What each mark means, by the source:
//!
//! - `(1)`, `(2)`, `(3)`, `((1))`, `((2))` — the circled numerals ①②③ of the
//!   `Грамматический словарь`, one cell each by the other pattern. They are
//!   read into [`Circled`] and the paradigm applies them.
//! - `, ё` — the stem alternates `е` with `ё` by where the scheme puts the
//!   stress: `жена́` against `жёны`. Read into a fact and applied.
//! - `−` (also typed `-`) — the plural is hypothetical: `ад` declines but
//!   `*а́ды` is starred in the printed table. The forms themselves follow the
//!   index; that they are starred is not modelled, so the mark is noted.
//! - `÷` — the genitive plural alone is hypothetical: `Москва́` but `*Москв`.
//!   Noted for the same reason.
//! - `^` — an individual departure the index does not encode; the printed table
//!   must be read. The one mark that is unreadable by design, noted.
//!
//! Anything else in the tail — the ` + ` of hyphenated compounds, the ` // `
//! of doubled indexes — is noted too: an index that says more than the core
//! understands is kept with the saying-so, never quietly read as plain.

use core::{iter::Peekable, str::Chars};

use super::circled::{Circled, Reach};

/// What the tail of an index said.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Tail {
    /// The circled numerals.
    pub circled: Circled,
    /// Whether the index writes `, ё`.
    pub yo:      bool,
    /// Whether the tail held marks the core does not understand.
    pub noted:   bool
}

/// Reads everything after the accent letter and its primes.
pub(super) fn read(letters: &mut Peekable<Chars<'_>>) -> Tail {
    let mut tail = Tail {
        circled: Circled::none(),
        yo:      false,
        noted:   false
    };

    while let Some(held) = letters.next() {
        match held {
            '(' => {
                if !numeral(letters, &mut tail.circled) {
                    tail.noted = true;
                }
            }
            ',' => {
                if yo(letters) {
                    tail.yo = true;
                } else {
                    tail.noted = true;
                }
            }
            _ => tail.noted = true
        }
    }

    tail
}

/// Reads one parenthesized numeral, the opening parenthesis already taken.
///
/// `(1)` and `((1))` differ only in the parentheses, which is what
/// [`Reach`] records.
fn numeral(letters: &mut Peekable<Chars<'_>>, circled: &mut Circled) -> bool {
    let doubled = letters.next_if_eq(&'(').is_some();
    let Some(digit) = letters.next() else {
        return false;
    };
    if letters.next_if_eq(&')').is_none() {
        return false;
    }
    if doubled && letters.next_if_eq(&')').is_none() {
        return false;
    }

    let reach = if doubled { Reach::Either } else { Reach::Whole };
    let held = match digit {
        '1' => &mut circled.nominative,
        '2' => &mut circled.genitive,
        '3' if !doubled => &mut circled.prepositional,
        _ => return false
    };
    *held = Some(reach);
    true
}

/// Reads the ` ё` after the comma.
fn yo(letters: &mut Peekable<Chars<'_>>) -> bool {
    letters.next_if_eq(&' ').is_some() && letters.next_if_eq(&'ё').is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tail(written: &str) -> Tail {
        read(&mut written.chars().peekable())
    }

    #[test]
    fn a_circled_numeral_is_read_into_its_cell() {
        assert_eq!(tail("(1)").circled.nominative, Some(Reach::Whole));
        assert_eq!(tail("(2)").circled.genitive, Some(Reach::Whole));
        assert_eq!(tail("(3)").circled.prepositional, Some(Reach::Whole));
        assert!(!tail("(1)").noted);
    }

    #[test]
    fn two_numerals_are_two_facts() {
        let held = tail("(1)(2)");

        assert_eq!(held.circled.nominative, Some(Reach::Whole));
        assert_eq!(held.circled.genitive, Some(Reach::Whole));
        assert!(!held.noted);
    }

    #[test]
    fn a_doubled_numeral_reaches_either_form() {
        assert_eq!(tail("((1))").circled.nominative, Some(Reach::Either));
        assert_eq!(tail("((2))").circled.genitive, Some(Reach::Either));
        assert!(!tail("((2))").noted);
    }

    #[test]
    fn the_yo_mark_is_read_off_its_comma() {
        let held = tail(", ё");

        assert!(held.yo);
        assert!(!held.noted);
    }

    #[test]
    fn hypothetical_form_marks_stay_noted() {
        assert!(tail("−").noted);
        assert!(tail("-").noted);
        assert!(tail("÷").noted);
        assert!(tail("^").noted);
    }

    #[test]
    fn a_noted_mark_does_not_hide_the_yo_after_it() {
        let held = tail("÷, ё");

        assert!(held.noted);
        assert!(held.yo);
    }

    #[test]
    fn a_compound_tail_is_noted_whole() {
        assert!(tail(" + 3*b").noted);
        assert!(tail(" // 1a").noted);
    }

    #[test]
    fn a_numeral_the_source_does_not_state_is_noted() {
        assert!(tail("(4)").noted);
        assert!(tail("((3))").noted);
        assert!(tail("(1").noted);
    }
}
