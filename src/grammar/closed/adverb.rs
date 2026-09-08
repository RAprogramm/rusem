// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The pronominal adverbs, which point the way a pronoun does.
//!
//! `там` is to `тот` what an adverb is to a noun: it names nothing and only
//! points, and what it points at is a place rather than a thing. The grammars
//! divide them by the same classes as the pronouns for that reason, and this
//! module keeps the division so a caller may ask one question of both.
//!
//! They are here rather than among the adverbs at large because the class is
//! closed. An adverb is made from any adjective — `быстро`, `красиво`, and
//! tomorrow's word too — but nothing new becomes a pointing one.
//!
//! The negative and the indefinite classes hold no list. Every word in them is
//! an asking adverb with a particle on it, and [`asking`] builds them.
//!
//! What they add over the pronouns is that they hang a clause without being
//! pronouns: `дом, где мы жили` opens a subordinate clause with an adverb.
//! That is why [`crate::grammar::closed::conjunction`] asks this module.

/// The adverbs that ask, and the same ones that hang a clause.
///
/// Interrogative and relative hold one list, as they do among the pronouns:
/// `где ты был?` asks and `дом, где мы жили` relates, and only the sentence
/// tells which.
///
/// Stated in [`asking`], beside the pronouns that ask: the negative and the
/// indefinite adverbs are built out of both, and building them in two places
/// would let the two disagree.
pub use super::asking::{self, Asking};

/// The negative and indefinite adverbs the building does not reach.
///
/// Everything else in the two classes is an asking adverb with `ни`, `не`,
/// `кое-` or a particle on it, and [`asking::built_from`] reaches it. These
/// are not. `нипочём`, `нимало` and `ничуть` are named by § 90 п. 2 and are
/// built on words that do not ask. The four with `не` are named because
/// modern Russian keeps only their negative reading: the indefinite ones the
/// older language had — `негде` for somewhere — are gone, so the writing is
/// enough.
///
/// `некогда` is deliberately absent. Its indefinite reading is fully alive —
/// `некогда популярный`, once popular — and is spelled and stressed exactly
/// as the negative `мне некогда`, so nothing on the word settles the class
/// and [`class`] answers nothing for it.
const UNBUILT: &[(&str, Class)] = &[
    ("негде", Class::Negative),
    ("незачем", Class::Negative),
    ("некуда", Class::Negative),
    ("неоткуда", Class::Negative),
    ("нимало", Class::Negative),
    ("нипочём", Class::Negative),
    ("ничуть", Class::Negative)
];

/// Which class a pronominal adverb belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Class {
    /// `там`, `тогда`: the place or time already named.
    Demonstrative,
    /// `где`, `когда`: asking, or hanging a clause.
    Asking,
    /// `всюду`, `всегда`: every one there is.
    Definitive,
    /// `нигде`, `никогда`: none there is.
    Negative,
    /// `где-то`, `когда-нибудь`: one, unsaid which.
    Indefinite,
    /// `по-моему`: by whose reckoning.
    Possessive
}

impl Class {
    /// The adverbs that point at what has already been said.
    pub const DEMONSTRATIVE: &[&str] = &[
        "везде",
        "затем",
        "здесь",
        "настолько",
        "оттого",
        "оттуда",
        "отсюда",
        "поэтому",
        "потому",
        "сюда",
        "так",
        "там",
        "тогда",
        "тут",
        "туда"
    ];

    /// The adverbs that take in every place, time or way there is.
    pub const DEFINITIVE: &[&str] = &[
        "всегда",
        "всюду",
        "всячески",
        "отовсюду",
        "по-всякому",
        "по-другому",
        "по-иному",
        "повсюду"
    ];

    /// The adverbs that say by whose reckoning.
    ///
    /// Some grammars give them a class of their own and others fold them into
    /// the possessive pronouns. They are listed apart because a checker
    /// meets them as adverbs — `по-моему` stands where `так` stands.
    pub const POSSESSIVE: &[&str] = &[
        "по-вашему",
        "по-моему",
        "по-нашему",
        "по-своему",
        "по-твоему"
    ];
}

