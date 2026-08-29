// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The stem an index leaves once the dictionary form's own ending is off.
//!
//! What has to come off depends on the gender and on what the stem ends in,
//! both of which the index and the dictionary state. A masculine noun of the
//! hard type ends in the stem itself — `стол` is `стол-` — while one of the
//! glide type ends in the glide it declines on, so `бой` is `бо-` plus the `й`
//! that every one of its endings begins with.

use crate::grammar::{
    Gender,
    declension::index::{Index, Kind}
};

/// The stem of a noun with a stated index.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Gender,
///     declension::{index, stated::stem::of}
/// };
///
/// let hard = index::read("1a").expect("a stated index");
/// assert_eq!(of("стол", Gender::Masculine, hard).as_deref(), Some("стол"));
///
/// let soft = index::read("2a").expect("a stated index");
/// assert_eq!(of("конь", Gender::Masculine, soft).as_deref(), Some("кон"));
///
/// let feminine = index::read("1a").expect("a stated index");
/// assert_eq!(
///     of("книга", Gender::Feminine, feminine).as_deref(),
///     Some("книг")
/// );
///
/// let iotated = index::read("7a").expect("a stated index");
/// assert_eq!(
///     of("армия", Gender::Feminine, iotated).as_deref(),
///     Some("арми")
/// );
/// ```
#[must_use]
pub fn of(lemma: &str, gender: Gender, index: Index) -> Option<String> {
    let letters: Vec<char> = lemma.chars().collect();
    let last = *letters.last()?;
    let cut = cut(gender, index.kind, last);
    let cut = if matches!(last, 'а' | 'я') { 1 } else { cut };
    let kept = letters.len().checked_sub(cut)?;

    Some(letters.into_iter().take(kept).collect())
}

/// How many letters the dictionary form ends in that are not the stem.
///
/// A noun written with `-а` or `-я` ends in its own vowel, and it comes off
/// whatever its gender: `мужчина` declines on `мужчин-` like `книга` on
/// `книг-`. So does the vowel of a neuter, and so does the sign or the glide a
/// masculine soft noun is written with, because the endings carry their own. A
/// masculine noun of the hard types ends in the stem and nothing comes off.
const fn cut(gender: Gender, kind: Kind, last: char) -> usize {
    match gender {
        Gender::Feminine | Gender::Neuter | Gender::Common => 1,
        Gender::Masculine => match (kind, last) {
            (Kind::Soft | Kind::Glide | Kind::Iotated, _) | (Kind::Third, 'ь') => 1,
            _ => 0
        }
    }
}
