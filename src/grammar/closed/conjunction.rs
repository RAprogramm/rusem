// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The conjunctions, in two classes that behave differently.
//!
//! A **coordinating** conjunction joins equals: two nouns, two clauses of the
//! same rank. What stands on either side of it agrees in case and could stand
//! alone.
//!
//! A **subordinating** conjunction hangs one clause under another. What
//! follows it is not an equal and cannot stand alone: `он сказал, что придёт`
//! has a clause that is only there because `что` put it there.
//!
//! The difference is why they are two lists. A gate looking for a compound
//! subject asks the first; a gate looking for a clause boundary asks the
//! second; and a comma before a subordinator is required by § 140 while a
//! comma before `и` is not.
//!
//! Both classes carry a second division, by what the joining or the hanging
//! means: [`coordination`] parts adding from opposing from choosing, and
//! [`subordination`] parts the eight relations a clause may hang by. A gate
//! written for one sense asks there rather than reading the whole class.

pub mod coordination;
pub mod subordination;

/// The conjunctions that join equals.
///
/// `да` is here in its joining sense — `хлеб да соль` — and in its opposing
/// one, `мал да удал`. Both are coordinating, so the list does not part them.
pub const COORDINATING: &[&str] = &[
    "а",
    "да",
    "же",
    "зато",
    "и",
    "либо",
    "ни",
    "но",
    "однако",
    "или",
    "притом",
    "причём",
    "также",
    "тоже"
];

/// The conjunctions that hang a clause under another.
pub const SUBORDINATING: &[&str] = &[
    "будто",
    "буде",
    "дабы",
    "едва",
    "ежели",
    "если",
    "ибо",
    "кабы",
    "как",
    "когда",
    "коли",
    "коль",
    "либо",
    "лишь",
    "нежели",
    "пока",
    "покуда",
    "поскольку",
    "пускай",
    "пусть",
    "раз",
    "словно",
    "точно",
    "хоть",
    "хотя",
    "чем",
    "что",
    "чтоб",
    "чтобы",
    "чуть"
];

/// The conjunctions that open a clause and nothing else.
///
/// A subset of the subordinating ones, and the one a comma rule wants: `и` may
/// open a clause too, but a comma before it turns on other things. These
/// always take one.
pub const CLAUSE_OPENING: &[&str] = &[
    "будто",
    "дабы",
    "ежели",
    "если",
    "ибо",
    "когда",
    "нежели",
    "поскольку",
    "словно",
    "хотя",
    "чем",
    "что",
    "чтоб",
    "чтобы"
];

/// The subordinating conjunctions written in more than one word.
///
/// They matter to a comma rule because the comma goes before the whole of
/// one: `она изменилась, потому что устала` sets the comma before `потому`
/// and never between `потому` and `что`. A rule that reads `что` alone would
/// ask for a second comma that Russian does not take.
pub const COMPOUND: &[&str] = &[
    "благодаря тому что",
    "в связи с тем что",
    "в силу того что",
    "в то время как",
    "ввиду того что",
    "вследствие того что",
    "для того чтобы",
    "до того как",
    "затем чтобы",
    "как будто",
    "как только",
    "лишь только",
    "несмотря на то что",
    "оттого что",
    "перед тем как",
    "по мере того как",
    "после того как",
    "потому что",
    "прежде чем",
    "с тем чтобы",
    "с тех пор как",
    "так как",
    "так что",
    "тем более что",
    "тогда как"
];

/// Reports whether a word closes a compound conjunction begun by the one
/// before it.
///
/// `потому что` answers true for `что` after `потому`, and false for `что`
/// after anything else.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::conjunction::closes_a_compound;
///
/// assert!(closes_a_compound("потому", "что"));
/// assert!(closes_a_compound("так", "как"));
/// assert!(!closes_a_compound("думаю", "что"));
/// ```
#[must_use]
pub fn closes_a_compound(before: &str, written: &str) -> bool {
    let tail = format!("{} {}", before.to_lowercase(), written.to_lowercase());

    COMPOUND
        .iter()
        .any(|held| *held == tail || held.ends_with(&format!(" {tail}")))
}

