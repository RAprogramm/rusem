// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 7. После приставки на согласную вместо и пишется ы.
//!
//! `розыск`, `предыдущий`, `подытожить`, `безынтересный`, `сымпровизировать`.
//! The prefix ends in a consonant, the root begins in `и`, and the spelling
//! follows the pronunciation.
//!
//! Three sets of prefixes are excepted. `меж-` and `сверх-` keep the `и` by
//! the paragraph's own first point — `межирригационный`, `сверхизысканный`;
//! for `меж-` the same follows from § 1, which refuses `ы` after `ж`, while
//! `х` is no sibilant and § 1 says nothing of it. The foreign prefixes
//! `пан-`, `суб-`, `транс-`, `контр-` and the like keep it by the second
//! point: `панисламизм`, `субинспектор`, `Трансиордания`. And `взимать` keeps
//! it because the `и` is pronounced.

use crate::{
    alphabet::is_vowel,
    rules::{Citation, Findings, Found, Parts, Scope, found::spelled, scope}
};

/// Where this rule is written.
pub const CITES: Citation = Citation::whole(7);

/// What this rule is about.
pub const SCOPE: Scope = Scope {
    parts:   &[],
    cases:   &[],
    numbers: &[],
    needs:   scope::Needs {
        stress: false,
        parts:  true
    }
};

/// The prefixes that keep the `и` after them.
///
/// `меж-` and `сверх-` by the paragraph's first point, the foreign ones by
/// its second, an open class the point closes with `и т. п.`.
/// The seam belongs to the last prefix of a chain, so a stacked word is asked
/// about the prefix it ends with: `сверхбезыдейный` turns by `без`, and a
/// word ending its chain in `сверх` would keep.
const KEEPING: &[&str] = &[
    "меж",
    "сверх",
    "пан",
    "суб",
    "транс",
    "пост",
    "супер",
    "дез",
    "интер",
    "гипер",
    "де",
    "контр"
];

/// The words the paragraph names as keeping their `и` against the rule.
const SPARED: &[&str] = &["взимать", "взимание"];

/// What the paragraph says when it is broken.
const SAYS: &str = "после приставки на согласную вместо и пишется ы";

/// The letter the paragraph refuses after such a prefix.
const REFUSED: char = 'и';

/// The letter it writes there instead.
const WRITTEN: char = 'ы';

/// What the paragraph finds in a written word.
///
/// The paragraph speaks of the seam between a prefix and a root, so it is
/// asked how the word is built and finds nothing when nobody knows. The
/// letters alone would have it fire on `синий`, which opens with what is
/// elsewhere the prefix `с-` and is one morpheme. The seam it is handed is
/// the seam it judges: the prefix is whatever stands before it, however the
/// word stacked its prefixes, and nothing is guessed from a list.
///
/// A prefix ending in a vowel is outside the paragraph — `на-` and `за-`
/// leave the `и` alone, `наиграть` — and a prefix ending in one of the
/// excepted set keeps it, so only a consonant the paragraph does speak of
/// turns the `и` to `ы`.
///
/// # Examples
///
/// ```
/// use rusem::rules::{Parts, svod::prefix_before_i::found};
///
/// let held = found("розиск", Parts::Prefixed(3));
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].at, 3);
/// assert_eq!(held[0].instead, "розыск");
///
/// assert!(found("розыск", Parts::Prefixed(3)).is_empty());
/// assert!(found("межирригационный", Parts::Prefixed(3)).is_empty());
/// assert!(found("взимать", Parts::Prefixed(2)).is_empty());
/// assert!(found("розиск", Parts::Unknown).is_empty());
/// ```
#[must_use]
pub fn found(written: &str, parts: Parts) -> Findings {
    let mut held = Findings::new();
    let Parts::Prefixed(length) = parts else {
        return held;
    };
    if SPARED.contains(&written) {
        return held;
    }

    let letters: std::vec::Vec<char> = written.chars().collect();
    let Some(last) = length
        .checked_sub(1)
        .and_then(|seam| letters.get(seam))
        .copied()
    else {
        return held;
    };
    if is_vowel(last) {
        return held;
    }

    let prefix: String = letters.iter().take(length).collect();
    if KEEPING.iter().any(|kept| prefix.ends_with(kept)) {
        return held;
    }
    if letters.get(length) != Some(&REFUSED) {
        return held;
    }

    held.push(Found::new(
        CITES,
        length,
        SAYS,
        spelled(written, length, WRITTEN)
    ));
    held
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_paragraph_is_cited() {
        assert_eq!(CITES.paragraph, 7);
        assert!(CITES.is_stated());
    }

    #[test]
    fn a_turning_prefix_before_i_is_found_and_spelled_out() {
        let held = found("розиск", Parts::Prefixed(3));

        assert_eq!(held.len(), 1);
        assert_eq!(held[0].at, 3);
        assert_eq!(held[0].instead, "розыск");

        let other = found("безинтересный", Parts::Prefixed(3));
        assert_eq!(other[0].instead, "безынтересный");
    }

    #[test]
    fn a_word_already_written_with_yi_is_left_alone() {
        for (one, seam) in [("розыск", 3), ("безынтересный", 3), ("предыдущий", 4)]
        {
            assert!(found(one, Parts::Prefixed(seam)).is_empty(), "{one}");
        }
    }

    #[test]
    fn a_keeping_prefix_is_left_alone() {
        for (one, seam) in [
            ("межирригационный", 3),
            ("сверхизысканный", 5),
            ("панисламизм", 3),
            ("субинспектор", 3)
        ] {
            assert!(found(one, Parts::Prefixed(seam)).is_empty(), "{one}");
        }
    }

    #[test]
    fn the_stated_seam_is_judged_rather_than_guessed() {
        assert!(found("расиск", Parts::Prefixed(2)).is_empty());

        let held = found("сверхбезидейный", Parts::Prefixed(8));
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].at, 8);
        assert_eq!(held[0].instead, "сверхбезыдейный");
    }

    #[test]
    fn the_word_the_paragraph_names_is_spared() {
        assert!(found("взимать", Parts::Prefixed(2)).is_empty());
    }

    #[test]
    fn a_word_whose_build_is_unknown_is_not_judged() {
        assert!(found("розиск", Parts::Unknown).is_empty());
        assert!(found("розиск", Parts::Bare).is_empty());
    }

    #[test]
    fn a_word_without_a_prefix_is_left_alone() {
        assert!(found("игра", Parts::Bare).is_empty());
        assert!(found("вода", Parts::Bare).is_empty());
    }
}
