// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Predicate frames: what a word governs and what may fill it.
//!
//! A frame is where grammar and meaning meet. It fixes the case and the
//! preposition of every participant — the part a grammar book states — and the
//! semantic restriction on what may occupy it — the part a dictionary only
//! hints at. Both halves are needed: `пить` takes the accusative, and it takes
//! a liquid, and a phrase that satisfies the case while violating the class is
//! the exact shape of a sentence that parses but means nothing.

use std::{boxed::Box, string::String, vec::Vec};

use crate::{
    grammar::{Animacy, Case},
    id::{FrameId, SenseId},
    sense::{SemanticClass, TimePlacement}
};

/// The part a participant plays in the situation the predicate names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum SemanticRole {
    /// The one who acts.
    Agent,
    /// The one the action changes.
    Patient,
    /// The one the action is about, without changing it.
    Theme,
    /// The one who perceives or feels.
    Experiencer,
    /// The one who receives.
    Recipient,
    /// The one for whose sake the action happens.
    Beneficiary,
    /// What the action is performed with.
    Instrument,
    /// Where the action starts.
    Origin,
    /// Where the action is aimed.
    Goal,
    /// Where the action takes place.
    Location,
    /// When the action takes place.
    Time,
    /// How the action is performed.
    Manner,
    /// Why the action happens.
    Cause,
    /// What the action is meant to achieve.
    Purpose,
    /// What is said, thought or written.
    Content,
    /// What the action produces.
    Result
}

/// How a slot is expressed on the surface.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SlotForm {
    /// The case the filler stands in, absent when the slot is not nominal.
    pub case:        Option<Case>,
    /// The preposition that introduces the filler, absent for a bare case.
    pub preposition: Option<String>,
    /// Whether an infinitive may fill the slot.
    pub infinitive:  bool,
    /// Whether a subordinate clause may fill the slot.
    pub clause:      bool
}

impl SlotForm {
    /// A bare case with no preposition.
    #[must_use]
    pub const fn bare(case: Case) -> Self {
        Self {
            case:        Some(case),
            preposition: None,
            infinitive:  false,
            clause:      false
        }
    }

    /// A case introduced by a preposition.
    #[must_use]
    pub const fn governed(preposition: String, case: Case) -> Self {
        Self {
            case:        Some(case),
            preposition: Some(preposition),
            infinitive:  false,
            clause:      false
        }
    }

    /// Reports whether a filler standing in `case` behind `preposition` fits
    /// this slot.
    ///
    /// A slot that names no case of its own — the one a clause or an
    /// infinitive fills — takes no noun: `читать, что написано` and `читает
    /// книгой` are not the same position filled two ways. A noun offered to
    /// such a slot is refused here, and the gates then say what is wrong
    /// with it.
    #[must_use]
    pub fn accepts(&self, preposition: Option<&str>, case: Option<Case>) -> bool {
        if self.case.is_none() && (self.clause || self.infinitive) && case.is_some() {
            return false;
        }

        let preposition_fits = match (self.preposition.as_deref(), preposition) {
            (Some(expected), Some(actual)) => expected == actual,
            (None, None) => true,
            _ => false
        };
        let case_fits = match (self.case, case) {
            (Some(expected), Some(actual)) => expected.merged() == actual.merged(),
            (None, _) => true,
            (Some(_), None) => false
        };

        preposition_fits && case_fits
    }
}

/// What the engine can be asked about a sense while checking a restriction.
///
/// The restriction language is kept free of storage: the graph and the lexicon
/// implement this trait, and the restriction only asks questions.
pub trait SenseFacts {
    /// Returns the semantic class of a sense.
    fn class_of(&self, sense: SenseId) -> Option<SemanticClass>;

    /// Reports whether `sense` is, directly or transitively, a hyponym of
    /// `ancestor`.
    fn is_kind_of(&self, sense: SenseId, ancestor: SenseId) -> bool;

    /// Returns the animacy a sense denotes, when the question applies.
    fn animacy_of(&self, sense: SenseId) -> Option<Animacy>;

