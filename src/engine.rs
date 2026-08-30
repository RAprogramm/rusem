// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The engine that turns a phrase into a verdict.
//!
//! The engine owns the morphological analyzer and runs every word through the
//! rules of the code of 1956. It reads no files and keeps no state between
//! phrases: what it knows about a word comes from the analyzer and from the
//! spelling itself.
//!
//! Some facts are held at their honest defaults until a dictionary-backed
//! adapter refines them. `native` is true because the engine has no reason to
//! call a word a borrowing; `parts` is [`Parts::Unknown`] because morpheme
//! structure is not read from the letters alone; stress is taken from the
//! spelling when the spelling settles it and left unknown otherwise. The crate
//! reads no files; the adapter that brings in a dictionary, a stress table or
//! a derivational index is a separate layer.
//!
//! # Raw writing and punctuation
//!
//! `Facts.writing.written` receives the token exactly as it stands in the
//! phrase, with its case and any trailing punctuation, because a rule about
//! what stands between two words — § 157 — needs to see whether the comma is
//! there. The normalized [`WordForm`] is kept for the analyzer and for the
//! violation payload. When a rule's correction carries that trailing
//! punctuation with it, the engine strips it at the mapping site so the
//! [`Violation::Misspelled::instead`] field names the word, not the mark.

use std::{
    borrow::Cow,
    fmt::{Debug, Formatter, Result as FormatResult},
    sync::Arc
};

use crate::{
    alphabet::is_letter,
    error::Result,
    morphology::WordForm,
    phonetics::stress::of_spelling,
    ports::{Checker, Morphology},
    rules::{
        self, Facts,
        facts::{About, Parts, Writing},
        found::Found,
        rule
    },
    verdict::{Reading, Verdict, Violation}
};

/// The checker that runs the code of 1956 over a phrase.
///
/// Owns the morphological analyzer behind an object-safe port, so the engine
/// can be built with any adapter that implements [`Morphology`].
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
///
/// use rusem::{
///     engine::Engine,
///     error::Result,
///     evidence::{Confidence, Evidenced, Provenance},
///     grammar::{GrammarTag, PartOfSpeech},
///     id::SourceId,
///     morphology::{Analysis, Readings, WordForm},
///     ports::{Checker, Morphology},
///     verdict::{Status, Verdict}
/// };
///
/// # struct Stub;
/// # impl Morphology for Stub {
/// #     fn analyze(&self, form: &WordForm) -> Result<Readings> {
/// #         let analysis = Analysis {
/// #             lemma: form.clone(),
/// #             lemma_id: None,
/// #             tag: GrammarTag::of(PartOfSpeech::Adverb),
/// #             predicted: false
/// #         };
/// #         let provenance = Provenance::new(SourceId::FIRST, form.to_string());
/// #         Ok(Readings::new(
/// #             form.clone(),
/// #             std::vec![Evidenced::new(analysis, provenance, Confidence::CERTAIN)]
/// #         ))
/// #     }
/// #     fn inflect(&self, _form: &WordForm, _tag: &GrammarTag) -> Result<Vec<WordForm>> {
/// #         Ok(Vec::new())
/// #     }
/// #     fn paradigm(&self, _form: &WordForm) -> Result<rusem::morphology::Paradigm> {
/// #         Ok(rusem::morphology::Paradigm {
/// #             lemma: WordForm::parse("тест").expect("valid form"),
/// #             forms: Vec::new()
/// #         })
/// #     }
/// # }
///
/// let engine = Engine::new(Arc::new(Stub));
/// let verdict = engine.check("вода")?;
/// assert_eq!(verdict.status(), Status::Accepted);
/// # Ok::<(), rusem::error::CoreError>(())
/// ```
#[derive(Clone)]
pub struct Engine {
    morphology: Arc<dyn Morphology>
}

impl Debug for Engine {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        formatter.debug_struct("Engine").finish_non_exhaustive()
    }
}

