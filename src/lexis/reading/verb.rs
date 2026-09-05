// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Reading a written verb into the cells it could be standing in.
//!
//! The finite forms are read off the two stems and the endings, the same way a
//! reader reads them, and the participles are read as the agreeing words they
//! are once their own dictionary form is built.
//!
//! The imperative is read too, and both endings the stress could give are
//! admitted: the reader does not know which one the word takes, and refusing
//! both would refuse a form the language wrote. Writing is stricter — a table
//! states one form or none — and that is the difference between saying what a
//! word is and recognizing what someone wrote.
//!
//! Which road the finite cells are read down — the index a dictionary states
//! or the class derived from the infinitive — is the word's own fact,
//! [`Verb::road`], the same one the writer goes by: one road, both
//! directions, so the two cannot disagree about a word they both know. The
//! participles are read off the derived path either way, because that is the
//! path that writes them.

use crate::{
    grammar::{
        Tense, Voice,
        conjugation::{index::VerbIndex, participle, reading, reflexive, stated},
        form::{Bare, Form, verb::VerbForm}
    },
    lexis::lexeme::{Road, Verb}
};

/// The four participles a verb may have.
const PARTICIPLES: [(Voice, Tense); 4] = [
    (Voice::Active, Tense::Present),
    (Voice::Active, Tense::Past),
    (Voice::Passive, Tense::Present),
    (Voice::Passive, Tense::Past)
];

/// Reports whether the core takes on reading a cell of this shape.
///
/// A reader that cannot read a form must say so rather than answer nothing:
/// nothing is also what it answers for a form that is simply wrong, and the
/// two must not be confused.
#[must_use]
pub const fn reads(form: Form) -> bool {
    matches!(
        form,
        Form::Verb(
            VerbForm::Infinitive
                | VerbForm::Present { .. }
                | VerbForm::Past(_)
                | VerbForm::Imperative(_)
                | VerbForm::Participle { .. }
        )
    )
}

/// The cells of a verb a written form could be standing in.
#[must_use]
pub fn of(lemma: &str, held: Verb, written: &str) -> Vec<Form> {
    let plain = reflexive::bare(lemma).unwrap_or_else(|| String::from(lemma));
    let Some(bare) = parted(written, held.reflexive) else {
        return Vec::new();
    };

    let mut found = finite(&plain, &bare, held.road());
    found.extend(borne(&plain, &bare));

    found
}

/// The form without the particle a reflexive verb carries.
fn parted(written: &str, reflexive: bool) -> Option<String> {
    if !reflexive {
        return Some(String::from(written));
    }

    reflexive::bare(written)
}

/// The finite cells a form could be standing in, read by the road the word
/// was written by.
fn finite(plain: &str, bare: &str, road: Road<VerbIndex>) -> Vec<Form> {
    let mut found = Vec::new();

    if bare == plain {
        found.push(Form::Verb(VerbForm::Infinitive));
    }

    let cells = match road {
        Road::Stated(index) => stated::reading::cells(bare, plain, index),
        Road::Derived => reading::cells(bare, plain)
    };
    for cell in cells {
        found.push(Form::Verb(match cell.tense {
            Tense::Past => VerbForm::Past(cell.gender.map_or(Bare::Plural, Bare::Singular)),
            _ => cell
                .person
                .map_or(VerbForm::Imperative(cell.number), |person| {
                    VerbForm::Present {
                        person,
                        number: cell.number
                    }
                })
        }));
    }

    found
}

/// The participle cells a form could be standing in.
fn borne(plain: &str, bare: &str) -> Vec<Form> {
    let mut found = Vec::new();

    for (voice, tense) in PARTICIPLES {
        let Some(dictionary) = participle::dictionary(plain, voice, tense) else {
            continue;
        };
        for form in super::agreeing::of(&dictionary, bare) {
            let Form::Adjective(crate::grammar::form::Adjectival::Full(agreed)) = form else {
                continue;
            };
            found.push(Form::Verb(VerbForm::Participle {
                voice,
                tense,
                form: crate::grammar::form::verb::Participle::Full(agreed)
            }));
        }
    }

    found
}
