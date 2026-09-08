// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The words that ask, and everything Russian builds out of them.
//!
//! Ten words ask: `кто`, `что`, `какой`, `который`, `чей`, `каков`, `где`,
//! `когда`, `куда`, `откуда`, and a few more of the same kind. They are the
//! smallest truly closed class in the language and the oldest — nothing has
//! joined them and nothing will.
//!
//! Every negative and every indefinite word is one of them with something put
//! on the front or the back:
//!
//! | Put on | Gives | Example |
//! | --- | --- | --- |
//! | `ни` in front | there is none at all | `никто`, `нигде` |
//! | `не` in front | one of the two, by where the stress falls | `некто`, `негде` |
//! | `кое` in front | one, and the speaker could say which | `кое-кто` |
//! | `-то` behind | one, unsaid which | `кто-то` |
//! | `-либо` behind | any one at all | `кто-либо` |
//! | `-нибудь` behind | some one or other | `кто-нибудь` |
//!
//! Ten asking words and six ways of building make sixty words, and every one
//! of them is reached by this module without any of them being written down.
//! A list of them would be a multiplication table copied out by hand: longer
//! than the rule that makes it, and able to fall out of step.
//!
//! The Свод states the same fact as spelling — § 88 п. 5 and § 90 п. 1–2 join
//! the prefixes, § 86 п. 3 hyphenates the particles — and
//! [`crate::rules::svod`] cites it there. Here it is what it is underneath:
//! how the words are made.

/// The words that ask.
///
/// Holds the two lists every negative and indefinite word is built from.
/// [`Built`] names what was put on one of them, and [`built_from`] answers
/// what a written word was built from.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::asking::Asking;
///
/// assert!(Asking::PRONOUNS.contains(&"кто"));
/// assert!(Asking::ADVERBS.contains(&"где"));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Asking;

impl Asking {
    /// The pronouns that ask.
    ///
    /// `сколько` is among them: it asks after a number and stands where a
    /// numeral stands, which is what the grammars call a pronominal numeral.
    pub const PRONOUNS: &[&str] = &["каков", "какой", "который", "кто", "сколько", "что", "чей"];

    /// The adverbs that ask.
    pub const ADVERBS: &[&str] = &[
        "где",
        "зачем",
        "как",
        "когда",
        "куда",
        "насколько",
        "откуда",
        "отчего",
        "почему"
    ];
}

/// What is put on an asking word, and what it makes of it.
///
/// Two of the three answer a class outright. The third does not, and saying so
/// is the point of it: `не` builds a negative when it is stressed — `нЕкого
/// спросить`, there is nobody to ask — and an indefinite when it is not —
/// `нектО пришёл`, somebody came. The spelling is the same both times. A
/// reader who has only the letters has only [`Built::Prefixed`], and must
/// reach for the stress to go further.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Built {
    /// `ни` in front: `никто`, `нигде`. There is none at all.
    Negative,
    /// `кое` in front or a particle behind: `кое-кто`, `кто-то`. There is one,
    /// and which is left unsaid.
    Indefinite,
    /// `не` in front: `некто`, `негде`. Which of the two it is depends on
    /// where the stress falls, and the writing does not say.
    Prefixed
}

/// The particles written in front, longest first.
const LEADING: &[(&str, Built)] = &[
    ("кое", Built::Indefinite),
    ("кой", Built::Indefinite),
    ("не", Built::Prefixed),
    ("ни", Built::Negative)
];

/// The particles written behind, longest first.
const TRAILING: &[&str] = &["нибудь", "либо", "то"];

/// Reports whether a written word is one of the ones that ask.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::asking::asks;
///
/// assert!(asks("кто"));
/// assert!(asks("где"));
/// assert!(!asks("стол"));
/// ```
#[must_use]
pub fn asks(written: &str) -> bool {
    asked(written).is_some()
}