impl Engine {
    /// Builds an engine around a morphological analyzer.
    ///
    /// # Arguments
    ///
    /// * `morphology` — the analyzer the engine asks about every word.
    ///
    /// # Returns
    ///
    /// A checker ready to run the rules of the code of 1956.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use rusem::{engine::Engine, ports::Morphology};
    /// # struct Stub;
    /// # impl Morphology for Stub {
    /// #     fn analyze(&self, form: &rusem::morphology::WordForm)
    /// #         -> rusem::error::Result<rusem::morphology::Readings> {
    /// #         Ok(rusem::morphology::Readings::new(form.clone(), Vec::new()))
    /// #     }
    /// #     fn inflect(&self, _form: &rusem::morphology::WordForm,
    /// #         _tag: &rusem::grammar::GrammarTag)
    /// #         -> rusem::error::Result<Vec<rusem::morphology::WordForm>> {
    /// #         Ok(Vec::new())
    /// #     }
    /// #     fn paradigm(&self, _form: &rusem::morphology::WordForm)
    /// #         -> rusem::error::Result<rusem::morphology::Paradigm> {
    /// #         Ok(rusem::morphology::Paradigm {
    /// #             lemma: rusem::morphology::WordForm::parse("тест").expect("valid form"),
    /// #             forms: Vec::new()
    /// #         })
    /// #     }
    /// # }
    /// let engine = Engine::new(Arc::new(Stub));
    /// ```
    #[must_use]
    pub fn new(morphology: Arc<dyn Morphology>) -> Self {
        Self {
            morphology
        }
    }

    /// Returns the analyzer the engine asks about words.
    ///
    /// # Arguments
    ///
    /// Takes no arguments.
    ///
    /// # Returns
    ///
    /// A reference to the owned [`Arc<dyn Morphology>`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use rusem::{engine::Engine, ports::Morphology};
    /// # struct Stub;
    /// # impl Morphology for Stub {
    /// #     fn analyze(&self, form: &rusem::morphology::WordForm)
    /// #         -> rusem::error::Result<rusem::morphology::Readings> {
    /// #         Ok(rusem::morphology::Readings::new(form.clone(), Vec::new()))
    /// #     }
    /// #     fn inflect(&self, _form: &rusem::morphology::WordForm,
    /// #         _tag: &rusem::grammar::GrammarTag)
    /// #         -> rusem::error::Result<Vec<rusem::morphology::WordForm>> {
    /// #         Ok(Vec::new())
    /// #     }
    /// #     fn paradigm(&self, _form: &rusem::morphology::WordForm)
    /// #         -> rusem::error::Result<rusem::morphology::Paradigm> {
    /// #         Ok(rusem::morphology::Paradigm {
    /// #             lemma: rusem::morphology::WordForm::parse("тест").expect("valid form"),
    /// #             forms: Vec::new()
    /// #         })
    /// #     }
    /// # }
    /// let engine = Engine::new(Arc::new(Stub));
    /// let _ = engine.morphology();
    /// ```
    #[must_use]
    pub fn morphology(&self) -> &Arc<dyn Morphology> {
        &self.morphology
    }
}