    /// Returns where on the time line a sense points, when it points anywhere.
    ///
    /// Most senses point nowhere and the default says so. A knowledge base
    /// that carries the fact overrides this, and the time gate holds what it
    /// answers against the tense of the predicate.
    fn orientation_of(&self, sense: SenseId) -> Option<TimePlacement> {
        let _ = sense;
        None
    }

    /// Returns the classes a sense stands under, its own and its ancestors'.
    ///
    /// The ontology is a tree, and a flat label is one node of it: `книга`
    /// carries the class of information, and stands under `изделие` all the
    /// same. A slot that admits a class admits everything filed beneath that
    /// class, which only the tree can say. The default sees no tree and
    /// answers with the sense's own class alone.
    fn classes_above(&self, sense: SenseId) -> Vec<SemanticClass> {
        self.class_of(sense).into_iter().collect()
    }
}

impl<T: SenseFacts + ?Sized> SenseFacts for std::sync::Arc<T> {
    fn classes_above(&self, sense: SenseId) -> Vec<SemanticClass> {
        (**self).classes_above(sense)
    }

    fn class_of(&self, sense: SenseId) -> Option<SemanticClass> {
        (**self).class_of(sense)
    }

    fn is_kind_of(&self, sense: SenseId, ancestor: SenseId) -> bool {
        (**self).is_kind_of(sense, ancestor)
    }

    fn animacy_of(&self, sense: SenseId) -> Option<Animacy> {
        (**self).animacy_of(sense)
    }

    fn orientation_of(&self, sense: SenseId) -> Option<TimePlacement> {
        (**self).orientation_of(sense)
    }
}

/// A restriction on what may fill a slot.
///
/// The language is small on purpose: a class test, a taxonomic test, an animacy
/// test, and the three connectives. Everything a source can state fits, and
/// nothing about it needs a model to evaluate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Constraint {
    /// Anything at all.
    Any,
    /// A sense of one of these classes.
    OfClass(Vec<SemanticClass>),
    /// A sense under this one in the taxonomy.
    ///
    /// The dictionary's hypernym chains are short and break early — `бор`
    /// climbs to `лес` and stops, never reaching `территория` — so a filler
    /// of the very class the named sense belongs to passes too: what the
    /// graph cannot connect, the coarse ontology still holds together.
    KindOf(SenseId),
    /// A sense of this animacy.
    OfAnimacy(Animacy),
    /// Every one of the nested restrictions holds.
    All(Vec<Self>),
    /// At least one of the nested restrictions holds.
    Either(Vec<Self>),
    /// The nested restriction does not hold.
    Not(Box<Self>)
}

impl Constraint {
    /// Evaluates the restriction against a sense.
    ///
    /// A missing fact is a failure, not a pass: the engine does not let a slot
    /// through because the store happens to know nothing about the filler.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::{
    ///     frame::{Constraint, SenseFacts},
    ///     grammar::Animacy,
    ///     id::SenseId,
    ///     sense::SemanticClass
    /// };
    ///
    /// struct Store;
    ///
    /// impl SenseFacts for Store {
    ///     fn class_of(&self, _sense: SenseId) -> Option<SemanticClass> {
    ///         Some(SemanticClass::Artifact)
    ///     }
    ///     fn is_kind_of(&self, _sense: SenseId, _ancestor: SenseId) -> bool {
    ///         false
    ///     }
    ///     fn animacy_of(&self, _sense: SenseId) -> Option<Animacy> {
    ///         Some(Animacy::Inanimate)
    ///     }
    /// }
    ///
    /// let brick = SenseId::new(1).expect("non-zero");
    /// let drinkable = Constraint::OfClass(vec![SemanticClass::Liquid]);
    ///
    /// assert!(!drinkable.holds_for(brick, &Store));
    /// ```
    #[must_use]
    pub fn holds_for<F: SenseFacts + ?Sized>(&self, sense: SenseId, facts: &F) -> bool {
        match self {
            Self::Any => true,
            Self::OfClass(classes) => facts
                .classes_above(sense)
                .iter()
                .any(|held| classes.contains(held)),
            Self::KindOf(ancestor) => {
                facts.is_kind_of(sense, *ancestor)
                    || facts
                        .class_of(sense)
                        .zip(facts.class_of(*ancestor))
                        .is_some_and(|(own, named)| own == named)
            }
            Self::OfAnimacy(animacy) => facts
                .animacy_of(sense)
                .is_some_and(|actual| actual == *animacy),
            Self::All(parts) => parts.iter().all(|part| part.holds_for(sense, facts)),
            Self::Either(parts) => parts.iter().any(|part| part.holds_for(sense, facts)),
            Self::Not(part) => !part.condemns(sense, facts)
        }
    }

