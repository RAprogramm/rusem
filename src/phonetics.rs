// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! How a word sounds, as against what it means.
//!
//! This is the level between the letters and the grammar, and it is not a step
//! on the way from one to the other: a syllable is not a smaller morpheme, and
//! a morpheme is not a bigger syllable. `водный` divides as `во-дный` by
//! sound and as `вод-н-ый` by sense, and the two boundaries fall in different
//! places on purpose. Neither module knows about the other.
//!
//! What lives here is what a reader needs to say a word aloud: how loud each
//! sound is, where the syllables part, and where the stress falls. Half the
//! orthography hangs on the last of those, which is why the code of 1956 keeps
//! asking, and § 117 asks about the first two.
//!
//! Everything here stands on [`crate::alphabet`] and on nothing else.

pub mod sonority;
pub mod stress;
pub mod syllable;

pub use self::{
    sonority::Sonority,
    stress::{Stress, Stressed},
    syllable::Syllable
};
