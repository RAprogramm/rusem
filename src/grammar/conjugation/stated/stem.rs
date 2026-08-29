// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The present stem each class derives from the infinitive, where one does.
//!
//! The class table of `Викисловарь:Шаблоны словоизменений/Глаголы` states
//! the derivation for every class, and this module writes out the ones the
//! index alone completes. Where the table itself says a stem needs the
//! dictionary's own note — the hidden consonant of `-сти` in class 7, the
//! velar of class 8, the nasal of class 14, the inserted vowel of `6°` and
//! of every starred index — no stem is derived, and the answer is [`None`].

use crate::grammar::conjugation::{
    class::Class,
    index::{VerbIndex, kind::Kind},
    listed, stems
};

/// The present stem the index derives, or nothing where it needs the
/// dictionary's note.
pub(super) fn present(infinitive: &str, index: VerbIndex) -> Option<String> {
    if index.signs.fleeting || (index.signs.ringed && !matches!(index.kind, Kind::Three)) {
        return None;
    }

    match index.kind {
        Kind::One => stems::present(infinitive, Class::Glided),
        Kind::Two => stems::present(infinitive, Class::Suffixed),
        Kind::Three => stems::present(infinitive, Class::Dropped),
        Kind::Four | Kind::Five => stems::present(infinitive, Class::Bare),
        Kind::Six => sixth(infinitive),
        Kind::Seven => seventh(infinitive),
        Kind::Nine => grown(infinitive, "ереть", 'р'),
        Kind::Ten => tenth(infinitive),
        Kind::Eleven => grown(infinitive, "ить", 'ь'),
        Kind::Twelve => twelfth(infinitive),
        Kind::Thirteen => grown(infinitive, "вать", 'й'),
        Kind::Fifteen => grown(infinitive, "ть", 'н'),
        Kind::Sixteen => grown(infinitive, "ть", 'в'),
        Kind::Eight | Kind::Fourteen | Kind::Isolated => None
    }
}

/// The infinitive with its class's tail cut and the class's letter grown:
/// `тереть` to `тр`, `бить` to `бь`, `давать` to `дай`, `стать` to `стан`,
/// `жить` to `жив`.
fn grown(infinitive: &str, tail: &str, letter: char) -> Option<String> {
    infinitive.strip_suffix(tail).map(|head| {
        let mut held = String::from(head);
        held.push(letter);
        held
    })
}

/// The class-6 stem: the vowel cut and the swap applied through the present.
///
/// The class table states the same standard alternation classes 4 and 5
/// apply in their first person alone, applied here to every cell — `писать,
/// пишу, пишешь` — which is why the one statement of that step,
/// [`stems::first_person`], is asked for it. A stem left in a vowel is the
/// `-еять`, `-аять` kind that keeps its glide instead — `лаять, лаю` — and
/// takes `й` the way class 1 does.
fn sixth(infinitive: &str) -> Option<String> {
    let bare = stems::present(infinitive, Class::Bare)?;
    if bare.chars().last().is_some_and(crate::alphabet::is_vowel) {
        let mut held = bare;
        held.push('й');
        return Some(held);
    }

    Some(stems::first_person(&bare, Class::Bare))
}

/// The class-7 stem, derived only where the infinitive shows it.
///
/// `-зти` and `-зть` keep their consonant — `везти, везу`; `лезть, лезу` —
/// and the cut is the stem. `-сти` and `-сть` hide it: `нести` holds `нес-`
/// but `вести` holds `вед-` and `мести` holds `мет-`, and the class table
/// says the dictionary gives the consonant as its own note, so no stem is
/// derived for them.
fn seventh(infinitive: &str) -> Option<String> {
    stems::cut(infinitive).filter(|held| held.ends_with('з'))
}

/// The class-10 stem: the infinitive without `-оть`, and the one stem the
/// class table itself states aside — `молоть` conjugates on `мел-`.
fn tenth(infinitive: &str) -> Option<String> {
    if listed(infinitive, &["молоть"]) {
        return infinitive
            .strip_suffix("олоть")
            .map(|head| String::from(head) + "ел");
    }

    infinitive.strip_suffix("оть").map(String::from)
}

