// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Word formation: how a word is built and what it is built from.
//!
//! Russian states a large part of its meaning in structure. `перестройка` is
//! not an opaque string: it is `пере-` over `строй` under `-к-а`, motivated by
//! `перестроить`, which is motivated by `строить`. A reader who knows the
//! morphemes can read a word met for the first time, and so can a machine —
//! which is exactly what this layer gives it.
//!
//! Two uses follow. A form no dictionary lists can still be explained instead
//! of guessed at. And a claimed meaning can be checked against the structure:
//! a word built from `строить` does not get to mean something unrelated to
//! building without the engine saying so.

use std::{string::String, vec::Vec};

use crate::{
    error::{CoreError, Result},
    id::{LemmaId, MorphemeId},
    morphology::WordForm
};

/// The role a morpheme plays inside a word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum MorphemeKind {
    /// Prefix, standing before the root.
    Prefix,
    /// Root, carrying the lexical meaning.
    Root,
    /// Linking vowel between two roots.
    Interfix,
    /// Suffix, standing after the root.
    Suffix,
    /// Inflectional ending.
    Ending,
    /// Postfix, standing after the ending, such as `-ся` or `-то`.
    Postfix
}

impl MorphemeKind {
    /// Reports whether the morpheme contributes lexical, rather than purely
    /// grammatical, meaning.
    #[must_use]
    pub const fn is_lexical(self) -> bool {
        matches!(
            self,
            Self::Prefix | Self::Root | Self::Suffix | Self::Postfix
        )
    }
}

/// What an inventory knows about a morpheme, beyond how it is written here.
///
/// Held apart from the morpheme because it is a different kind of fact. The
/// role and the written shape are read off the word; these are looked up, and
/// a morpheme recovered by rule has none of them.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Filed {
    /// Identity in the inventory.
    pub id:        Option<MorphemeId>,
    /// The invariant shape across the allomorphs, such as `бер` for `бир`.
    pub invariant: Option<String>,
    /// What the morpheme contributes to the meaning, in one phrase, as stated
    /// by the source that registered it.
    pub gloss:     Option<String>
}

impl Filed {
    /// Reports whether the inventory knows nothing about this morpheme.
    #[must_use]
    #[inline]
    pub const fn is_unknown(&self) -> bool {
        self.id.is_none() && self.invariant.is_none() && self.gloss.is_none()
    }
}

/// An entry of the morpheme inventory.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Morpheme {
    /// The role the morpheme plays.
    pub kind:  MorphemeKind,
    /// The written shape as it appears in this word.
    pub text:  String,
    /// What the inventory knows about it, empty for one recovered by rule.
    pub filed: Filed
}

/// One morpheme placed inside a concrete word form.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Segment {
    /// The morpheme occupying this stretch of the form.
    pub morpheme: Morpheme,
    /// Index of the first character of the stretch.
    pub start:    usize,
    /// Number of characters in the stretch, zero for a null ending.
    pub len:      usize
}

/// A word form cut into morphemes.
///
/// The invariant is that the segments tile the form: they are ordered, they do
/// not overlap, and they cover every character exactly once. A segmentation
/// that does not tile is a bug in an adapter, not a weaker hypothesis, so it is
/// rejected at construction.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "Tiling", try_from = "Tiling"))]
pub struct Segmentation {
    tiling: Tiling
}

/// A word form and the segments over it, before either is checked.
///
/// The shape a segmentation is stored in and read back from. It exists so
/// that the checked type and the wire form are one declaration rather than
/// two that must be kept alike.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tiling {
    /// The form the segments cover.
    pub form:     WordForm,
    /// The segments, in the order they stand.
    pub segments: Vec<Segment>
}

#[cfg(feature = "serde")]
impl From<Segmentation> for Tiling {
    fn from(held: Segmentation) -> Self {
        held.tiling
    }
}

#[cfg(feature = "serde")]
impl TryFrom<Tiling> for Segmentation {
    type Error = CoreError;

    fn try_from(held: Tiling) -> Result<Self> {
        Self::tiling(held.form, held.segments)
    }
}

