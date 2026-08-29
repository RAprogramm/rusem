// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Part of speech: the class a word belongs to.

/// Part of speech.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum PartOfSpeech {
    /// Noun.
    Noun,
    /// Adjective, full form.
    Adjective,
    /// Adjective, short form.
    ShortAdjective,
    /// Comparative degree.
    Comparative,
    /// Verb, finite form.
    Verb,
    /// Verb, infinitive.
    Infinitive,
    /// Participle, full form.
    Participle,
    /// Participle, short form.
    ShortParticiple,
    /// Adverbial participle.
    AdverbialParticiple,
    /// Numeral.
    Numeral,
    /// Adverb.
    Adverb,
    /// Pronoun used as a noun.
    Pronoun,
    /// Predicative.
    Predicative,
    /// Preposition.
    Preposition,
    /// Conjunction.
    Conjunction,
    /// Particle.
    Particle,
    /// Interjection.
    Interjection
}

impl PartOfSpeech {
    /// Reports whether the part of speech carries lexical meaning worth looking
    /// up in the lexicon.
    ///
    /// Function words are skipped by the sense selection gate: they are handled
    /// by frames and agreement, not by definitions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::grammar::PartOfSpeech;
    ///
    /// assert!(PartOfSpeech::Noun.is_content());
    /// assert!(!PartOfSpeech::Preposition.is_content());
    /// ```
    #[must_use]
    pub const fn is_content(self) -> bool {
        matches!(
            self,
            Self::Noun
                | Self::Adjective
                | Self::ShortAdjective
                | Self::Comparative
                | Self::Verb
                | Self::Infinitive
                | Self::Participle
                | Self::ShortParticiple
                | Self::AdverbialParticiple
                | Self::Numeral
                | Self::Adverb
                | Self::Predicative
        )
    }

    /// Reports whether forms of this part of speech can head a predicate frame.
    #[must_use]
    pub const fn is_predicative(self) -> bool {
        matches!(
            self,
            Self::Verb
                | Self::Infinitive
                | Self::Participle
                | Self::ShortParticiple
                | Self::AdverbialParticiple
                | Self::Predicative
                | Self::ShortAdjective
        )
    }
}
