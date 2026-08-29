// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The declined cells a verb's participles add to its table.
//!
//! A participle declines like an adjective, so its cells are the adjectival
//! ones and its spellings come from the adjectival tables. What is verbal
//! about it is which participles the verb has at all: the present two only
//! while the verb has a present, the passive two only while it can be said of
//! the one acted on.
//!
//! The accusative holds two spellings here for the same reason it does in an
//! adjective — the animacy is the noun's, not the participle's.
//!
//! A reflexive verb keeps its particle here too, and here it is always `-ся`:
//! `учащийся`, `учившаяся`, `учившиеся`. The vowel that would call for `-сь`
//! in a finite form does not call for it after an adjectival ending.

use crate::{
    grammar::{
        Animacy, Aspect, Case, Gender, Number, Tense, Voice,
        conjugation::participle,
        declension::adjective,
        form::{
            Agreed, Form,
            verb::{Participle, VerbForm}
        }
    },
    lexis::Verb,
    morphology::WordForm
};

/// The particle a reflexive participle carries, whatever it ends in.
const PARTICLE: &str = "ся";

/// One participle of a verb, while its cells are being written.
#[derive(Debug, Clone, Copy)]
struct Kind {
    /// Whether the one described acts or is acted on.
    voice:     Voice,
    /// When, relative to the sentence.
    tense:     Tense,
    /// Whether the verb carries the particle.
    reflexive: bool
}

/// Every declined cell of every participle the verb has.
#[must_use]
pub fn cells(plain: &str, held: Verb) -> Vec<(Form, Vec<WordForm>)> {
    let mut found = Vec::new();

    for kind in kinds(held) {
        let Some(dictionary) = participle::dictionary(plain, kind.voice, kind.tense) else {
            continue;
        };
        declined(&mut found, &dictionary, kind);
    }

    found
}

/// Which participles the verb has.
///
/// A perfective verb has no present, and therefore neither present participle.
/// An intransitive or reflexive verb has no passive, because there is no one
/// the action is done to for the participle to describe.
fn kinds(held: Verb) -> Vec<Kind> {
    let running = matches!(held.aspect, Aspect::Imperfective);
    let mut found = std::vec![kind(Voice::Active, Tense::Past, held)];

    if running {
        found.push(kind(Voice::Active, Tense::Present, held));
    }
    if held.takes_passive() {
        found.push(kind(Voice::Passive, Tense::Past, held));
        if running {
            found.push(kind(Voice::Passive, Tense::Present, held));
        }
    }

    found
}

/// One participle of a verb, named.
const fn kind(voice: Voice, tense: Tense, held: Verb) -> Kind {
    Kind {
        voice,
        tense,
        reflexive: held.reflexive
    }
}

/// Every cell of one participle, declined off its dictionary form.
fn declined(found: &mut Vec<(Form, Vec<WordForm>)>, dictionary: &str, kind: Kind) {
    for gender in Gender::STATED {
        for case in Case::STATED {
            push(found, dictionary, kind, case, Number::Singular, gender);
        }
    }
    for case in Case::STATED {
        push(
            found,
            dictionary,
            kind,
            case,
            Number::Plural,
            Gender::Masculine
        );
    }
}

/// Writes one cell into the table, if the core can spell it.
fn push(
    found: &mut Vec<(Form, Vec<WordForm>)>,
    dictionary: &str,
    kind: Kind,
    case: Case,
    number: Number,
    gender: Gender
) {
    let spellings = spelled(dictionary, kind.reflexive, case, number, gender);
    if !spellings.is_empty() {
        found.push((cell(kind, case, number, gender), spellings));
    }
}

/// The cell of the table one participle in one case names.
const fn cell(kind: Kind, case: Case, number: Number, gender: Gender) -> Form {
    Form::Verb(VerbForm::Participle {
        voice: kind.voice,
        tense: kind.tense,
        form:  Participle::Full(match number {
            Number::Singular => Agreed::Singular {
                case,
                gender
            },
            Number::Plural => Agreed::Plural {
                case
            }
        })
    })
}

/// What is written in one cell, once for every animacy that spells it
/// differently.
fn spelled(
    dictionary: &str,
    reflexive: bool,
    case: Case,
    number: Number,
    gender: Gender
) -> Vec<WordForm> {
    let mut held: Vec<WordForm> = Vec::new();

    for animacy in [Animacy::Inanimate, Animacy::Animate] {
        let Some(written) = adjective::written(dictionary, case, number, gender, animacy) else {
            continue;
        };
        let carried = if reflexive {
            written + PARTICLE
        } else {
            written
        };
        let Ok(one) = WordForm::parse(&carried) else {
            continue;
        };
        if !held.contains(&one) {
            held.push(one);
        }
    }

    held
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Transitivity, conjugation::Conjugation};

    fn word(aspect: Aspect, transitivity: Transitivity, reflexive: bool) -> Verb {
        Verb {
            aspect,
            transitivity,
            reflexive,
            conjugation: Conjugation::First
        }
    }

    fn nominative(cells: &[(Form, Vec<WordForm>)], voice: Voice, tense: Tense) -> Option<String> {
        let wanted = cell(
            Kind {
                voice,
                tense,
                reflexive: false
            },
            Case::Nominative,
            Number::Singular,
            Gender::Masculine
        );

        cells
            .iter()
            .find(|(held, _)| *held == wanted)
            .and_then(|(_, spellings)| spellings.first())
            .map(|written| written.as_str().to_owned())
    }

    #[test]
    fn an_imperfective_transitive_verb_states_all_four_participles() {
        let cells = cells(
            "читать",
            word(Aspect::Imperfective, Transitivity::Transitive, false)
        );

        assert_eq!(
            nominative(&cells, Voice::Active, Tense::Present).as_deref(),
            Some("читающий")
        );
        assert_eq!(
            nominative(&cells, Voice::Active, Tense::Past).as_deref(),
            Some("читавший")
        );
        assert_eq!(
            nominative(&cells, Voice::Passive, Tense::Present).as_deref(),
            Some("читаемый")
        );
    }

    #[test]
    fn a_perfective_verb_states_no_present_participle() {
        let cells = cells(
            "прочитать",
            word(Aspect::Perfective, Transitivity::Transitive, false)
        );

        assert!(nominative(&cells, Voice::Active, Tense::Present).is_none());
        assert_eq!(
            nominative(&cells, Voice::Passive, Tense::Past).as_deref(),
            Some("прочитанный")
        );
    }

    #[test]
    fn an_intransitive_verb_states_no_passive_participle() {
        let cells = cells(
            "читать",
            word(Aspect::Imperfective, Transitivity::Intransitive, false)
        );

        assert!(nominative(&cells, Voice::Passive, Tense::Present).is_none());
        assert!(nominative(&cells, Voice::Passive, Tense::Past).is_none());
    }

    #[test]
    fn a_reflexive_participle_carries_its_particle_in_every_cell() {
        let cells = cells(
            "учить",
            word(Aspect::Imperfective, Transitivity::Transitive, true)
        );

        assert_eq!(
            nominative(&cells, Voice::Active, Tense::Past).as_deref(),
            Some("учившийся")
        );
        assert!(cells.iter().all(|(_, spellings)| {
            spellings
                .iter()
                .all(|written| written.as_str().ends_with(PARTICLE))
        }));
    }
}
