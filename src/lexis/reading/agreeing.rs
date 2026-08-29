// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Reading a written adjective, participle or adjectival noun into its cells.
//!
//! An agreeing word collapses its categories heavily — `новой` is four cells
//! at once — and the reader hands back all of them. Which one is meant comes
//! from the noun the word leans on, and that is settled above, by the
//! sentence.
//!
//! The short cells and the comparative are read the way the noun reader
//! reads: each cell the writer can spell is written out and compared with
//! what arrived, folding `ё` into `е` where everything folds it, so `тёмен`
//! and `темен` are the same form. A cell the writer refuses is a cell the
//! reader never offers.

use crate::{
    alphabet::vowel::same,
    grammar::{
        Animacy, Gender,
        declension::{adjective, agreed as read},
        form::{Adjectival, Bare, Form}
    }
};

/// The short cells a reader asks about, in the order the tables walk them.
const BARE: [Bare; 4] = [
    Bare::Singular(Gender::Masculine),
    Bare::Singular(Gender::Feminine),
    Bare::Singular(Gender::Neuter),
    Bare::Plural
];

/// The cells of an agreeing word a written form could be standing in.
///
/// Both animacies are asked, because the word does not carry one: `новых` is
/// the accusative of an animate noun's adjective and the genitive of anyone's,
/// and both readings are true of the form standing alone.
///
/// # Examples
///
/// ```
/// use rusem::lexis::reading::agreeing;
///
/// assert_eq!(agreeing::of("новый", "новым").len(), 3);
/// assert_eq!(agreeing::of("красный", "красен").len(), 1);
/// assert!(agreeing::of("новый", "читал").is_empty());
/// ```
#[must_use]
pub fn of(dictionary: &str, written: &str) -> Vec<Form> {
    let mut found: Vec<Form> = Vec::new();

    for animacy in [Animacy::Inanimate, Animacy::Animate] {
        for cell in read::cells(written, dictionary, animacy) {
            let held = Form::Adjective(Adjectival::Full(super::agreed(
                cell.case,
                cell.number,
                cell.gender.unwrap_or(crate::grammar::Gender::Masculine)
            )));
            if !found.contains(&held) {
                found.push(held);
            }
        }
    }

    for cell in BARE {
        if adjective::short::written(dictionary, cell).is_some_and(|held| same(&held, written)) {
            found.push(Form::Adjective(Adjectival::Short(cell)));
        }
    }
    if adjective::compared::written(dictionary).is_some_and(|held| same(&held, written)) {
        found.push(Form::Adjective(Adjectival::Compared));
    }

    found
}
