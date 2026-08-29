// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The short form of an adjective, written from the stem of the full one.
//!
//! School grammar states the endings outright: the masculine is the bare
//! stem, the feminine adds `а`, the neuter `о`, the plural `ы` — `нов`,
//! `нова`, `ново`, `новы` — and a soft stem writes the same endings as `я`,
//! `е`, `и`, keeping its softness in the bare masculine with `ь`: `синь`,
//! `синя`, `сине`, `сини`. The letters then bend exactly as they bend in the
//! rest of the declension, by § 1 and § 2 of the 1956 code — `хороши`, not
//! `хорошы` — so the bending is not restated here but asked of
//! [`spelling::fitted`].
//!
//! The bare masculine may close on two consonants the full form's ending kept
//! apart from the end of the word. Before the `н` and `к` of the suffixes
//! `-н-` and `-к-` Russian parts them with the fleeting vowel of
//! [`spelling::parted`] — `красен`, `важен`, `резок` — and a soft sign
//! standing there gives way to the same vowel: `го́рек`, `дово́лен`. Any other
//! final pair is refused, because it may stand whole — `добр`, `храбр`,
//! `мёртв` — or take a vowel of its own — `хитёр`, `остёр`, `зол` — and
//! which of the two a stem does is stated by Zaliznyak's dictionary word by
//! word, not by the letters. The masculines the dictionary marks off the
//! rule's vowel — `по́лон`, `со́лон`, `досто́ин` — are refused by the same
//! silence.
//!
//! The lemma's `ё` is not kept: `ё` is a stressed letter and nothing else,
//! and the short form does not hold the dictionary form's stress — `тёмный`
//! says `темна́`, `зелёный` says `зе́лен`. The derived stem writes `е`, the
//! letter § 10 of the 1956 code admits everywhere.
//!
//! What the core cannot derive stays unstated. A relational adjective has no
//! short form at all. The neuter after a sibilant hangs its letter on the
//! stress — `хорошо́` under it against `ры́же` off it, § 4 — and the
//! dictionary form states the stress only for the endings of the full
//! paradigm, so that cell is refused rather than guessed.

use super::{ending_stressed, parted, relational, undotted};
use crate::{
    alphabet::is_consonant,
    grammar::{Gender, declension::spelling, form::Bare, stem::Stem}
};

/// The adjectives whose bare masculine the dictionary marks off the rule.
///
/// Zaliznyak states `по́лон`, `со́лон`, `досто́ин`, `недосто́ин` — `о` and `и`
/// where the fleeting vowel's rule writes `е`. The list is the dictionary's
/// own enumeration of the exception, not a tuning, and the vowel is each
/// entry's fact, so the cell is refused rather than written wrong.
const EXCEPTED: &[&str] = &["полный", "солёный", "достойный", "недостойный"];

/// The short form written in the cell asked for, when the core can spell it.
///
/// A cell the core cannot derive answers with nothing: the whole paradigm of
/// a relational adjective, the neuter whose letter hangs on an unstated
/// stress, the masculine whose closing pair only the dictionary can part, and
/// the common gender, which resolves by context and holds no row of its own,
/// [`Gender::STATED`].
///
/// # Examples
///
/// ```
/// use rusem::grammar::{Gender, declension::adjective::short, form::Bare};
///
/// let masculine = short::written("красный", Bare::Singular(Gender::Masculine));
/// assert_eq!(masculine.as_deref(), Some("красен"));
///
/// let plural = short::written("строгий", Bare::Plural);
/// assert_eq!(plural.as_deref(), Some("строги"));
///
/// assert_eq!(short::written("русский", Bare::Plural), None);
/// ```
#[must_use]
pub fn written(dictionary: &str, cell: Bare) -> Option<String> {
    let (base, shape) = parted(dictionary)?;
    if relational(&base) {
        return None;
    }
    let base = undotted(&base);
    let stressed = ending_stressed(dictionary);

    match cell {
        Bare::Singular(Gender::Masculine) => {
            if EXCEPTED.contains(&dictionary) {
                return None;
            }

            masculine(&base, shape, stressed)
        }
        Bare::Singular(Gender::Feminine) => Some(joined(&base, feminine(shape), stressed)),
        Bare::Singular(Gender::Neuter) => neuter(&base, shape, stressed),
        Bare::Singular(Gender::Common) => None,
        Bare::Plural => Some(joined(&base, plural(shape), stressed))
    }
}

