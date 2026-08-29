// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What a rule found wrong, where, and what should stand instead.
//!
//! A rule judges. It does not hand out a requirement for somebody else to
//! compare against the writing — that leaves the comparison outside the rule,
//! and with it the decision of what counts as broken. It reads what is written
//! and answers with what it found, which is nothing at all when the writing is
//! right.
//!
//! Three things in a finding, and every one of them is needed by whoever reads
//! it: where the mistake is, what is wrong in words a person can read, and
//! what to write instead. The last is what makes a finding worth having: a
//! judgement nobody can act on is an opinion.

use std::{string::String, vec::Vec};

use crate::rules::Citation;

/// One mistake a rule found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    /// Which paragraph found it.
    pub cites:   Citation,
    /// Where it stands, counting characters from the start of what was given.
    pub at:      usize,
    /// What is wrong, in the words of the paragraph.
    pub says:    &'static str,
    /// What should stand there instead.
    pub instead: String
}

impl Found {
    /// States a finding at a place.
    #[must_use]
    pub const fn new(cites: Citation, at: usize, says: &'static str, instead: String) -> Self {
        Self {
            cites,
            at,
            says,
            instead
        }
    }
}

/// Everything one rule found, which is usually nothing.
pub type Findings = Vec<Found>;

/// The word with one letter written the way a paragraph requires.
///
/// The spelling a finding offers is the whole word, not the letter: a reader
/// puts the word in place of the word, and a caller that had to assemble it
/// from a place and a letter would be doing the rule's work.
#[must_use]
pub fn spelled(written: &str, at: usize, letter: char) -> String {
    written
        .chars()
        .enumerate()
        .map(|(place, held)| if place == at { letter } else { held })
        .collect()
}

/// The word without the letter standing at a place.
#[must_use]
pub fn without(written: &str, at: usize) -> String {
    written
        .chars()
        .enumerate()
        .filter(|(place, _)| *place != at)
        .map(|(_, held)| held)
        .collect()
}
