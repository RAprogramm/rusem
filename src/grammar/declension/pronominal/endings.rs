// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The endings of the pronominal declension.
//!
//! Twenty-six written forms to a pronoun, and all of them are a stem and the
//! endings below. `тот` is `т`, `весь` is `вс`, `чей` is `чь`.
//!
//! # Two things decide an ending
//!
//! The **pattern** parts `тем`, `всем`, `кем` from `этим`, `моим`, `самим`,
//! and parts the plural `те`, `тех`, `теми` from `эти`, `этих`, `этими`. It is
//! the older declension against the newer one, and nothing but history is
//! behind it.
//!
//! The **stem** parts `того`, `этому`, `том` from `всего`, `моему`, `моём`,
//! and `то`, `та` from `всё`, `вся`. It is the ordinary hardness of Russian,
//! the same one the nouns and the adjectives answer to.
//!
//! The two are free of each other, and all four combinations are spoken:
//! `тот` is older and hard, `весь` older and soft, `этот` newer and hard,
//! `мой` newer and soft.

use crate::grammar::{
    Animacy, Case, Gender, Number, declension::attributive::Endings, stem::Stem
};

/// Which of the two pronominal patterns a word declines by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Pattern {
    /// `тот`, `весь`, `кто`, `что`: `тем`, `тех`, `теми`.
    Older,
    /// `этот`, `мой`, `чей`, `сам`: `этим`, `этих`, `этими`.
    Newer
}

impl Pattern {
    /// The instrumental of the masculine and neuter singular.
    const fn instrumental(self) -> &'static str {
        match self {
            Self::Older => "ем",
            Self::Newer => "им"
        }
    }
}

/// The endings a pronoun takes in the gender and number asked for.
#[must_use]
pub const fn table(pattern: Pattern, shape: Stem, gender: Gender, number: Number) -> Endings {
    match (number, gender) {
        (Number::Plural, _) => plural(pattern),
        (Number::Singular, Gender::Feminine) => feminine(shape),
        (Number::Singular, Gender::Neuter) => neuter(pattern, shape),
        (Number::Singular, _) => masculine(pattern, shape)
    }
}

/// The masculine singular.
///
/// The nominative is empty because no pronoun builds it out of this stem:
/// `тот`, `весь`, `мой` and `кто` each say it their own way, and the paradigm
/// takes it from the dictionary form rather than making it.
const fn masculine(pattern: Pattern, shape: Stem) -> Endings {
    match shape {
        Stem::Hard => Endings {
            nominative:    "",
            genitive:      "ого",
            dative:        "ому",
            accusative:    None,
            instrumental:  pattern.instrumental(),
            prepositional: "ом"
        },
        Stem::Soft => Endings {
            nominative:    "",
            genitive:      "его",
            dative:        "ему",
            accusative:    None,
            instrumental:  pattern.instrumental(),
            prepositional: "ём"
        }
    }
}

/// The neuter singular, which parts from the masculine in two cells.
const fn neuter(pattern: Pattern, shape: Stem) -> Endings {
    let vowel = match shape {
        Stem::Hard => "о",
        Stem::Soft => "ё"
    };

    Endings {
        nominative: vowel,
        accusative: Some(vowel),
        ..masculine(pattern, shape)
    }
}

/// The feminine singular, which the pattern does not touch.
const fn feminine(shape: Stem) -> Endings {
    match shape {
        Stem::Hard => Endings {
            nominative:    "а",
            genitive:      "ой",
            dative:        "ой",
            accusative:    Some("у"),
            instrumental:  "ой",
            prepositional: "ой"
        },
        Stem::Soft => Endings {
            nominative:    "я",
            genitive:      "ей",
            dative:        "ей",
            accusative:    Some("ю"),
            instrumental:  "ей",
            prepositional: "ей"
        }
    }
}

/// The plural, which states no gender and which the stem does not touch.
const fn plural(pattern: Pattern) -> Endings {
    match pattern {
        Pattern::Older => Endings {
            nominative:    "е",
            genitive:      "ех",
            dative:        "ем",
            accusative:    None,
            instrumental:  "еми",
            prepositional: "ех"
        },
        Pattern::Newer => Endings {
            nominative:    "и",
            genitive:      "их",
            dative:        "им",
            accusative:    None,
            instrumental:  "ими",
            prepositional: "их"
        }
    }
}

