// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Word forms and their morphological readings.
//!
//! A form is ambiguous far more often than a speaker notices: `стекло` is a
//! noun and a verb, `печь` is a noun and an infinitive. The domain therefore
//! never speaks of *the* reading of a form — it carries every reading an
//! analyzer produced, each with its own support, and lets the later gates throw
//! away what does not fit.

use core::fmt::{self, Display, Formatter};
use std::{borrow::Cow, string::String, vec::Vec};

use crate::{
    error::{CoreError, Result},
    evidence::Evidenced,
    grammar::GrammarTag,
    id::LemmaId
};

/// A word form accepted for analysis.
///
/// Construction normalizes case and the typographic apostrophes and dashes that
/// arrive from real text, and rejects anything that is not a single Russian
/// word, so the layers downstream never re-check the alphabet.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct WordForm(String);

impl WordForm {
    /// Normalizes and validates a word form.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::UnusableWordForm`] when the input is empty, holds
    /// whitespace or a character outside the Cyrillic alphabet, the hyphen and
    /// the apostrophe, or when a hyphen or an apostrophe does not stand
    /// between letters. The hyphen joins the written parts of one word —
    /// `кто-нибудь` — and the apostrophe stands inside borrowed names —
    /// `д'артаньян` — so neither can open or close a word.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::morphology::WordForm;
    ///
    /// let form = WordForm::parse("  Столом ")?;
    /// assert_eq!(form.as_str(), "столом");
    ///
    /// assert!(WordForm::parse("two words").is_err());
    /// # Ok::<(), rusem::error::CoreError>(())
    /// ```
    pub fn parse(input: &str) -> Result<Self> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(CoreError::UnusableWordForm {
                reason: "empty"
            });
        }

        let normalized: String = trimmed
            .chars()
            .map(|symbol| match symbol {
                '\u{2010}'..='\u{2015}' => '-',
                '\u{2019}' => '\'',
                other => other
            })
            .flat_map(char::to_lowercase)
            .collect();

        if normalized
            .chars()
            .any(|symbol| !(crate::alphabet::is_letter(symbol) || symbol == '-' || symbol == '\''))
        {
            return Err(CoreError::UnusableWordForm {
                reason: "not a single Cyrillic word"
            });
        }
        if !joins_letters(&normalized) {
            return Err(CoreError::UnusableWordForm {
                reason: "a hyphen or apostrophe not between letters"
            });
        }

        Ok(Self(normalized))
    }

    /// Returns the normalized text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the form with `ё` folded to `е`.
    ///
    /// A writer may always drop the diaeresis, so two spellings may be one
    /// form. The folding itself is stated once, in
    /// [`crate::alphabet::vowel::folded`], and this is that statement asked
    /// of a whole form — for a caller comparing forms or looking one up. It
    /// is never stored: it would erase the distinction between `все` and
    /// `всё`.
    #[must_use]
    pub fn folded(&self) -> Cow<'_, str> {
        crate::alphabet::vowel::folded(&self.0)
    }

    /// Returns the number of characters, not bytes.
    #[must_use]
    pub fn char_len(&self) -> usize {
        self.0.chars().count()
    }
}

/// Reports whether every hyphen and apostrophe stands between letters.
///
/// Both signs join what is around them — the hyphen joins the written parts
/// of one word, the apostrophe the pieces of a borrowed name — so a sign with
/// no letter on either side joins nothing and the input is not a word.
fn joins_letters(normalized: &str) -> bool {
    let letters: Vec<char> = normalized.chars().collect();

    letters.iter().enumerate().all(|(at, symbol)| {
        crate::alphabet::is_letter(*symbol)
            || (at
                .checked_sub(1)
                .and_then(|before| letters.get(before))
                .copied()
                .is_some_and(crate::alphabet::is_letter)
                && letters
                    .get(at + 1)
                    .copied()
                    .is_some_and(crate::alphabet::is_letter))
    })
}

impl Display for WordForm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<String> for WordForm {
    type Error = CoreError;

    fn try_from(input: String) -> Result<Self> {
        Self::parse(&input)
    }
}

impl From<WordForm> for String {
    fn from(form: WordForm) -> Self {
        form.0
    }
}

/// One morphological reading of one form.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Analysis {
    /// The dictionary form this reading points at.
    pub lemma:     WordForm,
    /// Identity of the lemma, when the analyzer's dictionary is indexed in the
    /// store. Absent for forms recovered by rule rather than by lookup.
    pub lemma_id:  Option<LemmaId>,
    /// Grammatical description of the analyzed form under this reading.
    pub tag:       GrammarTag,
    /// Whether the reading was guessed from the shape of the form rather than
    /// found in the dictionary.
    ///
    /// A guess is still a reading — Russian forms words faster than any
    /// dictionary records them — but it is not evidence that the word exists,
    /// and the gates above treat it accordingly.
    pub predicted: bool
}

impl Analysis {
    /// This reading as the form it describes, or nothing when the tag
    /// describes no form of Russian.
    ///
    /// The tag is what an adapter answers in: every category optional, so that
    /// an analyzer may stay silent about what it could not work out. The form
    /// is what the language has. A reading whose tag puts a verb in a case or
    /// a past tense in a person has no form, and this is where that is caught
    /// rather than carried upward.
    #[must_use]
    pub fn form(&self) -> Option<crate::grammar::form::Form> {
        crate::grammar::form::read::form(&self.tag)
    }

