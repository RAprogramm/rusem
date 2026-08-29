// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What the core writes, the core reads back into the same cell.
//!
//! Writing a form from a cell and reading a cell from a form are not two
//! pieces of knowledge that happen to agree. They are one piece asked in two
//! directions, and the language has no case where they may part: if `столы` is
//! how the nominative plural of `стол` is written, then `столы` is a
//! nominative plural of `стол` when it is read.
//!
//! In the core they are written apart. One side takes a cell and puts the
//! ending on the stem; the other takes a form and tries every ending against
//! it. Neither reads the other's answer, which is what makes this a check and
//! not a tautology: the core is being asked whether its two halves know the
//! same language.
//!
//! A cell the reader does not take on is not asked about. Nothing is what a
//! reader answers for a form it cannot read and for a form that is wrong, and
//! a law that could not tell those apart would report the core's silence as
//! the core's error.

use super::{Broken, Law, Says};
use crate::{
    grammar::form::Form,
    lexis::{Word, reading}
};

/// The law that the two directions agree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mirrored;

impl Law for Mirrored {
    fn states(&self) -> &'static str {
        "a form written into a cell reads back into that cell"
    }

    fn broken(&self, word: &Word) -> Vec<Broken> {
        let mut found = Vec::new();

        for (cell, spellings) in &word.paradigm().cells {
            if !read(*cell) {
                continue;
            }
            for written in spellings {
                if !reading::of(word, written).contains(cell) {
                    found.push(Broken {
                        cell:    *cell,
                        written: written.clone(),
                        law:     Says::Unread
                    });
                }
            }
        }

        found
    }
}

/// Reports whether the reader takes a cell of this shape on at all.
const fn read(cell: Form) -> bool {
    match cell {
        Form::Verb(_) => reading::verb::reads(cell),
        _ => true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        grammar::{
            Animacy, Aspect, Gender, Transitivity,
            conjugation::{self, Conjugation},
            declension::{Declension, index}
        },
        lexis::{Lexeme, Noun, Verb},
        morphology::WordForm
    };

    fn form(written: &str) -> WordForm {
        WordForm::parse(written).unwrap_or_else(|_| unreachable!("a written Russian word"))
    }

    fn noun(written: &str, gender: Gender, declension: Declension) -> Word {
        Word::new(
            form(written),
            Lexeme::Noun(Noun {
                gender,
                animacy: Animacy::Inanimate,
                declension,
                index: None
            })
        )
    }

    fn listed(written: &str, gender: Gender, animacy: Animacy, stated: &str) -> Word {
        Word::new(
            form(written),
            Lexeme::Noun(Noun {
                gender,
                animacy,
                declension: Declension::Second,
                index: Some(index::read(stated).unwrap_or_else(|| unreachable!("a stated index")))
            })
        )
    }

    #[test]
    fn a_noun_reads_back_into_every_cell_that_wrote_it() {
        for word in [
            noun("стол", Gender::Masculine, Declension::Second),
            noun("книга", Gender::Feminine, Declension::First),
            noun("армия", Gender::Feminine, Declension::First),
            noun("время", Gender::Neuter, Declension::Mixed),
            noun("ночь", Gender::Feminine, Declension::Third),
            noun("пальто", Gender::Neuter, Declension::Indeclinable),
            noun("мороженое", Gender::Neuter, Declension::Adjectival)
        ] {
            assert_eq!(
                Mirrored.broken(&word),
                Vec::new(),
                "{}",
                word.lemma.as_str()
            );
        }
    }

    #[test]
    fn an_indexed_noun_reads_back_into_every_cell_that_wrote_it() {
        for word in [
            listed("конь", Gender::Masculine, Animacy::Animate, "2b"),
            listed("лицо", Gender::Neuter, Animacy::Inanimate, "5d"),
            listed("платок", Gender::Masculine, Animacy::Inanimate, "3*b")
        ] {
            assert_eq!(
                Mirrored.broken(&word),
                Vec::new(),
                "{}",
                word.lemma.as_str()
            );
        }
    }

    #[test]
    fn an_adjective_reads_back_into_every_cell_that_wrote_it() {
        for written in ["новый", "синий", "строгий", "красный", "важный", "хороший"]
        {
            let word = Word::new(form(written), Lexeme::Adjective);

            assert_eq!(Mirrored.broken(&word), Vec::new(), "{written}");
        }
    }

    #[test]
    fn a_verb_reads_back_into_every_cell_that_wrote_it() {
        for (written, reflexive) in [("читать", false), ("учиться", true)] {
            let word = Word::new(
                form(written),
                Lexeme::Verb(Verb {
                    aspect: Aspect::Imperfective,
                    transitivity: Transitivity::Transitive,
                    reflexive,
                    conjugation: Conjugation::First,
                    index: None
                })
            );

            assert_eq!(Mirrored.broken(&word), Vec::new(), "{written}");
        }
    }

    fn conjugated(written: &str, aspect: Aspect, reflexive: bool, stated: &str) -> Word {
        Word::new(
            form(written),
            Lexeme::Verb(Verb {
                aspect,
                transitivity: Transitivity::Transitive,
                reflexive,
                conjugation: Conjugation::First,
                index: Some(
                    conjugation::index::read(stated)
                        .unwrap_or_else(|| unreachable!("a stated index"))
                )
            })
        )
    }

    #[test]
    fn an_indexed_verb_reads_back_into_every_cell_that_wrote_it() {
        for word in [
            conjugated("толкнуть", Aspect::Perfective, false, "3b"),
            conjugated("тянуть", Aspect::Imperfective, false, "3c"),
            conjugated("просить", Aspect::Imperfective, false, "4c"),
            conjugated("писать", Aspect::Imperfective, false, "6c"),
            conjugated("везти", Aspect::Imperfective, false, "7b/b"),
            conjugated("умереть", Aspect::Perfective, false, "9b/c(1)"),
            conjugated("жить", Aspect::Imperfective, false, "16b/c"),
            conjugated("смеяться", Aspect::Imperfective, true, "6b")
        ] {
            assert_eq!(
                Mirrored.broken(&word),
                Vec::new(),
                "{}",
                word.lemma.as_str()
            );
        }
    }
}
