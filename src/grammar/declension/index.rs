// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The index a dictionary states a noun's declension with.
//!
//! Zaliznyak's `Русское именное словоизменение` states the whole declension of
//! a noun as a short index: a digit for what the stem ends in, a letter for
//! where the stress falls, and marks for what departs from that. `стол` is
//! `1b`, `платок` is `3*b`, `время` is `8°c`. Everything the paradigm does
//! follows from it, which is why a dictionary that states it does not have to
//! print the forms.
//!
//! The engine reads the index rather than deriving it. Which pattern a noun
//! declines by can be worked out from the gender and the dictionary form for
//! most words and not for all — `пальто` declines by nothing and `мороженое`
//! by the adjectival endings, and no letters say so. Where the dictionary
//! states it, that is the fact; the derivation stays for words the dictionary
//! does not hold.
//!
//! What is not understood is not thrown away. An index carrying marks the core
//! has no rules for keeps them and says so, and a caller that would rather
//! write nothing than write a guess can ask.
//!
//! The marks the core does read are typed facts on the index. The star is the
//! fleeting vowel. The circled numerals are [`circled::Circled`], one cell
//! each taken from the other pattern. The `, ё` written after the index says
//! the stem trades `е` for `ё` wherever the scheme holds the stress on the
//! stem: `жена́` against `жёны`. The marks it does not read — the `°` of the
//! stems that grow between the numbers (`крестьянин`, `щенок`, `имя`), the
//! `−` and `÷` of the hypothetical plurals, and the `^` that Zaliznyak
//! himself defines as a departure only the printed table states — are noted,
//! never quietly dropped into a plain index.

pub mod circled;
pub mod falls;
mod marks;

/// What the stem of a noun ends in.
///
/// The digits of the index, and they are about the letters rather than about
/// the gender: `1` is a hard consonant, `5` is `ц`, `8` is the third
/// declension. The endings follow from this and from nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    /// 1, a hard consonant: `топор`, `стол`.
    Hard,
    /// 2, a soft consonant: `тюлень`.
    Soft,
    /// 3, one of `г`, `к`, `х`: `сапог`.
    Velar,
    /// 4, one of `ж`, `ш`, `ч`, `щ`: `калач`.
    Sibilant,
    /// 5, `ц`: `немец`.
    Tse,
    /// 6, a vowel or the glide: `бой`.
    Glide,
    /// 7, the glide written `и`: `полоний`, `армия`.
    Iotated,
    /// 8, the third declension: `боль`, `ночь`.
    Third
}

impl Kind {
    /// The digit this kind is written with.
    #[must_use]
    pub const fn digit(self) -> u8 {
        match self {
            Self::Hard => 1,
            Self::Soft => 2,
            Self::Velar => 3,
            Self::Sibilant => 4,
            Self::Tse => 5,
            Self::Glide => 6,
            Self::Iotated => 7,
            Self::Third => 8
        }
    }

    /// The kind a digit names.
    #[must_use]
    pub const fn of(digit: u8) -> Option<Self> {
        match digit {
            1 => Some(Self::Hard),
            2 => Some(Self::Soft),
            3 => Some(Self::Velar),
            4 => Some(Self::Sibilant),
            5 => Some(Self::Tse),
            6 => Some(Self::Glide),
            7 => Some(Self::Iotated),
            8 => Some(Self::Third),
            _ => None
        }
    }
}

/// Where the stress falls across a paradigm.
///
/// Six schemes and four variants of them, and each is a statement about two
/// numbers at once: `a` keeps the stress on the stem throughout, `c` moves it
/// to the ending in the plural, `f` keeps it on the ending everywhere but the
/// nominative singular. The primed variants take one cell out of the scheme
/// they are named after.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Accent {
    /// a: on the stem throughout.
    A,
    /// b: on the ending throughout.
    B,
    /// b′: on the ending, but on the stem in the instrumental singular.
    BPrime,
    /// c: on the stem in the singular, on the ending in the plural.
    C,
    /// d: on the ending in the singular, on the stem in the plural.
    D,
    /// d′: as d, but on the stem in the accusative singular.
    DPrime,
    /// e: on the stem in the singular and the nominative plural, on the ending
    /// in the rest of the plural.
    E,
    /// f: on the ending, but on the stem in the nominative singular.
    F,
    /// f′: on the ending, but on the stem in the accusative singular.
    FPrime,
    /// f″: on the ending, but on the stem in the instrumental singular and the
    /// nominative plural.
    FDouble
}

/// The index of one noun, as a dictionary writes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Index {
    /// What the stem ends in.
    pub kind:     Kind,
    /// Where the stress falls.
    pub accent:   Accent,
    /// Whether the stem parts with a fleeting vowel, written `*`.
    pub fleeting: bool,
    /// The circled numerals, each one cell by the other pattern.
    #[cfg_attr(feature = "serde", serde(default))]
    pub circled:  circled::Circled,
    /// Whether the stem trades `е` for `ё` with the stress, written `, ё`.
    ///
    /// `ё` is a stressed letter, and a word carrying this mark writes it in
    /// the last syllable of the stem exactly where the scheme holds the
    /// stress there: `жена́` but `жёны`, `мёд` but `меды́`. Without the mark
    /// a stem keeps its `е` under stress — `стена́`, `сте́ны` — which is why
    /// this is a stated fact and not a rule about every stem.
    #[cfg_attr(feature = "serde", serde(default))]
    pub yo:       bool,
    /// Whether the index carries marks the core has no rules for.
    ///
    /// The degree sign, the hypothetical-form signs `−` and `÷`, the `^` of
    /// the individual departures and the joints of compound indexes state
    /// departures the core has no rules for. A word carrying one declines as
    /// the pattern says except where it does not, and until each mark is
    /// written out, saying so is the honest answer.
    pub noted:    bool
}

