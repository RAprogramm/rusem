// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Cutting a word into morphemes by the tables alone.
//!
//! The tables say which strings can be a prefix, a suffix, an ending. They do
//! not say which of them *is* one in a given word: `нос` opens with something
//! the table lists as `но-` and is not a prefixed word, and `косточка` may be
//! read with `-очк-` or with `-к-` and both are strings the table holds.
//!
//! So nothing here chooses. Every cut the tables admit is offered, longest
//! affix first, and the layer above settles it by asking a dictionary whether
//! what remains is a word. That question is not the domain's to answer, and a
//! cut that guessed at it would be a guess wearing the clothes of a rule.
//!
//! Two things are refused rather than offered, because they are not choices:
//! a root must hold a vowel, and a root shorter than two letters is not one.

use crate::{
    alphabet,
    morphemics::{
        affix,
        derivation::{Filed, Morpheme, MorphemeKind, Segment, Segmentation}
    },
    morphology::WordForm
};

/// A compound: two roots joined by a connecting vowel.
///
/// `пар-о-ход`, `земл-е-делие`. The vowel is neither root nor suffix — it is
/// there only to join, and Russian writes `о` after a hard consonant and `е`
/// after a soft one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Joined {
    /// The root before the connecting vowel.
    pub first:    String,
    /// The connecting vowel itself.
    pub interfix: String,
    /// The root after it.
    pub second:   String
}

/// What stands around a root.
///
/// Held apart from the root because they are the part a table can find: a root
/// is what is left when they are taken off, and the tables say nothing about
/// it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Affixes {
    /// The prefixes taken off the front, in the order they stand.
    pub prefixes: Vec<String>,
    /// The suffixes taken off the back, in the order they stand.
    pub suffixes: Vec<String>,
    /// The ending, when the word carries one.
    pub ending:   Option<String>,
    /// The postfix, when the word carries one: `-ся`, `-нибудь`.
    pub postfix:  Option<String>
}

impl Affixes {
    /// Reports whether the word carries a zero ending.
    ///
    /// A masculine noun in the nominative — `стол`, `дом` — has an ending, and
    /// the ending is written with nothing. That is not the same as having
    /// none: `стол` and `стола` are one word in two cells, and a cut that saw
    /// no ending in the first would make them two.
    #[must_use]
    #[inline]
    pub const fn has_zero_ending(&self) -> bool {
        self.ending.is_none()
    }
}

/// One way a word may be cut.
///
/// The parts are named rather than placed, because a caller wanting the places
/// builds a [`Segmentation`] and gets them checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cut {
    /// What is left in the middle.
    pub root:    String,
    /// What stands around it.
    pub affixes: Affixes
}

impl Cut {
    /// How many suffixes may be taken off the end.
    pub const MOST_SUFFIXES: usize = 3;

    /// How many affixes may be taken off one end.
    ///
    /// Russian stacks two prefixes readily — `понавыдумывать` takes three — and
    /// three suffixes: `учи-тельн-иц-а`. Beyond that a cut is finding affixes
    /// in a root rather than in a word.
    pub const MOST_PREFIXES: usize = 3;

    /// The shortest a root may be.
    ///
    /// One letter is not a root of Russian: `в`, `с`, `к` are prepositions and
    /// prefixes, never roots.
    pub const SHORTEST_ROOT: usize = 2;

    /// The word this cut spells, which must be the word it was cut from.
    #[must_use]
    pub fn spells(&self) -> String {
        let mut held = String::new();

        for one in &self.affixes.prefixes {
            held.push_str(one);
        }
        held.push_str(&self.root);
        for one in &self.affixes.suffixes {
            held.push_str(one);
        }
        if let Some(one) = self.affixes.ending.as_ref() {
            held.push_str(one);
        }
        if let Some(one) = self.affixes.postfix.as_ref() {
            held.push_str(one);
        }

        held
    }

