// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The index a dictionary states a verb's conjugation with.
//!
//! Zaliznyak's `Грамматический словарь русского языка` states the whole
//! conjugation of a verb as a short index: a digit for how the present stem
//! is got from the infinitive, a letter for where the stress falls, a second
//! letter after a slash for the past, and marks for what departs from that.
//! `читать` is `1a`, `толкнуть` is `3b`, `нести` is `7b/b`, `умереть` is
//! `9b/c(1)`. The Russian Wiktionary carries the same index letter for
//! letter on every conjugation table it prints, which is where the shapes
//! read here are attested; its own documentation of the verb templates,
//! `Викисловарь:Шаблоны словоизменений/Глаголы`, states the classes, the
//! schemes and the numerals this module types.
//!
//! The engine reads the index rather than deriving it: which class a verb is
//! in is a fact about that verb, stated by its dictionary. What is not
//! understood is not thrown away — an index carrying the `^` of
//! hand-overridden tables, a dual paradigm, or any other sign without a rule
//! here keeps it and says so, and [`super::stated`] builds nothing on it.

pub mod circled;
pub mod kind;
mod marks;
pub mod scheme;

use core::{iter::Peekable, str::Chars};

use self::{circled::Circled, kind::Kind, scheme::Scheme};

/// The signs an index writes between the class digit and the scheme letter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Signs {
    /// Whether the class carries the ring, written `°`.
    ///
    /// The ring names a subtype of its class, and the dictionary writes it
    /// on two: `3°` drops `-ну-` in the past — `сохнуть, сох` against
    /// `толкнуть, толкнул` — and `6°` puts the endings on the bare stem with
    /// none of the class's swapping — `звать, зову` against `писать, пишу`.
    pub ringed:   bool,
    /// Whether a vowel comes and goes between the stems, written `*`.
    ///
    /// `вбить` conjugates `вобью`, `втереть` conjugates `вотру`: a vowel
    /// stands in one stem and not the other. Which stem has it is not in the
    /// index — sometimes the infinitive, as in `разостлать, расстелю` — so
    /// the star is kept as a stated fact and the present is not built on it.
    pub fleeting: bool
}

/// The index of one verb, as a dictionary writes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerbIndex {
    /// The class: how the present stem is got from the infinitive.
    pub kind:        Kind,
    /// The `°` and `*` between the digit and the letter.
    pub signs:       Signs,
    /// Where the stress falls in the present and the simple future.
    pub present:     Scheme,
    /// Where the stress falls in the past.
    ///
    /// Written after a slash, and an index with no slash puts the past in
    /// scheme `a`: `толкнуть 3b` conjugates `толкну́` but `толкну́л`.
    pub past:        Scheme,
    /// The circled numerals, each one cell's departure.
    #[cfg_attr(feature = "serde", serde(default))]
    pub circled:     Circled,
    /// Whether the verb forms no passive participles, written `X` — the wiki
    /// documentation's replacement for Zaliznyak's crossed boxes.
    #[cfg_attr(feature = "serde", serde(default))]
    pub passiveless: bool,
    /// Whether the stem trades `е` for `ё` under its own stress, written
    /// `, ё` in Zaliznyak's dictionary: `нести 7b/b, ё` puts `нёс`.
    ///
    /// The wiki categories never carry the mark — they print the `ё` in the
    /// forms instead — so a builder applies the trade from the schemes
    /// themselves, and the flag stands here as the dictionary's own way of
    /// stating it.
    #[cfg_attr(feature = "serde", serde(default))]
    pub yo:          bool,
    /// Whether the index carries marks the rules here do not read.
    ///
    /// The trailing `^` of hand-overridden tables — `лгать 6°b/c^` writes
    /// `лжёшь` no rule states; the `⌧` of `хотеть`; a dual paradigm like
    /// `махать 1a//6c`; template leaks. A verb carrying one conjugates as
    /// the index says except where it does not, and saying so is the honest
    /// answer.
    pub noted:       bool
}

