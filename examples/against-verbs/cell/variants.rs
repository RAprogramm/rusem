// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Splitting one printed cell into the variants it holds.
//!
//! Variants stand in one cell behind `//`, and a table of doubled stress
//! writes them behind a single ` / `; splitting on the one slash reads both,
//! since the empty piece between two of them is dropped. The dictionary also
//! prints an alternative in parentheses after the main form — `га́снул
//! (га́с)` — and one group may hold several behind commas — `есть (есь^†,
//! еси́^†)` — so every parenthesized group opens into further variants of the
//! same cell. A form may carry a leading usage label, a word the dictionary
//! closes with a period — `устар. свящу́` — which names when the form is
//! used, not how it is written, so it goes.
//!
//! Each variant is then cleaned to plain lowercase letters. The dictionary
//! prints stress with the combining acute and grave, marks footnotes with
//! `^`, `△`, `*` and the dagger `†` of an archaic form, and writes the
//! secondary stress on `и` and `е` as the precomposed `ѝ` and `ѐ`, which are
//! not letters of the alphabet and go back to the letters they dress. A cell
//! the dictionary declines to fill prints the bare footnote star and nothing
//! else — the imperative of `видеть` is `*` — and once the marks go, such a
//! cell is empty and drops out: the dictionary printed no form there. `ё` is
//! left exactly as printed: whether the engine writes it where the dictionary
//! does is part of what is checked.

/// The variants one printed cell holds, each cleaned to plain lowercase
/// letters, the form printed outside any parentheses standing first.
pub(crate) fn read(printed: &str) -> Vec<String> {
    printed
        .split('/')
        .flat_map(opened)
        .map(|variant| cleaned(unlabeled(&variant)))
        .filter(|variant| !variant.is_empty())
        .collect()
}

/// Opens one slash-separated piece into the form outside parentheses
/// followed by every form its parenthesized groups hold behind commas.
fn opened(piece: &str) -> Vec<String> {
    let mut outside = String::new();
    let mut grouped = Vec::new();
    let mut rest = piece;
    while let Some((before, held)) = rest.split_once('(') {
        outside.push_str(before);
        let (inside, tail) = held.split_once(')').unwrap_or((held, ""));
        grouped.extend(inside.split(',').map(str::to_owned));
        rest = tail;
    }
    outside.push_str(rest);
    let mut found = vec![outside];
    found.append(&mut grouped);
    found
}

/// The variant with its leading usage labels shed: a label is a word the
/// dictionary closes with a period — `устар.`, `разг.` — naming when the
/// form is used, no part of its letters.
fn unlabeled(variant: &str) -> &str {
    let held = variant.trim();
    match held.split_once(char::is_whitespace) {
        Some((label, tail)) if label.ends_with('.') => unlabeled(tail),
        _ => held
    }
}

/// The variant cleaned to plain lowercase letters: the stress and footnote
/// marks go, the precomposed grave letters return to the letters they dress.
fn cleaned(variant: &str) -> String {
    variant
        .trim()
        .chars()
        .filter_map(|letter| match letter {
            '\u{301}' | '\u{300}' | '^' | '△' | '*' | '†' => None,
            'ѝ' => Some('и'),
            'ѐ' => Some('е'),
            'Ѝ' => Some('И'),
            'Ѐ' => Some('Е'),
            kept => Some(kept)
        })
        .flat_map(char::to_lowercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_parenthesized_alternative_is_a_variant_of_the_same_cell() {
        assert_eq!(read("га́снул (га́с)"), ["гаснул", "гас"]);
    }

    #[test]
    fn a_doubled_stress_in_parentheses_reads_as_the_same_letters_twice() {
        assert_eq!(read("взя́лся (взялся́)"), ["взялся", "взялся"]);
    }

    #[test]
    fn a_group_holds_variants_behind_commas_and_sheds_the_dagger() {
        assert_eq!(read("есть (есь^†, еси́^†)"), ["есть", "есь", "еси"]);
    }

    #[test]
    fn a_leading_usage_label_goes() {
        assert_eq!(read("устар. свящу́"), ["свящу"]);
    }

    #[test]
    fn the_form_outside_the_parentheses_stands_first() {
        assert_eq!(read("будь (бу́ди^†)"), ["будь", "буди"]);
    }

    #[test]
    fn the_slash_still_splits_variants() {
        assert_eq!(read("про́клял / прокля́л"), ["проклял", "проклял"]);
    }

    #[test]
    fn a_bare_footnote_star_is_no_form() {
        assert_eq!(read("*"), Vec::<String>::new());
    }

    #[test]
    fn an_unclosed_group_reads_to_the_end_of_the_piece() {
        assert_eq!(read("га́снул (га́с"), ["гаснул", "гас"]);
    }
}
