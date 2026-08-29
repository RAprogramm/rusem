// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The prepositions, and the cases each of them takes.
//!
//! A preposition governs: it names the case of the noun after it, and a noun
//! in another case is wrong however well the sentence otherwise reads. `в
//! доме` and `в дом` are both right and mean different things; `в дома` is
//! neither.
//!
//! Several take more than one case and mean something different in each. `в`
//! with the accusative is motion into and with the prepositional is rest
//! inside; `с` takes three. So the table answers with a set, and a gate
//! refuses only what is outside it.
//!
//! Where each came from is stated in [`origin`]: a derived preposition can be
//! confused with the word it was made from, and a primitive one cannot.

use crate::grammar::Case;

pub mod origin;

/// The prepositions.
pub const PREPOSITIONS: &[&str] = &[
    "без",
    "безо",
    "близ",
    "в",
    "вблизи",
    "вдоль",
    "вместо",
    "вне",
    "внутри",
    "во",
    "возле",
    "вокруг",
    "впереди",
    "для",
    "до",
    "за",
    "из",
    "из-за",
    "из-под",
    "изо",
    "к",
    "кроме",
    "ко",
    "между",
    "мимо",
    "на",
    "над",
    "надо",
    "напротив",
    "о",
    "об",
    "обо",
    "около",
    "от",
    "ото",
    "перед",
    "передо",
    "по",
    "под",
    "подо",
    "позади",
    "помимо",
    "после",
    "посреди",
    "при",
    "про",
    "против",
    "ради",
    "с",
    "сверх",
    "сзади",
    "сквозь",
    "со",
    "среди",
    "у",
    "через"
];

/// The cases each preposition takes, in the order the prepositions are listed
/// above.
///
/// A preposition governing more than one case is here once with all of them:
/// `в` takes the accusative for motion and the prepositional for rest, and a
/// gate that refused either would refuse half the language.
const GOVERNMENT: &[(&str, &[Case])] = &[
    ("без", &[Case::Genitive]),
    ("безо", &[Case::Genitive]),
    ("близ", &[Case::Genitive]),
    ("в", &[Case::Accusative, Case::Prepositional]),
    ("вблизи", &[Case::Genitive]),
    ("вдоль", &[Case::Genitive]),
    ("вместо", &[Case::Genitive]),
    ("вне", &[Case::Genitive]),
    ("внутри", &[Case::Genitive]),
    ("во", &[Case::Accusative, Case::Prepositional]),
    ("возле", &[Case::Genitive]),
    ("вокруг", &[Case::Genitive]),
    ("впереди", &[Case::Genitive]),
    ("для", &[Case::Genitive]),
    ("до", &[Case::Genitive]),
    ("за", &[Case::Accusative, Case::Instrumental]),
    ("из", &[Case::Genitive]),
    ("из-за", &[Case::Genitive]),
    ("из-под", &[Case::Genitive]),
    ("изо", &[Case::Genitive]),
    ("к", &[Case::Dative]),
    ("ко", &[Case::Dative]),
    ("кроме", &[Case::Genitive]),
    ("между", &[Case::Instrumental, Case::Genitive]),
    ("мимо", &[Case::Genitive]),
    ("на", &[Case::Accusative, Case::Prepositional]),
    ("над", &[Case::Instrumental]),
    ("надо", &[Case::Instrumental]),
    ("напротив", &[Case::Genitive]),
    ("о", &[Case::Accusative, Case::Prepositional]),
    ("об", &[Case::Accusative, Case::Prepositional]),
    ("обо", &[Case::Accusative, Case::Prepositional]),
    ("около", &[Case::Genitive]),
    ("от", &[Case::Genitive]),
    ("ото", &[Case::Genitive]),
    ("перед", &[Case::Instrumental]),
    ("передо", &[Case::Instrumental]),
    ("по", &[Case::Dative, Case::Accusative, Case::Prepositional]),
    ("под", &[Case::Accusative, Case::Instrumental]),
    ("подо", &[Case::Accusative, Case::Instrumental]),
    ("позади", &[Case::Genitive]),
    ("помимо", &[Case::Genitive]),
    ("после", &[Case::Genitive]),
    ("посреди", &[Case::Genitive]),
    ("при", &[Case::Prepositional]),
    ("про", &[Case::Accusative]),
    ("против", &[Case::Genitive]),
    ("ради", &[Case::Genitive]),
    ("с", &[Case::Genitive, Case::Accusative, Case::Instrumental]),
    ("сверх", &[Case::Genitive]),
    ("сзади", &[Case::Genitive]),
    ("сквозь", &[Case::Accusative]),
    (
        "со",
        &[Case::Genitive, Case::Accusative, Case::Instrumental]
    ),
    ("среди", &[Case::Genitive]),
    ("у", &[Case::Genitive]),
    ("через", &[Case::Accusative])
];

