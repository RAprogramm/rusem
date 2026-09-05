// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The paradigm of a verb, built from what the verb is.
//!
//! The infinitive gives the stems, the class gives the endings, and the aspect
//! says which cells the word has at all. A perfective verb has no present: the
//! forms shaped like one are its simple future, and the table states them in
//! the present cells because that is the one shape they have — `прочитает` is
//! written where `читает` is, and what parts them is the aspect of the word,
//! which is stated once, on the word.
//!
//! The participles decline, so their cells are the adjectival ones and the
//! module beside this one builds them. A participle the core cannot spell —
//! the passive past of a verb whose suffix turns on an alternation — is left
//! unstated rather than guessed at.
//!
//! A reflexive verb is written as the plain verb inside it, with the particle
//! after every ending: `учится` is `учит` and `-ся`, `училась` is `учила` and
//! `-сь`. So the table is built on the verb without the particle and the
//! particle is put back on each cell, which is also what spares the class and
//! the stems from having to know about it.
//!
//! Which road the finite cells go by — the index a dictionary states or the
//! class § 44 derives from the infinitive — is the word's own fact,
//! [`Verb::road`], stated once for this writer and for the reader alike. An
//! indexed verb conjugates by what the dictionary said, so a cell the stated
//! path refuses stays unstated rather than falling back to the guess: the
//! refusal is the honest answer. The participles stay on the derived path
//! either way, because the stated generator does not state them yet.

use crate::{
    grammar::{
        Gender, Number, Person,
        conjugation::{class, imperative, index::VerbIndex, inflect, reflexive, stated, stems},
        form::{Bare, Form, verb::VerbForm}
    },
    lexis::{
        lexeme::{Road, Verb},
        paradigm::Paradigm
    },
    morphology::WordForm
};

/// Who the present states, in the order the tables print them.
const PERSONS: [Person; 3] = [Person::First, Person::Second, Person::Third];

/// Builds every cell of a verb's paradigm.
///
/// A verb flagged reflexive spells its particle in its lemma — that is what
/// the flag asserts. A lemma flagged reflexive that carries no `-ся`/`-сь`
/// states a contradiction, and a table built on either half of it would spell
/// forms nobody has written: `читать` taken as reflexive would fill its cells
/// with `читаюсь`, `читалась`. Such a word gets a table with no cells, the
/// crate's way of leaving unstated what it cannot know.
///
/// # Examples
///
/// ```
/// use rusem::{
///     grammar::{
///         Aspect, Number, Person, Transitivity,
///         conjugation::Conjugation,
///         form::{Form, verb::VerbForm}
///     },
///     lexis::{Verb, paradigm::verb},
///     morphology::WordForm
/// };
///
/// let table = verb::of(
///     &WordForm::parse("читать")?,
///     Verb {
///         aspect:       Aspect::Imperfective,
///         transitivity: Transitivity::Transitive,
///         reflexive:    false,
///         conjugation:  Conjugation::First,
///         index:        None
///     }
/// );
/// let cell = Form::Verb(VerbForm::Present {
///     person: Person::Third,
///     number: Number::Plural
/// });
///
/// assert_eq!(
///     table.fills(cell).first().map(WordForm::as_str),
///     Some("читают")
/// );
/// # Ok::<(), rusem::error::CoreError>(())
/// ```
#[must_use]
pub fn of(lemma: &WordForm, held: Verb) -> Paradigm {
    let plain = if held.reflexive {
        let Some(inner) = reflexive::bare(lemma.as_str()) else {
            return Paradigm {
                lemma: lemma.clone(),
                cells: Vec::new()
            };
        };
        inner
    } else {
        lemma.as_str().to_owned()
    };

    let mut cells = Vec::new();

    for form in shapes(&plain, held.road()) {
        if let Some(written) = spelling(&plain, held, form) {
            cells.push((Form::Verb(form), super::spelled(&written)));
        }
    }

    cells.extend(super::participle::cells(&plain, held));

    Paradigm {
        lemma: lemma.clone(),
        cells
    }
}

