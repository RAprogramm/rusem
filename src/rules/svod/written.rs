// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Every paragraph of the code, as a rule the checker can hold in a list.
//!
//! Each paragraph is written as a module: the text, the reasoning, the tables
//! it needs and a function shaped to the facts it actually uses. That shape is
//! right for reading and wrong for asking — ten functions with ten different
//! argument lists cannot be put in one list.
//!
//! So each one gets a name here and answers [`Rule`] the same way as the rest.
//! The module keeps its own function, which is what its tests and its readers
//! use; this file only says which facts that function is given.

use super::{
    hyphen_compound::{interjections, particles},
    interjection_comma, ne_together, ni_together, prefix_before_i, sibilant_vowels, soft_sign,
    ts_vowels, unstressed_ending, unstressed_o
};
use crate::rules::{Citation, Facts, Findings, Rule, Scope, scope};

/// Names a paragraph whose rule reads the writing and nothing else.
macro_rules! off_the_letters {
    ($name:ident, $module:ident) => {
        /// The paragraph, as a rule.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name;

        impl Rule for $name {
            fn cites(&self) -> Citation {
                $module::Rule::CITES
            }

            fn scope(&self) -> Scope {
                $module::Rule::SCOPE
            }

            fn found(&self, facts: &Facts<'_>) -> Findings {
                $module::found(facts.writing.written)
            }
        }
    };
}

off_the_letters!(SibilantVowels, sibilant_vowels);
off_the_letters!(DoubledInterjection, interjections);
off_the_letters!(HyphenedParticle, particles);
/// § 88, пункт 5, as a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NeTogether;

impl Rule for NeTogether {
    fn cites(&self) -> Citation {
        ne_together::pronouns::Rule::CITES
    }

    fn scope(&self) -> Scope {
        ne_together::pronouns::Rule::SCOPE
    }

    fn found(&self, facts: &Facts<'_>) -> Findings {
        ne_together::pronouns::found(facts.writing.written)
    }
}

/// § 90, пункт 1, as a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NiTogether;

impl Rule for NiTogether {
    fn cites(&self) -> Citation {
        ni_together::pronouns::Rule::CITES
    }

    fn scope(&self) -> Scope {
        ni_together::pronouns::Rule::SCOPE
    }

    fn found(&self, facts: &Facts<'_>) -> Findings {
        ni_together::pronouns::found(facts.writing.written)
    }
}

/// § 90, пункт 2, as a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NiTogetherAdverbs;

impl Rule for NiTogetherAdverbs {
    fn cites(&self) -> Citation {
        ni_together::adverbs::Rule::CITES
    }

    fn scope(&self) -> Scope {
        ni_together::adverbs::Rule::SCOPE
    }

    fn found(&self, facts: &Facts<'_>) -> Findings {
        ni_together::adverbs::found(facts.writing.written)
    }
}

/// § 3, which asks whether the word is a proper name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TsVowels;

impl Rule for TsVowels {
    fn cites(&self) -> Citation {
        ts_vowels::Rule::CITES
    }

    fn scope(&self) -> Scope {
        ts_vowels::Rule::SCOPE
    }

    fn found(&self, facts: &Facts<'_>) -> Findings {
        ts_vowels::found(facts.writing.written, facts.about.proper)
    }
}

/// § 5, which asks whether the word is Russian and where the stress falls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnstressedO;

impl Rule for UnstressedO {
    fn cites(&self) -> Citation {
        unstressed_o::Rule::CITES
    }

    fn scope(&self) -> Scope {
        unstressed_o::Rule::SCOPE
    }

    fn found(&self, facts: &Facts<'_>) -> Findings {
        unstressed_o::found(
            facts.writing.written,
            facts.about.native,
            facts.about.stress
        )
    }
}

/// § 7, which asks how the word is put together.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefixBeforeI;

impl Rule for PrefixBeforeI {
    fn cites(&self) -> Citation {
        prefix_before_i::Rule::CITES
    }

    fn scope(&self) -> Scope {
        prefix_before_i::Rule::SCOPE
    }

    fn found(&self, facts: &Facts<'_>) -> Findings {
        prefix_before_i::found(facts.writing.written, facts.about.parts)
    }
}

/// § 40, which asks for the form and the stress.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnstressedEnding;

impl Rule for UnstressedEnding {
    fn cites(&self) -> Citation {
        unstressed_ending::Rule::CITES
    }

    fn scope(&self) -> Scope {
        unstressed_ending::Rule::SCOPE
    }

    fn found(&self, facts: &Facts<'_>) -> Findings {
        unstressed_ending::found(facts.writing.written, facts.about.form, facts.about.stress)
    }
}

