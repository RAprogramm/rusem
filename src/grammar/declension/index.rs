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

pub mod falls;

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
    /// Whether the index carries marks the core has no rules for.
    ///
    /// The circled numerals, the degree sign and the rest state departures
    /// from the pattern, one departure to a mark. A word carrying one declines
    /// as the pattern says except where it does not, and until each mark is
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
/// let noted = read("1*a(2)").expect("a stated index");
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
    if letters.next().is_some() {
        noted = true;
    }

    Some(Index {
        kind,
        accent,
        fleeting,
        noted
    })
}

/// Reads the letter of the scheme and the primes after it.
fn accent(letters: &mut core::iter::Peekable<core::str::Chars<'_>>) -> Option<Accent> {
    let letter = letters.next()?;
    let mut primes = 0_usize;
    while letters
        .peek()
        .is_some_and(|held| *held == '\'' || *held == '′')
    {
        primes += 1;
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
