// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The ten vowels, and what each of them says about the consonant before it.
//!
//! Russian writes softness with the vowel rather than with the consonant:
//! `нос` and `нёс` differ in one letter, and the letter that differs is the
//! vowel. So a vowel carries two facts — the sound it stands for, and whether
//! the consonant before it is soft — and four of them carry a glide besides.
//!
//! `ё` carries a third: it is written only under stress, which makes it the
//! one letter of the alphabet that states where the stress falls.

pub mod softness;

/// One of the ten vowels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Vowel {
    /// `а`
    A,
    /// `е`
    Je,
    /// `ё`
    Jo,
    /// `и`
    I,
    /// `о`
    O,
    /// `у`
    U,
    /// `ы`
    Yi,
    /// `э`
    E,
    /// `ю`
    Ju,
    /// `я`
    Ja
}

impl Vowel {
    /// The vowel a small letter is, or nothing when it is not a vowel.
    #[must_use]
    #[inline]
    pub const fn of(letter: char) -> Option<Self> {
        Some(match letter {
            'а' => Self::A,
            'е' => Self::Je,
            'ё' => Self::Jo,
            'и' => Self::I,
            'о' => Self::O,
            'у' => Self::U,
            'ы' => Self::Yi,
            'э' => Self::E,
            'ю' => Self::Ju,
            'я' => Self::Ja,
            _ => return None
        })
    }

    /// How the vowel is written.
    #[must_use]
    #[inline]
    pub const fn written(self) -> char {
        match self {
            Self::A => 'а',
            Self::Je => 'е',
            Self::Jo => 'ё',
            Self::I => 'и',
            Self::O => 'о',
            Self::U => 'у',
            Self::Yi => 'ы',
            Self::E => 'э',
            Self::Ju => 'ю',
            Self::Ja => 'я'
        }
    }

    /// Reports whether the vowel is written only under stress.
    ///
    /// True of `ё` and of nothing else. This is why a word holding one needs
    /// no table of stresses.
    #[must_use]
    #[inline]
    pub const fn is_stressed(self) -> bool {
        matches!(self, Self::Jo)
    }
}

impl TryFrom<char> for Vowel {
    type Error = crate::alphabet::NotALetter;

    fn try_from(letter: char) -> Result<Self, Self::Error> {
        Self::of(crate::alphabet::folded(letter)).ok_or(crate::alphabet::NotALetter(letter))
    }
}

impl From<Vowel> for char {
    fn from(held: Vowel) -> Self {
        held.written()
    }
}

impl core::fmt::Display for Vowel {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{}", self.written())
    }
}

/// The spelling with `ё` written the way it is always allowed to be written.
///
/// The diaeresis is the one mark of the alphabet a writer may drop at
/// pleasure: `тёмен` and `темен` are the same form written twice. The letter
/// belongs to this module — [`Vowel::Jo`] is the vowel being folded — and the
/// folding is stated here once, for every reader in the engine that compares
/// spellings or looks one up. It is never stored: kept, it would erase the
/// distinction between `все` and `всё`.
#[must_use]
pub fn folded(written: &str) -> std::borrow::Cow<'_, str> {
    if written.contains('ё') {
        std::borrow::Cow::Owned(written.replace('ё', "е"))
    } else {
        std::borrow::Cow::Borrowed(written)
    }
}

/// Reports whether two spellings are the same form.
///
/// `ё` may always be written `е`, so a comparison that told them apart would
/// refuse half of what people write.
#[must_use]
pub fn same(left: &str, right: &str) -> bool {
    folded(left) == folded(right)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The five pairs the grammars state, written out to hold the match above
    /// against something other than itself.
    const PAIRED: &[(Vowel, Vowel)] = &[
        (Vowel::A, Vowel::Ja),
        (Vowel::O, Vowel::Jo),
        (Vowel::U, Vowel::Ju),
        (Vowel::E, Vowel::Je),
        (Vowel::Yi, Vowel::I)
    ];

    #[test]
    fn a_spelling_with_the_diaeresis_dropped_is_the_same_form() {
        assert!(same("тёмен", "темен"));
        assert!(same("стол", "стол"));
        assert!(!same("всё", "вся"));

        assert_eq!(folded("всё"), "все");
        assert_eq!(folded("стол"), "стол");
    }

    #[test]
    fn the_pairs_are_the_five_the_grammars_state() {
        for (hard, soft) in PAIRED {
            assert_eq!(hard.paired(), *soft);
            assert_eq!(soft.paired(), *hard);
        }

        assert_eq!(PAIRED.len() * 2, all().len());
    }

    fn all() -> Vec<Vowel> {
        crate::alphabet::vowels().collect()
    }

    #[test]
    fn the_alphabet_holds_ten_vowels() {
        assert_eq!(all().len(), 10);
    }

    #[test]
    fn every_vowel_reads_back_as_itself() {
        for held in all() {
            assert_eq!(Vowel::of(held.written()), Some(held));
        }
    }

    #[test]
    fn the_pairing_is_its_own_inverse() {
        for held in all() {
            assert_eq!(held.paired().paired(), held);
        }
    }

    #[test]
    fn a_pair_holds_one_soft_vowel_and_one_hard() {
        for held in all() {
            let other = held.paired();

            assert_ne!(other, held, "every vowel of Russian has a pair");
            assert_ne!(held.softens(), other.softens());
        }
    }

    #[test]
    fn jo_is_the_only_vowel_written_under_stress() {
        assert_eq!(all().iter().filter(|held| held.is_stressed()).count(), 1);
        assert!(Vowel::Jo.is_stressed());
    }

    #[test]
    fn a_glide_before_a_hard_vowel_writes_the_iotated_one() {
        assert_eq!(Vowel::U.carrying_glide(), Some(Vowel::Ju));
        assert_eq!(Vowel::A.carrying_glide(), Some(Vowel::Ja));
        assert_eq!(Vowel::E.carrying_glide(), Some(Vowel::Je));
        assert_eq!(Vowel::O.carrying_glide(), Some(Vowel::Jo));
    }

    #[test]
    fn a_vowel_that_already_carries_the_glide_stays_itself() {
        for held in [Vowel::Ja, Vowel::Ju, Vowel::Je, Vowel::Jo, Vowel::I] {
            assert_eq!(held.carrying_glide(), Some(held));
        }
    }

    #[test]
    fn yi_never_stands_after_a_glide() {
        assert_eq!(Vowel::Yi.carrying_glide(), None);
    }

    #[test]
    fn the_glide_pairing_is_not_the_hardness_pairing() {
        assert_eq!(Vowel::Yi.paired(), Vowel::I);
        assert_eq!(Vowel::Yi.carrying_glide(), None);
    }

    #[test]
    fn yi_and_i_are_a_pair_and_the_only_one_without_a_glide() {
        assert_eq!(Vowel::Yi.paired(), Vowel::I);
        assert!(Vowel::I.softens());
        assert!(!Vowel::Yi.softens());
    }

    #[test]
    fn the_standard_conversions_hold_both_ways() {
        for held in all() {
            let written = char::from(held);

            assert_eq!(Vowel::try_from(written), Ok(held));
            assert_eq!(held.to_string(), written.to_string());
        }
    }

    #[test]
    fn a_conversion_names_what_it_refused() {
        assert_eq!(Vowel::try_from('q'), Err(crate::alphabet::NotALetter('q')));
    }
}
