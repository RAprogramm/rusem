// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The verb engine checked against the dictionary it does not read from.
//!
//! Every Russian verb in the raw ruwiktionary dump states its conjugation as
//! Zaliznyak's index and prints the finite forms beside it. The engine builds
//! the same forms from the index alone, so the dump is a measurement: for
//! every verb whose index the core reads in full, every printed finite cell —
//! the present or the simple future by person and number, the past by gender
//! and number, the imperative — is compared with what the core writes. A
//! printed cell holding several variants passes when the core's spelling
//! matches any of them.
//!
//! A cell the stated road refuses — class 8's velar, class 14's nasal, a
//! starred index's vanishing vowel — is the core saying it does not know, not
//! a wrong form; such cells are counted apart, as refusals, and only a cell
//! the core fills with a spelling the dictionary never printed counts as
//! wrong. Nothing here feeds back into the engine — a wrong count is a fact
//! to be looked at, not a number to be lowered.
//!
//! The example is a binary, so nothing in it is reachable from outside and
//! `unreachable_pub` asks for `pub(crate)`, which the nursery lint calls
//! redundant inside a binary. One of the two has to yield, and it is the
//! nursery one.
#![allow(clippy::redundant_pub_crate)]

mod cell;
mod entry;

use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path
};

use entry::Entry;
use flate2::read::GzDecoder;
use rusem::{
    grammar::{
        conjugation::{self, index::VerbIndex},
        form::{Form, verb::VerbForm}
    },
    lexis::{Verb, paradigm::verb},
    morphology::WordForm
};

/// How many verbs are checked before the stream is closed.
const CAP: usize = 20_000;

/// How many wrong cells are printed in full.
const SHOWN: usize = 30;

/// The running totals and the first wrong cells written out in full.
#[derive(Default)]
struct Tally {
    verbs:    usize,
    cells:    usize,
    wrong:    usize,
    refused:  usize,
    by_class: BTreeMap<String, usize>,
    shown:    Vec<String>
}

fn main() -> Result<(), Error> {
    let raw = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/ruwiktionary.jsonl.gz");
    let lines = BufReader::new(GzDecoder::new(File::open(raw)?)).lines();

    let mut tally = Tally::default();
    for line in lines {
        if tally.verbs >= CAP {
            break;
        }
        if let Some(held) = entry::read(&line?) {
            check(&held, &mut tally);
        }
    }

    for (written, wrong) in &tally.by_class {
        println!("{written}: {wrong}");
    }
    for shown in &tally.shown {
        println!("{shown}");
    }
    println!(
        "verbs: {}, cells: {}, wrong: {}, refused: {}",
        tally.verbs, tally.cells, tally.wrong, tally.refused
    );
    Ok(())
}

/// Checks one dictionary verb against the paradigm the core fills.
fn check(held: &Entry, tally: &mut Tally) {
    let Some(stated) = conjugation::index::read(&held.written) else {
        return;
    };
    if stated.noted {
        return;
    }
    let Ok(lemma) = WordForm::parse(&held.lemma) else {
        return;
    };

    let table = verb::of(
        &lemma,
        Verb {
            aspect:       held.aspect,
            transitivity: held.transitivity,
            reflexive:    held.reflexive,
            conjugation:  conjugation::of(lemma.as_str()),
            index:        Some(stated)
        }
    );

    tally.verbs += 1;
    for (form, printed) in &held.cells {
        tally.cells += 1;
        let filled = table.fills(Form::Verb(*form));
        let Some(written) = filled.first().map(WordForm::as_str) else {
            tally.refused += 1;
            continue;
        };
        if !printed.iter().any(|variant| variant == written) {
            miss(tally, held, stated, *form, written, printed);
        }
    }
}

/// Records one wrong cell: the per-class count always, the full line while
/// there is room.
fn miss(
    tally: &mut Tally,
    held: &Entry,
    stated: VerbIndex,
    form: VerbForm,
    core: &str,
    printed: &[String]
) {
    tally.wrong += 1;
    *tally.by_class.entry(class(stated)).or_insert(0) += 1;
    if tally.shown.len() < SHOWN {
        tally.shown.push(format!(
            "{} [{}] {:?}: core {} — dictionary {}",
            held.lemma,
            held.written,
            form,
            core,
            printed.join(" / ")
        ));
    }
}

/// The class an index states, written the way the dictionary writes it: the
/// digit, the ring where the index carries one, `^` for an isolated verb.
///
/// The digit is padded to two places so that the report's string order is
/// the dictionary's numeric order, 1 to 16, with the isolated verbs last.
fn class(stated: VerbIndex) -> String {
    stated.kind.digit().map_or_else(
        || String::from("^"),
        |digit| {
            if stated.signs.ringed {
                format!("{digit:2}°")
            } else {
                format!("{digit:2}")
            }
        }
    )
}
