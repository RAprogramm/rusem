// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The pronouns, in the nine classes the grammars give them.
//!
//! A pronoun stands in for a noun, an adjective or a numeral and takes what
//! that word would have had. So what matters about it is not what it means but
//! what it points at, and that is what the classes divide by.
//!
//! Two of the nine hold the same words. An **interrogative** asks — `кто
//! пришёл?` — and a **relative** hangs a clause — `тот, кто пришёл`. The words
//! are identical and only what they do in the sentence tells them apart, so
//! they are one list here named for both.
//!
//! The lists hold dictionary forms. A pronoun declines, and `которому` is
//! reached by asking morphology for the lemma rather than by a longer list.
//! The one exception is the personal pronouns: they suppleate — `я` gives
//! `меня`, `он` gives `его` — and no analyzer relates the two by rule, so they
//! are listed whole.
//!
//! The negative and the indefinite classes hold no list at all. Every word in
//! them is an asking pronoun with `ни`, `не`, `кое-` or a particle on it, and
//! [`asking`] builds them.
//!
//! The declined forms live in [`forms`], apart from the dictionary ones: a
//! gate that reads a written word needs them, and a gate that reads a lemma
//! does not.

pub mod forms;

pub use self::forms::{Forms, asks_in_any_form, points_in_any_form, possesses_in_any_form};
/// The pronouns that ask, and the same ones that hang a clause.
///
/// Interrogative and relative are two classes of one list: `какой` asks in
/// `какой день?` and relates in `день, какой мы ждали`. Nothing in the word
/// tells which, and a caller that needs to know reads the sentence.
///
/// Stated in [`asking`], which is where the words that ask live: the negative
/// and the indefinite pronouns are built out of them, and building them in two
/// places would let the two disagree.
pub use super::asking::{self, Asking};

/// The pronouns carrying `не` whose class the writing does not settle.
///
/// Everything built with `ни` denies and everything built with a particle
/// leaves unsaid, so [`asking::built_from`] answers both without a list. `не`
/// is the one that does not answer: stressed it denies — `нЕкого спросить` —
/// and unstressed it leaves unsaid — `нектО пришёл` — and the two are written
/// the same. The words it makes are few and are named here because the letters
/// alone cannot tell them apart.
///
/// `некий` and `некоторый` are here for a second reason: they were built long
/// ago out of words that no longer ask, so nothing builds them now.
const UNBUILT: &[(&str, Class)] = &[
    ("некий", Class::Indefinite),
    ("некого", Class::Negative),
    ("некоторый", Class::Indefinite),
    ("некто", Class::Indefinite),
    ("несколько", Class::Indefinite),
    ("нечего", Class::Negative),
    ("нечто", Class::Indefinite)
];

/// Which class a pronoun belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Class {
    /// `я`, `ты`, `он`: who is speaking, spoken to, spoken of.
    Personal,
    /// `себя`: back on the one acting.
    Reflexive,
    /// `мой`, `свой`: whose.
    Possessive,
    /// `этот`, `тот`: which one, of those already said.
    Demonstrative,
    /// `весь`, `каждый`: which ones, of all there are.
    Definitive,
    /// `кто`, `какой`: asking, or hanging a clause.
    Asking,
    /// `никто`, `ничей`: none.
    Negative,
    /// `кто-то`, `некий`: one, unsaid which.
    Indefinite
}

impl Class {
    /// The personal pronouns, in every form they take.
    ///
    /// Listed whole because they suppleate: `я` and `меня` share no letters,
    /// and nothing but a list relates them.
    pub const PERSONAL: &[&str] = &[
        "вам",
        "вами",
        "вас",
        "вы",
        "его",
        "ей",
        "ему",
        "ею",
        "её",
        "им",
        "ими",
        "их",
        "меня",
        "мне",
        "мной",
        "мною",
        "мы",
        "нам",
        "нами",
        "нас",
        "него",
        "неё",
        "ней",
        "нем",
        "нему",
        "нею",
        "ним",
        "нём",
        "ними",
        "них",
        "он",
        "она",
        "они",
        "оно",
        "тебе",
        "тебя",
        "тобой",
        "тобою",
        "ты",
        "я"
    ];

    /// The reflexive pronoun, which has no nominative.
    ///
    /// One word, and the class holds nothing else.
    pub const REFLEXIVE: &[&str] = &["себе", "себя", "собой", "собою"];

    /// The possessive pronouns, in dictionary form.
    ///
    /// `его`, `её` and `их` are here and among the personal ones too: the same
    /// spelling is the genitive of a personal pronoun and a possessive that
    /// never declines. Only what it stands beside tells them apart.
    pub const POSSESSIVE: &[&str] = &["ваш", "его", "её", "их", "мой", "наш", "свой", "твой"];

    /// The demonstrative pronouns.
    pub const DEMONSTRATIVE: &[&str] =
        &["оный", "сей", "столько", "таков", "такой", "тот", "этот"];

    /// The definitive pronouns.
    pub const DEFINITIVE: &[&str] = &[
        "весь",
        "всякий",
        "всяческий",
        "другой",
        "иной",
        "каждый",
        "любой",
        "сам",
        "самый",
        "целый"
    ];
}

/// Every class, with the words in it.
const CLASSES: &[(Class, &[&str])] = &[
    (Class::Personal, Class::PERSONAL),
    (Class::Reflexive, Class::REFLEXIVE),
    (Class::Possessive, Class::POSSESSIVE),
    (Class::Demonstrative, Class::DEMONSTRATIVE),
    (Class::Definitive, Class::DEFINITIVE),
    (Class::Asking, Asking::PRONOUNS)
];

