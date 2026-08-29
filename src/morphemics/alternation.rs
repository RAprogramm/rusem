// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The consonants a Russian stem swaps when an ending is added.
//!
//! `пеку` beside `печёшь`, `писать` beside `пишу`, `любить` beside `люблю`.
//! The stem does not change its meaning and does not change its class; one
//! consonant is replaced by another, always the same one, in the same places.
//!
//! Three sets, and they are not interchangeable. The first velars swap before
//! a front vowel, and that swap runs through the whole conjugation of a verb
//! like `печь`. The second set is the one that shows up in the present stem of
//! a verb in `-ать` and in the first person of the second conjugation. The
//! labials do not swap at all: they take an `л` after them, and nothing else
//! in Russian does that.
//!
//! Every table here is written both ways, because a form is built in one
//! direction and read in the other.

/// The swaps a velar makes before a front vowel: `пеку` — `печёшь`.
const VELAR: &[(char, char)] = &[('к', 'ч'), ('г', 'ж'), ('х', 'ш')];

/// The swaps the present stem makes: `писать` — `пишу`, `водить` — `вожу`.
const PRESENT: &[(char, char)] = &[
    ('д', 'ж'),
    ('т', 'ч'),
    ('з', 'ж'),
    ('с', 'ш'),
    ('к', 'ч'),
    ('г', 'ж'),
    ('х', 'ш'),
    ('ц', 'ч')
];

/// The clusters the present stem swaps whole: `искать` — `ищу`, `ездить` —
/// `езжу`.
///
/// `зд` keeps its `з` in the swap — the `д` alone becomes `ж` — so the entry
/// is written `зж`, not `ж`: `езжу`, not `ежу`.
const CLUSTERS: &[(&str, &str)] = &[("ск", "щ"), ("ст", "щ"), ("зд", "зж")];

/// The consonants that take an `л` rather than swapping: `любить` — `люблю`.
const LABIAL: &[char] = &['б', 'п', 'в', 'ф', 'м'];

/// The letter a labial takes after it.
pub const EPENTHESIS: char = 'л';

/// Which set of swaps is being asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Kind {
    /// The swap a velar makes before a front vowel, through the conjugation.
    Velar,
    /// The swap the present stem makes against the infinitive.
    Present
}

/// The consonant a stem ends with once the swap has been made.
///
/// A stem that does not end in a swapping consonant comes back unchanged, so
/// the caller does not have to ask first.
///
/// # Examples
///
/// ```
/// use rusem::morphemics::alternation::{Kind, swapped};
///
/// assert_eq!(swapped('к', Kind::Velar), 'ч');
/// assert_eq!(swapped('д', Kind::Present), 'ж');
/// assert_eq!(swapped('н', Kind::Present), 'н');
/// ```
#[must_use]
pub fn swapped(last: char, kind: Kind) -> char {
    let table = match kind {
        Kind::Velar => VELAR,
        Kind::Present => PRESENT
    };

    table
        .iter()
        .find(|(from, _)| *from == last)
        .map_or(last, |(_, into)| *into)
}

/// The consonant a swapped stem was written with before the swap.
///
/// A swap is not always reversible on its own: `ж` stands for `д`, `з` and `г`
/// at once, and `ш` for `с` and `х`. Every letter that could have produced the
/// swapped one is returned, and the caller keeps them all.
///
/// # Examples
///
/// ```
/// use rusem::morphemics::alternation::{Kind, unswapped};
///
/// assert_eq!(unswapped('ж', Kind::Present), vec!['д', 'з', 'г']);
/// assert_eq!(unswapped('н', Kind::Present), vec!['н']);
/// ```
#[must_use]
pub fn unswapped(last: char, kind: Kind) -> Vec<char> {
    let table = match kind {
        Kind::Velar => VELAR,
        Kind::Present => PRESENT
    };

    let held: Vec<char> = table
        .iter()
        .filter(|(_, into)| *into == last)
        .map(|(from, _)| *from)
        .collect();

    if held.is_empty() { vec![last] } else { held }
}

/// The whole cluster a stem ends with once the swap has been made.
///
/// `искать` puts `ск` where the present puts `щ`, and no letter-by-letter rule
/// finds that. The stem is returned changed when it ends in such a cluster and
/// unchanged when it does not.
///
/// # Examples
///
/// ```
/// use rusem::morphemics::alternation::clustered;
///
/// assert_eq!(clustered("иск"), Some(String::from("ищ")));
/// assert_eq!(clustered("чита"), None);
/// ```
#[must_use]
pub fn clustered(stem: &str) -> Option<String> {
    CLUSTERS.iter().find_map(|(from, into)| {
        stem.strip_suffix(from)
            .map(|head| String::from(head) + into)
    })
}

/// Reports whether a consonant takes an `л` after it instead of swapping.
///
/// The labials are the one place Russian adds a letter rather than replacing
/// one: `любить` gives `люблю`, `спать` gives `сплю`. It happens in the first
/// person singular of the second conjugation and nowhere else in the paradigm,
/// which is why `любишь` has no `л`.
///
/// # Examples
///
/// ```
/// use rusem::morphemics::alternation::takes_epenthesis;
///
/// assert!(takes_epenthesis('б'));
/// assert!(!takes_epenthesis('т'));
/// ```
#[must_use]
pub fn takes_epenthesis(last: char) -> bool {
    LABIAL.contains(&last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_velar_swaps_before_a_front_vowel() {
        assert_eq!(swapped('к', Kind::Velar), 'ч');
        assert_eq!(swapped('г', Kind::Velar), 'ж');
        assert_eq!(swapped('х', Kind::Velar), 'ш');
    }

    #[test]
    fn the_present_stem_swaps_its_own_set() {
        assert_eq!(swapped('д', Kind::Present), 'ж');
        assert_eq!(swapped('т', Kind::Present), 'ч');
        assert_eq!(swapped('с', Kind::Present), 'ш');
        assert_eq!(swapped('ц', Kind::Present), 'ч');
    }

    #[test]
    fn a_consonant_outside_the_table_is_left_alone() {
        assert_eq!(swapped('н', Kind::Present), 'н');
        assert_eq!(swapped('р', Kind::Velar), 'р');
    }

    #[test]
    fn a_swap_that_two_letters_produce_is_read_back_as_both() {
        assert_eq!(unswapped('ж', Kind::Present), vec!['д', 'з', 'г']);
        assert_eq!(unswapped('ш', Kind::Present), vec!['с', 'х']);
        assert_eq!(unswapped('ж', Kind::Velar), vec!['г']);
    }

    #[test]
    fn a_letter_that_is_no_swap_reads_back_as_itself() {
        assert_eq!(unswapped('н', Kind::Present), vec!['н']);
    }

    #[test]
    fn a_cluster_swaps_whole() {
        assert_eq!(clustered("иск"), Some(String::from("ищ")));
        assert_eq!(clustered("прост"), Some(String::from("прощ")));
        assert_eq!(clustered("чита"), None);
    }

    #[test]
    fn a_zd_stem_keeps_its_z_in_the_swap() {
        assert_eq!(clustered("езд"), Some(String::from("езж")));
        assert_eq!(clustered("гвозд"), Some(String::from("гвозж")));
    }

    #[test]
    fn a_labial_takes_a_letter_rather_than_swapping() {
        assert!(takes_epenthesis('б'));
        assert!(takes_epenthesis('в'));
        assert!(takes_epenthesis('м'));
        assert!(!takes_epenthesis('т'));
        assert_eq!(swapped('б', Kind::Present), 'б');
    }
}