/// The bare masculine, refused where only the dictionary could spell it.
///
/// A soft stem left whole keeps its softness in `ь` — `синь` — the way a
/// third-declension noun writes a soft stem before a zero ending. A stem the
/// vowel parts ends on the bare consonant and needs no sign: `искренен`.
fn masculine(base: &str, shape: Stem, stressed: bool) -> Option<String> {
    if fleeting(base) {
        return Some(spelling::parted(base, stressed));
    }
    if let Some(held) = unsigned(base) {
        return Some(held);
    }
    if closed(base) {
        return None;
    }
    if shape.is_soft() {
        return Some(String::from(base) + "ь");
    }

    Some(base.to_owned())
}

/// Reports whether the bare stem calls the fleeting vowel in.
///
/// Two consonants close the stem and the last of them is the `н` or `к` of
/// the suffixes `-н-` and `-к-`: `красен`, `важен`, `резок`. The full form's
/// ending kept that pair from closing the word, and only there does the bare
/// form part it.
fn fleeting(base: &str) -> bool {
    let mut letters = base.chars().rev();
    let Some(last) = letters.next() else {
        return false;
    };
    let Some(before) = letters.next() else {
        return false;
    };

    matches!(last, 'н' | 'к') && is_consonant(before)
}

/// The bare masculine of a stem whose soft sign yields to the vowel.
///
/// The sign cannot stand between the parted consonants, and the vowel that
/// parts them takes its place: `го́рек` and `дово́лен` against the letters'
/// `горьк` and `довольн`. The vowel is written `е` — the writing § 10 admits
/// even where the word sounds it stressed, `силён` beside `си́лен` both
/// standing in the letters `силен`.
fn unsigned(base: &str) -> Option<String> {
    let mut letters = base.chars().rev();
    let last = letters.next()?;
    if !matches!(last, 'н' | 'к') || letters.next()? != 'ь' {
        return None;
    }

    let mut held: String = base.chars().take(base.chars().count() - 2).collect();
    held.push('е');
    held.push(last);
    Some(held)
}

/// Reports whether the stem closes on a pair only the dictionary can part.
///
/// Off the suffixes' `н` and `к` a final pair may stand whole — `добр`,
/// `храбр`, `мёртв` — or take a vowel the letters do not state: `хитёр`,
/// `остёр`, `зол`. Zaliznyak's dictionary says which word does which, one
/// entry at a time, so the cell answers nothing.
fn closed(base: &str) -> bool {
    let mut letters = base.chars().rev();
    let Some(last) = letters.next() else {
        return false;
    };
    let Some(before) = letters.next() else {
        return false;
    };

    is_consonant(last) && (is_consonant(before) || before == 'ь')
}

/// The neuter, refused where its letter hangs on a stress the core lacks.
///
/// Unstressed `о` does not stand after a sibilant or `ц` (§ 4, § 18), and
/// the short form does not keep the full form's stress: `хоро́ший` writes
/// `хорошо́` and `ры́жий` writes `ры́же`. Off the ending stress the dictionary
/// form states, the letter cannot be picked, and the cell stays unstated.
fn neuter(base: &str, shape: Stem, stressed: bool) -> Option<String> {
    let last = base.chars().last()?;
    if !stressed && spelling::bars_unstressed_o(last) {
        return None;
    }

    let ending = match shape {
        Stem::Hard => "о",
        Stem::Soft => "е"
    };
    Some(joined(base, ending, stressed))
}

/// The feminine ending, by the shape of the stem.
const fn feminine(shape: Stem) -> &'static str {
    match shape {
        Stem::Hard => "а",
        Stem::Soft => "я"
    }
}