/// The classes a written word belongs to.
///
/// A list, because a spelling may belong to two: `его` is a personal pronoun
/// in the genitive and a possessive that never declines, and only what stands
/// beside it tells which.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::pronoun::{Class, classes};
///
/// assert_eq!(classes("я"), vec![Class::Personal]);
/// assert_eq!(classes("который"), vec![Class::Asking]);
/// assert!(classes("его").contains(&Class::Possessive));
/// assert!(classes("стол").is_empty());
/// ```
#[must_use]
pub fn classes(written: &str) -> Vec<Class> {
    let held = written.to_lowercase();
    let mut found: Vec<Class> = CLASSES
        .iter()
        .filter(|(_, words)| words.contains(&held.as_str()))
        .map(|(class, _)| *class)
        .collect();

    found.extend(built(&held));
    found
}

/// The class a pronoun built out of an asking one belongs to.
///
/// `ни` denies and the particles leave unsaid, so both answer a class. `не`
/// answers by where the stress falls, which the writing does not say, so a
/// word carrying it is looked for among the four the building does not reach
/// and left alone when it is not there.
fn built(held: &str) -> Option<Class> {
    if let Some((_, class)) = UNBUILT.iter().find(|(word, _)| *word == held) {
        return Some(*class);
    }

    match asking::built_from(held)? {
        (asking::Built::Negative, _) => Some(Class::Negative),
        (asking::Built::Indefinite, _) => Some(Class::Indefinite),
        (asking::Built::Prefixed, _) => None
    }
}

/// Reports whether a written word is a pronoun of any class.
#[must_use]
pub fn is_pronoun(written: &str) -> bool {
    !classes(written).is_empty()
}

/// Reports whether a written word belongs to a class.
#[must_use]
pub fn is_of(written: &str, class: Class) -> bool {
    classes(written).contains(&class)
}

/// Reports whether a written word hangs a clause on the noun before it.
///
/// The forms of `который` alone: the other asking pronouns relate too, but a
/// comma rule wants the one that agrees with a noun.
#[must_use]
pub fn relates(written: &str) -> bool {
    let held = written.to_lowercase();

    held.starts_with("котор") && held.chars().count() > 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grammars_give_pronouns_eight_classes_and_two_are_built_rather_than_listed() {
        assert_eq!(CLASSES.len(), 6);

        for held in ["никто", "кто-то"] {
            let named = classes(held);

            assert_eq!(named.len(), 1, "{held}");
            assert!(
                matches!(named[0], Class::Negative | Class::Indefinite),
                "{held} is in neither built class"
            );
        }
    }

    #[test]
    fn every_class_is_sorted_and_holds_no_word_twice() {
        for (class, words) in CLASSES {
            let mut held = words.to_vec();
            held.sort_unstable();
            held.dedup();

            assert_eq!(held.len(), words.len(), "{class:?} holds a word twice");
        }
    }

    #[test]
    fn a_word_of_one_class_names_that_class() {
        assert_eq!(classes("я"), std::vec![Class::Personal]);
        assert_eq!(classes("себя"), std::vec![Class::Reflexive]);
        assert_eq!(classes("который"), std::vec![Class::Asking]);
        assert_eq!(classes("никто"), std::vec![Class::Negative]);
        assert_eq!(classes("некто"), std::vec![Class::Indefinite]);
    }

    #[test]
    fn the_pronouns_the_writing_does_not_settle_are_named_outright() {
        for (held, named) in UNBUILT {
            assert!(classes(held).contains(named), "{held}");
        }
    }

    #[test]
    fn a_prefix_the_stress_decides_and_no_one_named_is_left_alone() {
        assert!(classes("некакой").is_empty());
        assert!(classes("нечей").is_empty());
    }

    #[test]
    fn a_word_built_out_of_no_asking_pronoun_is_in_no_built_class() {
        assert!(classes("нельзя").is_empty());
        assert!(classes("никель").is_empty());
    }

    #[test]
    fn every_oblique_personal_form_is_listed() {
        for held in ["им", "ею", "нею", "тобой", "тобою", "мною", "нами"]
        {
            assert!(is_of(held, Class::Personal), "{held}");
        }
    }

    #[test]
    fn a_run_together_of_a_preposition_and_a_pronoun_is_no_pronoun() {
        assert!(classes("занеё").is_empty());
        assert!(classes("сним").is_empty());
    }

    #[test]
    fn a_word_of_two_classes_names_both() {
        let held = classes("его");

        assert!(held.contains(&Class::Personal));
        assert!(held.contains(&Class::Possessive));
    }

    #[test]
    fn a_word_that_is_no_pronoun_names_no_class() {
        assert!(classes("стол").is_empty());
        assert!(!is_pronoun("читать"));
        assert!(!is_pronoun(""));
    }

    #[test]
    fn every_class_answers_to_being_asked_about() {
        for (class, words) in CLASSES {
            for held in *words {
                assert!(is_of(held, *class), "{held} is no {class:?}");
                assert!(is_pronoun(held));
            }
        }
    }

    #[test]
    fn the_reflexive_class_holds_one_word_in_its_forms() {
        assert!(
            Class::REFLEXIVE
                .iter()
                .all(|held| held.starts_with("себ") || held.starts_with("соб"))
        );
    }

    #[test]
    fn a_relative_is_a_form_of_one_word() {
        assert!(relates("который"));
        assert!(relates("которых"));
        assert!(relates("КОТОРОМУ"));
        assert!(!relates("этот"));
        assert!(!relates("котор"));
    }

    #[test]
    fn the_case_a_word_is_written_in_does_not_matter() {
        assert_eq!(classes("Я"), std::vec![Class::Personal]);
        assert!(is_of("ВЕСЬ", Class::Definitive));
    }
}
