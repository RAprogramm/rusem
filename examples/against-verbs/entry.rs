// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Reading one dictionary line into the shape the check compares against.

use rusem::grammar::{Aspect, Transitivity, form::verb::VerbForm};
use serde_json::Value;

use crate::cell;

/// One verb as the dictionary prints it: the lemma, the stated index, what
/// the word is, and every printed finite cell with its variants.
pub(crate) struct Entry {
    /// The dictionary form.
    pub(crate) lemma:        String,
    /// The index as the category writes it, `1a`, `3°a`, `7b/b`.
    pub(crate) written:      String,
    /// The aspect the tags state.
    pub(crate) aspect:       Aspect,
    /// The transitivity the tags state.
    pub(crate) transitivity: Transitivity,
    /// Whether the tags call the verb reflexive.
    pub(crate) reflexive:    bool,
    /// The printed cells: a finite form, and every variant printed in it.
    pub(crate) cells:        Vec<(VerbForm, Vec<String>)>
}

/// The lead of the category that states the index, as the dump writes it on
/// all but a handful of pages.
const CATEGORY: &str = "Глаголы, спряжение ";

/// The same category's fuller spelling, which a few pages carry instead.
const NAMED: &str = "Русские глаголы, спряжение ";

/// Reads one JSONL line into an [`Entry`], or nothing when the line is not a
/// Russian verb with a stated index and a single stated aspect and
/// transitivity.
///
/// A verb the dump tags `biaspectual` states both aspects at once, which the
/// core's [`Aspect`] does not write, so such a verb is left out the same way
/// a noun of two stated genders is.
pub(crate) fn read(line: &str) -> Option<Entry> {
    let held: Value = serde_json::from_str(line).ok()?;
    if held["lang_code"].as_str() != Some("ru") || held["pos"].as_str() != Some("verb") {
        return None;
    }

    let written = held["categories"]
        .as_array()?
        .iter()
        .filter_map(Value::as_str)
        .find_map(|name| {
            name.strip_prefix(CATEGORY)
                .or_else(|| name.strip_prefix(NAMED))
        })?
        .to_owned();

    let tags = held["tags"].as_array()?;
    let aspect = aspect(tags)?;
    let transitivity = transitivity(tags)?;
    let reflexive = tags
        .iter()
        .filter_map(Value::as_str)
        .any(|tag| tag == "reflexive");
    let cells = cell::read(held["forms"].as_array()?);

    Some(Entry {
        lemma: held["word"].as_str()?.to_owned(),
        written,
        aspect,
        transitivity,
        reflexive,
        cells
    })
}

/// The one aspect the tags state, or nothing when they state none or both.
///
/// The dump writes the perfective as `perfect`, and a `biaspectual` tag
/// states both aspects in one word, so it fills the list past one and the
/// verb is left out.
fn aspect(tags: &[Value]) -> Option<Aspect> {
    let mut stated = Vec::new();
    for tag in tags.iter().filter_map(Value::as_str) {
        match tag {
            "imperfective" => stated.push(Aspect::Imperfective),
            "perfect" => stated.push(Aspect::Perfective),
            "biaspectual" => stated.extend([Aspect::Imperfective, Aspect::Perfective]),
            _ => {}
        }
    }
    match stated.as_slice() {
        [one] => Some(*one),
        _ => None
    }
}

/// The one transitivity the tags state, or nothing when they state none or
/// two.
fn transitivity(tags: &[Value]) -> Option<Transitivity> {
    let stated: Vec<Transitivity> = tags
        .iter()
        .filter_map(Value::as_str)
        .filter_map(|tag| match tag {
            "transitive" => Some(Transitivity::Transitive),
            "intransitive" => Some(Transitivity::Intransitive),
            _ => None
        })
        .collect();
    match stated.as_slice() {
        [one] => Some(*one),
        _ => None
    }
}
