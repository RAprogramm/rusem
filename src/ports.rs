// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The interfaces the knowledge layers implement.
//!
//! Each port is one question the engine can be asked, stated without any
//! reference to how it is answered. A morphological analyzer, a dictionary
//! import and a thesaurus import can each be replaced wholesale as long as the
//! port holds, and the checker above them never notices.
//!
//! Every port is object safe: the facade owns them behind trait objects, chosen
//! at start-up from what the store actually holds.

use std::{sync::Arc, vec::Vec};

use crate::{
    error::Result,
    evidence::{Evidenced, Source},
    frame::Frame,
    grammar::GrammarTag,
    id::{LemmaId, SenseId, SourceId},
    morphemics::derivation::{DerivationChain, Segmentation},
    morphology::{Analysis, Paradigm, Readings, WordForm},
    relation::{Relation, RelationKind},
    sense::{Idiom, Sense},
    syntax::Tree,
    verdict::Verdict
};

/// Analysis and generation of word forms.
pub trait Morphology {
    /// Returns every reading of a form.
    ///
    /// # Errors
    ///
    /// Returns an error when the underlying dictionary cannot be consulted. A
    /// form the dictionary does not contain is not an error: it yields an empty
    /// reading set.
    fn analyze(&self, form: &WordForm) -> Result<Readings>;

    /// Returns the forms of a word that realize a grammatical description.
    ///
    /// The description is read as a request, not as a filter: categories it
    /// leaves unset are taken from the form that was given.
    ///
    /// # Errors
    ///
    /// Returns an error when the dictionary cannot be consulted. A word the
    /// dictionary cannot inflect yields an empty list.
    fn inflect(&self, form: &WordForm, tag: &GrammarTag) -> Result<Vec<WordForm>>;

    /// Returns the whole paradigm the form belongs to.
    ///
    /// # Errors
    ///
    /// Returns an error when the dictionary cannot be consulted.
    fn paradigm(&self, form: &WordForm) -> Result<Paradigm>;
}

/// The syntactic layer.
///
/// One question, asked of a whole sentence: which word answers to which. The
/// gates above ask it instead of reading position, because position in Russian
/// answers a different question.
pub trait Syntax: core::fmt::Debug {
    /// Builds the dependency tree of an analyzed sentence.
    ///
    /// # Errors
    ///
    /// Returns an error when the layer cannot answer. A sentence it cannot
    /// parse is not an error: it yields a tree with words left unattached.
    fn parse(&self, forms: &[WordForm], readings: &[Analysis]) -> Result<Tree>;
}

/// Morpheme structure and motivation.
///
/// The port answers from the word itself — its morphemes and the words it could
/// be built from — and needs no index to do it. Questions that require knowing
/// every word in the language belong to [`DerivationalNest`] instead.
pub trait WordFormation {
    /// Cuts a form into morphemes, best hypothesis first.
    ///
    /// # Errors
    ///
    /// Returns an error when the morpheme tables cannot be consulted.
    fn segment(&self, form: &WordForm) -> Result<Vec<Evidenced<Segmentation>>>;

    /// Returns the derivation chain that explains a word, if one can be built.
    ///
    /// # Errors
    ///
    /// Returns an error when the tables cannot be consulted.
    fn motivation(&self, form: &WordForm) -> Result<Option<Evidenced<DerivationChain>>>;
}

/// Derivational nests, which only an index of the whole vocabulary can answer.
pub trait DerivationalNest {
    /// Returns the words sharing a root with this one.
    ///
    /// # Errors
    ///
    /// Returns an error when the index cannot be read.
    fn nest(&self, form: &WordForm) -> Result<Vec<WordForm>>;
}

/// Dictionary senses.
pub trait Lexicon: core::fmt::Debug {
    /// Returns the lemmas a form may belong to.
    ///
    /// # Errors
    ///
    /// Returns an error when the index cannot be read.
    fn lemmas_of(&self, form: &WordForm) -> Result<Vec<LemmaId>>;

    /// Returns every sense of a lemma, in dictionary order.
    ///
    /// # Errors
    ///
    /// Returns an error when the lemma is unknown or the index cannot be read.
    fn senses_of(&self, lemma: LemmaId) -> Result<Vec<Evidenced<Sense>>>;

    /// Returns one sense.
    ///
    /// # Errors
    ///
    /// Returns an error when the sense is unknown or the index cannot be read.
    fn sense(&self, sense: SenseId) -> Result<Evidenced<Sense>>;

    /// Returns the set expressions a lemma takes part in.
    ///
    /// # Errors
    ///
    /// Returns an error when the index cannot be read.
    fn idioms_with(&self, lemma: LemmaId) -> Result<Vec<Evidenced<Idiom>>>;

