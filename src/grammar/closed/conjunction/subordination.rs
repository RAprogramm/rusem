// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! What relation a subordinating conjunction hangs a clause by.
//!
//! [`super::SUBORDINATING`] says a word hangs a clause. What the clause then
//! says about the main one is a different question, and the answer decides
//! what may stand in it: a purpose clause takes an infinitive or a past form
//! and never a present one — `чтобы он пришёл`, never `чтобы он придёт` — and
//! a conditional one admits either.
//!
//! The eight relations are those the grammars give. Several conjunctions hang
//! by more than one — `как` compares in `бел как снег` and states time in `как
//! стемнело` — so [`relations`] answers a list.

/// What the subordinate clause says about the main one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Relation {
    /// `что`, `чтобы`: what was said, thought or wanted.
    Explanatory,
    /// `когда`, `пока`, `едва`: when.
    Temporal,
    /// `ибо`, `поскольку`: why.
    Causal,
    /// `чтобы`, `дабы`: what for.
    Purpose,
    /// `если`, `коли`, `раз`: on what condition.
    Conditional,
    /// `хотя`, `пусть`: against what.
    Concessive,
    /// `будто`, `словно`, `точно`: like what.
    Comparative,
    /// `так что`: with what result.
    Consecutive
}

/// Every relation, with the conjunctions that hang by it.
///
/// A conjunction appears under every relation it hangs by, so the lists
/// overlap on purpose: `что` explains and `так что` follows from, `как`
/// compares and states time.
const RELATIONS: &[(Relation, &[&str])] = &[
    (
        Relation::Explanatory,
        &["будто", "как", "что", "чтоб", "чтобы"]
    ),
    (
        Relation::Temporal,
        &["едва", "как", "когда", "лишь", "пока", "покуда", "чуть"]
    ),
    (Relation::Causal, &["ибо", "поскольку", "раз"]),
    (Relation::Purpose, &["дабы", "чтоб", "чтобы"]),
    (
        Relation::Conditional,
        &["буде", "ежели", "если", "кабы", "коли", "коль", "раз"]
    ),
    (Relation::Concessive, &["пускай", "пусть", "хоть", "хотя"]),
    (
        Relation::Comparative,
        &["будто", "как", "нежели", "словно", "точно", "чем"]
    )
];

/// The relations a subordinating conjunction hangs a clause by.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::conjunction::subordination::{Relation, relations};
///
/// assert_eq!(relations("если"), vec![Relation::Conditional]);
/// assert_eq!(relations("дабы"), vec![Relation::Purpose]);
/// assert!(relations("как").contains(&Relation::Temporal));
/// assert!(relations("стол").is_empty());
/// ```
#[must_use]
pub fn relations(written: &str) -> Vec<Relation> {
    let held = written.to_lowercase();

    RELATIONS
        .iter()
        .filter(|(_, words)| words.contains(&held.as_str()))
        .map(|(relation, _)| *relation)
        .collect()
}

/// Reports whether a conjunction can hang a clause by a relation.
#[must_use]
pub fn hangs_by(written: &str, relation: Relation) -> bool {
    relations(written).contains(&relation)
}

/// Reports whether a conjunction opens a clause of what was wanted or aimed at.
///
/// A purpose clause takes no present tense: `чтобы он пришёл` and `чтобы
/// прийти` are Russian and `чтобы он придёт` is not. A gate on the tense in
/// such a clause asks this first.
#[must_use]
pub fn states_a_purpose(written: &str) -> bool {
    hangs_by(written, Relation::Purpose)
}

#[cfg(test)]
mod tests {
    use super::{super::SUBORDINATING, *};

    #[test]
    fn the_grammars_give_eight_relations_and_seven_are_carried_by_one_word() {
        assert_eq!(
            RELATIONS.len(),
            7,
            "the consecutive one is written in two words"
        );
    }

    #[test]
    fn every_listed_conjunction_subordinates() {
        for (relation, words) in RELATIONS {
            for held in *words {
                assert!(
                    SUBORDINATING.contains(held),
                    "{held} is under {relation:?} and subordinates nothing"
                );
            }
        }
    }

    #[test]
    fn every_relation_is_sorted_and_holds_no_word_twice() {
        for (relation, words) in RELATIONS {
            let mut held = words.to_vec();
            held.sort_unstable();
            held.dedup();

            assert_eq!(held.len(), words.len(), "{relation:?} holds a word twice");
        }
    }

    #[test]
    fn a_conjunction_of_one_relation_names_it() {
        assert_eq!(relations("если"), std::vec![Relation::Conditional]);
        assert_eq!(relations("дабы"), std::vec![Relation::Purpose]);
        assert_eq!(relations("ибо"), std::vec![Relation::Causal]);
        assert_eq!(relations("ХОТЯ"), std::vec![Relation::Concessive]);
    }

    #[test]
    fn a_conjunction_of_several_relations_names_them_all() {
        let held = relations("как");

        assert!(held.contains(&Relation::Explanatory));
        assert!(held.contains(&Relation::Temporal));
        assert!(held.contains(&Relation::Comparative));
    }

    #[test]
    fn a_word_that_subordinates_nothing_hangs_by_nothing() {
        assert!(relations("стол").is_empty());
        assert!(!hangs_by("и", Relation::Causal));
        assert!(relations("").is_empty());
    }

    #[test]
    fn only_a_purpose_conjunction_states_a_purpose() {
        assert!(states_a_purpose("чтобы"));
        assert!(states_a_purpose("ДАБЫ"));
        assert!(!states_a_purpose("если"));
        assert!(!states_a_purpose("что"));
    }
}
