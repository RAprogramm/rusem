// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! § 3. После ц буквы ю и я допускаются только в иноязычных именах собственных.
//!
//! `Цюрих`, `Свенцяны`. Everywhere else `ц` takes `у` and `а`, as the sibilants
//! do in § 1.
//!
//! The exception is a fact about the word rather than about the letters, so it
//! is asked for rather than guessed: a caller that cannot tell a proper name
//! says so, and the rule stays silent instead of refusing `Цюрих`.

use crate::rules::{Citation, Findings, Found, Scope, found::spelled, scope};

/// Where this rule is written.
pub const CITES: Citation = Citation::whole(3);

/// What this rule is about.
///
/// Every word: `ц` takes the same letters after it whatever the word is.
pub const SCOPE: Scope = scope::ANY;

/// What the paragraph says when it is broken.
const SAYS: &str = "после ц пишутся у и а; ю и я стоят там только в иноязычных именах";

/// The letter the paragraph is about.
const TSE: char = 'ц';

/// What the paragraph finds in a written word.
///
/// A proper name is left alone, because the paragraph admits it. A caller that
/// does not know whether the word is one says so and gets nothing back:
/// refusing `Цюрих` would be worse than saying nothing about `цюпля`.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::ts_vowels::found;
///
/// assert!(found("цирк", false).is_empty());
/// assert!(found("Цюрих", true).is_empty());
///
/// let held = found("цюпля", false);
/// assert_eq!(held.len(), 1);
/// assert_eq!(held[0].at, 1);
/// assert_eq!(held[0].instead, "цупля");
/// ```
#[must_use]
pub fn found(written: &str, proper: bool) -> Findings {
    let mut held = Findings::new();
    if proper {
        return held;
    }

    let letters: std::vec::Vec<char> = written.chars().collect();
    for (at, letter) in letters.iter().enumerate() {
        if *letter != TSE {
            continue;
        }
        let after = at + 1;
        let Some(next) = letters.get(after) else {
            continue;
        };
        let Some(instead) = hard(*next) else {
            continue;
        };

        held.push(Found::new(
            CITES,
            after,
            SAYS,
            spelled(written, after, instead)
        ));
    }

    held
}

/// The letter the paragraph writes in place of a refused one.
///
/// `ю` and `я` state that what stands before them is soft, and `ц` is hard
/// whatever follows, so the hard letters for the same vowels are written.
const fn hard(refused: char) -> Option<char> {
    match refused {
        'ю' => Some('у'),
        'я' => Some('а'),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_paragraph_is_cited() {
        assert_eq!(CITES.paragraph, 3);
        assert!(CITES.is_stated());
    }

    #[test]
    fn a_word_the_paragraph_admits_is_left_alone() {
        for held in ["цирк", "цапля", "цугом", "отец", "вода"] {
            assert!(found(held, false).is_empty(), "{held}");
        }
    }

    #[test]
    fn a_foreign_proper_name_is_left_alone() {
        assert!(found("цюрих", true).is_empty());
        assert!(found("свенцяны", true).is_empty());
    }

    #[test]
    fn the_refused_letters_are_found_and_spelled_out() {
        let one = found("цюпля", false);
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].at, 1);
        assert_eq!(one[0].instead, "цупля");

        let two = found("свенцяны", false);
        assert_eq!(two.len(), 1);
        assert_eq!(two[0].at, 5);
        assert_eq!(two[0].instead, "свенцаны");
    }
}