impl Segmentation {
    /// Builds a segmentation from segments that tile the form.
    ///
    /// Named for what it checks rather than called `new`: this does not assign
    /// fields, it refuses a cut that leaves a gap, overlaps, misses a letter or
    /// finds no root. A caller that gets one back has a tiling and not merely
    /// a list.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::BrokenSegmentation`] when the segments leave a gap,
    /// overlap, run past the end of the form, or contain no root.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::{
    ///     morphemics::derivation::{Morpheme, MorphemeKind, Segment, Segmentation},
    ///     morphology::WordForm
    /// };
    ///
    /// fn segment(kind: MorphemeKind, text: &str, start: usize) -> Segment {
    ///     Segment {
    ///         morpheme: Morpheme {
    ///             kind,
    ///             text: text.to_owned(),
    ///             filed: Default::default()
    ///         },
    ///         start,
    ///         len: text.chars().count()
    ///     }
    /// }
    ///
    /// let form = WordForm::parse("столик")?;
    /// let cut = Segmentation::tiling(
    ///     form,
    ///     vec![
    ///         segment(MorphemeKind::Root, "стол", 0),
    ///         segment(MorphemeKind::Suffix, "ик", 4),
    ///         segment(MorphemeKind::Ending, "", 6),
    ///     ]
    /// )?;
    ///
    /// assert_eq!(cut.root().map(|root| root.text.as_str()), Some("стол"));
    /// # Ok::<(), rusem::error::CoreError>(())
    /// ```
    pub fn tiling(form: WordForm, segments: Vec<Segment>) -> Result<Self> {
        let mut cursor = 0_usize;
        for segment in &segments {
            if segment.start != cursor {
                return Err(CoreError::BrokenSegmentation {
                    reason: "segments leave a gap or overlap"
                });
            }
            cursor = cursor.saturating_add(segment.len);
        }

        if cursor != form.char_len() {
            return Err(CoreError::BrokenSegmentation {
                reason: "segments do not cover the whole form"
            });
        }

        if !segments
            .iter()
            .any(|segment| segment.morpheme.kind == MorphemeKind::Root)
        {
            return Err(CoreError::BrokenSegmentation {
                reason: "no root"
            });
        }

        Ok(Self {
            tiling: Tiling {
                form,
                segments
            }
        })
    }

    /// The form that was cut.
    #[must_use]
    pub const fn form(&self) -> &WordForm {
        &self.tiling.form
    }

    /// The segments, in the order they appear in the form.
    #[must_use]
    pub fn segments(&self) -> &[Segment] {
        &self.tiling.segments
    }
}

/// What a segmentation says about the word it cut.
///
/// Held apart from the checking above: building a segmentation is verifying a
/// tiling, and reading one is asking questions of a word. The two are
/// different jobs, and the questions grow while the checking does not.
impl Segmentation {
    /// The root of the word, when the cut found one.
    #[must_use]
    pub fn root(&self) -> Option<&Morpheme> {
        self.segments()
            .iter()
            .map(|segment| &segment.morpheme)
            .find(|morpheme| morpheme.kind == MorphemeKind::Root)
    }

    /// Reports whether the word holds more than one root.
    ///
    /// A compound is built of two words rather than of a word and an affix, so
    /// the layers above treat it differently.
    #[must_use]
    pub fn is_compound(&self) -> bool {
        self.segments()
            .iter()
            .filter(|segment| segment.morpheme.kind == MorphemeKind::Root)
            .count()
            > 1
    }

    /// The stem: the word without its ending.
    ///
    /// This is what inflection acts on, so it is asked for often enough to be
    /// worth naming.
    #[must_use]
    pub fn stem(&self) -> String {
        self.segments()
            .iter()
            .filter(|segment| segment.morpheme.kind != MorphemeKind::Ending)
            .map(|segment| segment.morpheme.text.as_str())
            .collect()
    }
}

/// The operation that produced a word from its motivating word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum DerivationWay {
    /// A prefix was added: `строить` to `перестроить`.
    Prefixation,
    /// A suffix was added: `стол` to `столик`.
    Suffixation,
    /// A prefix and a suffix were added at once: `окно` to `подоконник`.
    PrefixSuffixation,
    /// A postfix was added: `мыть` to `мыться`.
    Postfixation,
    /// Two stems were joined: `пар` and `ходить` to `пароход`.
    Compounding,
    /// The stem was cut: `заместитель` to `зам`.
    Truncation,
    /// Initials were joined: `высшее учебное заведение` to `вуз`.
    Abbreviation,
    /// The part of speech changed with no affix: `столовая` from the adjective.
    Conversion
}

/// One step from a motivating word to the word it motivates.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DerivationStep {
    /// The motivating word.
    pub from:    Word,
    /// The motivated word.
    pub to:      Word,
    /// How the step was performed.
    pub way:     DerivationWay,
    /// The affixes the step added.
    pub affixes: Vec<Morpheme>
}

/// A word, with its identity when the store holds one.
///
/// A step names two words and each is named the same way, so the pair is one
/// concept rather than two fields repeated.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Word {
    /// How the word is written.
    pub form: WordForm,
    /// Identity in the store, absent for a word not filed there.
    pub id:   Option<LemmaId>
}

impl Word {
    /// A word the store does not hold.
    #[must_use]
    #[inline]
    pub const fn unfiled(form: WordForm) -> Self {
        Self {
            form,
            id: None
        }
    }
}

/// The chain from an underived word down to the word in question.
///
/// The first step starts at a word that nothing motivates — the top of the
/// nest. Reading the chain is reading the word's biography, and the meaning of
/// the last word is expected to be a function of the first one and the steps
/// between.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DerivationChain {
    /// The underived word the chain starts from.
    pub base:  WordForm,
    /// The steps, in the order they applied.
    pub steps: Vec<DerivationStep>
}

impl DerivationChain {
    /// A chain of an underived word: the word itself, and nothing before it.
    #[must_use]
    pub const fn underived(base: WordForm) -> Self {
        Self {
            base,
            steps: Vec::new()
        }
    }

    /// The word the chain arrives at.
    #[must_use]
    pub fn head(&self) -> &WordForm {
        self.steps.last().map_or(&self.base, |step| &step.to.form)
    }