/// The asking word a written one is, in the form the lists hold it.
///
/// Answers a word out of [`Asking::PRONOUNS`] or [`Asking::ADVERBS`] rather
/// than a piece of what was passed in, so what comes back is always the
/// dictionary form and always outlives the call.
fn asked(written: &str) -> Option<&'static str> {
    let held = written.to_lowercase();

    Asking::PRONOUNS
        .iter()
        .chain(Asking::ADVERBS)
        .find(|word| **word == held)
        .copied()
}

/// What a written word was built from, when it was built out of an asking one.
///
/// Answers what it became and the asking word underneath. `никогда` gives
/// [`Built::Negative`] and `когда`; `кто-нибудь` gives [`Built::Indefinite`]
/// and `кто`.
///
/// A run holding a space is answered by nothing: a preposition standing
/// between the particle and the word parts them — `не у кого`, `кое в чём` —
/// and what is parted was not built.
///
/// Only the dictionary forms of the asking words are reached: `некого` and
/// `никем` are built on declined forms and answer nothing here, because a
/// declined pronoun is read through its lemma, and
/// [`crate::grammar::closed::pronoun`] names the few like `некого` outright.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::asking::{Built, built_from};
///
/// assert_eq!(built_from("никогда"), Some((Built::Negative, "когда")));
/// assert_eq!(built_from("кто-нибудь"), Some((Built::Indefinite, "кто")));
/// assert_eq!(built_from("кое-что"), Some((Built::Indefinite, "что")));
/// assert_eq!(built_from("некто"), Some((Built::Prefixed, "кто")));
/// assert_eq!(
///     built_from("НИКОГДА"),
///     Some((Built::Negative, "когда")),
///     "always the small form"
/// );
/// assert_eq!(built_from("стол"), None);
/// assert_eq!(built_from("не у кого"), None);
/// ```
#[must_use]
pub fn built_from(written: &str) -> Option<(Built, &'static str)> {
    if written.contains(char::is_whitespace) {
        return None;
    }

    behind(written).or_else(|| ahead(written))
}

/// What a word with a particle behind it was built from.
fn behind(written: &str) -> Option<(Built, &'static str)> {
    let (under, particle) = written.rsplit_once('-')?;

    TRAILING
        .contains(&particle.to_lowercase().as_str())
        .then(|| asked(under))
        .flatten()
        .map(|asking| (Built::Indefinite, asking))
}

/// What a word with a particle in front of it was built from.
fn ahead(written: &str) -> Option<(Built, &'static str)> {
    let held = written.to_lowercase();

    LEADING.iter().find_map(|(particle, built)| {
        let rest = held.strip_prefix(particle)?;
        let under = rest.strip_prefix('-').unwrap_or(rest);

        asked(under).map(|asking| (*built, asking))
    })
}

/// Reports whether a written word denies every one there is.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::asking::denies;
///
/// assert!(denies("никто"));
/// assert!(denies("нигде"));
/// assert!(!denies("некто"));
/// ```
#[must_use]
pub fn denies(written: &str) -> bool {
    matches!(built_from(written), Some((Built::Negative, _)))
}

