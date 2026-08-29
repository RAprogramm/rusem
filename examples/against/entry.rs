// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Reading one dictionary line into the shape the check compares against.

use rusem::grammar::{Animacy, Case, Gender, Number};
use serde_json::Value;

/// One noun as the dictionary prints it: the lemma, the stated index, what
/// the word is, and every printed cell with its variants.
pub(crate) struct Entry {
    /// The dictionary form.
    pub(crate) lemma:   String,
    /// The index as the category writes it, `1a`, `3*b`.
    pub(crate) written: String,
    /// The gender the tags state.
    pub(crate) gender:  Gender,
    /// The animacy the tags state.
    pub(crate) animacy: Animacy,
    /// The printed cells: a case and a number, and every variant printed
    /// in that cell.
    pub(crate) cells:   Vec<(Case, Number, Vec<String>)>
}

/// The lead of the category that states the index.
const CATEGORY: &str = "Русские существительные, склонение ";

/// Reads one JSONL line into an [`Entry`], or nothing when the line is not a
/// Russian noun with a stated index and a single stated gender and animacy.
pub(crate) fn read(line: &str) -> Option<Entry> {
    let held: Value = serde_json::from_str(line).ok()?;
    if held["lang_code"].as_str() != Some("ru") || held["pos"].as_str() != Some("noun") {
        return None;
    }

    let written = held["categories"]
        .as_array()?
        .iter()
        .filter_map(Value::as_str)
        .find_map(|name| name.strip_prefix(CATEGORY))?
        .to_owned();

    let tags = held["tags"].as_array()?;
    let gender = gender(tags)?;
    let animacy = animacy(tags)?;
    let cells = cells(held["forms"].as_array()?);

    Some(Entry {
        lemma: held["word"].as_str()?.to_owned(),
        written,
        gender,
        animacy,
        cells
    })
}

/// The one gender the tags state, or nothing when they state none or two.
fn gender(tags: &[Value]) -> Option<Gender> {
    let stated: Vec<Gender> = tags
        .iter()
        .filter_map(Value::as_str)
        .filter_map(|tag| match tag {
            "masculine" => Some(Gender::Masculine),
            "feminine" => Some(Gender::Feminine),
            "neuter" => Some(Gender::Neuter),
            "common" => Some(Gender::Common),
            _ => None
        })
        .collect();
    match stated.as_slice() {
        [one] => Some(*one),
        _ => None
    }
}

/// The one animacy the tags state, or nothing when they state none or two.
fn animacy(tags: &[Value]) -> Option<Animacy> {
    let stated: Vec<Animacy> = tags
        .iter()
        .filter_map(Value::as_str)
        .filter_map(|tag| match tag {
            "animate" => Some(Animacy::Animate),
            "inanimate" => Some(Animacy::Inanimate),
            _ => None
        })
        .collect();
    match stated.as_slice() {
        [one] => Some(*one),
        _ => None
    }
}

/// Gathers the printed forms into cells, one cell to a case and a number,
/// every variant of the cell in one list.
fn cells(forms: &[Value]) -> Vec<(Case, Number, Vec<String>)> {
    let mut held: Vec<(Case, Number, Vec<String>)> = Vec::new();

    for form in forms {
        let Some((case, number)) = named(form) else {
            continue;
        };
        let Some(printed) = form["form"].as_str() else {
            continue;
        };
        let variants = variants(printed);
        if let Some((_, _, held)) = held.iter_mut().find(|(a, b, _)| *a == case && *b == number) {
            held.extend(variants);
        } else {
            held.push((case, number, variants));
        }
    }

    held
}

/// The case and the number one printed form's tags name, when they name
/// exactly one of each of the six stated cases.
fn named(form: &Value) -> Option<(Case, Number)> {
    let tags = form["tags"].as_array()?;
    let mut case = None;
    let mut number = None;

    for tag in tags.iter().filter_map(Value::as_str) {
        match tag {
            "singular" => number = Some(Number::Singular),
            "plural" => number = Some(Number::Plural),
            "nominative" => case = Some(Case::Nominative),
            "genitive" => case = Some(Case::Genitive),
            "dative" => case = Some(Case::Dative),
            "accusative" => case = Some(Case::Accusative),
            "instrumental" => case = Some(Case::Instrumental),
            "prepositional" => case = Some(Case::Prepositional),
            _ => {}
        }
    }

    Some((case?, number?))
}

/// The variants one printed cell holds, each cleaned to plain lowercase
/// letters.
///
/// The dictionary prints stress with the combining acute and grave, marks
/// footnotes with `^` and `△`, and writes the secondary stress on `и` and `е`
/// as the precomposed `ѝ` and `ѐ`, which are not letters of the alphabet and
/// go back to the letters they dress. `ё` is left exactly as printed: whether
/// the engine writes it where the dictionary does is part of what is checked.
fn variants(printed: &str) -> Vec<String> {
    printed
        .split("//")
        .map(|variant| {
            variant
                .trim()
                .chars()
                .filter_map(|letter| match letter {
                    '\u{301}' | '\u{300}' | '^' | '△' => None,
                    'ѝ' => Some('и'),
                    'ѐ' => Some('е'),
                    'Ѝ' => Some('И'),
                    'Ѐ' => Some('Е'),
                    kept => Some(kept)
                })
                .flat_map(char::to_lowercase)
                .collect()
        })
        .filter(|variant: &String| !variant.is_empty())
        .collect()
}