    /// Evaluates the restriction as grounds for exclusion.
    ///
    /// An affirmative class test climbs the taxonomy, because a slot that
    /// admits a class admits everything beneath it. A negated one must not
    /// climb: two steps up every chain blurs into `объект` and `понятие`,
    /// and a word is not a feeling because a distant ancestor is filed as
    /// one. Exclusion judges the sense by its own class alone.
    fn condemns<F: SenseFacts + ?Sized>(&self, sense: SenseId, facts: &F) -> bool {
        match self {
            Self::OfClass(classes) => facts
                .class_of(sense)
                .is_some_and(|held| classes.contains(&held)),
            Self::All(parts) => parts.iter().all(|part| part.condemns(sense, facts)),
            Self::Either(parts) => parts.iter().any(|part| part.condemns(sense, facts)),
            Self::Any | Self::KindOf(_) | Self::OfAnimacy(_) | Self::Not(_) => {
                self.holds_for(sense, facts)
            }
        }
    }
}

/// One participant of a predicate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Slot {
    /// The part the participant plays.
    pub role:       SemanticRole,
    /// How the participant is expressed.
    pub form:       SlotForm,
    /// What may occupy the slot.
    pub constraint: Constraint,
    /// Whether the predicate is complete without the slot being filled.
    pub optional:   bool
}

/// The government pattern of one sense of a predicate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Frame {
    /// Identity of the frame.
    pub id:    FrameId,
    /// The sense whose government this is.
    pub sense: SenseId,
    /// The participants, subject first.
    pub slots: Vec<Slot>
}

impl Frame {
    /// Returns the slots that must be filled.
    pub fn required(&self) -> impl Iterator<Item = &Slot> {
        self.slots.iter().filter(|slot| !slot.optional)
    }

    /// Returns the slot playing a given role, if the frame has one.
    #[must_use]
    pub fn slot_for(&self, role: SemanticRole) -> Option<&Slot> {
        self.slots.iter().find(|slot| slot.role == role)
    }