    /// This cut as a checked segmentation.
    ///
    /// The segmentation refuses a cut that does not tile the word, which is
    /// the one thing a cut can get wrong without anyone noticing.
    ///
    /// # Errors
    ///
    /// When the parts do not spell the word, or no root is among them.
    pub fn segmented(&self, form: &WordForm) -> crate::error::Result<Segmentation> {
        let mut at = 0_usize;
        let mut held = Vec::new();
        let mut place = |text: &str, kind: MorphemeKind, held: &mut Vec<Segment>| {
            let len = text.chars().count();
            held.push(Segment {
                morpheme: Morpheme {
                    kind,
                    text: String::from(text),
                    filed: Filed::default()
                },
                start: at,
                len
            });
            at += len;
        };

        for one in &self.affixes.prefixes {
            place(one, MorphemeKind::Prefix, &mut held);
        }
        place(&self.root, MorphemeKind::Root, &mut held);
        for one in &self.affixes.suffixes {
            place(one, MorphemeKind::Suffix, &mut held);
        }
        if let Some(one) = self.affixes.ending.as_ref() {
            place(one, MorphemeKind::Ending, &mut held);
        }
        if let Some(one) = self.affixes.postfix.as_ref() {
            place(one, MorphemeKind::Postfix, &mut held);
        }

        Segmentation::tiling(form.clone(), held)
    }
}

/// Every cut the tables admit, longest affix first.
///
/// The order is the order to try them in: a longer affix is the likelier
/// reading, and a caller that stops at the first cut whose root is a word gets
/// the answer a person would give.
///
/// # Examples
///
/// ```
/// use rusem::morphemics::cut;
///
/// let held = cut::ways("переходный");
/// assert!(held.iter().any(|one| one.root == "ход"));
/// assert!(held.iter().all(|one| one.spells() == "переходный"));
/// ```
#[must_use]
pub fn ways(written: &str) -> Vec<Cut> {
    let mut held = Vec::new();

    for (prefixes, after) in fronts(written) {
        for (postfix, before) in backs(&after) {
            for (suffixes, ending, root) in tails(&before) {
                held.push(Cut {
                    root,
                    affixes: Affixes {
                        prefixes: prefixes.clone(),
                        suffixes,
                        ending,
                        postfix: postfix.clone()
                    }
                });
            }
        }
    }

    held
}

/// Reports whether what is left in the middle can be a root.
///
/// A root holds a vowel and is at least two letters long. `вств` is a cluster,
/// `в` is a preposition.
#[must_use]
pub fn is_a_root(text: &str) -> bool {
    text.chars().count() >= Cut::SHORTEST_ROOT && text.chars().any(alphabet::is_vowel)
}

/// The compound a word may be, when a connecting vowel joins two roots.
///
/// Only the vowels the language joins with are looked for, and only where a
/// root stands on either side. `пароход` is a compound; `пора` is not, though
/// it holds an `о` between two consonants — nothing stands after it to be a
/// second root.
///
/// # Examples
///
/// ```
/// use rusem::morphemics::cut;
///
/// let held = cut::joined("пароход").expect("a compound");
/// assert_eq!(held.first, "пар");
/// assert_eq!(held.interfix, "о");
/// assert_eq!(held.second, "ход");
///
/// assert!(cut::joined("нос").is_none());
/// ```
#[must_use]
pub fn joined(written: &str) -> Option<Joined> {
    let letters: Vec<char> = written.chars().collect();

    for at in Cut::SHORTEST_ROOT..letters.len().checked_sub(Cut::SHORTEST_ROOT)? {
        let held = *letters.get(at)?;
        if !affix::INTERFIXES.contains(&held.to_string().as_str()) {
            continue;
        }
        let first: String = letters.get(..at)?.iter().collect();
        let second: String = letters.get(at + 1..)?.iter().collect();
        if !is_a_root(&first) || !is_a_root(&second) {
            continue;
        }

        return Some(Joined {
            first,
            interfix: held.to_string(),
            second
        });
    }

    None
}

