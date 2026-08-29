// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Reading a written adjective, participle or adjectival noun into its cells.
//!
//! An agreeing word collapses its categories heavily — `новой` is four cells
//! at once — and the reader hands back all of them. Which one is meant comes
//! from the noun the word leans on, and that is settled above, by the sentence.

use crate::grammar::{
    Animacy,
    declension::agreed as read,
    form::{Adjectival, Form}
};

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

    found
}