/// The cells the verb has, before anything is written in them.
///
/// On the derived road the imperative is stated only when the stem settles
/// its ending. `читай` and `помни` follow from the stem; `неси` and `брось`
/// follow from the stress, which a table built without one does not have — so
/// that cell is left unstated rather than written the commoner way and hoped
/// over. A stated index carries the stress in its scheme letter, so an
/// indexed verb always asks for the cell and the stated builder refuses it
/// only where the index alone does not state it.
fn shapes(plain: &str, road: Road<VerbIndex>) -> Vec<VerbForm> {
    let mut forms = std::vec![VerbForm::Infinitive];

    for number in [Number::Singular, Number::Plural] {
        for person in PERSONS {
            forms.push(VerbForm::Present {
                person,
                number
            });
        }
    }

    for gender in Gender::STATED {
        forms.push(VerbForm::Past(Bare::Singular(gender)));
    }
    forms.push(VerbForm::Past(Bare::Plural));

    if matches!(road, Road::Stated(_)) || bidding(plain) {
        forms.push(VerbForm::Imperative(Number::Singular));
        forms.push(VerbForm::Imperative(Number::Plural));
    }

    forms
}

/// Reports whether the imperative of this verb is settled by its stem.
fn bidding(plain: &str) -> bool {
    stems::present(plain, class::of(plain)).is_some_and(|stem| imperative::settled(&stem))
}