impl Checker for Engine {
    /// Runs the code of 1956 over a phrase.
    ///
    /// # Arguments
    ///
    /// * `phrase` — the text to check.
    ///
    /// # Returns
    ///
    /// A [`Verdict`] with the readings that survived and every violation found.
    fn check(&self, phrase: &str) -> Result<Verdict> {
        let mut violations = Vec::new();
        let tokens = tokenize(phrase, &mut violations);
        let mut readings = Vec::new();

        for (place, (form, raw)) in tokens.iter().enumerate() {
            let analyzed = self.morphology.analyze(form)?;
            if analyzed.readings.is_empty() {
                violations.push(Violation::UnknownWord {
                    form: form.clone()
                });
                continue;
            }
            if analyzed
                .readings
                .iter()
                .all(|reading| reading.value.predicted)
            {
                violations.push(Violation::UnlistedWord {
                    form: form.clone()
                });
                continue;
            }

            readings.extend(
                analyzed
                    .readings
                    .iter()
                    .filter(|reading| !reading.value.predicted)
                    .map(|_| Reading {
                        senses:      Vec::new(),
                        frame_sense: None
                    })
            );

            let Some(best) = analyzed
                .readings
                .iter()
                .find(|reading| !reading.value.predicted)
            else {
                continue;
            };
            let Some(about_form) = best.value.form() else {
                continue;
            };

            let stress = of_spelling(form.as_str());
            let proper = raw.chars().next().is_some_and(char::is_uppercase);
            let next = tokens.get(place + 1).map(|(_, next_raw)| next_raw.as_str());
            let facts = Facts {
                writing: Writing {
                    written: raw.as_str(),
                    next
                },
                about:   About {
                    form: about_form,
                    stress: &stress,
                    native: true,
                    proper,
                    parts: Parts::Unknown
                }
            };

            for rule in rules::all() {
                for found in rule::asked(rule, &facts) {
                    violations.push(into_misspelled(form.clone(), &found));
                }
            }
        }

        Ok(Verdict::checked(readings, violations))
    }
}

const fn is_token_char(symbol: char) -> bool {
    is_letter(symbol) || symbol == '-' || symbol == '\''
}

fn letters_only(raw: &str) -> Option<WordForm> {
    let kept: String = raw.chars().filter(|symbol| is_letter(*symbol)).collect();
    WordForm::parse(&kept).ok()
}

fn tokenize(phrase: &str, violations: &mut Vec<Violation>) -> Vec<(WordForm, String)> {
    let mut tokens = Vec::new();
    let mut chars = phrase.chars().peekable();

    while let Some(symbol) = chars.next() {
        if !is_token_char(symbol) {
            continue;
        }

        let mut raw = String::new();
        raw.push(symbol);

        while let Some(&next) = chars.peek() {
            if !is_token_char(next) {
                break;
            }
            raw.push(next);
            chars.next();
        }

        while let Some(&next) = chars.peek() {
            if next.is_whitespace() || is_token_char(next) {
                break;
            }
            raw.push(next);
            chars.next();
        }

        let word: String = raw
            .chars()
            .take_while(|&held| is_token_char(held))
            .collect();

        if let Ok(form) = WordForm::parse(&word) {
            tokens.push((form, raw));
        } else if word.chars().any(is_letter)
            && let Some(form) = letters_only(&word)
        {
            violations.push(Violation::UnknownWord {
                form
            });
        }
    }

    tokens
}

fn clean_instead(instead: &str) -> String {
    let letters: Vec<char> = instead.chars().collect();
    let trailing = letters
        .iter()
        .rev()
        .take_while(|symbol| !is_token_char(**symbol))
        .count();
    let keep = letters.len().saturating_sub(trailing);

    letters.into_iter().take(keep).collect()
}