/// Reads an index a dictionary states.
///
/// # Examples
///
/// ```
/// use rusem::grammar::declension::index::{Accent, Kind, read};
///
/// let plain = read("1a").expect("a stated index");
/// assert_eq!(plain.kind, Kind::Hard);
/// assert_eq!(plain.accent, Accent::A);
/// assert!(!plain.fleeting);
///
/// let parted = read("3*b").expect("a stated index");
/// assert_eq!(parted.kind, Kind::Velar);
/// assert_eq!(parted.accent, Accent::B);
/// assert!(parted.fleeting);
///
/// let yoed = read("1d, ё").expect("a stated index");
/// assert!(yoed.yo);
/// assert!(!yoed.noted);
///
/// let noted = read("1°a").expect("a stated index");
/// assert!(noted.noted);
///
/// assert!(read("gibberish").is_none());
/// ```
#[must_use]
pub fn read(written: &str) -> Option<Index> {
    let mut letters = written.chars().peekable();
    let digit = letters.next()?.to_digit(10)?;
    let kind = Kind::of(u8::try_from(digit).ok()?)?;

    let mut fleeting = false;
    let mut noted = false;
    while let Some(held) = letters.peek() {
        match held {
            '*' => fleeting = true,
            'a'..='f' => break,
            _ => noted = true
        }
        letters.next();
    }

    let accent = accent(&mut letters)?;
    let tail = marks::read(&mut letters);

    Some(Index {
        kind,
        accent,
        fleeting,
        circled: tail.circled,
        yo: tail.yo,
        noted: noted || tail.noted
    })
}

/// Reads the letter of the scheme and the primes after it.
///
/// The dictionary source types the primes three ways: an apostrophe, U+2032,
/// and the doubled prime as a straight quote or two apostrophes — `8f"` and
/// `8f''` are the same index. Each straight quote counts for two.
fn accent(letters: &mut core::iter::Peekable<core::str::Chars<'_>>) -> Option<Accent> {
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
        ('a', 0) => Some(Accent::A),
        ('b', 0) => Some(Accent::B),
        ('b', _) => Some(Accent::BPrime),
        ('c', 0) => Some(Accent::C),
        ('d', 0) => Some(Accent::D),
        ('d', _) => Some(Accent::DPrime),
        ('e', 0) => Some(Accent::E),
        ('f', 0) => Some(Accent::F),
        ('f', 1) => Some(Accent::FPrime),
        ('f', _) => Some(Accent::FDouble),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::{circled::Reach, *};

    fn stated(written: &str) -> Index {
        read(written).unwrap_or_else(|| unreachable!("a stated index"))
    }

    #[test]
    fn the_yo_mark_is_a_typed_fact_and_not_a_note() {
        let held = stated("4b, ё");

        assert!(held.yo);
        assert!(!held.noted);
        assert_eq!(held.kind, Kind::Sibilant);
        assert_eq!(held.accent, Accent::B);
    }

    #[test]
    fn the_star_and_the_yo_mark_stand_together() {
        let held = stated("1*d, ё");

        assert!(held.fleeting);
        assert!(held.yo);
        assert!(!held.noted);
    }

    #[test]
    fn a_circled_numeral_is_a_typed_fact_and_not_a_note() {
        let held = stated("1c(1)");

        assert_eq!(held.circled.nominative, Some(Reach::Whole));
        assert!(!held.noted);

        let both = stated("1c(1)(2)");

        assert_eq!(both.circled.nominative, Some(Reach::Whole));
        assert_eq!(both.circled.genitive, Some(Reach::Whole));

        assert_eq!(stated("7a(3)").circled.prepositional, Some(Reach::Whole));
        assert_eq!(stated("1a((2))").circled.genitive, Some(Reach::Either));
    }

    #[test]
    fn the_doubled_prime_is_read_in_both_spellings() {
        assert_eq!(stated("8f\"").accent, Accent::FDouble);
        assert_eq!(stated("8f''").accent, Accent::FDouble);
        assert!(!stated("8f\"").noted);
    }

    #[test]
    fn a_mark_without_a_rule_stays_noted() {
        assert!(stated("1°a").noted);
        assert!(stated("1a−").noted);
        assert!(stated("1b-").noted);
        assert!(stated("1a÷").noted);
        assert!(stated("1a^").noted);
        assert!(stated("1a− + 3*b").noted);
        assert!(stated("1*b // 1a").noted);
    }

    #[test]
    fn a_noted_mark_keeps_the_understood_ones_beside_it() {
        let held = stated("3d(1)−");

        assert_eq!(held.circled.nominative, Some(Reach::Whole));
        assert!(held.noted);

        let grown = stated("8°c, ё");

        assert!(grown.yo);
        assert!(grown.noted);
    }
}
