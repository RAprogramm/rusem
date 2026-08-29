// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The engine checked against the dictionary it does not read from.
//!
//! Every Russian noun in the raw ruwiktionary dump states its declension as
//! Zaliznyak's index and prints the forms beside it. The engine builds the
//! same forms from the index alone, so the dump is a measurement: for every
//! noun whose index the core reads in full, every printed cell of the six
//! stated cases is compared with what the core writes. A printed cell holding
//! several variants passes when the core's spelling matches any of them.
//! Nothing here feeds back into the engine — a wrong count is a fact to be
//! looked at, not a number to be lowered.
//!
//! The example is a binary, so nothing in it is reachable from outside and
//! `unreachable_pub` asks for `pub(crate)`, which the nursery lint calls
//! redundant inside a binary. One of the two has to yield, and it is the
//! nursery one.
#![allow(clippy::redundant_pub_crate)]

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
        Case, Number,
        declension::{self, index},
        form::{Agreed, Form}
    },
    lexis::{Noun, paradigm::noun},
    morphology::WordForm
};

/// How many nouns are checked before the stream is closed.
const CAP: usize = 20_000;

/// How many wrong cells are printed in full.
const SHOWN: usize = 30;

/// The running totals and the first wrong cells written out in full.
#[derive(Default)]
struct Tally {
    nouns:    usize,
    cells:    usize,
    wrong:    usize,
    by_index: BTreeMap<String, usize>,
    shown:    Vec<String>
}

fn main() -> Result<(), Error> {
    let raw = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/ruwiktionary.jsonl.gz");
    let lines = BufReader::new(GzDecoder::new(File::open(raw)?)).lines();

    let mut tally = Tally::default();
    for line in lines {
        if tally.nouns >= CAP {
            break;
        }
        if let Some(held) = entry::read(&line?) {
            check(&held, &mut tally);
        }
    }

    for (written, wrong) in &tally.by_index {
        println!("{written}: {wrong}");
    }
    for shown in &tally.shown {
        println!("{shown}");
    }
    println!(
        "nouns: {}, cells: {}, wrong: {}",
        tally.nouns, tally.cells, tally.wrong
    );
    Ok(())
}

/// Checks one dictionary noun against the paradigm the core fills.
fn check(held: &Entry, tally: &mut Tally) {
    let Some(stated) = index::read(&held.written) else {
        return;
    };
    if stated.noted {
        return;
    }
    let Ok(lemma) = WordForm::parse(&held.lemma) else {
        return;
    };

    let table = noun::of(
        &lemma,
        Noun {
            gender:     held.gender,
            animacy:    held.animacy,
            declension: declension::of(lemma.as_str(), held.gender),
            index:      Some(stated)
        }
    );

    tally.nouns += 1;
    for (case, number, printed) in &held.cells {
        let written = table
            .fills(cell(*case, *number, held))
            .first()
            .map_or("∅", WordForm::as_str);
        tally.cells += 1;
        if !printed.iter().any(|variant| variant == written) {
            miss(tally, held, *case, *number, written, printed);
        }
    }
}

/// Records one wrong cell: the per-index count always, the full line while
/// there is room.
fn miss(
    tally: &mut Tally,
    held: &Entry,
    case: Case,
    number: Number,
    core: &str,
    printed: &[String]
) {
    tally.wrong += 1;
    *tally.by_index.entry(held.written.clone()).or_insert(0) += 1;
    if tally.shown.len() < SHOWN {
        tally.shown.push(format!(
            "{} [{}] {:?} {:?}: core {} — dictionary {}",
            held.lemma,
            held.written,
            case,
            number,
            core,
            printed.join(" / ")
        ));
    }
}

/// The cell of the core's table a case and a number name.
const fn cell(case: Case, number: Number, held: &Entry) -> Form {
    Form::Noun(match number {
        Number::Singular => Agreed::Singular {
            case,
            gender: held.gender
        },
        Number::Plural => Agreed::Plural {
            case
        }
    })
}
