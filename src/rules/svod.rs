// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The code of rules of Russian orthography and punctuation of 1956.
//!
//! Two hundred and three paragraphs, one module to a paragraph, named by what
//! the paragraph is about rather than by its number — the number is stated
//! inside, where it can be read beside the rule it belongs to.
//!
//! A paragraph is written here when the engine has the facts to ask it. § 1
//! needs only the letters; § 40 needs the case, the stem and the place of the
//! stress. A paragraph whose facts the engine does not yet hold stays in
//! `rules/svod1956.txt` as the text it always was, because a rule that cannot
//! be asked is not a rule the engine has — it is a file that compiles.

pub mod hyphen_compound;
pub mod interjection_comma;
pub mod ne_together;
pub mod ni_together;
pub mod prefix_before_i;
pub mod sibilant_vowels;
pub mod soft_sign;
pub mod ts_vowels;
pub mod unstressed_ending;
pub mod unstressed_o;
pub mod written;