/// Every class, with the words in it.
const CLASSES: &[(Class, &[&str])] = &[
    (Class::Demonstrative, Class::DEMONSTRATIVE),
    (Class::Asking, Asking::ADVERBS),
    (Class::Definitive, Class::DEFINITIVE),
    (Class::Possessive, Class::POSSESSIVE)
];

/// The class a written adverb belongs to, or nothing when it points at nothing.
///
/// One class at most, unlike the pronouns: no spelling here is shared between
/// two classes.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::adverb::{Class, class};
///
/// assert_eq!(class("там"), Some(Class::Demonstrative));
/// assert_eq!(class("где"), Some(Class::Asking));
/// assert_eq!(class("никогда"), Some(Class::Negative));
/// assert_eq!(class("быстро"), None);
/// ```
#[must_use]
pub fn class(written: &str) -> Option<Class> {
    let held = written.to_lowercase();

    CLASSES
        .iter()
        .find(|(_, words)| words.contains(&held.as_str()))
        .map(|(class, _)| *class)
        .or_else(|| built(&held))
}

/// The class an adverb built out of an asking one belongs to.
///
/// The words the writing does not settle are looked for first, so that the
/// building answers only where it has an answer.
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

/// Reports whether a written word is a pronominal adverb of any class.
#[must_use]
pub fn is_pronominal(written: &str) -> bool {
    class(written).is_some()
}

/// Reports whether a written adverb can hang a clause on what stands before it.
///
/// The asking ones and none other: `дом, где мы жили` relates, `дом, там мы
/// жили` is not Russian.
#[must_use]
pub fn relates(written: &str) -> bool {
    class(written) == Some(Class::Asking)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grammars_give_six_classes_and_two_are_built_rather_than_listed() {
        assert_eq!(CLASSES.len(), 4);

        assert_eq!(class("нигде"), Some(Class::Negative));
        assert_eq!(class("где-то"), Some(Class::Indefinite));
        assert_eq!(class("кое-где"), Some(Class::Indefinite));
    }

    #[test]
    fn the_adverbs_the_writing_does_not_settle_are_named_outright() {
        for (held, named) in UNBUILT {
            assert_eq!(class(held), Some(*named), "{held}");
        }
    }

    #[test]
    fn a_prefix_the_stress_decides_and_no_one_named_is_left_alone() {
        assert_eq!(class("некак"), None);
        assert_eq!(class("непочему"), None);
    }

    #[test]
    fn the_word_of_two_living_readings_is_left_undecided() {
        assert_eq!(
            class("некогда"),
            None,
            "некогда denies and means once with one spelling and one stress"
        );
    }

    #[test]
    fn no_class_holds_a_word_twice_and_no_word_is_of_two_classes() {
        let mut every: Vec<&str> = CLASSES
            .iter()
            .flat_map(|(_, words)| *words)
            .copied()
            .collect();
        let counted = every.len();
        every.sort_unstable();
        every.dedup();

        assert_eq!(every.len(), counted, "a pronominal adverb is listed twice");
    }

    #[test]
    fn every_listed_adverb_names_its_class() {
        for (held, class) in CLASSES
            .iter()
            .flat_map(|(class, words)| words.iter().map(move |held| (*held, *class)))
        {
            assert_eq!(super::class(held), Some(class), "{held}");
            assert!(is_pronominal(held));
        }
    }

    #[test]
    fn an_adverb_that_points_at_nothing_names_no_class() {
        assert_eq!(class("быстро"), None);
        assert!(!is_pronominal("красиво"));
        assert!(!is_pronominal(""));
    }

    #[test]
    fn only_the_asking_ones_hang_a_clause() {
        assert!(relates("где"));
        assert!(relates("КУДА"));
        assert!(!relates("там"));
        assert!(!relates("никогда"));
    }

    #[test]
    fn the_case_a_word_is_written_in_does_not_matter() {
        assert_eq!(class("ТАМ"), Some(Class::Demonstrative));
    }
}