/// Reads an index a dictionary states, from the tail of the category that
/// names it.
///
/// The one string normalized away beforehand is a trailing `/ru`, the
/// template-name leak the dump carries on `внять`'s `14*b/c/ru`: it states
/// the language of the page, not a mark of the index.
///
/// # Examples
///
/// ```
/// use rusem::grammar::conjugation::index::{kind::Kind, read, scheme::Scheme};
///
/// let plain = read("1a").expect("a stated index");
/// assert_eq!(plain.kind, Kind::One);
/// assert_eq!(plain.present, Scheme::A);
/// assert_eq!(plain.past, Scheme::A);
///
/// let carried = read("7b/b").expect("a stated index");
/// assert_eq!(carried.past, Scheme::B);
///
/// let ringed = read("6°*b/c").expect("a stated index");
/// assert!(ringed.signs.ringed);
/// assert!(ringed.signs.fleeting);
///
/// assert!(read("8c/b^").expect("a stated index").noted);
/// assert!(read("gibberish").is_none());
/// ```
#[must_use]
pub fn read(written: &str) -> Option<VerbIndex> {
    let bare = written.strip_suffix("/ru").unwrap_or(written);
    let mut letters = bare.chars().peekable();
    let kind = head(&mut letters)?;
    let signs = signs(&mut letters);
    let present = scheme::read(&mut letters)?;
    let tail = marks::read(&mut letters);

    Some(VerbIndex {
        kind,
        signs,
        present,
        past: tail.past.unwrap_or(Scheme::A),
        circled: tail.circled,
        passiveless: tail.passiveless,
        yo: tail.yo,
        noted: tail.noted
    })
}

/// Reads the class: the digits 1 to 16, or the `^` of an isolated verb.
fn head(letters: &mut Peekable<Chars<'_>>) -> Option<Kind> {
    if letters.next_if_eq(&'^').is_some() {
        return Some(Kind::Isolated);
    }

    let mut digits = String::new();
    while let Some(held) = letters.next_if(char::is_ascii_digit) {
        digits.push(held);
    }

    Kind::of(digits.parse().ok()?)
}

/// Reads the `°` and `*` between the digit and the letter.
fn signs(letters: &mut Peekable<Chars<'_>>) -> Signs {
    let mut held = Signs::default();
    loop {
        if letters.next_if_eq(&'°').is_some() {
            held.ringed = true;
        } else if letters.next_if_eq(&'*').is_some() {
            held.fleeting = true;
        } else {
            return held;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{circled::Reach, *};

    fn stated(written: &str) -> VerbIndex {
        read(written).unwrap_or_else(|| unreachable!("a stated index"))
    }

    #[test]
    fn a_missing_past_letter_is_scheme_a() {
        let held = stated("3b");
        assert_eq!(held.past, Scheme::A);
        assert_eq!(stated("5b/c").past, Scheme::C);
    }

    #[test]
    fn the_ringed_class_keeps_its_numerals() {
        let held = stated("3°a((5))(6)");
        assert!(held.signs.ringed);
        assert_eq!(held.circled.masculine_kept, Some(Reach::Either));
        assert_eq!(held.circled.bygone_kept, Some(Reach::Whole));
        assert!(!held.noted);
    }

    #[test]
    fn the_numerals_are_read_on_either_side_of_the_past() {
        assert_eq!(stated("7b/b(9)").circled.gerund_future, Some(Reach::Whole));
        assert_eq!(
            stated("16b/c((1))").circled.past_prefixed,
            Some(Reach::Either)
        );
        assert_eq!(stated("4c(7)").circled.passive_stressed, Some(Reach::Whole));
    }

    #[test]
    fn the_x_and_the_hat_are_read_apart() {
        let plain = stated("5c(4)X");
        assert!(plain.passiveless);
        assert!(!plain.noted);
        assert_eq!(plain.circled.acting_retracted, Some(Reach::Whole));

        let hatted = stated("6°b/cX^");
        assert!(hatted.passiveless);
        assert!(hatted.noted);
    }

    #[test]
    fn an_isolated_verb_is_read_by_its_own_sign() {
        let held = stated("^b/c'");
        assert_eq!(held.kind, Kind::Isolated);
        assert_eq!(held.present, Scheme::B);
        assert_eq!(held.past, Scheme::CPrime);
    }

    #[test]
    fn the_leaks_the_dump_carries_are_survived() {
        assert!(!stated("14*b/c/ru").noted);
        assert_eq!(stated("14*b/c/ru").past, Scheme::C);
        assert_eq!(stated("1а").present, Scheme::A);
        assert!(stated("6°b/cX1^").noted);
        assert!(stated("5c'⌧^").noted);
        assert!(stated("6°b/c-сяСВ^").noted);
    }

    #[test]
    fn a_dual_paradigm_keeps_its_first_half_and_says_so() {
        let doubled = stated("6a//1а");
        assert_eq!(doubled.kind, Kind::Six);
        assert!(doubled.noted);
        assert!(stated("4c/4b").noted);
    }

    #[test]
    fn what_is_no_index_at_all_is_refused() {
        assert_eq!(read("Спряжение"), None);
        assert_eq!(read("17a"), None);
        assert_eq!(read(""), None);
    }
}
