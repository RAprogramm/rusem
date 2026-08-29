// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The vowel that is there in one cell of a paradigm and gone in the rest.
//!
//! The index writes it as a star. What the star says depends on where the
//! word's own ending is, not on its gender: a noun whose nominative has no
//! ending carries the vowel there and drops it everywhere else — `платок`,
//! `платка`, `платку` — while a noun whose nominative ends in a vowel has it
//! nowhere until the genitive plural takes that vowel away and leaves two
//! consonants to be parted: `сосна`, `сосен`; `окно`, `окон`; and `дедушка`,
//! `дедушек`, masculine though it is.
//!
//! So the two are one rule seen from two sides: the vowel stands where nothing
//! else follows the stem. Which vowel it is follows from what stands around
//! it, and that is [`crate::grammar::declension::spelling::parted`], stated
//! once and used by both.

use crate::{
    alphabet::is_vowel,
    grammar::{
        Animacy, Case, Gender, Number,
        declension::{index::Kind, spelling}
    },
    rules::svod::soft_sign
};

/// What the word is, as far as the fleeting vowel is concerned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word {
    /// The gender the dictionary states.
    pub gender:  Gender,
    /// What the stem ends in.
    pub kind:    Kind,
    /// Whether the word names something alive.
    pub animacy: Animacy,
    /// Whether the word declines by the first paradigm.
    pub opens:   bool
}

/// The stem of a word whose index carries the star, in one cell.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Animacy, Case, Gender, Number,
///     declension::{
///         index::Kind,
///         stated::fleeting::{Word, of}
///     }
/// };
///
/// let held = Word {
///     gender:  Gender::Masculine,
///     kind:    Kind::Velar,
///     animacy: Animacy::Inanimate,
///     opens:   false
/// };
/// assert_eq!(
///     of("платок", held, Case::Genitive, Number::Singular, false),
///     "платк"
/// );
/// assert_eq!(
///     of("платок", held, Case::Nominative, Number::Singular, false),
///     "платок"
/// );
///
/// let feminine = Word {
///     gender:  Gender::Feminine,
///     kind:    Kind::Hard,
///     animacy: Animacy::Inanimate,
///     opens:   true
/// };
/// assert_eq!(
///     of("сосн", feminine, Case::Genitive, Number::Plural, false),
///     "сосен"
/// );
/// ```
#[must_use]
pub fn of(stem: &str, word: Word, case: Case, number: Number, stressed: bool) -> String {
    let Word {
        gender,
        kind,
        animacy,
        opens
    } = word;

    if matches!(gender, Gender::Masculine) && !opens {
        return if standing(case, number, animacy) {
            String::from(stem)
        } else {
            dropped(stem)
        };
    }

    if bare(case, number, animacy) {
        return kept(&spelling::parted(stem, stressed), kind);
    }

    String::from(stem)
}

/// The parted stem with the softness of a final `л` written out.
///
/// `цапля` parts as `цапел` and is written `цапель`, `земля` as `земель`,
/// `кровля` as `кровель`. Nothing else keeps its softness there: `песня`
/// gives `песен` and `вишня` gives `вишен`.
fn kept(parted: &str, kind: Kind) -> String {
    if matches!(kind, Kind::Soft) && parted.ends_with('л') {
        return String::from(parted) + "ь";
    }

    String::from(parted)
}

/// Reports whether a cell is the one that takes no vowel after the stem.
///
/// The genitive plural, and the accusative plural of a living being because it
/// repeats the genitive. Those are the cells where two consonants would close
/// the word, and where the vowel comes back to part them.
const fn bare(case: Case, number: Number, animacy: Animacy) -> bool {
    if !matches!(number, Number::Plural) {
        return false;
    }

    match case.merged() {
        Case::Genitive => true,
        Case::Accusative => matches!(animacy, Animacy::Animate),
        _ => false
    }
}

/// Reports whether a masculine noun is in the cell that keeps the vowel.
///
/// The nominative, and the accusative of a thing because it repeats the
/// nominative. Everywhere else an ending follows the stem and the vowel goes.
const fn standing(case: Case, number: Number, animacy: Animacy) -> bool {
    if !matches!(number, Number::Singular) {
        return false;
    }

    match case.merged() {
        Case::Nominative | Case::Vocative => true,
        Case::Accusative => matches!(animacy, Animacy::Inanimate),
        _ => false
    }
}

/// The stem without the vowel that stands between its last two consonants.
///
/// `платок` gives `платк` and `отец` gives `отц`: the vowel goes and nothing
/// stands in its place. Two things do stand in its place. A vowel before the
/// fleeting one means the softness was carried by a glide, and the glide is
/// written: `боец` gives `бойц`. And `ё`, or `е` after `л`, leaves the
/// softness of the consonant before it, which is then written with a sign:
/// `конёк` gives `коньк`, `лев` gives `льв`.
///
/// A stem with no such vowel is left as it is: the star is a fact about the
/// word, and a word that carries it and shows nothing to drop is not made up
/// for here.
#[must_use]
pub fn dropped(stem: &str) -> String {
    let letters: Vec<char> = stem.chars().collect();
    let Some((at, held)) = fleeting(&letters) else {
        return String::from(stem);
    };
    let before = at
        .checked_sub(1)
        .and_then(|place| letters.get(place))
        .copied();
    let after = letters.get(at + 1).copied();

    let mut written: String = letters.iter().take(at).copied().collect();
    if let Some(mark) = left(held, before, after) {
        written.push(mark);
    }
    written.extend(letters.iter().skip(at + 1));
    written
}

/// The letter the dropped vowel leaves behind, if it leaves one.
///
/// The sign is not decided here: § 72 decides it and is asked. The consonant
/// that stood before the vowel was soft — that is what a `е` or a `ё` after it
/// means — and of the paragraph's points only the fourth can be asked off the
/// letters: `л` keeps its softness written, `лев` gives `львы`. The others
/// need to know whether that softness survives the change — `конец` drops it
/// in `конца`, `конёк` keeps it in `коньки` — and nothing here holds that
/// fact, so nothing is claimed: the missed sign of `коньки` is the honest
/// remainder, not a case to be patched.
///
/// What is decided here is the glide, which is what a vowel before the
/// fleeting one means — the softness was carried by `й`, and the letter comes
/// back when the vowel that hid it goes.
fn left(held: char, before: Option<char>, after: Option<char>) -> Option<char> {
    if before.is_some_and(is_vowel) {
        return Some('й');
    }
    if !matches!(held, 'е' | 'ё') {
        return None;
    }

    let (Some(before), Some(after)) = (before, after) else {
        return None;
    };

    soft_sign::before_l::keeps(before, after).then_some(soft_sign::SIGN)
}

/// Where the fleeting vowel stands and which it is: the last vowel with a
/// consonant after it and nothing but consonants to the end.
///
/// The vowel comes back with its place so that the caller holds what was
/// found rather than looking the place up again and answering for the day
/// the two could disagree.
fn fleeting(letters: &[char]) -> Option<(usize, char)> {
    let last = letters.len().checked_sub(1)?;
    if is_vowel(*letters.get(last)?) {
        return None;
    }

    (0..last).rev().find_map(|place| {
        letters
            .get(place)
            .copied()
            .filter(|held| is_vowel(*held))
            .map(|held| (place, held))
    })
}