/// The ending one cell of the paradigm is written with.
#[must_use]
pub fn of(
    pattern: Pattern,
    shape: Stem,
    gender: Gender,
    number: Number,
    case: Case,
    animacy: Animacy
) -> &'static str {
    table(pattern, shape, gender, number).of(case, animacy)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole of one gender and number, written out on a stem.
    fn written(
        pattern: Pattern,
        shape: Stem,
        gender: Gender,
        number: Number,
        stem: &str
    ) -> Vec<String> {
        Case::STATED
            .iter()
            .map(|case| {
                std::format!(
                    "{stem}{}",
                    of(pattern, shape, gender, number, *case, Animacy::Inanimate)
                )
            })
            .collect()
    }

    #[test]
    fn the_older_hard_pattern_writes_the_paradigm_of_tot() {
        assert_eq!(
            written(
                Pattern::Older,
                Stem::Hard,
                Gender::Neuter,
                Number::Singular,
                "т"
            ),
            std::vec!["то", "того", "тому", "то", "тем", "том"]
        );
        assert_eq!(
            written(
                Pattern::Older,
                Stem::Hard,
                Gender::Feminine,
                Number::Singular,
                "т"
            ),
            std::vec!["та", "той", "той", "ту", "той", "той"]
        );
        assert_eq!(
            written(
                Pattern::Older,
                Stem::Hard,
                Gender::Masculine,
                Number::Plural,
                "т"
            ),
            std::vec!["те", "тех", "тем", "те", "теми", "тех"]
        );
    }

    #[test]
    fn the_older_soft_pattern_writes_the_paradigm_of_ves() {
        assert_eq!(
            written(
                Pattern::Older,
                Stem::Soft,
                Gender::Neuter,
                Number::Singular,
                "вс"
            ),
            std::vec!["всё", "всего", "всему", "всё", "всем", "всём"]
        );
        assert_eq!(
            written(
                Pattern::Older,
                Stem::Soft,
                Gender::Feminine,
                Number::Singular,
                "вс"
            ),
            std::vec!["вся", "всей", "всей", "всю", "всей", "всей"]
        );
        assert_eq!(
            written(
                Pattern::Older,
                Stem::Soft,
                Gender::Masculine,
                Number::Plural,
                "вс"
            ),
            std::vec!["все", "всех", "всем", "все", "всеми", "всех"]
        );
    }

    #[test]
    fn the_newer_hard_pattern_writes_the_paradigm_of_etot() {
        assert_eq!(
            written(
                Pattern::Newer,
                Stem::Hard,
                Gender::Neuter,
                Number::Singular,
                "эт"
            ),
            std::vec!["это", "этого", "этому", "это", "этим", "этом"]
        );
        assert_eq!(
            written(
                Pattern::Newer,
                Stem::Hard,
                Gender::Masculine,
                Number::Plural,
                "эт"
            ),
            std::vec!["эти", "этих", "этим", "эти", "этими", "этих"]
        );
    }

    #[test]
    fn the_newer_soft_pattern_writes_the_paradigm_of_moy() {
        assert_eq!(
            written(
                Pattern::Newer,
                Stem::Soft,
                Gender::Neuter,
                Number::Singular,
                "мо"
            ),
            std::vec!["моё", "моего", "моему", "моё", "моим", "моём"]
        );
        assert_eq!(
            written(
                Pattern::Newer,
                Stem::Soft,
                Gender::Feminine,
                Number::Singular,
                "мо"
            ),
            std::vec!["моя", "моей", "моей", "мою", "моей", "моей"]
        );
        assert_eq!(
            written(
                Pattern::Newer,
                Stem::Soft,
                Gender::Masculine,
                Number::Plural,
                "мо"
            ),
            std::vec!["мои", "моих", "моим", "мои", "моими", "моих"]
        );
    }

    #[test]
    fn a_living_being_takes_the_genitive_for_its_accusative() {
        assert_eq!(
            of(
                Pattern::Older,
                Stem::Hard,
                Gender::Masculine,
                Number::Singular,
                Case::Accusative,
                Animacy::Animate
            ),
            "ого"
        );
        assert_eq!(
            of(
                Pattern::Older,
                Stem::Hard,
                Gender::Masculine,
                Number::Plural,
                Case::Accusative,
                Animacy::Animate
            ),
            "ех"
        );
    }

    #[test]
    fn a_thing_repeats_the_nominative_and_the_masculine_singular_has_none() {
        assert_eq!(
            of(
                Pattern::Older,
                Stem::Hard,
                Gender::Masculine,
                Number::Singular,
                Case::Accusative,
                Animacy::Inanimate
            ),
            ""
        );
    }

    #[test]
    fn the_pattern_decides_the_instrumental_and_the_stem_does_not() {
        for shape in [Stem::Hard, Stem::Soft] {
            assert_eq!(masculine(Pattern::Older, shape).instrumental, "ем");
            assert_eq!(masculine(Pattern::Newer, shape).instrumental, "им");
        }
    }

    #[test]
    fn the_stem_decides_the_genitive_and_the_pattern_does_not() {
        for pattern in [Pattern::Older, Pattern::Newer] {
            assert_eq!(masculine(pattern, Stem::Hard).genitive, "ого");
            assert_eq!(masculine(pattern, Stem::Soft).genitive, "его");
        }
    }
}
