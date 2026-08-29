// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The one root behind its several shapes.
//!
//! A root of Russian is not one string. `рука` and `ручной` share a root and
//! write it two ways; `сон` and `сна` share one and write it two ways again.
//! A cut that took the written shape for the root would call these different
//! words unrelated, which is the one thing a morphemic level exists to avoid.
//!
//! Two things move a root, and they are different in kind.
//!
//! A consonant **swaps**: `к` gives `ч`, `г` gives `ж`, `х` gives `ш` before
//! the front vowel of a suffix. The swap is stated in [`super::alternation`]
//! and is reversible — several letters may lie behind one, so reading it back
//! answers a list.
//!
//! A vowel is **fleeting**: it stands in one form and is gone in the next.
//! `сон` against `сна`, `отец` against `отца`. The vowel belongs to the root
//! and is not an ending, so a cut that dropped it would leave `сн` and call it
//! a root.
//!
//! Which of the two a given vowel is cannot be read off the word. `сон` and
//! `нос` are the same three sounds in the same order and only one of them
//! parts. So what is offered here is every shape a root *may* be written as,
//! and the dictionary settles which of them a word actually has — the same
//! division of labour as in [`super::cut`].

use crate::{
    grammar::declension::spelling,
    morphemics::alternation::{self, Kind}
};

/// Every shape a root may be written as, the given one first.
///
/// This is what makes two cuts comparable: a caller holding `руч` and a caller
/// holding `рук` both find the other in this list, and neither has to know
/// which shape the dictionary happens to store.
///
/// The given shape comes first because it is the one actually written; the
/// rest are what it could also be. Could, not does: `нос` is offered without
/// its `о` because the shape allows it, and only a dictionary knows that this
/// particular word does not part.
///
/// # Examples
///
/// ```
/// use rusem::morphemics::root::shapes;
///
/// let held = shapes("руч");
/// assert_eq!(held.first().map(String::as_str), Some("руч"));
/// assert!(held.iter().any(|one| one == "рук"));
///
/// let dream = shapes("сон");
/// assert!(dream.iter().any(|one| one == "сн"));
/// ```
#[must_use]
pub fn shapes(root: &str) -> Vec<String> {
    let mut held = std::vec![String::from(root)];

    for one in unswapped(root) {
        if !held.contains(&one) {
            held.push(one);
        }
    }
    if let Some(one) = without_fleeting(root)
        && !held.contains(&one)
    {
        held.push(one);
    }
    for stressed in [false, true] {
        let parted = spelling::parted(root, stressed);
        if parted != root && !held.contains(&parted) {
            held.push(parted);
        }
    }

    held
}

/// Reports whether two written roots can be the same root.
///
/// Can, not are. The question a caller usually wants is whether two cuts are
/// worth holding against a dictionary together, and that is what this answers.
///
/// # Examples
///
/// ```
/// use rusem::morphemics::root::same;
///
/// assert!(same("рук", "руч"));
/// assert!(same("сон", "сн"));
/// assert!(!same("рук", "нос"));
/// ```
#[must_use]
pub fn same(one: &str, other: &str) -> bool {
    one == other
        || shapes(one).contains(&String::from(other))
        || shapes(other).contains(&String::from(one))
}

/// The roots a swapped consonant could have been written with.
///
/// `руч` gives `рук`, and `нож` gives `ног`, `нод` and `ноз` at once — the
/// swap is not reversible on its own, so every letter that could have produced
/// it is offered.
fn unswapped(root: &str) -> Vec<String> {
    let Some(last) = root.chars().last() else {
        return Vec::new();
    };
    let front: String = root.chars().take(root.chars().count() - 1).collect();

    let mut held = Vec::new();
    for kind in [Kind::Velar, Kind::Present] {
        for one in alternation::unswapped(last, kind) {
            if one == last {
                continue;
            }
            let mut shape = front.clone();
            shape.push(one);
            if !held.contains(&shape) {
                held.push(shape);
            }
        }
    }

    held
}

/// The root with its fleeting vowel taken out.
///
/// `сон` gives `сн`, `отец` gives `отц`. The vowel is fleeting only when a
/// consonant stands on either side of it, which is what makes it a vowel that
/// parted two consonants rather than one belonging to the root.
fn without_fleeting(root: &str) -> Option<String> {
    let letters: Vec<char> = root.chars().collect();
    let count = letters.len();
    let last = *letters.get(count.checked_sub(1)?)?;
    let vowel = *letters.get(count.checked_sub(2)?)?;
    let before = *letters.get(count.checked_sub(3)?)?;

    if !crate::alphabet::is_consonant(last) {
        return None;
    }
    if !matches!(vowel, 'о' | 'е' | 'ё') {
        return None;
    }
    if !crate::alphabet::is_consonant(before) {
        return None;
    }

    let mut held: String = letters.get(..count - 2)?.iter().collect();
    held.push(last);

    Some(held)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_root_is_always_among_its_own_shapes() {
        for held in ["рук", "сон", "нос", "ход"] {
            assert_eq!(shapes(held).first().map(String::as_str), Some(held));
        }
    }

    #[test]
    fn a_swapped_consonant_is_read_back() {
        assert!(shapes("руч").iter().any(|held| held == "рук"));
        assert!(shapes("нож").iter().any(|held| held == "ног"));
        assert!(shapes("суш").iter().any(|held| held == "сух"));
    }

    #[test]
    fn a_swap_that_two_letters_produce_offers_both() {
        let held = shapes("нож");

        assert!(held.iter().any(|one| one == "ног"));
        assert!(held.iter().any(|one| one == "нод"));
    }

    #[test]
    fn a_vowel_between_two_consonants_is_offered_as_fleeting() {
        assert!(shapes("сон").iter().any(|held| held == "сн"));
        assert!(shapes("отец").iter().any(|held| held == "отц"));
    }

    #[test]
    fn a_vowel_that_could_not_be_fleeting_is_not_offered() {
        assert!(!shapes("вода").iter().any(|held| held == "вда"));
        assert!(!shapes("до").iter().any(|held| held == "д"));
    }

    #[test]
    fn the_shape_is_offered_and_not_asserted() {
        let held = shapes("нос");

        assert!(
            held.iter().any(|one| one == "нс"),
            "the shape allows it, and only a dictionary refuses it"
        );
    }

    #[test]
    fn two_shapes_of_one_root_are_named_the_same_root() {
        assert!(same("рук", "руч"));
        assert!(same("руч", "рук"));
        assert!(same("сон", "сн"));
        assert!(same("ход", "ход"));
    }

    #[test]
    fn two_different_roots_are_not_named_the_same() {
        assert!(!same("рук", "нос"));
        assert!(!same("ход", "вод"));
    }

    #[test]
    fn a_stem_that_parts_two_consonants_offers_the_parted_shape() {
        let held = shapes("окн");

        assert!(
            held.len() > 1,
            "the genitive plural parts the cluster: {held:?}"
        );
    }

    #[test]
    fn an_empty_root_has_no_shapes_but_itself() {
        assert_eq!(shapes(""), std::vec![String::new()]);
    }

    #[test]
    fn a_vowel_after_a_vowel_is_not_fleeting() {
        assert!(!shapes("моок").iter().any(|held| held == "мок"));
    }

    #[test]
    fn a_root_of_one_letter_has_only_itself() {
        assert_eq!(shapes("о"), std::vec![String::from("о")]);
    }
}
