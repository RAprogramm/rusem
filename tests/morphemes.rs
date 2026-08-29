// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The one law of the cutting: every cut spells the word it was cut from.
//!
//! How much of a marked-up corpus the cutting reproduces is a number that
//! moves as the tables grow, and it is measured, not asserted. What must hold
//! whatever the tables say is that a cut is a tiling: reassembling the parts
//! gives back the word, letter for letter.

use rusem::morphemics::cut;

#[test]
fn every_cut_spells_the_word_it_was_cut_from() {
    for word in [
        "водный",
        "переход",
        "перестройка",
        "подводник",
        "рыболов",
        "стол",
        "избежать",
        "приходить"
    ] {
        for held in cut::ways(word) {
            assert_eq!(held.spells(), word, "a cut of {word} spells another word");
        }
    }
}
