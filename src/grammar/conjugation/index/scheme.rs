// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The stress schemes of a verb, one letter each in the index.
//!
//! Zaliznyak writes two letters at most: one for the present and the simple
//! future, one after a slash for the past, and an index with no slash puts
//! the past in scheme `a`. The letters say where the stress falls cell by
//! cell, and the spelling hangs on them: the first conjugation writes `ё`
//! only where the ending is stressed, and the imperative writes `-и` only
//! where its ending is.

use core::{iter::Peekable, str::Chars};

/// One stress scheme, as the index letters it.
///
/// What each letter says of the present: `a` keeps the stress on the stem in
/// all six person forms and the imperative — `ве́рю, ве́ришь, ве́рь`; `b`
/// puts it on the ending everywhere — `храню́, храни́шь, храни́`; `c` puts it
/// on the ending in the first person singular and the imperative alone —
/// `учу́, у́чишь, учи́`.
///
/// What each letter says of the past: `a` keeps the infinitive's syllable —
/// `де́лал, де́лала`; `b` stresses the ending outside the masculine —
/// `нёс, несла́, несло́, несли́`; `c` stresses it in the feminine alone —
/// `спа́л, спала́, спа́ло`. The primed `c′` doubles the neuter — `да́ло` and
/// `дало́` — and the dictionary states it only for `дать`, `взять` and their
/// prefixed kin; `c″` is its reflexive counterpart, feminine on `-ла́сь` and
/// the rest doubled — `подня́лся́, подняла́сь`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Scheme {
    /// a: on the stem throughout.
    A,
    /// b: on the ending throughout.
    B,
    /// c: on the ending in one cell — the first singular of the present, the
    /// feminine of the past.
    C,
    /// c′: as c, with the neuter of the past doubled.
    CPrime,
    /// c″: as c′ for reflexive verbs, the feminine fixed on `-ла́сь`.
    CDouble
}

/// Reads one scheme letter and the primes after it.
///
/// The sources type the prime three ways — an apostrophe, U+2032, and the
/// doubled prime as a straight quote — so each straight quote counts for
/// two, the same way the declension index is read. The letter `а` is also
/// accepted in Cyrillic: the dump writes `признавать` under `1а` and
/// `чезнуть` under `3а`, typed by hand.
pub(super) fn read(letters: &mut Peekable<Chars<'_>>) -> Option<Scheme> {
    let letter = letters.next()?;
    let mut primes = 0_usize;
    while let Some(held) = letters.peek() {
        match held {
            '\'' | '′' => primes += 1,
            '"' => primes += 2,
            _ => break
        }
        letters.next();
    }

    match (letter, primes) {
        ('a' | 'а', 0) => Some(Scheme::A),
        ('b', 0) => Some(Scheme::B),
        ('c', 0) => Some(Scheme::C),
        ('c', 1) => Some(Scheme::CPrime),
        ('c', 2) => Some(Scheme::CDouble),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scheme(written: &str) -> Option<Scheme> {
        read(&mut written.chars().peekable())
    }

    #[test]
    fn the_three_plain_letters_are_read() {
        assert_eq!(scheme("a"), Some(Scheme::A));
        assert_eq!(scheme("b"), Some(Scheme::B));
        assert_eq!(scheme("c"), Some(Scheme::C));
    }

    #[test]
    fn the_primes_are_read_in_every_spelling() {
        assert_eq!(scheme("c'"), Some(Scheme::CPrime));
        assert_eq!(scheme("c′"), Some(Scheme::CPrime));
        assert_eq!(scheme("c\""), Some(Scheme::CDouble));
        assert_eq!(scheme("c''"), Some(Scheme::CDouble));
    }

    #[test]
    fn the_cyrillic_letter_is_read_as_the_latin_one() {
        assert_eq!(scheme("а"), Some(Scheme::A));
    }

    #[test]
    fn a_letter_outside_the_schemes_is_refused() {
        assert_eq!(scheme("d"), None);
        assert_eq!(scheme("a'"), None);
        assert_eq!(scheme(""), None);
    }
}