/// The prepositions written as more than one word.
///
/// `в течение дня`, `несмотря на дождь`. They govern the same way the simple
/// ones do, but a tokenizer sees two or three words where the grammar sees
/// one, so they are matched against a run of words rather than against a word.
pub const COMPOUND: &[(&str, Case)] = &[
    ("в отличие от", Case::Genitive),
    ("в продолжение", Case::Genitive),
    ("в связи с", Case::Instrumental),
    ("в силу", Case::Genitive),
    ("в течение", Case::Genitive),
    ("в ходе", Case::Genitive),
    ("во время", Case::Genitive),
    ("вследствие", Case::Genitive),
    ("за счёт", Case::Genitive),
    ("на протяжении", Case::Genitive),
    ("наряду с", Case::Instrumental),
    ("несмотря на", Case::Accusative),
    ("по мере", Case::Genitive),
    ("по поводу", Case::Genitive),
    ("по причине", Case::Genitive),
    ("согласно", Case::Dative),
    ("благодаря", Case::Dative),
    ("вопреки", Case::Dative),
    ("навстречу", Case::Dative),
    ("наперекор", Case::Dative)
];

/// The case a compound preposition takes, when the words open with one.
///
/// The longest match wins: `в связи с` is one preposition and not `в` followed
/// by two words.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Case, closed::preposition::compound};
///
/// assert_eq!(
///     compound("в течение дня"),
///     Some(("в течение", Case::Genitive))
/// );
/// assert_eq!(
///     compound("согласно приказу"),
///     Some(("согласно", Case::Dative))
/// );
/// assert_eq!(compound("стол стоит"), None);
/// ```
#[must_use]
pub fn compound(written: &str) -> Option<(&'static str, Case)> {
    let held = written.to_lowercase();

    COMPOUND
        .iter()
        .filter(|(preposition, _)| held.starts_with(preposition))
        .max_by_key(|(preposition, _)| preposition.chars().count())
        .map(|(preposition, case)| (*preposition, *case))
}

/// Reports whether a written word is a preposition.
#[must_use]
pub fn is_preposition(written: &str) -> bool {
    PREPOSITIONS.contains(&written.to_lowercase().as_str())
}

/// The cases a preposition takes, empty when the word is no preposition.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Case, closed::preposition::governs};
///
/// assert_eq!(governs("к"), [Case::Dative]);
/// assert!(governs("в").contains(&Case::Prepositional));
/// assert!(governs("стол").is_empty());
/// ```
#[must_use]
pub fn governs(written: &str) -> &'static [Case] {
    let held = written.to_lowercase();

    GOVERNMENT
        .iter()
        .find(|(preposition, _)| *preposition == held)
        .map_or(&[], |(_, cases)| *cases)
}

/// Reports whether a preposition admits a noun in this case.
///
/// A word that is no preposition admits nothing, so a caller must ask whether
/// it is one before reading anything into a refusal.
#[must_use]
pub fn admits(written: &str, case: Case) -> bool {
    governs(written)
        .iter()
        .any(|held| held.merged() == case.merged())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preposition_has_a_case_and_every_governed_word_is_a_preposition() {
        for held in PREPOSITIONS {
            assert!(!governs(held).is_empty(), "{held} governs nothing");
        }
        for (held, _) in GOVERNMENT {
            assert!(is_preposition(held), "{held} governs and is no preposition");
        }
    }

    #[test]
    fn a_preposition_of_one_case_takes_that_one() {
        assert_eq!(governs("к"), [Case::Dative]);
        assert_eq!(governs("без"), [Case::Genitive]);
        assert_eq!(governs("при"), [Case::Prepositional]);
    }

    #[test]
    fn a_preposition_of_several_cases_takes_them_all() {
        assert!(governs("в").contains(&Case::Accusative));
        assert!(governs("в").contains(&Case::Prepositional));
        assert_eq!(governs("с").len(), 3);
    }

    #[test]
    fn a_word_that_is_no_preposition_governs_nothing() {
        assert!(governs("стол").is_empty());
        assert!(!admits("стол", Case::Genitive));
    }

    #[test]
    fn a_case_outside_what_a_preposition_takes_is_refused() {
        assert!(admits("к", Case::Dative));
        assert!(!admits("к", Case::Genitive));
        assert!(admits("В", Case::Prepositional));
    }

    #[test]
    fn a_compound_preposition_names_its_case() {
        assert_eq!(
            compound("в течение дня"),
            Some(("в течение", Case::Genitive))
        );
        assert_eq!(
            compound("несмотря на дождь"),
            Some(("несмотря на", Case::Accusative))
        );
        assert_eq!(
            compound("Согласно приказу"),
            Some(("согласно", Case::Dative))
        );
    }

    #[test]
    fn the_longest_compound_wins() {
        let (held, _) = compound("в связи с этим").expect("a compound");

        assert_eq!(held, "в связи с");
    }

    #[test]
    fn a_run_of_words_that_is_no_preposition_names_nothing() {
        assert_eq!(compound("стол стоит"), None);
        assert_eq!(compound(""), None);
    }

    #[test]
    fn every_compound_is_written_small_and_governs_one_case() {
        for (held, _) in COMPOUND {
            assert_eq!(*held, held.to_lowercase(), "{held} is not written small");
            assert!(compound(held).is_some(), "{held} names no case");
        }
    }

    #[test]
    fn a_partitive_is_admitted_where_the_genitive_is() {
        assert!(admits("без", Case::Partitive));
    }
}