    /// Number of derivation steps between the base and the head.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.steps.len()
    }

    /// Reports whether nothing motivates the word.
    #[must_use]
    pub const fn is_underived(&self) -> bool {
        self.steps.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn segment(kind: MorphemeKind, text: &str, start: usize) -> Segment {
        Segment {
            morpheme: Morpheme {
                kind,
                text: String::from(text),
                filed: Filed::default()
            },
            start,
            len: text.chars().count()
        }
    }

    fn form(text: &str) -> WordForm {
        WordForm::parse(text).expect("valid form")
    }

    #[test]
    fn tiling_segmentation_is_accepted() {
        let cut = Segmentation::tiling(
            form("перестройка"),
            std::vec![
                segment(MorphemeKind::Prefix, "пере", 0),
                segment(MorphemeKind::Root, "строй", 4),
                segment(MorphemeKind::Suffix, "к", 9),
                segment(MorphemeKind::Ending, "а", 10),
            ]
        )
        .expect("tiling segmentation");

        assert_eq!(cut.root().map(|root| root.text.as_str()), Some("строй"));
        assert_eq!(cut.stem(), "перестройк");
        assert!(!cut.is_compound());
    }

    #[test]
    fn null_ending_is_allowed() {
        let cut = Segmentation::tiling(
            form("стол"),
            std::vec![
                segment(MorphemeKind::Root, "стол", 0),
                segment(MorphemeKind::Ending, "", 4),
            ]
        )
        .expect("tiling segmentation");

        assert_eq!(cut.segments().len(), 2);
        assert_eq!(cut.stem(), "стол");
    }

    #[test]
    fn a_lexical_morpheme_is_named_as_one() {
        assert!(MorphemeKind::Root.is_lexical());
        assert!(MorphemeKind::Prefix.is_lexical());
        assert!(MorphemeKind::Suffix.is_lexical());
        assert!(!MorphemeKind::Ending.is_lexical());
    }

    #[test]
    fn a_morpheme_no_inventory_knows_says_so() {
        assert!(Filed::default().is_unknown());
        assert!(
            !Filed {
                gloss: Some(String::from("over")),
                ..Filed::default()
            }
            .is_unknown()
        );
    }

    #[test]
    fn compound_reports_two_roots() {
        let cut = Segmentation::tiling(
            form("пароход"),
            std::vec![
                segment(MorphemeKind::Root, "пар", 0),
                segment(MorphemeKind::Interfix, "о", 3),
                segment(MorphemeKind::Root, "ход", 4),
                segment(MorphemeKind::Ending, "", 7),
            ]
        )
        .expect("tiling segmentation");

        assert!(cut.is_compound());
    }

    #[test]
    fn gap_is_rejected() {
        let broken = Segmentation::tiling(
            form("столик"),
            std::vec![
                segment(MorphemeKind::Root, "стол", 0),
                segment(MorphemeKind::Suffix, "ик", 5),
            ]
        );

        assert!(broken.is_err());
    }

    #[test]
    fn short_cover_is_rejected() {
        let broken = Segmentation::tiling(
            form("столик"),
            std::vec![segment(MorphemeKind::Root, "стол", 0)]
        );

        assert!(broken.is_err());
    }

    #[test]
    fn rootless_cut_is_rejected() {
        let broken = Segmentation::tiling(
            form("пере"),
            std::vec![segment(MorphemeKind::Prefix, "пере", 0)]
        );

        assert!(broken.is_err());
    }

    #[test]
    fn chain_head_is_the_last_step() {
        let chain = DerivationChain {
            base:  form("строить"),
            steps: std::vec![
                DerivationStep {
                    from:    Word::unfiled(form("строить")),
                    to:      Word::unfiled(form("перестроить")),
                    way:     DerivationWay::Prefixation,
                    affixes: Vec::new()
                },
                DerivationStep {
                    from:    Word::unfiled(form("перестроить")),
                    to:      Word::unfiled(form("перестройка")),
                    way:     DerivationWay::Suffixation,
                    affixes: Vec::new()
                },
            ]
        };

        assert_eq!(chain.head().as_str(), "перестройка");
        assert_eq!(chain.depth(), 2);
        assert!(!chain.is_underived());
    }

    #[test]
    fn underived_chain_heads_at_its_base() {
        let chain = DerivationChain::underived(form("строить"));

        assert!(chain.is_underived());
        assert_eq!(chain.head().as_str(), "строить");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn deserialization_re_checks_the_tiling() {
        let sound = Segmentation::tiling(
            form("столик"),
            std::vec![
                segment(MorphemeKind::Root, "стол", 0),
                segment(MorphemeKind::Suffix, "ик", 4),
            ]
        )
        .expect("a tiling cut");
        let text = serde_json::to_string(&sound).expect("serializable");

        let restored: Segmentation = serde_json::from_str(&text).expect("a tiling cut");
        assert_eq!(restored, sound);

        let gapped = text.replace(r#""start":4"#, r#""start":5"#);
        assert!(serde_json::from_str::<Segmentation>(&gapped).is_err());
    }
}
