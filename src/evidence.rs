// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Where a fact came from and how far it can be trusted.
//!
//! The engine returns no fact without a provenance record. That rule is what
//! separates a lookup from a guess: a caller can always ask which source said
//! this, and a model quoting the engine can always cite it.

use core::fmt::{self, Display, Formatter};
use std::string::String;

use crate::{
    error::{CoreError, Result},
    id::SourceId
};

/// A knowledge source registered in the store.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Source {
    /// Identity of the source inside the store.
    pub id:       SourceId,
    /// Short machine name, stable across rebuilds, such as `dahl` or
    /// `opencorpora`.
    pub name:     String,
    /// Human readable title of the source.
    pub title:    String,
    /// Kind of material the source provides.
    pub kind:     SourceKind,
    /// Revision of the imported material, as reported by the source itself.
    pub revision: String
}

/// The kind of material a source contributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum SourceKind {
    /// A morphological dictionary of word forms and paradigms.
    Morphology,
    /// An explanatory dictionary of senses.
    Explanatory,
    /// A thesaurus of relations between senses.
    Thesaurus,
    /// A frame inventory describing predicate government.
    FrameInventory,
    /// A rule table authored in this repository.
    RuleTable,
    /// An official codified rulebook of the language itself.
    RuleBook,
    /// A text corpus used to seed or weight the tables above.
    Corpus
}

/// How strongly a fact is supported, on a closed unit scale.
///
/// The scale is not a probability and is never produced by a model: it comes
/// from the source itself — a dictionary reading is certain, an analyzer's
/// second-best parse is not, a rule-derived guess is weaker still.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "f32", into = "f32"))]
pub struct Confidence(f32);

impl Confidence {
    /// Stated outright by a source.
    pub const CERTAIN: Self = Self(1.0);
    /// Derived by a rule the engine owns, with no source disagreeing.
    pub const DERIVED: Self = Self(0.6);
    /// Guessed from structure alone, with no dictionary support.
    pub const GUESSED: Self = Self(0.3);

    /// Builds a confidence value, rejecting anything outside `0.0..=1.0`.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::ConfidenceOutOfRange`] when the value is not a
    /// finite number within the unit interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::evidence::Confidence;
    ///
    /// assert!(Confidence::new(0.5).is_ok());
    /// assert!(Confidence::new(1.5).is_err());
    /// ```
    pub fn new(value: f32) -> Result<Self> {
        if value.is_finite() && (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(CoreError::ConfidenceOutOfRange {
                value
            })
        }
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn get(self) -> f32 {
        self.0
    }

    /// Combines two independent supports for the same fact, keeping the weaker
    /// one.
    ///
    /// A chain is as strong as its weakest link: a certain definition reached
    /// through a guessed segmentation is a guess.
    #[must_use]
    pub fn weakest(self, other: Self) -> Self {
        if self.0 <= other.0 { self } else { other }
    }
}

impl Display for Confidence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

impl TryFrom<f32> for Confidence {
    type Error = CoreError;

    fn try_from(value: f32) -> Result<Self> {
        Self::new(value)
    }
}

impl From<Confidence> for f32 {
    fn from(confidence: Confidence) -> Self {
        confidence.0
    }
}

/// A pointer to the exact place a fact was read from.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Provenance {
    /// The source that stated the fact.
    pub source:  SourceId,
    /// Locator of the entry inside that source: a headword, an article id, a
    /// rule name.
    pub locator: String
}

impl Provenance {
    /// Builds a provenance record.
    #[must_use]
    pub const fn new(source: SourceId, locator: String) -> Self {
        Self {
            source,
            locator
        }
    }
}

/// A fact together with its support.
///
/// Every value crossing the engine boundary is wrapped in this type. There is
/// no constructor that omits the provenance, which is the mechanical form of
/// the rule "evidence or silence".
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Evidenced<T> {
    /// The fact itself.
    pub value:      T,
    /// Where it was read from.
    pub provenance: Provenance,
    /// How strongly it is supported.
    pub confidence: Confidence
}

impl<T> Evidenced<T> {
    /// Wraps a value with its support.
    #[must_use]
    pub const fn new(value: T, provenance: Provenance, confidence: Confidence) -> Self {
        Self {
            value,
            provenance,
            confidence
        }
    }

    /// Applies a function to the value, carrying the support unchanged.
    #[must_use]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Evidenced<U> {
        Evidenced {
            value:      f(self.value),
            provenance: self.provenance,
            confidence: self.confidence
        }
    }
}

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use super::*;

    fn source() -> SourceId {
        SourceId::new(1).expect("non-zero")
    }

    #[test]
    fn confidence_rejects_out_of_range() {
        assert!(Confidence::new(-0.1).is_err());
        assert!(Confidence::new(f32::NAN).is_err());
        assert!(Confidence::new(0.0).is_ok());
    }

    #[test]
    fn chain_keeps_the_weakest_link() {
        let combined = Confidence::CERTAIN.weakest(Confidence::GUESSED);
        assert_eq!(combined, Confidence::GUESSED);
    }

    #[test]
    fn mapping_preserves_support() {
        let fact = Evidenced::new(
            1_u8,
            Provenance::new(source(), "стол".to_string()),
            Confidence::CERTAIN
        );
        let mapped = fact.clone().map(u16::from);

        assert_eq!(mapped.value, 1);
        assert_eq!(mapped.provenance, fact.provenance);
        assert_eq!(mapped.confidence, fact.confidence);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialization_re_validates_the_interval() {
        let accepted: Confidence = serde_json::from_str("0.5").expect("a unit-interval value");
        assert!((accepted.get() - 0.5).abs() < f32::EPSILON);

        assert!(serde_json::from_str::<Confidence>("1.5").is_err());
    }
}
