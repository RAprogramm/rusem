// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The pronoun forms a gate meets written out.
//!
//! [`super`] holds dictionary forms, which is what a lemma is compared
//! against. A gate reads the word as it stands on the page, and `того` is on
//! the page far more often than `тот`. Listing the forms is what lets it ask
//! its question without putting the whole analyzer between itself and an
//! answer.
//!
//! Only three classes are here, and only because a gate asks for them. The
//! rest decline as adjectives and morphology reaches them by rule.

/// The forms `этот` and `тот` take.
///
/// Listed whole rather than reached by rule: a gate matches a written word,
/// and asking morphology for a lemma before asking whether the word points at
/// something would put the whole analyzer between a gate and a list of seven
/// words.
pub const DEMONSTRATIVE_FORMS: &[&str] = &[
    "та",
    "те",
    "тем",
    "теми",
    "тех",
    "то",
    "того",
    "той",
    "том",
    "тому",
    "тот",
    "тою",
    "ту",
    "эта",
    "эти",
    "этим",
    "этими",
    "этих",
    "это",
    "этого",
    "этой",
    "этом",
    "этому",
    "этот",
    "эту",
    "этою"
];

/// The forms the possessive pronouns take.
pub const POSSESSIVE_FORMS: &[&str] = &[
    "ваш",
    "ваша",
    "ваше",
    "ваши",
    "вашего",
    "вашей",
    "вашем",
    "вашему",
    "вашим",
    "вашими",
    "ваших",
    "вашу",
    "его",
    "её",
    "их",
    "моего",
    "моей",
    "моем",
    "моём",
    "моему",
    "мои",
    "моими",
    "моих",
    "мой",
    "моим",
    "моя",
    "моё",
    "наш",
    "наша",
    "наше",
    "наши",
    "нашего",
    "нашей",
    "нашем",
    "нашему",
    "нашим",
    "нашими",
    "наших",
    "нашу",
    "своего",
    "своей",
    "своем",
    "своём",
    "своему",
    "свои",
    "своим",
    "своими",
    "своих",
    "свой",
    "своя",
    "своё",
    "твоего",
    "твоей",
    "твоем",
    "твоём",
    "твоему",
    "твои",
    "твоим",
    "твоими",
    "твоих",
    "твой",
    "твоя",
    "твоё"
];

/// The forms `кто` and `что` take, with the asking words that decline by rule.
///
/// `кто` and `что` are irregular — `кого`, `чего`, `чем` — and no rule reaches
/// them from the lemma.
pub const ASKING_FORMS: &[&str] = &[
    "каков",
    "какой",
    "кем",
    "ком",
    "кого",
    "кому",
    "который",
    "кто",
    "сколько",
    "чего",
    "чей",
    "чем",
    "чему",
    "что",
    "чём"
];

/// Reports whether a written word points, in any form it takes.
///
/// Wider than [`super::classes`], which reads dictionary forms: `того` and
/// `этим` point as surely as `тот` and `этот` do.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::pronoun::points_in_any_form;
///
/// assert!(points_in_any_form("тот"));
/// assert!(points_in_any_form("того"));
/// assert!(!points_in_any_form("мой"));
/// ```
#[must_use]
pub fn points_in_any_form(written: &str) -> bool {
    DEMONSTRATIVE_FORMS.contains(&written.to_lowercase().as_str())
}

/// Reports whether a written word says whose, in any form it takes.
#[must_use]
pub fn possesses_in_any_form(written: &str) -> bool {
    POSSESSIVE_FORMS.contains(&written.to_lowercase().as_str())
}

/// Reports whether a written word asks, in any form it takes.
///
/// # Examples
///
/// ```
/// use rusem::grammar::closed::pronoun::asks_in_any_form;
///
/// assert!(asks_in_any_form("кто"));
/// assert!(asks_in_any_form("кого"));
/// assert!(!asks_in_any_form("стол"));
/// ```
#[must_use]
pub fn asks_in_any_form(written: &str) -> bool {
    ASKING_FORMS.contains(&written.to_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::{
        super::{ASKING, POSSESSIVE},
        *
    };

    #[test]
    fn every_dictionary_form_is_among_the_forms() {
        for held in ["тот", "этот"] {
            assert!(points_in_any_form(held), "{held} points and is not listed");
        }
        for held in POSSESSIVE {
            assert!(
                possesses_in_any_form(held),
                "{held} possesses and is not listed"
            );
        }
        for held in ASKING {
            assert!(asks_in_any_form(held), "{held} asks and is not listed");
        }
    }

    #[test]
    fn a_declined_form_points_as_the_dictionary_form_does() {
        assert!(points_in_any_form("того"));
        assert!(points_in_any_form("этим"));
        assert!(points_in_any_form("ТУ"));
        assert!(!points_in_any_form("мой"));
    }

    #[test]
    fn a_declined_possessive_says_whose() {
        assert!(possesses_in_any_form("моего"));
        assert!(possesses_in_any_form("нашими"));
        assert!(!possesses_in_any_form("тот"));
    }

    #[test]
    fn a_declined_asking_word_asks() {
        assert!(asks_in_any_form("кого"));
        assert!(asks_in_any_form("чем"));
        assert!(!asks_in_any_form("тот"));
    }

    #[test]
    fn the_forms_hold_no_word_twice() {
        for class in [DEMONSTRATIVE_FORMS, POSSESSIVE_FORMS, ASKING_FORMS] {
            let mut held = class.to_vec();
            held.sort_unstable();
            held.dedup();

            assert_eq!(held.len(), class.len(), "a form is listed twice");
        }
    }
}
