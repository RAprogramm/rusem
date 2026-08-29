// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The cases of Russian, and the reductions between them.

/// Grammatical case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Case {
    /// Nominative.
    Nominative,
    /// Genitive.
    Genitive,
    /// Dative.
    Dative,
    /// Accusative.
    Accusative,
    /// Instrumental.
    Instrumental,
    /// Prepositional.
    Prepositional,
    /// Partitive, the second genitive.
    Partitive,
    /// Locative, the second prepositional.
    Locative,
    /// Vocative.
    Vocative
}

impl Case {
    /// The cases a paradigm states a cell for, in the order the grammars
    /// name them.
    ///
    /// Six of the nine: the partitive, the locative and the vocative are not
    /// cells the tables state — where a word has one, the dictionary marks it
    /// with signs of its own — and wherever cases are compared they count by
    /// [`Case::merged`] instead. Every table in the engine walks its cells in
    /// this order, so the order is stated beside the cases rather than once
    /// per table.
    pub const STATED: [Self; 6] = [
        Self::Nominative,
        Self::Genitive,
        Self::Dative,
        Self::Accusative,
        Self::Instrumental,
        Self::Prepositional
    ];

    /// The case this one counts as when forms are compared.
    ///
    /// Russian keeps two remnants of older cases inside two of the living
    /// ones. `в лесу` is the second locative and `чашка чаю` the second
    /// genitive: the noun takes a shape of its own, but the adjective before
    /// it and the preposition governing it behave as though the case were
    /// prepositional and genitive. Comparing the shapes rather than the cases
    /// calls `в нашем лесу` a disagreement.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::grammar::Case;
    ///
    /// assert_eq!(Case::Locative.merged(), Case::Prepositional);
    /// assert_eq!(Case::Partitive.merged(), Case::Genitive);
    /// assert_eq!(Case::Dative.merged(), Case::Dative);
    /// ```
    #[must_use]
    pub const fn merged(self) -> Self {
        match self {
            Self::Locative => Self::Prepositional,
            Self::Partitive => Self::Genitive,
            held => held
        }
    }
}