/// What is written in one cell, by the road the word states.
///
/// On the stated road the index carries the stress and the departures, and
/// the cell is written from it exactly; what the index alone does not state
/// comes back [`None`] and the cell stays unstated, never handed to the
/// guess. On the derived road the stress of the ending is not known, and the
/// unstressed spelling is the one taken: a paradigm built without a stress
/// mark states what it can and does not invent the rest.
fn spelling(plain: &str, held: Verb, form: VerbForm) -> Option<String> {
    let written = match held.road() {
        Road::Stated(index) => stated::written(plain, index, form, false),
        Road::Derived => inflect::written(plain, form, false)
    }?;

    Some(if held.reflexive {
        reflexive::attached(&written)
    } else {
        written
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Aspect, Transitivity, conjugation::Conjugation};

    fn form(written: &str) -> WordForm {
        WordForm::parse(written).unwrap_or_else(|_| unreachable!("a written Russian word"))
    }

    fn word(aspect: Aspect, reflexive: bool) -> Verb {
        Verb {
            aspect,
            transitivity: Transitivity::Transitive,
            reflexive,
            conjugation: Conjugation::First,
            index: None
        }
    }

    fn listed(aspect: Aspect, reflexive: bool, stated: &str) -> Verb {
        Verb {
            index: Some(
                crate::grammar::conjugation::index::read(stated)
                    .unwrap_or_else(|| unreachable!("a stated index"))
            ),
            ..word(aspect, reflexive)
        }
    }

    fn cell_of(table: &Paradigm, form: VerbForm) -> Option<String> {
        table
            .fills(Form::Verb(form))
            .first()
            .map(|written| written.as_str().to_owned())
    }

    #[test]
    fn a_verb_states_the_infinitive_the_present_the_past_and_the_imperative() {
        let table = of(&form("читать"), word(Aspect::Imperfective, false));
        let finite = table
            .cells
            .iter()
            .filter(|(cell, _)| !matches!(cell, Form::Verb(VerbForm::Participle { .. })))
            .count();

        assert_eq!(finite, 13);
    }

    #[test]
    fn a_verb_whose_imperative_the_stress_settles_leaves_that_cell_unstated() {
        let table = of(&form("нести"), word(Aspect::Imperfective, false));

        assert!(cell_of(&table, VerbForm::Imperative(Number::Singular)).is_none());
    }

    #[test]
    fn the_present_is_written_out_by_person_and_number() {
        let table = of(&form("читать"), word(Aspect::Imperfective, false));

        assert_eq!(
            cell_of(
                &table,
                VerbForm::Present {
                    person: Person::First,
                    number: Number::Singular
                }
            )
            .as_deref(),
            Some("читаю")
        );
        assert_eq!(
            cell_of(
                &table,
                VerbForm::Present {
                    person: Person::Third,
                    number: Number::Plural
                }
            )
            .as_deref(),
            Some("читают")
        );
    }

    #[test]
    fn the_past_agrees_in_gender_and_number() {
        let table = of(&form("читать"), word(Aspect::Imperfective, false));

        assert_eq!(
            cell_of(&table, VerbForm::Past(Bare::Singular(Gender::Feminine))).as_deref(),
            Some("читала")
        );
        assert_eq!(
            cell_of(&table, VerbForm::Past(Bare::Plural)).as_deref(),
            Some("читали")
        );
    }

    #[test]
    fn a_perfective_verb_writes_its_simple_future_in_the_present_cells() {
        let table = of(&form("прочитать"), word(Aspect::Perfective, false));

        assert_eq!(
            cell_of(
                &table,
                VerbForm::Present {
                    person: Person::Third,
                    number: Number::Singular
                }
            )
            .as_deref(),
            Some("прочитает")
        );
    }

    #[test]
    fn a_reflexive_verb_carries_its_particle_into_every_cell() {
        let table = of(&form("учиться"), word(Aspect::Imperfective, true));

        assert_eq!(
            cell_of(&table, VerbForm::Infinitive).as_deref(),
            Some("учиться")
        );
        assert_eq!(
            cell_of(&table, VerbForm::Past(Bare::Singular(Gender::Feminine))).as_deref(),
            Some("училась")
        );
        assert_eq!(
            cell_of(
                &table,
                VerbForm::Present {
                    person: Person::Third,
                    number: Number::Singular
                }
            )
            .as_deref(),
            Some("учится")
        );
    }

    #[test]
    fn a_reflexive_flag_on_a_lemma_without_a_particle_states_no_cells() {
        let table = of(&form("читать"), word(Aspect::Imperfective, true));

        assert!(table.cells.is_empty());
    }

    #[test]
    fn an_indexed_verb_writes_its_cells_by_the_stated_index() {
        let table = of(&form("толкнуть"), listed(Aspect::Perfective, false, "3b"));

        assert_eq!(
            cell_of(
                &table,
                VerbForm::Present {
                    person: Person::Second,
                    number: Number::Singular
                }
            )
            .as_deref(),
            Some("толкнёшь")
        );
        assert_eq!(
            cell_of(&table, VerbForm::Imperative(Number::Singular)).as_deref(),
            Some("толкни")
        );
        assert_eq!(
            cell_of(&table, VerbForm::Past(Bare::Singular(Gender::Feminine))).as_deref(),
            Some("толкнула")
        );
    }

    #[test]
    fn an_indexed_reflexive_verb_carries_its_particle_into_every_cell() {
        let table = of(&form("смеяться"), listed(Aspect::Imperfective, true, "6b"));

        assert_eq!(
            cell_of(
                &table,
                VerbForm::Present {
                    person: Person::First,
                    number: Number::Singular
                }
            )
            .as_deref(),
            Some("смеюсь")
        );
        assert_eq!(
            cell_of(&table, VerbForm::Imperative(Number::Singular)).as_deref(),
            Some("смейся")
        );
    }

    #[test]
    fn a_cell_the_stated_road_refuses_stays_unstated() {
        let table = of(&form("нести"), listed(Aspect::Imperfective, false, "7b/b"));

        assert_eq!(
            cell_of(&table, VerbForm::Infinitive).as_deref(),
            Some("нести")
        );
        assert!(
            cell_of(
                &table,
                VerbForm::Present {
                    person: Person::First,
                    number: Number::Singular
                }
            )
            .is_none()
        );
        assert!(cell_of(&table, VerbForm::Past(Bare::Plural)).is_none());
    }

    #[test]
    fn a_written_form_reads_back_into_the_cell_it_fills() {
        let table = of(&form("читать"), word(Aspect::Imperfective, false));

        let cells = table.cells_of(&form("читай"));

        assert_eq!(
            cells,
            std::vec![Form::Verb(VerbForm::Imperative(Number::Singular))]
        );
    }
}