/// § 72, judged by the letters alone.
///
/// The paragraph's points also answer callers that hold the softness of a
/// consonant — the decliner does — but what the letters show is judged here,
/// on every word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoftSign;

impl Rule for SoftSign {
    fn cites(&self) -> Citation {
        soft_sign::Rule::CITES
    }

    fn scope(&self) -> Scope {
        scope::Scope::ANY
    }

    fn found(&self, facts: &Facts<'_>) -> Findings {
        soft_sign::judged(facts.writing.written)
    }
}

/// § 157, which asks what stands after the word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterjectionComma;

impl Rule for InterjectionComma {
    fn cites(&self) -> Citation {
        interjection_comma::Rule::CITES
    }

    fn scope(&self) -> Scope {
        interjection_comma::Rule::SCOPE
    }

    fn found(&self, facts: &Facts<'_>) -> Findings {
        interjection_comma::found(facts.writing.written, facts.writing.next)
    }
}

/// Every paragraph the engine has, as rules the checker can ask.
///
/// A namespace for the list below: one paragraph written and not listed is one
/// nothing will ever ask.
///
/// # Examples
///
/// ```
/// use rusem::rules::svod::written::Rules;
///
/// assert!(!Rules::RULES.is_empty());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rules;

impl Rules {
    /// Every paragraph the engine has, in the order they stand in the code.
    ///
    /// A paragraph is written when the engine holds the facts to ask it. One
    /// that is written and not here is one nothing will ever ask, which is
    /// the mistake this list exists to make impossible.
    pub const RULES: &[&dyn Rule] = &[
        &SibilantVowels,
        &TsVowels,
        &UnstressedO,
        &PrefixBeforeI,
        &UnstressedEnding,
        &SoftSign,
        &DoubledInterjection,
        &HyphenedParticle,
        &NeTogether,
        &NiTogether,
        &NiTogetherAdverbs,
        &InterjectionComma
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        grammar::form::Form,
        phonetics::stress::Stressed,
        rules::{
            Parts,
            facts::{About, Writing},
            rule::{admits, asked}
        }
    };