    /// Returns the slots a filler in this surface position could occupy.
    pub fn slots_accepting(
        &self,
        preposition: Option<&str>,
        case: Option<Case>
    ) -> impl Iterator<Item = &Slot> {
        self.slots
            .iter()
            .filter(move |slot| slot.form.accepts(preposition, case))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Facts {
        class:    Option<SemanticClass>,
        ancestor: Option<SenseId>,
        animacy:  Option<Animacy>
    }

    impl SenseFacts for Facts {
        fn class_of(&self, _sense: SenseId) -> Option<SemanticClass> {
            self.class
        }

        fn is_kind_of(&self, _sense: SenseId, ancestor: SenseId) -> bool {
            self.ancestor == Some(ancestor)
        }

        fn animacy_of(&self, _sense: SenseId) -> Option<Animacy> {
            self.animacy
        }
    }

    fn sense(raw: u32) -> SenseId {
        SenseId::new(raw).expect("non-zero")
    }

    fn facts(class: SemanticClass) -> Facts {
        Facts {
            class:    Some(class),
            ancestor: None,
            animacy:  None
        }
    }

    #[test]
    fn class_restriction_rejects_the_wrong_class() {
        let drinkable = Constraint::OfClass(std::vec![SemanticClass::Liquid, SemanticClass::Food]);

        assert!(drinkable.holds_for(sense(1), &facts(SemanticClass::Liquid)));
        assert!(!drinkable.holds_for(sense(1), &facts(SemanticClass::Artifact)));
    }

    #[test]
    fn unknown_fact_fails_the_restriction() {
        let unknown = Facts {
            class:    None,
            ancestor: None,
            animacy:  None
        };
        let drinkable = Constraint::OfClass(std::vec![SemanticClass::Liquid]);

        assert!(!drinkable.holds_for(sense(1), &unknown));
    }

    #[test]
    fn taxonomic_restriction_follows_the_graph() {
        struct Filed;

        impl SenseFacts for Filed {
            fn class_of(&self, sense: SenseId) -> Option<SemanticClass> {
                match sense.get() {
                    1 | 10 => Some(SemanticClass::Place),
                    _ => Some(SemanticClass::Artifact)
                }
            }
            fn is_kind_of(&self, _sense: SenseId, _ancestor: SenseId) -> bool {
                false
            }
            fn animacy_of(&self, _sense: SenseId) -> Option<Animacy> {
                None
            }
        }

        let vehicle = sense(10);
        let restriction = Constraint::KindOf(vehicle);
        let known = Facts {
            class:    None,
            ancestor: Some(vehicle),
            animacy:  None
        };

        assert!(restriction.holds_for(sense(1), &known));
        assert!(restriction.holds_for(sense(1), &Filed));
        assert!(!restriction.holds_for(sense(2), &Filed));
    }

    #[test]
    fn connectives_compose() {
        let animate_person = Constraint::All(std::vec![
            Constraint::OfClass(std::vec![SemanticClass::Person]),
            Constraint::Not(Box::new(Constraint::OfClass(std::vec![
                SemanticClass::Artifact
            ]))),
        ]);

        assert!(animate_person.holds_for(sense(1), &facts(SemanticClass::Person)));
        assert!(!animate_person.holds_for(sense(1), &facts(SemanticClass::Artifact)));
    }

    #[test]
    fn exclusion_ignores_the_ancestors() {
        struct Towering;

        impl SenseFacts for Towering {
            fn classes_above(&self, _sense: SenseId) -> std::vec::Vec<SemanticClass> {
                std::vec![SemanticClass::Artifact, SemanticClass::Abstraction]
            }
            fn class_of(&self, _sense: SenseId) -> Option<SemanticClass> {
                Some(SemanticClass::Artifact)
            }
            fn is_kind_of(&self, _sense: SenseId, _ancestor: SenseId) -> bool {
                false
            }
            fn animacy_of(&self, _sense: SenseId) -> Option<Animacy> {
                None
            }
        }

        let excluded = |class: SemanticClass| {
            Constraint::Not(Box::new(Constraint::OfClass(std::vec![class])))
        };

        assert!(
            Constraint::OfClass(std::vec![SemanticClass::Abstraction])
                .holds_for(sense(1), &Towering)
        );
        assert!(excluded(SemanticClass::Abstraction).holds_for(sense(1), &Towering));
        assert!(!excluded(SemanticClass::Artifact).holds_for(sense(1), &Towering));
    }

    #[test]
    fn slot_form_matches_case_and_preposition() {
        let into_goal = SlotForm::governed(String::from("в"), Case::Accusative);

        assert!(into_goal.accepts(Some("в"), Some(Case::Accusative)));
        assert!(!into_goal.accepts(None, Some(Case::Accusative)));
        assert!(!into_goal.accepts(Some("в"), Some(Case::Prepositional)));
    }

    #[test]
    fn frame_reports_required_slots_and_roles() {
        let frame = Frame {
            id:    FrameId::new(1).expect("non-zero"),
            sense: sense(1),
            slots: std::vec![
                Slot {
                    role:       SemanticRole::Agent,
                    form:       SlotForm::bare(Case::Nominative),
                    constraint: Constraint::OfClass(std::vec![SemanticClass::Person]),
                    optional:   false
                },
                Slot {
                    role:       SemanticRole::Instrument,
                    form:       SlotForm::bare(Case::Instrumental),
                    constraint: Constraint::Any,
                    optional:   true
                },
            ]
        };

        assert_eq!(frame.required().count(), 1);
        assert!(frame.slot_for(SemanticRole::Instrument).is_some());
        assert_eq!(
            frame
                .slots_accepting(None, Some(Case::Instrumental))
                .count(),
            1
        );
    }
}
