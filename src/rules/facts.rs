// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What a rule is allowed to know about a word.
//!
//! Before this type every paragraph took its own arguments: one wanted only
//! the letters, another the stress, a third whether the word is a proper name.
//! Nothing could ask them all the same question, so nothing could hold them in
//! one list — and a rule that cannot be held in a list has to be called by
//! name, which means the checker grows a line for every paragraph ever
//! written.
//!
//! So the facts are gathered once and every rule reads what it needs.
//!
//! # Why these facts and no others
//!
//! A rule of the code of 1956 speaks about a word as it stands on the page,
//! about what that word is, and — for the paragraphs on punctuation — about
//! what stands next to it. Nothing in the code needs more, and giving a rule
//! more would let it do the checker's work: reach across the sentence, look
//! things up, decide how badly a breach reads.

use crate::{grammar::form::Form, phonetics::stress::Stressed};

/// What is known about how the word is put together.
///
/// A paragraph about the seam between a prefix and a root cannot work from the
/// letters: `розыск` opens with the prefix `роз-` and `синий` opens with
/// letters that are also a prefix and is not a prefixed word. Which it is
/// belongs to the word, so the word says it — and says when it does not know,
/// which is a third answer and not a no.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Parts {
    /// The word is built with a prefix that many letters long.
    Prefixed(usize),
    /// The word is built without a prefix.
    Bare,
    /// Nothing is known about how the word is put together.
    Unknown
}

/// How a word stands on the page.
#[derive(Debug, Clone, Copy)]
pub struct Writing<'a> {
    /// The word as written, marks and all.
    pub written: &'a str,
    /// The word after it, when the rule is about a pair.
    pub next:    Option<&'a str>
}

/// What is known about the word itself.
#[derive(Debug, Clone, Copy)]
pub struct About<'a> {
    /// The form it stands in.
    pub form:   Form,
    /// Where it may be stressed.
    pub stress: &'a Stressed,
    /// Whether it is a word of Russian rather than a borrowing.
    pub native: bool,
    /// Whether it is a proper name.
    pub proper: bool,
    /// How the word is put together, as far as anything knows.
    pub parts:  Parts
}

/// Everything a rule may ask about a word.
///
/// Two groups and no methods. A rule reads `writing.written` or `about.stress`
/// and takes what it needs: naming the fields is shorter than naming a getter
/// for each of them, and it keeps the type what it is — a bundle of facts, not
/// an object with behaviour.
#[derive(Debug, Clone, Copy)]
pub struct Facts<'a> {
    /// How it stands on the page.
    pub writing: Writing<'a>,
    /// What it is.
    pub about:   About<'a>
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{PartOfSpeech, form::Form};

    fn facts<'a>(written: &'a str, stress: &'a Stressed) -> Facts<'a> {
        Facts {
            writing: Writing {
                written,
                next: Some("дом")
            },
            about:   About {
                form: Form::Adverb,
                stress,
                native: true,
                proper: false,
                parts: Parts::Unknown
            }
        }
    }

    #[test]
    fn every_fact_is_read_back_as_it_was_given() {
        let stress = Stressed::unknown();
        let held = facts("вода", &stress);

        assert_eq!(held.writing.written, "вода");
        assert_eq!(held.writing.next, Some("дом"));
        assert_eq!(held.about.form.part_of_speech(), PartOfSpeech::Adverb);
        assert!(held.about.native);
        assert!(!held.about.proper);
        assert!(!held.about.stress.is_settled());
    }

    #[test]
    fn a_word_standing_alone_has_nothing_after_it() {
        let stress = Stressed::unknown();
        let mut held = facts("вода", &stress);
        held.writing.next = None;

        assert_eq!(held.writing.next, None);
    }
}
