// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The ending a stated index puts on a noun.
//!
//! Three paradigms, chosen by how the dictionary form ends and by the index:
//! the first for the nouns in `-а` whatever their gender — `мужчина` and
//! `слуга` are masculine and decline like `книга` — the second for the rest of
//! the masculines and the neuters, the third for the nouns the index writes
//! `8`. Which of the two shapes an ending takes —
//! `-ой` or `-ей`, `-ом` or `-ем`, `-о` or `-е` — is settled by the kind of
//! stem and by whether the stress falls on the ending, and both are known
//! before this is asked.
//!
//! What the alphabet does afterwards is not here. `ы` after `к` and the
//! unstressed `о` after `ц` are the same two rules for every ending in the
//! language, and they are applied once, where the stem and the ending meet.

pub mod first;
pub mod second;
pub mod third;

use crate::grammar::{
    Animacy, Case, Gender, Number,
    declension::index::{
        Index, Kind,
        circled::{Circled, Reach},
        falls
    }
};

/// What the cell's own shape has already settled about the word.
///
/// Three answers that the stem and the scheme give before an ending is chosen:
/// where the stress fell, whether the word declines by the first paradigm, and
/// whether a fleeting vowel has just been put back into the stem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape {
    /// Whether the stress falls on the ending.
    pub stressed: bool,
    /// Whether the noun declines by the first paradigm.
    pub opens:    bool,
    /// Whether a fleeting vowel has been put back into the stem.
    pub parted:   bool
}

/// Reports whether the endings of this kind of stem are the soft ones.
#[must_use]
pub const fn soft(kind: Kind) -> bool {
    matches!(kind, Kind::Soft | Kind::Glide | Kind::Iotated | Kind::Third)
}

/// The ending one cell takes.
///
/// # Examples
///
/// ```
/// use rusem::grammar::{
///     Animacy, Case, Gender, Number,
///     declension::{
///         index,
///         stated::ending::{Shape, of}
///     }
/// };
///
/// let held = index::read("2b").expect("a stated index");
/// let one = of(
///     Gender::Masculine,
///     held,
///     Case::Instrumental,
///     Number::Singular,
///     Animacy::Animate,
///     Shape {
///         stressed: true,
///         opens:    false,
///         parted:   false
///     }
/// );
/// assert_eq!(one, "ём");
/// ```
#[must_use]
pub const fn of(
    gender: Gender,
    index: Index,
    case: Case,
    number: Number,
    animacy: Animacy,
    shape: Shape
) -> &'static str {
    if matches!(index.kind, Kind::Third) {
        return third::of(case, number, animacy);
    }
    if shape.opens {
        return first::of(
            index.kind,
            case,
            number,
            animacy,
            shape.stressed,
            shape.parted
        );
    }
    if traded(index, case, number) {
        return "е";
    }

    second::of(
        crossed(gender, index.circled, case, number, animacy),
        index.kind,
        case,
        number,
        animacy,
        shape.stressed
    )
}

/// Reports whether ③ trades this cell's `-и` for `-е`.
///
/// The numeral speaks of the stems the index writes `7`, whose prepositional
/// singular is `-ии`: `Бабий` is `7a(3)` and its prepositional is `о Баби́е`
/// where `полоний` writes `о поло́нии`.
const fn traded(index: Index, case: Case, number: Number) -> bool {
    matches!(index.kind, Kind::Iotated)
        && matches!(index.circled.prepositional, Some(Reach::Whole))
        && matches!(case.merged(), Case::Prepositional)
        && matches!(number, Number::Singular)
}

/// The gender whose row ① or ② tells this cell to read.
///
/// The circled numerals say a cell goes `по чужому образцу` — by the other
/// pattern. Within the second paradigm the other pattern is the other gender's
/// row of the same table: ① sends the masculine nominative plural to the
/// neuter's `-а́` — `дома́`, `снега́` — and the neuter's to the masculine's
/// `-ы` — `я́блоки`; ② does the same for the genitive plural — `сапо́г`,
/// `глаз` by the neuter's bare stem, and a neuter's `-ов` by the masculine.
///
/// The accusative plural states no row of its own, so it is resolved first to
/// the row it repeats — the same resolution the stress reader makes, and made
/// by the same function, so the two cannot drift. A doubled numeral leaves the
/// row alone: both forms live, and the pattern's own is written.
const fn crossed(
    gender: Gender,
    circled: Circled,
    case: Case,
    number: Number,
    animacy: Animacy
) -> Gender {
    if !matches!(number, Number::Plural) {
        return gender;
    }

    let held = match falls::repeated(case.merged(), number, animacy) {
        Case::Nominative => circled.nominative,
        Case::Genitive => circled.genitive,
        _ => None
    };
    if !matches!(held, Some(Reach::Whole)) {
        return gender;
    }

    match gender {
        Gender::Masculine => Gender::Neuter,
        Gender::Neuter => Gender::Masculine,
        other => other
    }
}
