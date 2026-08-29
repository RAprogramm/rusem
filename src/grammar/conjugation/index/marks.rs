// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The tail of a verb index: the past scheme, the numerals, the signs.
//!
//! After the present letter Zaliznyak may write, in any of the orders the
//! sources actually type: circled numerals — `3°a((5))(6)`; a slash and the
//! past scheme — `7b/b(9)`; the letter `X`, the wiki documentation's
//! replacement for his crossed boxes, saying the verb forms no passive
//! participles — `длить 4bX`; and `, ё` for the stems that trade `е` for `ё`
//! under their own stress. What else the sources type is not part of the
//! index — the `^` of hand-overridden tables, the `⌧` of `хотеть`, the
//! second half of a dual paradigm like `махать 1a//6c`, template leaks like
//! `-сяСВ` — and every such sign is kept as the saying that the index holds
//! more than the rules here read, never dropped into a plain one.

use core::{iter::Peekable, str::Chars};

use super::{
    circled::{self, Circled},
    scheme::{self, Scheme}
};

/// What the tail of an index said.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Tail {
    /// The past scheme after the slash, absent when the index writes none.
    pub past:        Option<Scheme>,
    /// The circled numerals.
    pub circled:     Circled,
    /// Whether the index writes `X`.
    pub passiveless: bool,
    /// Whether the index writes `, ё`.
    pub yo:          bool,
    /// Whether the tail held marks the rules here do not read.
    pub noted:       bool
}

/// Reads everything after the present letter and its primes.
pub(super) fn read(letters: &mut Peekable<Chars<'_>>) -> Tail {
    let mut tail = Tail {
        past:        None,
        circled:     Circled::none(),
        passiveless: false,
        yo:          false,
        noted:       false
    };

    while let Some(held) = letters.next() {
        match held {
            '(' => {
                if !circled::numeral(letters, &mut tail.circled) {
                    tail.noted = true;
                }
            }
            '/' => {
                if !parted(letters, &mut tail) {
                    tail.noted = true;
                    letters.for_each(drop);
                    break;
                }
            }
            'X' => {
                tail.passiveless = true;
                if letters.next_if_eq(&'1').is_some() {
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

/// Reads the past scheme after a slash, and refuses the slashes that open
/// something else.
///
/// A slash is the past scheme's only when a scheme letter follows and no
/// past has been read yet. What else a slash opens is the second half of a
/// dual paradigm — `4b//4c`, and `4c/4b` with a single slash, told apart
/// from a past by the digit after it — and that half is a second index, not
/// a mark on this one, so the caller notes it and reads no further.
fn parted(letters: &mut Peekable<Chars<'_>>, tail: &mut Tail) -> bool {
    if tail.past.is_some() || !matches!(letters.peek(), Some('a' | 'а' | 'b' | 'c')) {
        return false;
    }

    tail.past = scheme::read(letters);
    tail.past.is_some()
}

/// Reads the ` ё` after the comma.
fn yo(letters: &mut Peekable<Chars<'_>>) -> bool {
    letters.next_if_eq(&' ').is_some() && letters.next_if_eq(&'ё').is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::declension::index::circled::Reach;

    fn tail(written: &str) -> Tail {
        read(&mut written.chars().peekable())
    }

    #[test]
    fn the_past_scheme_is_read_off_its_slash() {
        assert_eq!(tail("/b").past, Some(Scheme::B));
        assert_eq!(tail("/c'").past, Some(Scheme::CPrime));
        assert_eq!(tail("/c\"").past, Some(Scheme::CDouble));
        assert!(!tail("/b").noted);
    }

    #[test]
    fn a_numeral_stands_on_either_side_of_the_past() {
        let before = tail("((5))(6)");
        assert_eq!(before.circled.masculine_kept, Some(Reach::Either));
        assert_eq!(before.circled.bygone_kept, Some(Reach::Whole));

        let after = tail("/b(9)");
        assert_eq!(after.past, Some(Scheme::B));
        assert_eq!(after.circled.gerund_future, Some(Reach::Whole));
        assert!(!after.noted);
    }

    #[test]
    fn the_x_is_read_and_its_one_off_variant_is_noted() {
        assert!(tail("X").passiveless);
        assert!(!tail("X").noted);
        assert!(tail("X1").passiveless);
        assert!(tail("X1").noted);
    }

    #[test]
    fn a_dual_paradigm_is_noted_whole() {
        assert!(tail("//4c").noted);
        assert!(tail("/4b").noted);
        assert_eq!(tail("/4b").past, None);
        assert_eq!(tail("//4c(9)").circled, Circled::none());
    }

    #[test]
    fn the_overriding_hat_is_noted() {
        let held = tail("/cX^");
        assert_eq!(held.past, Some(Scheme::C));
        assert!(held.passiveless);
        assert!(held.noted);
    }

    #[test]
    fn the_yo_mark_is_read_off_its_comma() {
        assert!(tail(", ё").yo);
        assert!(!tail(", ё").noted);
        assert!(tail(",x").noted);
    }

    #[test]
    fn a_template_leak_is_noted() {
        assert!(tail("/c-сяСВ^").noted);
        assert!(tail("'⌧^").noted);
    }
}
