// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What a rule is about, so that only the rules that could answer are asked.
//!
//! § 40 speaks of nouns in the prepositional and the dative, in the singular.
//! Asking it about a verb wastes the question and invites a wrong answer from
//! a rule that was never about verbs. Two hundred rules asked of every word
//! are two hundred chances to fire where nothing was meant.
//!
//! So a rule declares its class, the checker indexes by it, and a word meets
//! the handful of rules that speak of its kind.
//!
//! The scope is a filter over the form, and only over the form. Whether the
//! word is in the dictionary, whether the sentence holds together, whether the
//! stress is known — none of that is here. A scope that could refuse for those
//! reasons would be doing the checking rather than routing it.

use crate::{
    grammar::{Case, Number, PartOfSpeech, form::Form},
    rules::facts::{About, Parts}
};

/// The facts a rule cannot judge without.
///
/// The scope names them, and the base keeps a rule away from a word whose
/// facts are not known — so a rule never has to answer "I cannot tell", and
/// its silence always means the writing satisfied it. A word with an unknown
/// stress simply never meets § 5, and that is visible from outside: the rule
/// was not asked, rather than asked and mute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Needs {
    /// The rule reads where the stress falls.
    pub stress: bool,
    /// The rule reads how the word is put together.
    pub parts:  bool
}

/// A rule that judges the letters alone.
pub const NOTHING: Needs = Needs {
    stress: false,
    parts:  false
};

/// The class of word a rule speaks of.
///
/// An empty list means the rule does not filter on that category. `§ 1` is
/// about a letter after a sibilant whatever the word is, and its scope names
/// no part of speech at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scope {
    /// The parts of speech the rule speaks of, or none to speak of all.
    pub parts:   &'static [PartOfSpeech],
    /// The cases the rule speaks of, or none to speak of all.
    pub cases:   &'static [Case],
    /// The numbers the rule speaks of, or none to speak of both.
    pub numbers: &'static [Number],
    /// The facts the rule cannot judge without.
    pub needs:   Needs
}

/// A scope that admits every word and needs nothing but the letters.
pub const ANY: Scope = Scope {
    parts:   &[],
    cases:   &[],
    numbers: &[],
    needs:   NOTHING
};

impl Scope {
    /// Reports whether a form is one this rule speaks of.
    ///
    /// A category the rule does not name admits anything, including a form
    /// that states nothing there: § 1 is about spelling and admits a
    /// preposition, which has no case, because it named no cases.
    ///
    /// A category the rule does name admits only a form that states one of the
    /// values listed. A rule about the prepositional does not admit a word
    /// standing in no case at all, because there is nothing for it to be about.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::{
    ///     grammar::{
    ///         Case, Gender, Number, PartOfSpeech,
    ///         form::{Agreed, Form}
    ///     },
    ///     rules::{Scope, scope}
    /// };
    ///
    /// const NOUNS: Scope = Scope {
    ///     parts:   &[PartOfSpeech::Noun],
    ///     cases:   &[Case::Prepositional],
    ///     numbers: &[Number::Singular],
    ///     needs:   scope::NOTHING
    /// };
    ///
    /// let held = Form::Noun(Agreed::Singular {
    ///     case:   Case::Prepositional,
    ///     gender: Gender::Masculine
    /// });
    /// assert!(NOUNS.admits(held));
    ///
    /// let other = Form::Noun(Agreed::Plural {
    ///     case: Case::Prepositional
    /// });
    /// assert!(!NOUNS.admits(other));
    /// ```
    #[must_use]
    pub fn admits(&self, form: Form) -> bool {
        self.admits_part(form) && self.admits_case(form) && self.admits_number(form)
    }

    /// Reports whether the facts the rule needs are known of this word.
    ///
    /// A missing fact is not a broken word: the rule simply cannot judge, and
    /// the base keeps it away. Whether that happened is visible here, at the
    /// gate, instead of being buried in a rule's silence.
    #[must_use]
    pub const fn judgeable(&self, about: &About<'_>) -> bool {
        if self.needs.stress && !about.stress.is_settled() {
            return false;
        }
        if self.needs.parts && matches!(about.parts, Parts::Unknown) {
            return false;
        }

        true
    }

    /// Reports whether the part of speech is one the rule speaks of.
    fn admits_part(&self, form: Form) -> bool {
        self.parts.is_empty() || self.parts.contains(&form.part_of_speech())
    }

    /// Reports whether the case is one the rule speaks of.
    ///
    /// The case is merged first: a rule about the genitive is about the
    /// partitive too, because the partitive is a genitive the language writes
    /// differently.
    fn admits_case(&self, form: Form) -> bool {
        if self.cases.is_empty() {
            return true;
        }
        let Some(held) = form.case() else {
            return false;
        };

        self.cases
            .iter()
            .any(|stated| stated.merged() == held.merged())
    }

    /// Reports whether the number is one the rule speaks of.
    fn admits_number(&self, form: Form) -> bool {
        if self.numbers.is_empty() {
            return true;
        }
        let Some(held) = form.number() else {
            return false;
        };

        self.numbers.contains(&held)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{Gender, form::Agreed};

    const NOUNS: Scope = Scope {
        parts:   &[PartOfSpeech::Noun],
        cases:   &[Case::Prepositional],
        numbers: &[Number::Singular],
        needs:   NOTHING
    };

    fn noun(case: Case) -> Form {
        Form::Noun(Agreed::Singular {
            case,
            gender: Gender::Masculine
        })
    }

    #[test]
    fn a_scope_that_names_nothing_admits_everything() {
        assert!(ANY.admits(noun(Case::Genitive)));
        assert!(ANY.admits(Form::Preposition));
    }

    #[test]
    fn a_named_part_of_speech_admits_only_itself() {
        assert!(NOUNS.admits(noun(Case::Prepositional)));
        assert!(!NOUNS.admits(Form::Adverb));
    }

    #[test]
    fn a_named_case_refuses_the_others() {
        assert!(!NOUNS.admits(noun(Case::Genitive)));
    }

    #[test]
    fn a_named_case_refuses_a_form_that_stands_in_none() {
        let held = Scope {
            parts: &[],
            ..NOUNS
        };

        assert!(!held.admits(Form::Adverb));
    }

    #[test]
    fn a_named_number_refuses_the_other() {
        let plural = Form::Noun(Agreed::Plural {
            case: Case::Prepositional
        });

        assert!(!NOUNS.admits(plural));
    }

    #[test]
    fn a_genitive_rule_speaks_of_the_partitive_too() {
        let held = Scope {
            parts:   &[PartOfSpeech::Noun],
            cases:   &[Case::Genitive],
            numbers: &[],
            needs:   NOTHING
        };

        assert!(held.admits(noun(Case::Partitive)));
    }
}
