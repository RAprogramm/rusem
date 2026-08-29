// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Failures of the domain layer.
//!
//! The domain fails in few, well-named ways: a value violated an invariant, or
//! something was asked about an entity the store does not hold. Adapter-level
//! failures — a missing index file, a malformed dump — belong to the adapters
//! and are converted at their boundary.

use std::string::String;

use masterror::Error;

use crate::id::{FrameId, LemmaId, SenseId, SourceId};

/// Result alias of the domain layer.
pub type Result<T> = core::result::Result<T, CoreError>;

/// A failure in the domain layer.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CoreError {
    /// A confidence value fell outside the unit interval.
    #[error("confidence {value} is outside 0.0..=1.0")]
    ConfidenceOutOfRange {
        /// The rejected value.
        value: f32
    },

    /// A word form was empty or held characters no analyzer accepts.
    #[error("word form is not analyzable: {reason}")]
    UnusableWordForm {
        /// Why the form was rejected.
        reason: &'static str
    },

    /// A segmentation did not cover its word form exactly once.
    #[error("segmentation does not tile the form: {reason}")]
    BrokenSegmentation {
        /// Why the segmentation was rejected.
        reason: &'static str
    },

    /// The store holds no lemma under this identifier.
    #[error("unknown lemma {id}")]
    UnknownLemma {
        /// The identifier that resolved to nothing.
        id: LemmaId
    },

    /// The store holds no sense under this identifier.
    #[error("unknown sense {id}")]
    UnknownSense {
        /// The identifier that resolved to nothing.
        id: SenseId
    },

    /// The store holds no frame under this identifier.
    #[error("unknown frame {id}")]
    UnknownFrame {
        /// The identifier that resolved to nothing.
        id: FrameId
    },

    /// The store holds no source under this identifier.
    #[error("unknown source {id}")]
    UnknownSource {
        /// The identifier that resolved to nothing.
        id: SourceId
    },

    /// An adapter failed below the domain.
    ///
    /// The domain does not model file systems, dumps or sockets. An adapter
    /// that hits one of those states the failure in words and names itself, so
    /// a caller can tell a missing index apart from a missing word.
    #[error("{adapter}: {message}")]
    AdapterFailure {
        /// The adapter that failed.
        adapter: &'static str,
        /// What went wrong, as the adapter described it.
        message: String
    }
}

impl CoreError {
    /// Builds an adapter failure.
    #[must_use]
    pub const fn adapter(adapter: &'static str, message: String) -> Self {
        Self::AdapterFailure {
            adapter,
            message
        }
    }
}

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use super::*;

    #[test]
    fn messages_name_the_entity() {
        let id = SenseId::new(9).expect("non-zero");
        let error = CoreError::UnknownSense {
            id
        };

        assert_eq!(error.to_string(), "unknown sense sense:9");
    }

    #[test]
    fn range_failure_reports_the_value() {
        let error = CoreError::ConfidenceOutOfRange {
            value: 2.0
        };

        assert!(error.to_string().contains('2'));
    }
}