/// Every way the front of a word may be read as prefixes.
///
/// The empty reading comes first: most words carry no prefix, and a caller
/// stopping at the first workable cut should meet the simplest one.
fn fronts(written: &str) -> Vec<(Vec<String>, String)> {
    let mut held = std::vec![(Vec::new(), String::from(written))];
    let mut at = 0_usize;

    while at < held.len() {
        let (taken, rest) = held.get(at).cloned().unwrap_or_default();
        at += 1;
        if taken.len() >= Cut::MOST_PREFIXES {
            continue;
        }

        for one in affix::all_leading(affix::PREFIXES, &rest) {
            let left: String = rest.chars().skip(one.chars().count()).collect();
            if !is_a_root(&left) {
                continue;
            }
            let mut grown = taken.clone();
            grown.push(String::from(one));
            if !held.iter().any(|(prefixes, _)| *prefixes == grown) {
                held.push((grown, left));
            }
        }
    }

    held
}

/// Every way the back of a word may be read as a postfix.
fn backs(written: &str) -> Vec<(Option<String>, String)> {
    let mut held = std::vec![(None, String::from(written))];

    if let Some(one) = affix::trailing(affix::POSTFIXES, written) {
        let left: String = written
            .chars()
            .take(written.chars().count() - one.chars().count())
            .collect();
        if is_a_root(&left) {
            held.push((Some(String::from(one)), left));
        }
    }

    held
}

/// Every way what is left may be read as suffixes, an ending and a root.
fn tails(written: &str) -> Vec<(Vec<String>, Option<String>, String)> {
    let mut held = Vec::new();

    for ending in endings(written) {
        let rest: String = written
            .chars()
            .take(written.chars().count() - ending.as_deref().map_or(0, |one| one.chars().count()))
            .collect();

        let mut queue = std::vec![(Vec::<String>::new(), rest.clone())];
        let mut at = 0_usize;

        while at < queue.len() {
            let (taken, root) = queue.get(at).cloned().unwrap_or_default();
            at += 1;
            held.push((taken.clone(), ending.clone(), root.clone()));
            if taken.len() >= Cut::MOST_SUFFIXES {
                continue;
            }

            for one in affix::all_trailing(affix::SUFFIXES, &root) {
                let left: String = root
                    .chars()
                    .take(root.chars().count() - one.chars().count())
                    .collect();
                if !is_a_root(&left) {
                    continue;
                }
                let mut grown = taken.clone();
                grown.insert(0, String::from(one));
                if !queue.iter().any(|(suffixes, _)| *suffixes == grown) {
                    queue.push((grown, left));
                }
            }
        }
    }

    held
}