    fn built<'a>(
        written: &'a str,
        next: Option<&'a str>,
        stress: &'a Stressed,
        parts: Parts
    ) -> Facts<'a> {
        spoken(written, next, stress, parts, Form::Adverb)
    }

    fn spoken<'a>(
        written: &'a str,
        next: Option<&'a str>,
        stress: &'a Stressed,
        parts: Parts,
        form: Form
    ) -> Facts<'a> {
        Facts {
            writing: Writing {
                written,
                next
            },
            about:   About {
                form,
                stress,
                native: true,
                proper: false,
                parts
            }
        }
    }

    fn facts<'a>(written: &'a str, next: Option<&'a str>, stress: &'a Stressed) -> Facts<'a> {
        Facts {
            writing: Writing {
                written,
                next
            },
            about:   About {
                form: Form::Adverb,
                stress,
                native: true,
                proper: false,
                parts: Parts::Unknown
            }
        }
    }

    #[test]
    fn every_paragraph_written_is_in_the_list() {
        let stated = [
            sibilant_vowels::Rule::CITES,
            ts_vowels::Rule::CITES,
            unstressed_o::Rule::CITES,
            prefix_before_i::Rule::CITES,
            unstressed_ending::Rule::CITES,
            soft_sign::Rule::CITES,
            soft_sign::end_of_word::Rule::CITES,
            soft_sign::before_hard::Rule::CITES,
            soft_sign::before_hardening::Rule::CITES,
            soft_sign::before_l::Rule::CITES,
            soft_sign::not_before_soft::Rule::CITES,
            interjections::Rule::CITES,
            particles::Rule::CITES,
            ne_together::pronouns::Rule::CITES,
            ni_together::pronouns::Rule::CITES,
            ni_together::adverbs::Rule::CITES,
            interjection_comma::Rule::CITES
        ];

        for held in stated {
            assert!(
                Rules::RULES
                    .iter()
                    .any(|rule| rule.cites().paragraph == held.paragraph),
                "§ {} is written and nothing will ever ask it",
                held.paragraph
            );
        }
    }

    #[test]
    fn no_paragraph_is_listed_twice() {
        let mut cited: Vec<(u16, u16)> = Rules::RULES
            .iter()
            .map(|rule| {
                let held = rule.cites();

                (held.paragraph, held.point)
            })
            .collect();
        let counted = cited.len();
        cited.sort_unstable();
        cited.dedup();

        assert_eq!(cited.len(), counted, "a paragraph stands in the list twice");
    }

    #[test]
    fn every_rule_cites_a_paragraph_of_the_code() {
        for rule in Rules::RULES {
            let held = rule.cites();

            assert!(held.is_stated(), "{held} cites nothing");
            assert!(held.paragraph <= Citation::LAST, "{held} is past the end");
        }
    }

    #[test]
    fn a_word_meets_the_rule_that_speaks_of_it() {
        let stress = Stressed::unknown();
        let held = built("розиск", None, &stress, Parts::Prefixed(3));

        assert!(
            !asked(&PrefixBeforeI, &held).is_empty(),
            "§ 7 sees the prefix"
        );
    }

    #[test]
    fn a_word_no_paragraph_speaks_of_meets_none_of_them() {
        let stress = Stressed::unknown();
        let held = facts("вода", None, &stress);

        for rule in Rules::RULES {
            let cited = rule.cites();

            assert!(asked(*rule, &held).is_empty(), "{cited} fired on вода");
        }
    }

    #[test]
    fn no_paragraph_fires_on_a_word_none_of_them_is_about() {
        let stress = Stressed::settled(crate::phonetics::stress::Stress::On(1));

        for rule in [
            &SibilantVowels as &dyn Rule,
            &TsVowels,
            &PrefixBeforeI,
            &DoubledInterjection,
            &UnstressedO
        ] {
            let held = built("вода", None, &stress, Parts::Bare);
            let cited = rule.cites();

            assert!(asked(rule, &held).is_empty(), "{cited} fired on вода");
        }
    }

    #[test]
    fn the_punctuation_paragraph_reads_what_stands_after_the_word() {
        let stress = Stressed::unknown();
        let parted = facts("увы", Some("он"), &stress);
        let joined = facts("ах", Some("ты"), &stress);

        let held = asked(&InterjectionComma, &parted);
        assert_eq!(held.len(), 1);
        assert_eq!(held[0].instead, "увы, он");

        assert!(asked(&InterjectionComma, &joined).is_empty());
    }

    #[test]
    fn the_paragraph_about_a_noun_ending_is_asked_about_a_noun() {
        let stress = Stressed::settled(crate::phonetics::stress::Stress::On(1));
        let held = Facts {
            writing: Writing {
                written: "гение",
                next:    None
            },
            about:   About {
                form:   Form::Noun(crate::grammar::form::Agreed::Singular {
                    case:   crate::grammar::Case::Prepositional,
                    gender: crate::grammar::Gender::Masculine
                }),
                stress: &stress,
                native: true,
                proper: false,
                parts:  Parts::Unknown
            }
        };

        assert!(admits(&UnstressedEnding, &held), "§ 40 speaks of this noun");

        let found = asked(&UnstressedEnding, &held);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].instead, "гении");
    }

    #[test]
    fn every_rule_that_speaks_off_the_letters_is_heard_saying_something() {
        let stress = Stressed::settled(crate::phonetics::stress::Stress::On(1));

        for (written, rule, parts, form) in [
            (
                "жыр",
                &SibilantVowels as &dyn Rule,
                Parts::Bare,
                Form::Adverb
            ),
            ("цюпля", &TsVowels, Parts::Bare, Form::Adverb),
            ("розиск", &PrefixBeforeI, Parts::Prefixed(3), Form::Adverb),
            (
                "хахаха",
                &DoubledInterjection,
                Parts::Bare,
                Form::Interjection
            ),
            ("шопот", &UnstressedO, Parts::Bare, Form::Adverb)
        ] {
            let held = spoken(written, None, &stress, parts, form);

            let cited = rule.cites();

            assert!(
                !asked(rule, &held).is_empty(),
                "{cited} says nothing about {written}"
            );
        }
    }

    #[test]
    fn every_rule_that_reads_a_pair_is_heard_saying_something() {
        let stress = Stressed::unknown();

        for (written, next, rule) in [
            ("кто то", Some("то"), &HyphenedParticle as &dyn Rule),
            ("ни кто", Some("кто"), &NiTogether),
            ("увы", Some("он"), &InterjectionComma)
        ] {
            let held = Facts {
                writing: Writing {
                    written,
                    next
                },
                about:   About {
                    form:   Form::Adverb,
                    stress: &stress,
                    native: true,
                    proper: false,
                    parts:  Parts::Unknown
                }
            };

            let cited = rule.cites();

            assert!(
                !asked(rule, &held).is_empty(),
                "{cited} says nothing about {written} before {next:?}"
            );
        }
    }

    #[test]
    fn every_rule_is_asked_and_none_of_them_panics() {
        let stress = Stressed::unknown();

        for written in [
            "",
            "вода",
            "розиск",
            "хахаха",
            "кто то",
            "ни кто",
            "увы",
            "жыр"
        ] {
            let held = facts(written, Some("дом"), &stress);

            for rule in Rules::RULES {
                let _ = asked(*rule, &held);
            }
        }
    }
}
