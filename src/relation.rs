// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Relations between senses.
//!
//! The relations form the network the engine walks to answer "is this a kind of
//! that", "does this contradict that", and "how are these two words connected".
//! They hold between senses, never between lemmas: `лук` is a hyponym of
//! `оружие` in one sense and of `растение` in another, and collapsing the two
//! would let the network prove nonsense.

use crate::id::SenseId;

/// The kind of link between two senses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum RelationKind {
    /// The target is a broader class than the source: `стол` to `мебель`.
    Hypernym,
    /// The target is a narrower class than the source: `мебель` to `стол`.
    Hyponym,
    /// The two senses are interchangeable in some contexts.
    Synonym,
    /// The two senses are opposed along one dimension.
    Antonym,
    /// The source is a part of the target: `ножка` to `стол`.
    PartOf,
    /// The target is a part of the source: `стол` to `ножка`.
    HasPart,
    /// The source is a member of the target: `солдат` to `армия`.
    MemberOf,
    /// The target is a member of the source.
    HasMember,
    /// The source is made of the target: `стол` to `дерево`.
    MadeOf,
    /// The target is made of the source.
    MaterialFor,
    /// The source brings the target about: `нагревать` to `нагреться`.
    Causes,
    /// The source is brought about by the target.
    CausedBy,
    /// The source cannot happen without the target having happened:
    /// `проснуться` to `спать`.
    Entails,
    /// The target cannot happen without the source having happened.
    EntailedBy,
    /// The source is derived from the target by word formation.
    DerivedFrom,
    /// The target is derived from the source by word formation.
    BaseOf,
    /// The source belongs to the subject field the target names.
    InDomain,
    /// The target belongs to the subject field the source names.
    DomainOf,
    /// The source names an individual of the class the target names.
    InstanceOf,
    /// The target names an individual of the class the source names.
    HasInstance
}

impl RelationKind {
    /// Returns the kind that states the same fact from the other end.
    ///
    /// Storage keeps one direction and answers both, so the inverse must be
    /// total and must be its own inverse.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::relation::RelationKind;
    ///
    /// assert_eq!(RelationKind::Hypernym.inverse(), RelationKind::Hyponym);
    /// assert_eq!(RelationKind::Synonym.inverse(), RelationKind::Synonym);
    /// ```
    #[must_use]
    pub const fn inverse(self) -> Self {
        match self {
            Self::Hypernym => Self::Hyponym,
            Self::Hyponym => Self::Hypernym,
            Self::Synonym => Self::Synonym,
            Self::Antonym => Self::Antonym,
            Self::PartOf => Self::HasPart,
            Self::HasPart => Self::PartOf,
            Self::MemberOf => Self::HasMember,
            Self::HasMember => Self::MemberOf,
            Self::MadeOf => Self::MaterialFor,
            Self::MaterialFor => Self::MadeOf,
            Self::Causes => Self::CausedBy,
            Self::CausedBy => Self::Causes,
            Self::Entails => Self::EntailedBy,
            Self::EntailedBy => Self::Entails,
            Self::DerivedFrom => Self::BaseOf,
            Self::BaseOf => Self::DerivedFrom,
            Self::InDomain => Self::DomainOf,
            Self::DomainOf => Self::InDomain,
            Self::InstanceOf => Self::HasInstance,
            Self::HasInstance => Self::InstanceOf
        }
    }

    /// Reports whether the relation may be followed transitively.
    ///
    /// Only the taxonomic and partitive chains may: a hypernym of a hypernym is
    /// a hypernym, while a synonym of a synonym drifts and an antonym of an
    /// antonym is not the word it started from.
    #[must_use]
    pub const fn is_transitive(self) -> bool {
        matches!(
            self,
            Self::Hypernym | Self::Hyponym | Self::PartOf | Self::HasPart | Self::InstanceOf
        )
    }
}

/// A link between two senses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Relation {
    /// The sense the link starts at.
    pub from: SenseId,
    /// The kind of link.
    pub kind: RelationKind,
    /// The sense the link ends at.
    pub to:   SenseId
}

impl Relation {
    /// Builds a relation.
    #[must_use]
    pub const fn new(from: SenseId, kind: RelationKind, to: SenseId) -> Self {
        Self {
            from,
            kind,
            to
        }
    }

    /// Returns the same fact stated from the other end.
    #[must_use]
    pub const fn inverted(self) -> Self {
        Self {
            from: self.to,
            kind: self.kind.inverse(),
            to:   self.from
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KINDS: [RelationKind; 20] = [
        RelationKind::Hypernym,
        RelationKind::Hyponym,
        RelationKind::Synonym,
        RelationKind::Antonym,
        RelationKind::PartOf,
        RelationKind::HasPart,
        RelationKind::MemberOf,
        RelationKind::HasMember,
        RelationKind::MadeOf,
        RelationKind::MaterialFor,
        RelationKind::Causes,
        RelationKind::CausedBy,
        RelationKind::Entails,
        RelationKind::EntailedBy,
        RelationKind::DerivedFrom,
        RelationKind::BaseOf,
        RelationKind::InDomain,
        RelationKind::DomainOf,
        RelationKind::InstanceOf,
        RelationKind::HasInstance
    ];

    #[test]
    fn inverse_is_an_involution() {
        for kind in KINDS {
            assert_eq!(kind.inverse().inverse(), kind);
        }
    }

    #[test]
    fn only_taxonomic_links_are_transitive() {
        assert!(RelationKind::Hypernym.is_transitive());
        assert!(!RelationKind::Synonym.is_transitive());
        assert!(!RelationKind::Causes.is_transitive());
    }

    #[test]
    fn inverting_a_relation_swaps_the_ends() {
        let table = SenseId::new(1).expect("non-zero");
        let furniture = SenseId::new(2).expect("non-zero");
        let relation = Relation::new(table, RelationKind::Hypernym, furniture);

        assert_eq!(
            relation.inverted(),
            Relation::new(furniture, RelationKind::Hyponym, table)
        );
    }
}
