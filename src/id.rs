// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Stable identifiers of knowledge base entities.
//!
//! Every entity the engine can point at has a numeric identity that survives
//! serialization, indexing and cross-crate traffic. The identifiers are opaque:
//! a lemma cannot be passed where a sense is expected, and the zero value is
//! reserved so that an identifier never has a silent default.

use core::{
    fmt::{self, Display, Formatter},
    num::NonZeroU32
};

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident, $prefix:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        pub struct $name(NonZeroU32);

        impl $name {
            /// The smallest valid identifier, handed out first when an index is
            /// built.
            pub const FIRST: Self = Self(NonZeroU32::MIN);

            /// Wraps a raw value, rejecting zero.
            ///
            /// # Examples
            ///
            /// ```
            #[doc = concat!("use rusem::id::", stringify!($name), ";")]
            ///
            #[doc = concat!("assert!(", stringify!($name), "::new(1).is_some());")]
            #[doc = concat!("assert!(", stringify!($name), "::new(0).is_none());")]
            /// ```
            #[must_use]
            pub const fn new(raw: u32) -> Option<Self> {
                match NonZeroU32::new(raw) {
                    Some(value) => Some(Self(value)),
                    None => None,
                }
            }

            /// Returns the raw value behind the identifier.
            #[must_use]
            pub const fn get(self) -> u32 {
                self.0.get()
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, concat!($prefix, "{}"), self.0.get())
            }
        }
    };
}

define_id!(
    /// Identity of a lemma — a dictionary headword with its whole paradigm.
    LemmaId,
    "lemma:"
);

define_id!(
    /// Identity of one sense of one lemma.
    ///
    /// Senses, not lemmas, are what the semantic layers relate and constrain:
    /// `ключ` as a spring and `ключ` as a tool share a lemma and share nothing
    /// else.
    SenseId,
    "sense:"
);

define_id!(
    /// Identity of a morpheme in the morpheme inventory.
    MorphemeId,
    "morph:"
);

define_id!(
    /// Identity of a predicate frame — one government pattern of one sense.
    FrameId,
    "frame:"
);

define_id!(
    /// Identity of a knowledge source: a dictionary, a thesaurus, an authored
    /// table.
    SourceId,
    "source:"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_rejected() {
        assert!(LemmaId::new(0).is_none());
        assert!(SenseId::new(0).is_none());
    }

    #[test]
    fn raw_value_round_trips() {
        let id = MorphemeId::new(42).expect("non-zero");
        assert_eq!(id.get(), 42);
    }

    #[test]
    fn first_identifier_is_one() {
        assert_eq!(LemmaId::FIRST.get(), 1);
    }

    #[test]
    fn display_is_prefixed() {
        let id = FrameId::new(7).expect("non-zero");
        assert_eq!(id.to_string(), "frame:7");
    }
}
