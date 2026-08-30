// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Gathering the printed forms of one verb into the finite cells the check
//! compares.
//!
//! The dump tags each printed form in English. The present and the simple
//! future carry a person and a number; the past carries genders — one for a
//! singular, all three for the plural, which agrees in none; the imperative
//! of the second person carries no number at all, so the two forms it prints
//! are told apart by `-те`, the postfix Zaliznyak's dictionary states the
//! plural imperative adds to the singular — before the reflexive `-сь` when
//! the verb carries one, so `-те` and `-тесь` are the plural's two tails.
//!
//! What is not a finite cell is left where it lies: the participles decline
//! and are checked nowhere here, the compound future `буду читать` is two
//! words and no cell of the synthetic paradigm, and the hortative the dump
//! tags a first-person imperative is a form the core's paradigm does not
//! state.

mod variants;

use rusem::grammar::{
    Gender, Number, Person,
    form::{Bare, verb::VerbForm}
};
use serde_json::Value;

/// Gathers the printed forms into cells, one cell to a finite form, every
/// variant of the cell in one list.
pub(crate) fn read(forms: &[Value]) -> Vec<(VerbForm, Vec<String>)> {
    let mut held: Vec<(VerbForm, Vec<String>)> = Vec::new();

    for form in forms {
        let Some(printed) = form["form"].as_str() else {
            continue;
        };
        let variants = variants::read(printed);
        let Some(first) = variants.first() else {
            continue;
        };
        let Some(cell) = named(form, first) else {
            continue;
        };
        if let Some((_, kept)) = held.iter_mut().find(|(found, _)| *found == cell) {
            kept.extend(variants);
        } else {
            held.push((cell, variants));
        }
    }

    held
}

/// The finite cell one printed form's tags name, when they name one.
fn named(form: &Value, first: &str) -> Option<VerbForm> {
    let tags: Vec<&str> = form["tags"]
        .as_array()?
        .iter()
        .filter_map(Value::as_str)
        .collect();

    if tags
        .iter()
        .any(|tag| matches!(*tag, "participle" | "adverbial" | "infinitive"))
    {
        return None;
    }
    if tags.contains(&"imperative") {
        return bidden(&tags, first);
    }
    if tags.contains(&"past") {
        return gone(&tags);
    }
    if tags.contains(&"present") || tags.contains(&"future") {
        return current(&tags);
    }
    None
}

/// The imperative cell, told singular from plural by the `-те` tail.
///
/// Only the second person is a cell of the paradigm; the hortative the dump
/// tags `first-person` is refused here, not compared.
fn bidden(tags: &[&str], first: &str) -> Option<VerbForm> {
    if !tags.contains(&"second-person") {
        return None;
    }
    let number = if first.ends_with("те") || first.ends_with("тесь") {
        Number::Plural
    } else {
        Number::Singular
    };
    Some(VerbForm::Imperative(number))
}

/// The past cell: one stated gender is a singular, all three at once are the
/// plural, which agrees in none.
fn gone(tags: &[&str]) -> Option<VerbForm> {
    let genders: Vec<Gender> = tags
        .iter()
        .filter_map(|tag| match *tag {
            "masculine" => Some(Gender::Masculine),
            "feminine" => Some(Gender::Feminine),
            "neuter" => Some(Gender::Neuter),
            _ => None
        })
        .collect();
    match genders.as_slice() {
        [one] => Some(VerbForm::Past(Bare::Singular(*one))),
        [_, _, _] => Some(VerbForm::Past(Bare::Plural)),
        _ => None
    }
}

/// The present cell, which is also where a perfective verb's simple future
/// stands — the dump tags it `future` and the paradigm writes it in the
/// present cells, both stating the same one shape.
///
/// The compound future of an imperfective verb is printed with no person or
/// number and so never names a cell.
fn current(tags: &[&str]) -> Option<VerbForm> {
    let mut person = None;
    let mut number = None;
    for tag in tags {
        match *tag {
            "first-person" => person = Some(Person::First),
            "second-person" => person = Some(Person::Second),
            "third-person" => person = Some(Person::Third),
            "singular" => number = Some(Number::Singular),
            "plural" => number = Some(Number::Plural),
            _ => {}
        }
    }
    Some(VerbForm::Present {
        person: person?,
        number: number?
    })
}
