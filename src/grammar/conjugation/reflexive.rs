// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The particle a reflexive verb carries after its ending.
//!
//! One rule and no exceptions: after a vowel the particle is written `-сь`,
//! everywhere else `-ся`. `учусь` but `учишься`, `училась` but `учился`,
//! `учитесь` but `учиться` — the ending decides, and the ending is already
//! written by the time the particle is added, which is why this is asked of
//! the written form rather than of the verb.

use crate::alphabet::is_vowel;

/// The particle after a vowel.
const AFTER_VOWEL: &str = "сь";

/// The particle everywhere else.
const AFTER_CONSONANT: &str = "ся";

/// The particle this form takes.
///
/// # Examples
///
/// ```
/// use rusem::grammar::conjugation::reflexive::particle;
///
/// assert_eq!(particle("учу"), "сь");
/// assert_eq!(particle("учишь"), "ся");
/// assert_eq!(particle("учил"), "ся");
/// ```
#[must_use]
pub fn particle(written: &str) -> &'static str {
    match written.chars().last() {
        Some(last) if is_vowel(last) => AFTER_VOWEL,
        _ => AFTER_CONSONANT
    }
}

/// The form with its particle written after it.
///
/// # Examples
///
/// ```
/// use rusem::grammar::conjugation::reflexive::attached;
///
/// assert_eq!(attached("учу"), "учусь");
/// assert_eq!(attached("учит"), "учится");
/// assert_eq!(attached("учила"), "училась");
/// ```
#[must_use]
pub fn attached(written: &str) -> String {
    String::from(written) + particle(written)
}

/// The verb without the particle, if it carries one.
///
/// # Examples
///
/// ```
/// use rusem::grammar::conjugation::reflexive::bare;
///
/// assert_eq!(bare("учиться").as_deref(), Some("учить"));
/// assert_eq!(bare("вернуться").as_deref(), Some("вернуть"));
/// assert_eq!(bare("читать"), None);
/// ```
#[must_use]
pub fn bare(infinitive: &str) -> Option<String> {
    infinitive
        .strip_suffix(AFTER_CONSONANT)
        .or_else(|| infinitive.strip_suffix(AFTER_VOWEL))
        .map(String::from)
}
