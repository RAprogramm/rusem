// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The parenthetical words, which stand beside a sentence rather than in it.
//!
//! `конечно, он придёт` says the speaker is sure. Strike `конечно` and the
//! sentence stands unharmed, because the word was never a part of it: it is
//! the speaker's aside about the sentence. That is what parenthesis means, and
//! it is why the commas are there — they mark the word off as not belonging.
//!
//! # Three answers, not two
//!
//! A comma rule wants to know whether a word takes commas, and Russian gives
//! three answers rather than two.
//!
//! Some words are parenthetical wherever they stand: `итак`, `во-первых`,
//! `по-видимому`. They lost their own meaning and are nothing else now.
//!
//! Some never are, however much they sound like it: `вдруг`, `ведь`, `якобы`,
//! `почти`. Rosenthal lists them because writers comma them off by ear, and
//! the comma is a mistake every time.
//!
//! And some are one or the other by what they do in the sentence. `наконец`
//! is parenthetical when it closes a list and an adverb when it means at last:
//! `он, наконец, решился` against `он наконец пришёл`. No list settles that,
//! and this module does not pretend to — [`needs_commas`] answers [`None`] and
//! leaves the sentence to the syntax.

//! What each aside says about the sentence is stated in [`sense`].

pub mod sense;

/// The parenthetical words.
///
/// Holds the words that are parenthetical wherever they stand, the ones
/// that never are, and the ones where the sentence decides.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::parenthetical::Parenthetical;
///
/// assert!(Parenthetical::ALWAYS.contains(&"конечно"));
/// assert!(Parenthetical::NEVER.contains(&"вдруг"));
/// assert!(Parenthetical::EITHER.contains(&"наконец"));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parenthetical;

impl Parenthetical {
    /// The words that are parenthetical wherever they stand.
    pub const ALWAYS: &[&str] = &[
        "безусловно",
        "бесспорно",
        "в-пятых",
        "в-третьих",
        "в-четвёртых",
        "во-вторых",
        "во-первых",
        "дескать",
        "итак",
        "конечно",
        "мол",
        "например",
        "по-видимому",
        "пожалуй",
        "разумеется",
        "следовательно"
    ];

    /// The words that are never parenthetical and never take commas.
    ///
    /// Rosenthal gives the list because the mistake is common: these sound
    /// like an aside and are ordinary members of the sentence.
    pub const NEVER: &[&str] = &[
        "авось",
        "буквально",
        "будто",
        "вдобавок",
        "вдруг",
        "ведь",
        "весьма",
        "вот",
        "вряд",
        "всё-таки",
        "даже",
        "единственно",
        "именно",
        "иногда",
        "исключительно",
        "лишь",
        "небось",
        "непременно",
        "неужели",
        "обязательно",
        "определённо",
        "особенно",
        "отчасти",
        "пока",
        "поистине",
        "положительно",
        "по-прежнему",
        "почти",
        "приблизительно",
        "примерно",
        "просто",
        "пускай",
        "пусть",
        "разве",
        "решительно",
        "словно",
        "только",
        "якобы"
    ];

    /// The words that are parenthetical or not by what they do in the
    /// sentence.
    ///
    /// `однако` opens a clause as a conjunction and stands inside one as an
    /// aside; `правда` is a noun and a concession both. A gate that meets one
    /// of these reads the sentence and does not read a list.
    pub const EITHER: &[&str] = &[
        "бывало",
        "верно",
        "видимо",
        "возможно",
        "вообще",
        "впрочем",
        "действительно",
        "естественно",
        "значит",
        "кажется",
        "кстати",
        "наверное",
        "наконец",
        "наоборот",
        "напротив",
        "однако",
        "правда"
    ];
}

/// Whether a written word takes the commas of an aside.
///
/// [`Some(true)`](Some) when it always does, `Some(false)` when it never
/// does, and [`None`] when the sentence decides — either because the word
/// belongs to [`Parenthetical::EITHER`] or because it is no parenthetical at
/// all.
///
/// The two reasons for [`None`] are told apart by [`is_either`]: a caller that
/// must know asks that first.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::parenthetical::needs_commas;
///
/// assert_eq!(needs_commas("конечно"), Some(true));
/// assert_eq!(needs_commas("вдруг"), Some(false));
/// assert_eq!(needs_commas("наконец"), None);
/// assert_eq!(needs_commas("стол"), None);
/// ```
#[must_use]
pub fn needs_commas(written: &str) -> Option<bool> {
    let held = written.to_lowercase();

    if Parenthetical::ALWAYS.contains(&held.as_str()) {
        return Some(true);
    }
    if Parenthetical::NEVER.contains(&held.as_str()) {
        return Some(false);
    }

    None
}

/// Reports whether a written word is parenthetical wherever it stands.
#[must_use]
pub fn is_always(written: &str) -> bool {
    needs_commas(written) == Some(true)
}

/// Reports whether a written word is never parenthetical.
#[must_use]
pub fn is_never(written: &str) -> bool {
    needs_commas(written) == Some(false)
}

/// Reports whether the sentence decides, rather than the word.
#[must_use]
pub fn is_either(written: &str) -> bool {
    Parenthetical::EITHER.contains(&written.to_lowercase().as_str())
}

/// Reports whether a written word can ever stand as an aside.
#[must_use]
pub fn is_parenthetical(written: &str) -> bool {
    is_always(written) || is_either(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_list_holds_a_word_twice_and_no_word_is_on_two_lists() {
        let mut every: Vec<&str> = [
            Parenthetical::ALWAYS,
            Parenthetical::NEVER,
            Parenthetical::EITHER
        ]
        .concat();
        let counted = every.len();
        every.sort_unstable();
        every.dedup();

        assert_eq!(every.len(), counted, "a word is listed twice");
    }

    #[test]
    fn a_word_that_is_always_an_aside_always_takes_commas() {
        for held in Parenthetical::ALWAYS {
            assert_eq!(needs_commas(held), Some(true), "{held}");
            assert!(is_always(held));
            assert!(is_parenthetical(held));
        }
    }

    #[test]
    fn a_word_that_is_never_an_aside_never_takes_them() {
        for held in Parenthetical::NEVER {
            assert_eq!(needs_commas(held), Some(false), "{held}");
            assert!(is_never(held));
            assert!(!is_parenthetical(held));
        }
    }

    #[test]
    fn a_word_the_sentence_decides_is_left_undecided_here() {
        for held in Parenthetical::EITHER {
            assert_eq!(needs_commas(held), None, "{held}");
            assert!(is_either(held));
            assert!(is_parenthetical(held));
        }
    }

    #[test]
    fn a_word_that_is_no_aside_is_left_undecided_and_is_not_either() {
        assert_eq!(needs_commas("стол"), None);
        assert!(!is_either("стол"));
        assert!(!is_parenthetical("стол"));
        assert!(!is_parenthetical(""));
    }

    #[test]
    fn the_case_a_word_is_written_in_does_not_matter() {
        assert_eq!(needs_commas("Конечно"), Some(true));
        assert_eq!(needs_commas("ВДРУГ"), Some(false));
        assert!(is_either("Однако"));
    }
}
