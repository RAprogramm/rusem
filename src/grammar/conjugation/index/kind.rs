// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The sixteen classes Zaliznyak numbers a verb with, and the verbs outside
//! them.
//!
//! The `Грамматический словарь русского языка` sorts every verb by how its
//! present stem is got from its infinitive, and numbers the ways 1 to 16.
//! The number is the fact a dictionary states; everything mechanical about
//! the present stem follows from it, and what does not follow — which velar
//! `-чь` hides, which nasal a class-14 verb keeps — the dictionary states
//! beside the number, not inside it.
//!
//! A handful of verbs fit no class at all — `дать`, `есть`, `идти`, `быть` —
//! and the dictionary marks them isolated instead of numbering them.

/// One class of Zaliznyak's verb classification.
///
/// Each variant's note gives the infinitive shape the class reads and one
/// verb the dictionary states in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Kind {
    /// 1: `-ать` with the glide kept — `читать, читаю`.
    One,
    /// 2: `-овать`, `-евать` trading the suffix for `-у-`, `-ю-` —
    /// `рисовать, рисую`.
    Two,
    /// 3: `-нуть` — `толкнуть, толкну`; the `3°` subtype drops `-ну-` in the
    /// past — `сохнуть, сох`.
    Three,
    /// 4: `-ить` of the second conjugation — `говорить, говорю`.
    Four,
    /// 5: `-ать`, `-ять`, `-еть` of the second conjugation — `слышать, слышу`.
    Five,
    /// 6: `-ать`, `-ять` with the stem swapped throughout the present —
    /// `писать, пишу`; the `6°` subtype swaps nothing — `звать, зову`.
    Six,
    /// 7: `-зти`, `-зть`, `-сти`, `-сть` on a consonant stem — `везти, везу`;
    /// what `-сти` hides is the dictionary's own note.
    Seven,
    /// 8: `-чь` on a velar stem — `печь, пеку`; which velar is the
    /// dictionary's own note.
    Eight,
    /// 9: `-ереть` cut to a bare `р` — `тереть, тру`.
    Nine,
    /// 10: `-ороть`, `-олоть` cut to the liquid — `колоть, колю`.
    Ten,
    /// 11: a monosyllabic `-ить` conjugating on `-ью` — `бить, бью`.
    Eleven,
    /// 12: `-ыть`, `-уть`, `-ить` trading the vowel before a glide —
    /// `мыть, мою`.
    Twelve,
    /// 13: `-авать` losing `-ва-` in the present — `давать, даю`.
    Thirteen,
    /// 14: `-ать`, `-ять` on a hidden nasal stem — `жать, жму`; which nasal
    /// is the dictionary's own note.
    Fourteen,
    /// 15: `-ть` taking `н` — `стать, стану`.
    Fifteen,
    /// 16: `-ть` taking `в` — `жить, живу`.
    Sixteen,
    /// `^`: a verb outside the sixteen, whose forms the dictionary prints —
    /// `дать`, `есть`, `идти`, `быть`.
    Isolated
}

impl Kind {
    /// The class a digit names, or nothing for a digit the dictionary does
    /// not use.
    #[must_use]
    pub const fn of(digit: u8) -> Option<Self> {
        match digit {
            1 => Some(Self::One),
            2 => Some(Self::Two),
            3 => Some(Self::Three),
            4 => Some(Self::Four),
            5 => Some(Self::Five),
            6 => Some(Self::Six),
            7 => Some(Self::Seven),
            8 => Some(Self::Eight),
            9 => Some(Self::Nine),
            10 => Some(Self::Ten),
            11 => Some(Self::Eleven),
            12 => Some(Self::Twelve),
            13 => Some(Self::Thirteen),
            14 => Some(Self::Fourteen),
            15 => Some(Self::Fifteen),
            16 => Some(Self::Sixteen),
            _ => None
        }
    }

    /// The digit the class is written with, absent for an isolated verb,
    /// which the dictionary marks `^` instead of numbering.
    #[must_use]
    pub const fn digit(self) -> Option<u8> {
        match self {
            Self::One => Some(1),
            Self::Two => Some(2),
            Self::Three => Some(3),
            Self::Four => Some(4),
            Self::Five => Some(5),
            Self::Six => Some(6),
            Self::Seven => Some(7),
            Self::Eight => Some(8),
            Self::Nine => Some(9),
            Self::Ten => Some(10),
            Self::Eleven => Some(11),
            Self::Twelve => Some(12),
            Self::Thirteen => Some(13),
            Self::Fourteen => Some(14),
            Self::Fifteen => Some(15),
            Self::Sixteen => Some(16),
            Self::Isolated => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_digit_reads_back_to_itself() {
        for digit in 1..=16 {
            let held = Kind::of(digit).unwrap_or_else(|| unreachable!("a stated class"));
            assert_eq!(held.digit(), Some(digit));
        }
    }

    #[test]
    fn a_digit_the_dictionary_does_not_use_names_nothing() {
        assert_eq!(Kind::of(0), None);
        assert_eq!(Kind::of(17), None);
    }

    #[test]
    fn an_isolated_verb_has_no_digit() {
        assert_eq!(Kind::Isolated.digit(), None);
    }
}