/// The plural ending, by the shape of the stem.
const fn plural(shape: Stem) -> &'static str {
    match shape {
        Stem::Hard => "ы",
        Stem::Soft => "и"
    }
}

/// A stem joined to its ending, once the alphabet has bent it.
fn joined(base: &str, ending: &str, stressed: bool) -> String {
    String::from(base) + &spelling::fitted(base, ending, stressed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cell(gender: Gender) -> Bare {
        Bare::Singular(gender)
    }

    #[test]
    fn a_bare_stem_takes_the_nominal_endings() {
        assert_eq!(
            written("новый", cell(Gender::Masculine)).as_deref(),
            Some("нов")
        );
        assert_eq!(
            written("новый", cell(Gender::Feminine)).as_deref(),
            Some("нова")
        );
        assert_eq!(
            written("новый", cell(Gender::Neuter)).as_deref(),
            Some("ново")
        );
        assert_eq!(written("новый", Bare::Plural).as_deref(), Some("новы"));
    }

    #[test]
    fn a_pair_closed_by_the_suffix_is_parted() {
        assert_eq!(
            written("красный", cell(Gender::Masculine)).as_deref(),
            Some("красен")
        );
        assert_eq!(
            written("важный", cell(Gender::Masculine)).as_deref(),
            Some("важен")
        );
        assert_eq!(
            written("резкий", cell(Gender::Masculine)).as_deref(),
            Some("резок")
        );
    }

    #[test]
    fn a_pair_off_the_suffixes_is_refused_not_guessed() {
        for held in ["храбрый", "добрый", "хитрый", "острый", "злой"]
        {
            assert_eq!(written(held, cell(Gender::Masculine)), None, "{held}");
        }
        assert_eq!(
            written("хитрый", cell(Gender::Feminine)).as_deref(),
            Some("хитра")
        );
    }

    #[test]
    fn a_soft_sign_before_the_suffix_yields_to_the_vowel() {
        assert_eq!(
            written("горький", cell(Gender::Masculine)).as_deref(),
            Some("горек")
        );
        assert_eq!(
            written("довольный", cell(Gender::Masculine)).as_deref(),
            Some("доволен")
        );
    }

    #[test]
    fn a_masculine_the_dictionary_marks_off_the_rule_is_refused() {
        for held in ["полный", "солёный", "достойный", "недостойный"]
        {
            assert_eq!(written(held, cell(Gender::Masculine)), None, "{held}");
        }
        assert_eq!(
            written("полный", cell(Gender::Feminine)).as_deref(),
            Some("полна")
        );
    }

    #[test]
    fn the_lemmas_yo_is_written_e_off_its_stress() {
        assert_eq!(
            written("тёмный", cell(Gender::Masculine)).as_deref(),
            Some("темен")
        );
        assert_eq!(
            written("тёмный", cell(Gender::Feminine)).as_deref(),
            Some("темна")
        );
        assert_eq!(
            written("зелёный", cell(Gender::Masculine)).as_deref(),
            Some("зелен")
        );
        assert_eq!(written("весёлый", Bare::Plural).as_deref(), Some("веселы"));
    }

    #[test]
    fn a_soft_stem_keeps_its_softness_in_the_bare_masculine() {
        assert_eq!(
            written("синий", cell(Gender::Masculine)).as_deref(),
            Some("синь")
        );
        assert_eq!(
            written("синий", cell(Gender::Feminine)).as_deref(),
            Some("синя")
        );
        assert_eq!(written("синий", Bare::Plural).as_deref(), Some("сини"));
    }

    #[test]
    fn the_neuter_after_a_sibilant_is_unstated_off_the_ending_stress() {
        assert_eq!(written("хороший", cell(Gender::Neuter)), None);
        assert_eq!(
            written("смешной", cell(Gender::Neuter)).as_deref(),
            Some("смешно")
        );
    }

    #[test]
    fn a_relational_adjective_has_no_short_form() {
        for gender in Gender::STATED {
            assert_eq!(written("русский", cell(gender)), None);
        }
        assert_eq!(written("русский", Bare::Plural), None);
    }
}
