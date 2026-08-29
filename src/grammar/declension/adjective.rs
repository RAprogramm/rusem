// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Writing out an agreeing word, given its dictionary form and the cell wanted.
//!
//! The reader beside this one goes the other way: it takes a written form and
//! answers which cells could have spelled it. Both stand on the same two facts
//! — where the dictionary form ends and which shape it declines on — so the
//! splitting is stated here once and read from both directions.

pub mod compared;
pub mod short;

use super::{Stem, attributive, spelling};
use crate::grammar::{Animacy, Case, Gender, Number};

/// Splits a dictionary form into the stem and the shape it declines on.
///
/// The dictionary lists an adjective in the masculine nominative, whose ending
/// is two letters. Which shape they name is not always plain: `ий` spells the
/// soft shape, but it also spells the hard one on a stem that refuses `ы`, and
/// that is why `строгий` declines like `новый` and not like `синий`.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{declension::adjective::parted, stem::Stem};
///
/// assert_eq!(parted("новый"), Some((String::from("нов"), Stem::Hard)));
/// assert_eq!(parted("синий"), Some((String::from("син"), Stem::Soft)));
/// assert_eq!(parted("строгий"), Some((String::from("строг"), Stem::Hard)));
/// ```
#[must_use]
pub fn parted(dictionary: &str) -> Option<(String, Stem)> {
    let letters: Vec<char> = dictionary.chars().collect();
    let kept = letters.len().checked_sub(2)?;
    let base: String = letters.iter().take(kept).collect();
    let ending: String = letters.iter().skip(kept).collect();
    let last = *letters.get(kept.checked_sub(1)?)?;

    let shape = if ending.starts_with('и') && !spelling::bars_yi(last) {
        Stem::Soft
    } else {
        Stem::Hard
    };

    Some((base, shape))
}

/// Reports whether the dictionary form says the endings carry the stress.
///
/// The masculine nominative writes the stressed hard ending `-ой` and the
/// unstressed one `-ый` or `-ий` — Zaliznyak's schemes `b` and `a` — so the
/// dictionary form itself states where the stress falls, and the whole
/// paradigm follows it: `большой` keeps the `о` of `большого` that the
/// sibilant would bend to `е` off the stress, as `хороший` bends it in
/// `хорошего`.
#[must_use]
pub fn ending_stressed(dictionary: &str) -> bool {
    dictionary.ends_with("ой")
}

/// Reports whether a stem is closed by the relational suffix `-ск-`.
///
/// School grammar parts adjectives into qualitative and relational, and
/// states that a relational adjective has no short form and no degrees of
/// comparison: `русский`, `морской` name a relation, not a quality that could
/// hold more or less of itself. The suffix that builds them is `-ск-`, so a
/// stem closed by it refuses the short cells and the comparative. The letters
/// are all this check reads: a qualitative stem that happens to end in `ск` —
/// `плоский` — is refused with them, and an unstated cell is silence, not a
/// wrong form.
#[must_use]
pub fn relational(stem: &str) -> bool {
    stem.ends_with("ск")
}

/// The stem with its `ё` written `е`, for the cells that do not keep the
/// dictionary form's stress.
///
/// `ё` is a stressed letter and nothing else in Russian, and the full
/// paradigm keeps the stem's stress where the dictionary form put it —
/// `тёмный`, `тёмного`. The short forms and the comparative do not: `тёмный`
/// says `темна́` and `лёгкий` says `легче`, so the lemma's `ё` would assert a
/// stress those cells may have lost. The 1956 code writes `ё` only where a
/// misreading must be forestalled (§ 10), which makes `е` the spelling every
/// word admits, and the derived stem is written with it.
fn undotted(base: &str) -> String {
    base.replace('ё', "е")
}

/// The agreeing word written out in the cell asked for.
///
/// The animacy is asked for because the accusative repeats the nominative for
/// a thing and the genitive for a living being, and an adjective takes it from
/// the noun it leans on rather than stating one of its own.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Animacy, Case, Gender, Number, declension::adjective::written};
///
/// let held = written(
///     "новый",
///     Case::Instrumental,
///     Number::Singular,
///     Gender::Feminine,
///     Animacy::Inanimate
/// );
/// assert_eq!(held.as_deref(), Some("новой"));
///
/// let alive = written(
///     "новый",
///     Case::Accusative,
///     Number::Singular,
///     Gender::Masculine,
///     Animacy::Animate
/// );
/// assert_eq!(alive.as_deref(), Some("нового"));
/// ```
#[must_use]
pub fn written(
    dictionary: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy
) -> Option<String> {
    let (base, shape) = parted(dictionary)?;
    let stressed = ending_stressed(dictionary);
    let table = attributive::table(shape, gender, number, stressed);

    Some(base.clone() + &spelling::fitted(&base, table.of(case, animacy), stressed))
}