fn into_misspelled(form: WordForm, found: &Found) -> Violation {
    Violation::Misspelled {
        form,
        cites: found.cites,
        at: found.at,
        says: Cow::Borrowed(found.says),
        instead: clean_instead(&found.instead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        evidence::{Confidence, Evidenced, Provenance},
        grammar::{GrammarTag, PartOfSpeech},
        id::SourceId,
        morphology::{Analysis, Paradigm, Readings},
        verdict::Status
    };

    #[derive(Debug)]
    struct Stub;

    fn parsed(text: &str) -> WordForm {
        WordForm::parse(text).expect("valid form")
    }

    fn reading(form: &WordForm, part: PartOfSpeech, predicted: bool) -> Evidenced<Analysis> {
        let value = Analysis {
            lemma: form.clone(),
            lemma_id: None,
            tag: GrammarTag::of(part),
            predicted
        };
        let provenance = Provenance::new(SourceId::FIRST, form.to_string());
        let confidence = if predicted {
            Confidence::GUESSED
        } else {
            Confidence::CERTAIN
        };

        Evidenced::new(value, provenance, confidence)
    }

    impl Morphology for Stub {
        fn analyze(&self, form: &WordForm) -> Result<Readings> {
            let readings = match form.as_str() {
                "вода" | "жыр" | "увы" => {
                    std::vec![reading(form, PartOfSpeech::Adverb, false)]
                }
                "бббб" => std::vec![reading(form, PartOfSpeech::Noun, true)],
                _ => Vec::new()
            };

            Ok(Readings::new(form.clone(), readings))
        }

        fn inflect(&self, _form: &WordForm, _tag: &GrammarTag) -> Result<Vec<WordForm>> {
            Ok(Vec::new())
        }

        fn paradigm(&self, _form: &WordForm) -> Result<Paradigm> {
            Ok(Paradigm {
                lemma: parsed("тест"),
                forms: Vec::new()
            })
        }
    }

    #[test]
    fn a_clean_phrase_passes_with_no_violations() {
        let engine = Engine::new(Arc::new(Stub));
        let verdict = engine.check("вода").expect("a verdict");

        assert!(verdict.violations.is_empty());
        assert_eq!(verdict.status(), Status::Accepted);
    }

    #[test]
    fn a_misspelling_is_named_with_the_word_that_should_stand() {
        let engine = Engine::new(Arc::new(Stub));
        let verdict = engine.check("жыр").expect("a verdict");

        assert_eq!(verdict.violations.len(), 1);
        let held = &verdict.violations[0];
        assert_eq!(held.kind(), "misspelled");
        assert!(matches!(
            held,
            Violation::Misspelled {
                instead,
                ..
            } if instead == "жир"
        ));
    }

    #[test]
    fn a_trailing_comma_does_not_pollute_the_correction() {
        let engine = Engine::new(Arc::new(Stub));
        let verdict = engine.check("жыр,").expect("a verdict");

        assert_eq!(verdict.violations.len(), 1);
        assert!(matches!(
            &verdict.violations[0],
            Violation::Misspelled {
                instead,
                ..
            } if instead == "жир"
        ));
    }

    #[test]
    fn a_comma_after_an_interjection_is_not_a_violation() {
        let engine = Engine::new(Arc::new(Stub));
        let verdict = engine.check("увы, вода").expect("a verdict");

        assert!(verdict.violations.is_empty());
    }

    #[test]
    fn a_missing_comma_after_an_interjection_is_found() {
        let engine = Engine::new(Arc::new(Stub));
        let verdict = engine.check("увы вода").expect("a verdict");

        assert_eq!(verdict.violations.len(), 1);
        let held = &verdict.violations[0];
        assert_eq!(held.kind(), "misspelled");
        assert!(matches!(
            held,
            Violation::Misspelled {
                instead,
                ..
            } if instead == "увы, вода"
        ));
    }

    #[test]
    fn an_unknown_word_is_reported_as_such() {
        let engine = Engine::new(Arc::new(Stub));
        let verdict = engine.check("кккк").expect("a verdict");

        assert_eq!(verdict.violations.len(), 1);
        assert!(matches!(
            verdict.violations[0],
            Violation::UnknownWord { .. }
        ));
    }

    #[test]
    fn a_latin_or_digit_phrase_yields_nothing() {
        let engine = Engine::new(Arc::new(Stub));
        let verdict = engine.check("123 hello").expect("a verdict");

        assert!(verdict.violations.is_empty());
        assert!(verdict.readings.is_empty());
    }

    #[test]
    fn a_predicted_reading_is_an_unlisted_word() {
        let engine = Engine::new(Arc::new(Stub));
        let verdict = engine.check("бббб").expect("a verdict");

        assert_eq!(verdict.violations.len(), 1);
        assert!(matches!(
            verdict.violations[0],
            Violation::UnlistedWord { .. }
        ));
    }

    #[test]
    fn a_malformed_token_with_letters_is_unknown() {
        let engine = Engine::new(Arc::new(Stub));
        let verdict = engine.check("стол-").expect("a verdict");

        assert_eq!(verdict.violations.len(), 1);
        assert!(matches!(
            verdict.violations[0],
            Violation::UnknownWord { .. }
        ));
    }
}