    /// Returns the sources the lexicon was built from.
    ///
    /// # Errors
    ///
    /// Returns an error when the index cannot be read.
    fn sources(&self) -> Result<Vec<Source>>;

    /// Returns one source by identity.
    ///
    /// # Errors
    ///
    /// Returns an error when the source is unknown.
    fn source(&self, source: SourceId) -> Result<Source>;
}

/// The network of relations between senses.
pub trait SemanticNetwork {
    /// Returns the relations a sense takes part in, optionally of one kind.
    ///
    /// # Errors
    ///
    /// Returns an error when the sense is unknown or the index cannot be read.
    fn relations_of(
        &self,
        sense: SenseId,
        kind: Option<RelationKind>
    ) -> Result<Vec<Evidenced<Relation>>>;

    /// Reports whether a sense is a descendant of another in the taxonomy.
    ///
    /// # Errors
    ///
    /// Returns an error when either sense is unknown or the index cannot be
    /// read.
    fn is_kind_of(&self, sense: SenseId, ancestor: SenseId) -> Result<bool>;

    /// Returns the shortest chain of relations connecting two senses, bounded
    /// by a maximum length.
    ///
    /// The chain is what an explanation is built from: it is shown to the
    /// caller rather than summarized, so the reasoning stays inspectable.
    ///
    /// # Errors
    ///
    /// Returns an error when either sense is unknown or the index cannot be
    /// read.
    fn path_between(
        &self,
        from: SenseId,
        to: SenseId,
        max_steps: usize
    ) -> Result<Vec<Evidenced<Relation>>>;
}

/// The government patterns of predicates.
pub trait FrameInventory {
    /// Returns the frames of a sense.
    ///
    /// # Errors
    ///
    /// Returns an error when the sense is unknown or the index cannot be read.
    fn frames_of(&self, sense: SenseId) -> Result<Vec<Evidenced<Frame>>>;
}

macro_rules! forward {
    ($holder:ty, $trait:ident, $($method:ident($($argument:ident: $kind:ty),*) -> $result:ty;)+) => {
        impl<T: $trait + ?Sized> $trait for $holder {
            $(
                fn $method(&self, $($argument: $kind),*) -> $result {
                    (**self).$method($($argument),*)
                }
            )+
        }
    };
}

forward!(Arc<T>, Morphology,
    analyze(form: &WordForm) -> Result<Readings>;
    inflect(form: &WordForm, tag: &GrammarTag) -> Result<Vec<WordForm>>;
    paradigm(form: &WordForm) -> Result<Paradigm>;
);

forward!(&T, Morphology,
    analyze(form: &WordForm) -> Result<Readings>;
    inflect(form: &WordForm, tag: &GrammarTag) -> Result<Vec<WordForm>>;
    paradigm(form: &WordForm) -> Result<Paradigm>;
);

forward!(Arc<T>, WordFormation,
    segment(form: &WordForm) -> Result<Vec<Evidenced<Segmentation>>>;
    motivation(form: &WordForm) -> Result<Option<Evidenced<DerivationChain>>>;
);

forward!(&T, WordFormation,
    segment(form: &WordForm) -> Result<Vec<Evidenced<Segmentation>>>;
    motivation(form: &WordForm) -> Result<Option<Evidenced<DerivationChain>>>;
);

forward!(Arc<T>, Lexicon,
    lemmas_of(form: &WordForm) -> Result<Vec<LemmaId>>;
    senses_of(lemma: LemmaId) -> Result<Vec<Evidenced<Sense>>>;
    sense(sense: SenseId) -> Result<Evidenced<Sense>>;
    idioms_with(lemma: LemmaId) -> Result<Vec<Evidenced<Idiom>>>;
    sources() -> Result<Vec<Source>>;
    source(source: SourceId) -> Result<Source>;
);

forward!(Arc<T>, SemanticNetwork,
    relations_of(sense: SenseId, kind: Option<RelationKind>) -> Result<Vec<Evidenced<Relation>>>;
    is_kind_of(sense: SenseId, ancestor: SenseId) -> Result<bool>;
    path_between(from: SenseId, to: SenseId, max_steps: usize) -> Result<Vec<Evidenced<Relation>>>;
);

forward!(Arc<T>, FrameInventory,
    frames_of(sense: SenseId) -> Result<Vec<Evidenced<Frame>>>;
);

/// The checker that turns a phrase into a verdict.
pub trait Checker {
    /// Runs the gates over a phrase.
    ///
    /// # Errors
    ///
    /// Returns an error when a layer below cannot be consulted. A phrase that
    /// does not read is not an error: it is a rejected verdict.
    fn check(&self, phrase: &str) -> Result<Verdict>;
}