/// Reports whether a written word can open a subordinate clause.
///
/// Three sets do it and a clause-cutting gate wants all three: the
/// subordinating conjunctions, the relative pronouns, and the asking adverbs
/// of [`crate::grammar::closed::adverb`] — `дом, где мы жили` hangs a clause
/// on an adverb as surely as `что` does.
/// Coordinating conjunctions are not among them — `и` joins equals and opens
/// nothing under anything.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::conjunction::opens_a_subordinate_clause;
///
/// assert!(opens_a_subordinate_clause("что"));
/// assert!(opens_a_subordinate_clause("который"));
/// assert!(opens_a_subordinate_clause("где"));
/// assert!(!opens_a_subordinate_clause("и"));
/// ```
#[must_use]
pub fn opens_a_subordinate_clause(written: &str) -> bool {
    let held = written.to_lowercase();

    subordinates(&held)
        || crate::grammar::closed::adverb::relates(&held)
        || crate::grammar::closed::pronoun::relates(&held)
        || crate::grammar::closed::pronoun::ASKING.contains(&held.as_str())
}

/// Reports whether a written word joins equals.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::conjunction::coordinates;
///
/// assert!(coordinates("и"));
/// assert!(coordinates("И"));
/// assert!(!coordinates("чтобы"));
/// ```
#[must_use]
pub fn coordinates(written: &str) -> bool {
    COORDINATING.contains(&written.to_lowercase().as_str())
}

/// Reports whether a written word hangs a clause under another.
#[must_use]
pub fn subordinates(written: &str) -> bool {
    SUBORDINATING.contains(&written.to_lowercase().as_str())
}

/// Reports whether a written word always opens a clause.
#[must_use]
pub fn opens_a_clause(written: &str) -> bool {
    CLAUSE_OPENING.contains(&written.to_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_coordinating_conjunction_is_named_as_one() {
        assert!(coordinates("и"));
        assert!(coordinates("но"));
        assert!(coordinates("либо"));
        assert!(!coordinates("стол"));
    }

    #[test]
    fn a_subordinating_conjunction_is_named_as_one() {
        assert!(subordinates("чтобы"));
        assert!(subordinates("если"));
        assert!(!subordinates("но"));
    }

    #[test]
    fn a_word_that_is_both_is_in_both_lists() {
        assert!(coordinates("либо"));
        assert!(subordinates("либо"));
    }

    #[test]
    fn every_clause_opener_is_a_subordinator() {
        for held in CLAUSE_OPENING {
            assert!(
                subordinates(held),
                "{held} opens a clause and subordinates nothing"
            );
        }
    }

    #[test]
    fn a_clause_opens_with_a_conjunction_a_pronoun_or_an_adverb() {
        assert!(opens_a_subordinate_clause("что"));
        assert!(opens_a_subordinate_clause("чтобы"));
        assert!(opens_a_subordinate_clause("который"));
        assert!(opens_a_subordinate_clause("которого"));
        assert!(opens_a_subordinate_clause("где"));
        assert!(opens_a_subordinate_clause("КУДА"));
    }

    #[test]
    fn a_coordinating_conjunction_opens_nothing_under_anything() {
        for held in ["и", "но", "зато", "однако"] {
            assert!(!opens_a_subordinate_clause(held), "{held}");
        }
    }

    #[test]
    fn a_word_that_joins_nothing_opens_nothing() {
        assert!(!opens_a_subordinate_clause("стол"));
        assert!(!opens_a_subordinate_clause(""));
    }

    #[test]
    fn the_case_a_word_is_written_in_does_not_matter() {
        assert!(coordinates("И"));
        assert!(subordinates("Если"));
        assert!(opens_a_clause("ЧТО"));
    }
}
