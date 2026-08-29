// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What holds a sentence together.
//!
//! A gate that looks at neighbouring words is reading position, not grammar.
//! `отрезанных лесорубом дисков` puts a noun between a participle and the word
//! it describes, and `Королев сменил научрука, поддался поветрию и защитил
//! диссертацию` puts three predicates in one sentence. Both are ordinary
//! Russian, and both defeat a window. What settles them is which word answers
//! to which — a tree, not a line.

use std::{vec, vec::Vec};

/// What one word is to the word it answers to.
///
/// The inventory is Universal Dependencies, cut down to the relations the
/// gates actually consult. Everything else is [`Relation::Other`]: the parser
/// still attaches the word, it just says nothing about why.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Relation {
    /// The word the sentence is built around.
    Root,
    /// Subject of a predicate.
    Subject,
    /// Direct object.
    Object,
    /// Indirect object.
    Indirect,
    /// An oblique participant, usually behind a preposition.
    Oblique,
    /// An adjective or participle describing a noun.
    Attribute,
    /// A noun depending on another noun.
    Modifier,
    /// A number counting a noun.
    Counter,
    /// A preposition marking the case of what follows.
    Marker,
    /// A word coordinated with another.
    Coordinate,
    /// A conjunction joining coordinates.
    Junction,
    /// An adverbial.
    Adverbial,
    /// A clause depending on a predicate.
    Clause,
    /// Something the parser attached without saying what it is.
    Other
}

impl Relation {
    /// Every relation, in a fixed order, for indexing the parser's actions.
    pub const ALL: [Self; 14] = [
        Self::Root,
        Self::Subject,
        Self::Object,
        Self::Indirect,
        Self::Oblique,
        Self::Attribute,
        Self::Modifier,
        Self::Counter,
        Self::Marker,
        Self::Coordinate,
        Self::Junction,
        Self::Adverbial,
        Self::Clause,
        Self::Other
    ];

    /// Reads a Universal Dependencies label.
    ///
    /// A label may carry a subtype after a colon — `nmod:poss`, `obl:tmod` —
    /// and the part before it is the relation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::syntax::Relation;
    ///
    /// assert_eq!(Relation::read("nsubj:pass"), Relation::Subject);
    /// assert_eq!(Relation::read("amod"), Relation::Attribute);
    /// assert_eq!(Relation::read("discourse"), Relation::Other);
    /// ```
    #[must_use]
    pub fn read(label: &str) -> Self {
        let stem = label.split(':').next().unwrap_or(label);

        match stem {
            "root" => Self::Root,
            "nsubj" | "csubj" => Self::Subject,
            "obj" => Self::Object,
            "iobj" => Self::Indirect,
            "obl" => Self::Oblique,
            "amod" | "acl" => Self::Attribute,
            "nmod" | "appos" => Self::Modifier,
            "nummod" | "det" => Self::Counter,
            "case" => Self::Marker,
            "conj" => Self::Coordinate,
            "cc" => Self::Junction,
            "advmod" | "advcl" => Self::Adverbial,
            "ccomp" | "xcomp" => Self::Clause,
            _ => Self::Other
        }
    }

    /// The position of this relation in [`Relation::ALL`].
    #[must_use]
    pub fn index(self) -> usize {
        Self::ALL
            .iter()
            .position(|held| *held == self)
            .unwrap_or(Self::ALL.len() - 1)
    }

    /// Reports whether the relation is one an attribute holds to its noun.
    #[must_use]
    pub const fn describes(self) -> bool {
        matches!(self, Self::Attribute | Self::Counter)
    }

    /// Reports whether the relation is one a participant holds to a predicate.
    #[must_use]
    pub const fn participates(self) -> bool {
        matches!(
            self,
            Self::Subject | Self::Object | Self::Indirect | Self::Oblique
        )
    }
}

/// The words of one sentence and what each answers to.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tree {
    /// For each word, the word it answers to, or `None` for the root.
    pub heads:  Vec<Option<usize>>,
    /// For each word, what it is to the word it answers to.
    pub labels: Vec<Relation>
}

impl Tree {
    /// A tree with every word left unattached.
    #[must_use]
    pub fn loose(length: usize) -> Self {
        Self {
            heads:  vec![None; length],
            labels: vec![Relation::Other; length]
        }
    }

    /// How many words the tree holds.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.heads.len()
    }

    /// Reports whether the tree holds no words.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.heads.is_empty()
    }

    /// The word a position answers to.
    #[must_use]
    pub fn head_of(&self, position: usize) -> Option<usize> {
        self.heads.get(position).copied().flatten()
    }

    /// What a position is to the word it answers to.
    #[must_use]
    pub fn label_of(&self, position: usize) -> Option<Relation> {
        self.labels.get(position).copied()
    }

    /// The positions that answer to one word.
    #[must_use]
    pub fn children_of(&self, position: usize) -> Vec<usize> {
        self.heads
            .iter()
            .enumerate()
            .filter_map(|(held, head)| (*head == Some(position)).then_some(held))
            .collect()
    }

    /// The predicate a position ultimately answers to, if any.
    ///
    /// A participant answers to its predicate directly; an attribute answers
    /// to a noun which answers to one. Climbing stops at the root, and at a
    /// depth that no real sentence exceeds.
    #[must_use]
    pub fn governor_of(&self, position: usize) -> Option<usize> {
        let mut standing = position;

        for _ in 0..self.len().min(CLIMB) {
            let head = self.head_of(standing)?;
            if self.label_of(standing).is_some_and(Relation::participates) {
                return Some(head);
            }
            standing = head;
        }

        None
    }
}

/// How far up a tree the search for a predicate climbs.
const CLIMB: usize = 16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_label_is_read_without_its_subtype() {
        assert_eq!(Relation::read("obl:tmod"), Relation::Oblique);
        assert_eq!(Relation::read("nummod:gov"), Relation::Counter);
    }

    #[test]
    fn a_tree_reports_what_answers_to_what() {
        let tree = Tree {
            heads:  vec![Some(1), None, Some(1)],
            labels: vec![Relation::Subject, Relation::Root, Relation::Object]
        };

        assert_eq!(tree.head_of(0), Some(1));
        assert_eq!(tree.children_of(1), vec![0, 2]);
        assert_eq!(tree.governor_of(2), Some(1));
    }
}