    /// Reports whether the tag of this reading describes a form of Russian.
    ///
    /// A reading that does not is not necessarily wrong about the word — the
    /// analyzer may simply have said less than a form needs — but nothing
    /// above may reason about it as though it were a settled form.
    #[must_use]
    pub fn is_a_form(&self) -> bool {
        self.form().is_some()
    }
}

/// A morphological reading with its support.
pub type Reading = Evidenced<Analysis>;

/// Every reading an analyzer produced for one form, strongest first.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Readings {
    /// The analyzed form, normalized.
    pub form:     WordForm,
    /// The readings, ordered by descending confidence.
    pub readings: Vec<Reading>
}

impl Readings {
    /// Builds a reading set, ordering it by descending confidence.
    #[must_use]
    pub fn new(form: WordForm, mut readings: Vec<Reading>) -> Self {
        readings.sort_by(|left, right| {
            right
                .confidence
                .partial_cmp(&left.confidence)
                .unwrap_or(core::cmp::Ordering::Equal)
        });
        Self {
            form,
            readings
        }
    }

    /// Reports whether the dictionary holds no reading of the form.
    ///
    /// A form the analyzer only guessed at counts as unknown: the guess
    /// describes how the form would inflect, not that anyone uses it. An
    /// unrecognized form is not an error either way — it is the entry point of
    /// the word-formation layer, which tries to motivate the word from known
    /// morphemes.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.readings.iter().all(|reading| reading.value.predicted)
    }

    /// Returns the best supported reading, if any.
    #[must_use]
    pub fn best(&self) -> Option<&Reading> {
        self.readings.first()
    }
}

/// The inflected forms of one lemma.
///
/// The paradigm is stated in forms, not identifiers: the morphological layer
/// works on the language itself and knows nothing about what the store happens
/// to have indexed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Paradigm {
    /// The dictionary form the paradigm belongs to.
    pub lemma: WordForm,
    /// The forms, each with the grammatical description it realizes.
    pub forms: Vec<(WordForm, GrammarTag)>
}

#[cfg(test)]
mod tests {
    use std::string::ToString;

    use super::*;
    use crate::{
        evidence::{Confidence, Provenance},
        grammar::PartOfSpeech,
        id::SourceId
    };

    fn reading(lemma: &str, confidence: Confidence) -> Reading {
        Evidenced::new(
            Analysis {
                lemma:     WordForm::parse(lemma).expect("valid form"),
                lemma_id:  None,
                tag:       GrammarTag::of(PartOfSpeech::Noun),
                predicted: false
            },
            Provenance::new(SourceId::new(1).expect("non-zero"), lemma.to_string()),
            confidence
        )
    }

    #[test]
    fn parsing_normalizes_case_and_spacing() {
        let form = WordForm::parse(" СТОЛОМ ").expect("valid form");
        assert_eq!(form.as_str(), "столом");
    }

    #[test]
    fn parsing_rejects_non_words() {
        assert!(WordForm::parse("").is_err());
        assert!(WordForm::parse("два слова").is_err());
        assert!(WordForm::parse("table").is_err());
        assert!(WordForm::parse("стол2").is_err());
    }

    #[test]
    fn parsing_keeps_hyphenated_words() {
        let form = WordForm::parse("кто-нибудь").expect("valid form");
        assert_eq!(form.as_str(), "кто-нибудь");
    }

    #[test]
    fn parsing_keeps_an_apostrophe_between_letters() {
        let form = WordForm::parse("д'Артаньян").expect("valid form");
        assert_eq!(form.as_str(), "д'артаньян");
    }

    #[test]
    fn a_sign_that_joins_no_letters_is_rejected() {
        assert!(WordForm::parse("-стол").is_err());
        assert!(WordForm::parse("стол-").is_err());
        assert!(WordForm::parse("---").is_err());
        assert!(WordForm::parse("'стол").is_err());
        assert!(WordForm::parse("кто--нибудь").is_err());
    }

    #[test]
    fn typographic_dashes_are_folded() {
        let form = WordForm::parse("кто\u{2011}нибудь").expect("valid form");
        assert_eq!(form.as_str(), "кто-нибудь");
    }

    #[test]
    fn folding_yo_does_not_change_storage() {
        let form = WordForm::parse("всё").expect("valid form");

        assert_eq!(form.folded(), "все");
        assert_eq!(form.as_str(), "всё");
    }

    #[test]
    fn readings_are_ordered_by_support() {
        let set = Readings::new(
            WordForm::parse("стекло").expect("valid form"),
            std::vec![
                reading("стекать", Confidence::GUESSED),
                reading("стекло", Confidence::CERTAIN),
            ]
        );

        assert_eq!(
            set.best().map(|reading| reading.value.lemma.as_str()),
            Some("стекло")
        );
    }

    #[test]
    fn empty_reading_set_is_unknown() {
        let set = Readings::new(
            WordForm::parse("кибербуряк").expect("valid form"),
            Vec::new()
        );

        assert!(set.is_unknown());
        assert!(set.best().is_none());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialization_re_validates_the_alphabet() {
        let accepted: WordForm = serde_json::from_str(r#""Столом""#).expect("a Russian word");
        assert_eq!(accepted.as_str(), "столом");

        assert!(serde_json::from_str::<WordForm>(r#""table""#).is_err());
    }
}