/// Reports whether a written word leaves unsaid which one it means.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::asking::leaves_unsaid;
///
/// assert!(leaves_unsaid("кто-то"));
/// assert!(leaves_unsaid("кое-где"));
/// assert!(!leaves_unsaid("некто"), "the stress decides that one");
/// assert!(!leaves_unsaid("никто"));
/// ```
#[must_use]
pub fn leaves_unsaid(written: &str) -> bool {
    matches!(built_from(written), Some((Built::Indefinite, _)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_asking_word_asks_and_an_ordinary_one_does_not() {
        assert!(asks("кто"));
        assert!(asks("ГДЕ"));
        assert!(!asks("стол"));
        assert!(!asks(""));
    }

    #[test]
    fn every_asking_word_builds_a_negative_and_five_indefinites() {
        for held in Asking::PRONOUNS.iter().chain(Asking::ADVERBS) {
            assert!(denies(&std::format!("ни{held}")), "ни{held}");
            assert_eq!(
                built_from(&std::format!("не{held}")).map(|(built, _)| built),
                Some(Built::Prefixed),
                "не{held}"
            );
            assert!(leaves_unsaid(&std::format!("кое-{held}")), "кое-{held}");
            for particle in TRAILING {
                let word = std::format!("{held}-{particle}");

                assert!(leaves_unsaid(&word), "{word}");
            }
        }
    }

    #[test]
    fn every_word_the_code_of_1956_names_is_built_by_the_rule() {
        for held in [
            "никто",
            "ничто",
            "никакой",
            "ничей",
            "никогда",
            "нигде",
            "никуда",
            "никак"
        ] {
            assert!(denies(held), "§ 90 names {held}");
        }
        for held in ["некто", "нечто", "некогда", "негде", "некуда", "неоткуда"]
        {
            assert_eq!(
                built_from(held).map(|(built, _)| built),
                Some(Built::Prefixed),
                "§ 88 names {held}"
            );
        }
        for held in [
            "кое-что",
            "кое-кто",
            "кое-какой",
            "кой-куда",
            "кто-нибудь",
            "кто-либо"
        ] {
            assert!(leaves_unsaid(held), "§ 86 names {held}");
        }
    }

    #[test]
    fn what_a_word_was_built_from_is_named() {
        assert_eq!(built_from("никогда"), Some((Built::Negative, "когда")));
        assert_eq!(built_from("кто-нибудь"), Some((Built::Indefinite, "кто")));
        assert_eq!(built_from("кое-что"), Some((Built::Indefinite, "что")));
        assert_eq!(built_from("ниоткуда"), Some((Built::Negative, "откуда")));
    }

    #[test]
    fn a_particle_on_a_word_that_does_not_ask_builds_nothing() {
        assert_eq!(built_from("нельзя"), None);
        assert_eq!(built_from("никель"), None);
        assert_eq!(built_from("несмотря"), None);
        assert_eq!(built_from("стол-то"), None);
        assert_eq!(built_from("кое-стол"), None);
    }

    #[test]
    fn a_word_that_is_parted_by_a_preposition_was_not_built() {
        assert_eq!(built_from("не у кого"), None);
        assert_eq!(built_from("ни с каким"), None);
        assert_eq!(built_from("кое в чём"), None);
    }

    #[test]
    fn a_particle_written_behind_a_word_of_no_use_is_read_no_further() {
        assert_eq!(built_from("кое-кто-то"), None);
        assert_eq!(built_from("-то"), None);
        assert_eq!(built_from("кто-"), None);
    }

    #[test]
    fn a_word_with_nothing_on_it_was_built_out_of_nothing() {
        assert_eq!(built_from("кто"), None);
        assert_eq!(built_from("стол"), None);
        assert_eq!(built_from(""), None);
        assert_eq!(built_from("по-русски"), None);
    }

    #[test]
    fn nothing_both_denies_and_leaves_unsaid() {
        for held in ["никто", "некто", "нигде", "кто-то", "кое-где", "стол"]
        {
            assert!(!(denies(held) && leaves_unsaid(held)), "{held}");
        }
    }

    #[test]
    fn the_prefix_the_stress_decides_is_left_undecided() {
        for held in ["некто", "нечто", "негде", "некуда"] {
            assert_eq!(
                built_from(held).map(|(built, _)| built),
                Some(Built::Prefixed),
                "{held} is built and left undecided"
            );
            assert!(
                !denies(held),
                "{held} is not called a denial by the letters alone"
            );
            assert!(
                !leaves_unsaid(held),
                "{held} is not called indefinite by them either"
            );
        }
    }

    #[test]
    fn a_particle_on_a_declined_form_is_not_reached() {
        assert_eq!(built_from("некого"), None);
        assert_eq!(built_from("никем"), None);
        assert_eq!(built_from("ничему"), None);
    }

    #[test]
    fn the_case_a_word_is_written_in_does_not_matter() {
        assert!(denies("НИКОГДА"));
        assert!(leaves_unsaid("Кто-То"));
        assert_eq!(
            built_from("НЕКТО").map(|(built, _)| built),
            Some(Built::Prefixed)
        );
    }
}