/// The class-12 stem: the infinitive's vowel traded before the glide.
///
/// The class table states the trade vowel by vowel — `-ыть` to `-ою` (`мыть,
/// мою`), `-уть` to `-ую` (`дуть, дую`), `-ить` to `-ию` (`гнить, гнию`) —
/// and lists the three verbs in `-еть` beside it by name: `греть, грею`,
/// `петь, пою`, `брить, брею`. A shape outside that statement derives
/// nothing.
fn twelfth(infinitive: &str) -> Option<String> {
    for (verb, stem) in [("петь", "по"), ("брить", "бре"), ("греть", "гре")]
    {
        if listed(infinitive, &[verb]) {
            return infinitive
                .strip_suffix(verb)
                .map(|head| String::from(head) + stem + "й");
        }
    }

    let held = stems::cut(infinitive)?;
    let last = held.chars().last()?;
    let traded = match last {
        'ы' => 'о',
        'у' | 'и' => last,
        _ => return None
    };

    let mut grown: String = held.chars().take(held.chars().count() - 1).collect();
    grown.push(traded);
    grown.push('й');
    Some(grown)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::conjugation::index;

    fn stem(infinitive: &str, written: &str) -> Option<String> {
        let held = index::read(written).unwrap_or_else(|| unreachable!("a stated index"));
        present(infinitive, held)
    }

    #[test]
    fn the_derivable_classes_derive_their_stems() {
        assert_eq!(stem("читать", "1a").as_deref(), Some("читай"));
        assert_eq!(stem("рисовать", "2a").as_deref(), Some("рису"));
        assert_eq!(stem("толкнуть", "3b").as_deref(), Some("толкн"));
        assert_eq!(stem("сохнуть", "3°a((5))(6)").as_deref(), Some("сохн"));
        assert_eq!(stem("говорить", "4b").as_deref(), Some("говор"));
        assert_eq!(stem("слышать", "5a").as_deref(), Some("слыш"));
        assert_eq!(stem("тереть", "9b").as_deref(), Some("тр"));
        assert_eq!(stem("давать", "13b").as_deref(), Some("дай"));
        assert_eq!(stem("стать", "15a").as_deref(), Some("стан"));
        assert_eq!(stem("жить", "16b/c").as_deref(), Some("жив"));
    }

    #[test]
    fn the_sixth_class_swaps_or_keeps_its_glide() {
        assert_eq!(stem("писать", "6c").as_deref(), Some("пиш"));
        assert_eq!(stem("плакать", "6a").as_deref(), Some("плач"));
        assert_eq!(stem("искать", "6c").as_deref(), Some("ищ"));
        assert_eq!(stem("сыпать", "6a").as_deref(), Some("сыпл"));
        assert_eq!(stem("лаять", "6a").as_deref(), Some("лай"));
        assert_eq!(stem("смеять", "6b").as_deref(), Some("смей"));
    }

    #[test]
    fn the_seventh_class_derives_only_what_its_spelling_shows() {
        assert_eq!(stem("везти", "7b/b").as_deref(), Some("вез"));
        assert_eq!(stem("лезть", "7a").as_deref(), Some("лез"));
        assert_eq!(stem("нести", "7b/b"), None);
        assert_eq!(stem("вести", "7b/b"), None);
    }

    #[test]
    fn the_vowel_trading_classes_trade_as_the_table_states() {
        assert_eq!(stem("мыть", "12a").as_deref(), Some("мой"));
        assert_eq!(stem("дуть", "12a").as_deref(), Some("дуй"));
        assert_eq!(stem("гнить", "12b/c").as_deref(), Some("гний"));
        assert_eq!(stem("петь", "12b").as_deref(), Some("пой"));
        assert_eq!(stem("запеть", "12b").as_deref(), Some("запой"));
        assert_eq!(stem("брить", "12a").as_deref(), Some("брей"));
        assert_eq!(stem("бить", "11b").as_deref(), Some("бь"));
        assert_eq!(stem("колоть", "10c").as_deref(), Some("кол"));
        assert_eq!(stem("молоть", "10c").as_deref(), Some("мел"));
        assert_eq!(stem("помолоть", "10c").as_deref(), Some("помел"));
    }

    #[test]
    fn what_needs_the_dictionarys_note_derives_nothing() {
        assert_eq!(stem("печь", "8b/b"), None);
        assert_eq!(stem("жать", "14b"), None);
        assert_eq!(stem("звать", "6°b/c"), None);
        assert_eq!(stem("втереть", "9*b"), None);
        assert_eq!(stem("дать", "^b/c'"), None);
    }
}