/// Every way the end of a word may be read as an ending.
///
/// The bare ending comes first: a masculine noun in the nominative has none,
/// and that is not a failure to find one.
fn endings(written: &str) -> Vec<Option<String>> {
    let mut held = std::vec![None];

    if let Some(one) = affix::trailing(affix::ENDINGS, written) {
        let left: String = written
            .chars()
            .take(written.chars().count() - one.chars().count())
            .collect();
        if is_a_root(&left) {
            held.push(Some(String::from(one)));
        }
    }

    held
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roots(word: &str) -> Vec<String> {
        ways(word).into_iter().map(|held| held.root).collect()
    }

    #[test]
    fn every_cut_spells_the_word_it_was_cut_from() {
        for word in ["переходный", "косточка", "нос", "перестройка", "умываться"]
        {
            for held in ways(word) {
                assert_eq!(held.spells(), word, "{word}");
            }
        }
    }

    #[test]
    fn the_bare_word_is_always_among_the_cuts() {
        assert!(roots("нос").contains(&String::from("нос")));
        assert!(roots("стол").contains(&String::from("стол")));
    }

    #[test]
    fn a_prefixed_word_offers_its_root() {
        assert!(roots("переходный").contains(&String::from("ход")));
        assert!(roots("подводный").contains(&String::from("вод")));
    }

    #[test]
    fn a_word_with_a_postfix_offers_it() {
        let held = ways("умываться");

        assert!(
            held.iter()
                .any(|one| one.affixes.postfix.as_deref() == Some("ся"))
        );
    }

    #[test]
    fn a_root_holds_a_vowel_and_two_letters() {
        assert!(is_a_root("ход"));
        assert!(is_a_root("но"));
        assert!(!is_a_root("н"));
        assert!(!is_a_root("вств"));
        assert!(!is_a_root(""));
    }

    #[test]
    fn nothing_is_cut_down_to_a_root_that_is_not_one() {
        for word in ["переходный", "косточка", "перестройка", "стол"]
        {
            for held in ways(word) {
                assert!(is_a_root(&held.root), "{word}: {}", held.root);
            }
        }
    }

    #[test]
    fn a_word_offering_more_than_one_cut_offers_them_all() {
        let held = roots("косточка");

        assert!(held.len() > 1, "one cut only: {held:?}");
    }

    #[test]
    fn a_cut_becomes_a_checked_segmentation() {
        let form = WordForm::parse("переходный").expect("a word form");
        let held = ways("переходный")
            .into_iter()
            .find(|one| one.root == "ход")
            .expect("a cut on the root");

        let segmented = held.segmented(&form).expect("a segmentation");

        assert_eq!(segmented.form().as_str(), "переходный");
        assert_eq!(segmented.root().map(|one| one.text.as_str()), Some("ход"));
    }

    #[test]
    fn a_word_with_no_written_ending_carries_a_zero_one() {
        let held = ways("стол");

        assert!(held.iter().any(|one| one.affixes.has_zero_ending()));
    }

    #[test]
    fn a_postfixed_word_is_cut_and_spelled_back() {
        let held = ways("умываться")
            .into_iter()
            .find(|one| one.affixes.postfix.as_deref() == Some("ся"))
            .expect("a cut with a postfix");
        let form = WordForm::parse("умываться").expect("a word form");

        assert_eq!(held.spells(), "умываться");
        assert!(held.segmented(&form).is_ok());
    }

    #[test]
    fn a_word_too_short_to_hold_an_affix_is_left_whole() {
        for word in ["ум", "он"] {
            let held = ways(word);

            assert!(
                held.iter().all(|one| one.affixes.prefixes.is_empty()),
                "{word}"
            );
            assert!(held.iter().all(|one| one.root == word), "{word}");
        }
    }

    #[test]
    fn a_compound_is_parted_at_its_connecting_vowel() {
        let held = joined("пароход").expect("a compound");

        assert_eq!(held.first, "пар");
        assert_eq!(held.interfix, "о");
        assert_eq!(held.second, "ход");
    }

    #[test]
    fn a_cut_that_would_leave_no_root_is_not_offered() {
        for held in ways("оса") {
            assert!(is_a_root(&held.root), "{}", held.root);
        }

        assert!(
            joined("тсотсе").is_none(),
            "no root on the left of the vowel"
        );
        assert!(
            joined("паровс").is_none(),
            "no root on the right of the vowel"
        );
    }

    #[test]
    fn a_word_that_is_no_compound_is_not_parted() {
        assert!(joined("нос").is_none());
        assert!(joined("стол").is_none());
        assert!(joined("").is_none());
    }

    #[test]
    fn a_compound_needs_a_root_on_either_side() {
        assert!(joined("оход").is_none());
        assert!(joined("паро").is_none());
    }

    #[test]
    fn no_more_suffixes_are_taken_than_the_language_stacks() {
        let held = ways("учительницами");

        assert!(!held.is_empty());
        assert!(
            held.iter()
                .all(|one| one.affixes.suffixes.len() <= Cut::MOST_SUFFIXES)
        );
        assert!(
            held.iter()
                .any(|one| one.affixes.suffixes.len() == Cut::MOST_SUFFIXES),
            "the limit is reached and not merely respected"
        );
    }

    #[test]
    fn no_more_prefixes_are_taken_than_the_language_stacks() {
        for held in ways("понавыдумывать") {
            assert!(held.affixes.prefixes.len() <= Cut::MOST_PREFIXES);
            assert!(held.affixes.suffixes.len() <= Cut::MOST_SUFFIXES);
        }
    }
}
