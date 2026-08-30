// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! The result of checking a phrase.
//!
//! A verdict never says "wrong" without saying where. Every violation names the
//! words involved and the expectation they broke, so a model reading the
//! verdict can repair the phrase instead of resampling it, and a person reading
//! it can disagree with the engine on the merits.

use std::{borrow::Cow, string::String, vec::Vec};

use crate::{
    frame::{Constraint, SemanticRole, SlotForm},
    grammar::Case,
    id::SenseId,
    morphology::WordForm,
    relation::RelationKind,
    rules::Citation
};

/// How much weight a violation carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Severity {
    /// The phrase cannot be read this way at all.
    Fatal,
    /// The phrase can be read, but something in it is unsupported or marked.
    Doubt,
    /// The phrase is sound; the reader is told something about it anyway.
    ///
    /// Reading a word through a transferred or a far sense is worth saying —
    /// it is how `строить воду` passes — but it is not a reason to doubt the
    /// phrase. Ordinary Russian reaches for a transferred sense constantly,
    /// and doubting every time buries the findings that matter. A sense the
    /// dictionary marked is the one exception, and it is a doubt: see
    /// [`Remoteness`].
    ///
    /// A claim no source states is a remark for the same reason. The network
    /// holds what a dictionary happened to write down, which is a fraction of
    /// what is true; its silence is not evidence.
    Note
}

/// Something the checker found wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[non_exhaustive]
pub enum Violation {
    /// No analyzer recognized the form and no rule could motivate it.
    UnknownWord {
        /// The form in question.
        form: WordForm
    },

    /// No dictionary holds the form, but it inflects like a Russian word.
    ///
    /// A dictionary of any size is behind the language it describes. A form
    /// the analyzer can decline but has never seen is far more often a word
    /// nobody wrote down than a mistake, so it is a doubt.
    UnlistedWord {
        /// The form in question.
        form: WordForm
    },

    /// A spelling the code of 1956 refuses outright.
    ///
    /// The rule names the paragraph, the place and the whole word that should
    /// stand there. The refusal is fatal because the code does not admit the
    /// written form under any reading: `жыр` is not a spelling that survives
    /// anywhere in the norm, and the only thing to do is write `жир`.
    Misspelled {
        /// The form in question.
        form:    WordForm,
        /// The paragraph that found it.
        cites:   Citation,
        /// Where the breach stands, counting characters from the word's start.
        at:      usize,
        /// What the paragraph says, in its own words.
        says:    Cow<'static, str>,
        /// The whole word the paragraph writes instead.
        instead: String
    },

    /// The form is not in any dictionary, but its structure explains it.
    ///
    /// This is a doubt, not a rejection: Russian forms words freely, and an
    /// occasional formation with a readable structure is a word.
    OccasionalWord {
        /// The form in question.
        form:         WordForm,
        /// The word it is built from.
        motivated_by: WordForm
    },

    /// Two words that must agree do not.
    AgreementMismatch {
        /// The word that sets the expectation, usually the head.
        head:      WordForm,
        /// The word that broke it.
        dependent: WordForm,
        /// The category they disagree in.
        category:  AgreementCategory
    },

    /// A filler stands in a form the predicate does not govern.
    GovernmentMismatch {
        /// The governing word.
        predicate:   WordForm,
        /// The filler.
        filler:      WordForm,
        /// The surface form the frame expects.
        expected:    SlotForm,
        /// The case the filler actually stands in.
        actual_case: Option<Case>
    },

    /// A filler is grammatically right and semantically impossible.
    ///
    /// This is the violation the engine exists for: the words do not fit each
    /// other by meaning, and the report says which slot and which restriction.
    SelectionalMismatch {
        /// The governing word.
        predicate:       WordForm,
        /// The sense of the predicate under which the check ran.
        predicate_sense: SenseId,
        /// The part the filler was to play.
        role:            SemanticRole,
        /// The filler.
        filler:          WordForm,
        /// What the slot admits.
        expected:        Constraint
    },

    /// The word is known, but no sense of it survives the context.
    NoSenseFits {
        /// The form in question.
        form:       WordForm,
        /// How many senses were considered before all were dropped.
        considered: usize
    },

    /// The statement asserts a relation the network denies.
    ContradictsKnowledge {
        /// The sense the claim starts at.
        subject: SenseId,
        /// The relation the claim asserts.
        claimed: RelationKind,
        /// The sense the claim ends at.
        object:  SenseId,
        /// The relation the network holds instead.
        known:   RelationKind
    },

    /// The phrase asserts a relation nothing in the network supports.
    ///
    /// This is weaker than a contradiction and stronger than silence. The
    /// network does not hold the opposite — it holds nothing — so the claim may
    /// well be true and simply absent from the sources. What the verdict says
    /// is that the engine cannot stand behind it, which is exactly what a model
    /// about to assert it needs to hear.
    UnsupportedClaim {
        /// The sense the claim starts at.
        subject: SenseId,
        /// The relation the claim asserts.
        claimed: RelationKind,
        /// The sense the claim ends at.
        object:  SenseId
    },

    /// A `-нибудь` indefinite stands in a realized assertion.
    ///
    /// The particle promises an unchosen referent, and an assertion about a
    /// completed fact has chosen one: `он её чем-нибудь обидел` asserts the
    /// offence while refusing to say with what, which Russian says with
    /// `чем-то`. A question, a condition, a wish or a habit keeps the promise
    /// open, and there the particle is at home.
    StrandedIndefinite {
        /// The indefinite form in question.
        form: WordForm
    },
    /// A form no dictionary holds, declined off a word every dictionary holds.
    ///
    /// `пара чулков` misdeclines `чулок`, whose genitive plural the
    /// dictionary spells `чулок`. A form built on an unknown stem may be a
    /// word nobody wrote down; a form built on a known stem against its own
    /// paradigm is the paradigm applied wrongly.
    Misdeclined {
        /// The form in question.
        form:  WordForm,
        /// The dictionary word it misdeclines.
        lemma: WordForm
    },
    /// A written form the norm refuses however many people say it.
    ///
    /// `ехай`, `ложить`, `ихний`: the analyzer reads them because people
    /// write them, and every handbook names the form that belongs in their
    /// place. The violation carries that form, so the refusal teaches.
    VernacularForm {
        /// The form as written.
        form:     WordForm,
        /// The form the norm writes instead.
        standard: WordForm
    },
    /// A sentence-closing mark written twice.
    ///
    /// `собака. .` closes the sentence and then closes it again: two full
    /// stops with nothing but space between them are typesetting debris, not
    /// punctuation — the ellipsis and `?!` are spelled without a gap.
    DoubledStop {
        /// The marks as written.
        written: String
    },
    /// A time adverb pointing one way and a predicate conjugated the other.
    ///
    /// `вчера магазин закрыт` sets yesterday against the present: the adverb
    /// names a past the sentence's verb refuses to stand in.
    TimeDiscord {
        /// The adverb that names the time.
        adverb:    WordForm,
        /// The predicate conjugated against it.
        predicate: WordForm
    },
    /// A verb that leads only the imperfective handed a perfective infinitive.
    ///
    /// A phase verb names a stretch of an action, and only an action with
    /// duration has one to name: `начал читать` opens a reading, while `начал
    /// прочитать` opens what grammar refuses to stretch. The analytic future
    /// is built the same way — `буду читать` stands, `буду прочитать` does
    /// not, because a perfective verb makes its future by conjugating.
    AspectMismatch {
        /// The verb that demands an imperfective infinitive.
        predicate:  WordForm,
        /// The perfective infinitive it was handed.
        infinitive: WordForm
    },
    /// The future of `быть` set over a conjugated verb.
    ///
    /// The analytic future takes an infinitive and nothing else: `будет
    /// финансироваться` stands, `будет финансируется` sets two conjugated
    /// verbs over one another and is not a Russian clause.
    BrokenFuture {
        /// The future form of `быть`.
        auxiliary:  WordForm,
        /// The conjugated verb standing where the infinitive belongs.
        conjugated: WordForm
    },
    /// A preposition a word does not govern through.
    ///
    /// `оплатить за проезд` and `указать о недостатках` put a preposition
    /// where the word governs a bare case: `оплатить проезд`, `указать на
    /// недостатки`. The pairs are the closed list every style manual opens
    /// with, so naming the word and the preposition is the whole finding.
    WrongPreposition {
        /// The governing word.
        predicate:   WordForm,
        /// The preposition it does not govern through.
        preposition: WordForm
    },
    /// An infinitive hung on a word that cannot lead one.
    ///
    /// `начали строить` stands; `было начато строить` does not, because the
    /// passive of a phase verb takes a thing, not an action to perform.
    MisplacedInfinitive {
        /// The word the infinitive wrongly answers to.
        predicate:  WordForm,
        /// The infinitive.
        infinitive: WordForm
    },
    /// A copula spelled out beside a predicate that is one already.
    ///
    /// `стаканы поставлены` is a whole clause; `стаканы есть поставлены`
    /// writes the copula twice, which Russian does not.
    StrayCopula {
        /// The copula.
        copula:    WordForm,
        /// The predicate that needed no copula.
        predicate: WordForm
    },
    /// A correlate answering a conjunction that never takes one.
    ///
    /// `если … то` is a pair; `чтобы … то` is not. A clause opened with
    /// `чтобы` and answered with `то` borrowed the correlate from the wrong
    /// conjunction.
    FalsePair {
        /// The conjunction that opened the clause.
        opener:   WordForm,
        /// The correlate that wrongly answers it.
        follower: WordForm
    },
    /// A bare `что` clause standing where the sentence needed `то, что`.
    ///
    /// A sentence may open with a `что` clause only when `что` fills a gap
    /// in it — `что случилось, никто не знал`. A clause whose places are
    /// all taken hangs on the sentence with nothing to hold it: `что он
    /// приехал, никто не обрадовался` needed the correlate `то, что`.
    UnanchoredClause {
        /// The conjunction that opened the clause.
        conjunction: WordForm
    },
    /// A clause of time and the clause it dates standing in different times.
    ///
    /// `когда друг поступит на завод, он приобретал квалификацию` dates a
    /// past event by a future one. The two clauses of one sentence of time
    /// share a line of time, and the past never lies ahead of the future.
    TenseDiscord {
        /// The verb of the clause that gives the date.
        dating: WordForm,
        /// The verb of the clause it dates.
        dated:  WordForm
    },
    /// A stretch of time measured off a state already reached.
    ///
    /// `доклад написан уже час` says the writing is over and still going on
    /// at once. What is finished has no duration left to measure.
    MeasuredResult {
        /// The word naming the state.
        state:   WordForm,
        /// The word measuring the stretch.
        measure: WordForm
    },
    /// A correlate standing before a clause that leaves nothing unsaid.
    ///
    /// `он сказал то, что не успеет` puts a pronoun where the clause itself
    /// is the whole of what was said. A correlate stands for something the
    /// clause omits, and this clause omits nothing.
    IdleCorrelate {
        /// The correlate that stands for nothing.
        correlate:   WordForm,
        /// The word opening the clause behind it.
        conjunction: WordForm
    },
    /// A preposition standing with nothing left for it to govern.
    ///
    /// `написано в` breaks off where the governed word belongs. Russian
    /// stands no preposition at the end of a sentence.
    StrandedPreposition {
        /// The preposition left standing.
        form: WordForm
    },
    /// The analytic future denied on both of its halves.
    ///
    /// `не будет не улыбаться` denies the auxiliary and the infinitive at
    /// once. One predicate takes one denial.
    DoubledDenial {
        /// The auxiliary the first denial fell on.
        auxiliary:  WordForm,
        /// The infinitive the second denial fell on.
        infinitive: WordForm
    },
    /// A coordinated verb left with nothing it can govern.
    ///
    /// `помнят и гордятся победой` hands one filler to two verbs that take
    /// different cases, and the first is left empty.
    UnsharedFiller {
        /// The verb left with nothing.
        left:  WordForm,
        /// The verb that kept the filler.
        right: WordForm
    },
    /// A verb form standing where a determiner promised a noun.
    ///
    /// `этот будь` points a demonstrative at an imperative. A determiner
    /// agrees with a noun, and a verb form has no case to agree in.
    BorrowedNoun {
        /// The determiner that pointed.
        determiner: WordForm,
        /// The verb form standing in the noun's place.
        form:       WordForm
    },
    /// Existence asserted of something the sentence already points at.
    ///
    /// `твой мобильник есть на кухне` says a mobile there is, of a mobile
    /// already named. Russian places a known thing without a copula.
    AssertedExistence {
        /// The thing already pointed at.
        named:  WordForm,
        /// The copula asserting it.
        copula: WordForm
    },
    /// A clause naming a doer and a thing done, with nothing done.
    ///
    /// `маша книгу, и саша читает` leaves the first clause without a verb
    /// and without anything that could stand in for one.
    PredicatelessClause {
        /// The word naming the doer.
        doer: WordForm,
        /// The word naming the thing done.
        done: WordForm
    },
    /// A clause opened as a statement and marked as a question.
    ///
    /// `что` folds a statement in; `ли` asks. A clause carrying both is
    /// neither told nor asked.
    ToldAndAsked {
        /// The conjunction that opened the statement.
        conjunction: WordForm,
        /// The particle that asked.
        particle:    WordForm
    },
    /// A passive standing beside a denied being over a genitive noun.
    ///
    /// `статей не было прочитано` says of one thing both that it was absent
    /// and that something was done to it.
    DeniedPassive {
        /// The noun said to be missing.
        named: WordForm,
        /// The passive said of it.
        done:  WordForm
    },
    /// A prepositional case with no preposition to hold it.
    ///
    /// `севере невозможно оставаться` lost the `на` a translation had no
    /// case for. The prepositional is named after what it cannot do without.
    UnheldPrepositional {
        /// The word left standing in the prepositional.
        form: WordForm
    },
    /// A short preposition standing where the long form belongs.
    ///
    /// `с служащим` runs three consonants together; Russian says `со
    /// служащим`.
    MisspeltPreposition {
        /// The preposition as it was written.
        written:  WordForm,
        /// The form the sound of the next word calls for.
        expected: String,
        /// The word that calls for it.
        before:   WordForm
    },
    /// A preposition on a noun filled in a case that noun never takes.
    ///
    /// `уверенность в успех` puts the accusative where `уверенность` has
    /// only ever taken the prepositional, however readily the verb under it
    /// takes the accusative.
    MisgovernedNoun {
        /// The noun the phrase hangs on.
        head:   WordForm,
        /// What stood behind its preposition.
        filler: WordForm
    },
    /// A clause tied to its sentence by a relative and a conjunction at once.
    ///
    /// `случаи, о которых, кажется, что я читал` opens one clause twice.
    /// Either the relative carries it or the conjunction does.
    DoublyTied {
        /// The relative that opened the clause.
        relative:    WordForm,
        /// The conjunction that opened it a second time.
        conjunction: WordForm
    },
    /// A pointing word answered by a phrase where a clause belongs.
    ///
    /// `те бактерии, живущие в полости` promises to say which bacteria and
    /// then only describes them. `тот` is answered by `который`.
    UnansweredPointer {
        /// The word that pointed forward.
        pointer: WordForm,
        /// The phrase standing where the clause belongs.
        phrase:  WordForm
    },
    /// A gerund standing over a clause built in the passive.
    ///
    /// `обнаружив неполадки, он должен быть отключён` hangs the finding on a
    /// subject the clause says nothing is done by, only done to.
    PassiveGerund {
        /// The gerund left with nobody to hang on.
        gerund: WordForm,
        /// The passive that took the doer away.
        done:   WordForm
    },
    /// A participial phrase cut in two by the noun it describes.
    ///
    /// `приехавшие делегаты на конференцию` sets the noun down in the middle
    /// of what the participle governs. The phrase stands whole on one side
    /// of the noun or the other.
    PartedPhrase {
        /// The participle the phrase opens with.
        attribute: WordForm,
        /// What it governs from the far side of the noun.
        filler:    WordForm
    },
    /// The two halves of a paired conjunction built unlike each other.
    ///
    /// `не только помогал хлебом, но и одеждой` puts a verb on one side of
    /// the pair and a noun on the other. A paired conjunction joins likes:
    /// what stands after `не только` and what stands after `но и` answer the
    /// same question in the same form.
    UnpairedCoordination {
        /// What stands after the first half of the pair.
        first:  WordForm,
        /// What stands after the second half.
        second: WordForm
    },
    /// A bare accusative standing beside a verb turned back on itself.
    ///
    /// `-ся` is what an object would have been, and the verb has no place
    /// left for one.
    ReflexiveObject {
        /// The reflexive verb.
        verb:   WordForm,
        /// The accusative standing beside it.
        filler: WordForm
    },
    /// A phrase a collection of rules names as a mistake.
    ///
    /// Every word is a word and every form agrees; what is wrong is the
    /// combination, and a rule that names it outright is the only thing
    /// that knows.
    AdvisedAgainst {
        /// What the rule is called.
        name:    String,
        /// What it says.
        message: String
    },
    /// An imperative standing inside a clause that `что` reports.
    ///
    /// `что` retells: what follows it is somebody's words folded into the
    /// speaker's own sentence, and a command does not fold. `сказал, что
    /// приходи` crosses the two ways of quoting — the command is either
    /// direct speech after a colon or a `чтобы` clause.
    CommandedClause {
        /// The conjunction that opened the retelling.
        conjunction: WordForm,
        /// The imperative standing inside it.
        imperative:  WordForm
    },
    /// An enclitic standing where nothing precedes it.
    ///
    /// `ли`, `же` and `бы` lean on the word before them — that is what an
    /// enclitic is — and a sentence has no word before its first. `Ли он
    /// придёт` is not a Russian question; `придёт ли он` is.
    LeadingClitic {
        /// The enclitic that opens the sentence.
        form: WordForm
    },
    /// A subordinator that always follows a comma, standing bare.
    ///
    /// The rulebook is categorical about `который` and `чтобы`: the clause
    /// they open is set off with a comma. A sentence that runs into either
    /// without one is missing the comma, not exercising a stylistic choice.
    MissingComma {
        /// The word the comma belongs before.
        before: WordForm
    },
    /// A dash the code of rules puts between two words, left out.
    ///
    /// `Москва столица` states a subject and a nominal predicate with
    /// nothing between them; the book puts a dash where the copula would
    /// have stood, and reading the pair without it takes the reader a second
    /// try.
    MissingDash {
        /// The word the dash belongs before.
        before: WordForm
    },
    /// A sentence-opening gerund clause with no one to do it.
    ///
    /// A gerund borrows its doer from the subject of the sentence. When the
    /// sentence offers no nominative subject, or one that cannot do what the
    /// gerund names, the clause hangs in the air — `подъезжая к станции, у
    /// меня слетела шляпа` is the schoolbook example.
    DanglingGerund {
        /// The gerund that opens the clause.
        form: WordForm
    },
    /// The reading survives only through a marked or figurative sense.
    MarkedReadingOnly {
        /// The form in question.
        form:  WordForm,
        /// The sense that had to be used.
        sense: SenseId,
        /// How far the sense stands from the plain reading of the word.
        mark:  Remoteness
    }
}

/// How far a sense stands from the plain reading of its word.
///
/// The three are not worth the same. A transferred sense is ordinary Russian —
/// a phrase reaching for one is not thereby doubtful — and a sense low in an
/// entry is a sense a dictionary printed, no more. A stylistic mark is
/// different: `гром` is a shopping bag in criminal jargon and a spoon in
/// prison slang, and a phrase that holds together only through such a reading
/// is not a phrase anyone meant. The engine says so.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Remoteness {
    /// The sense carries a stylistic mark: jargon, dialect, coarse, obsolete.
    Marked,
    /// The sense is a transferred one.
    Figurative,
    /// The sense stands past the everyday ones of its entry.
    Late {
        /// Which sense of the entry it is.
        index: u16
    }
}

impl Remoteness {
    /// How much weight reading a phrase through such a sense carries.
    #[must_use]
    pub const fn severity(self) -> Severity {
        match self {
            Self::Marked => Severity::Doubt,
            Self::Figurative
            | Self::Late {
                ..
            } => Severity::Note
        }
    }
}

impl Violation {
    /// The name the violation is reported under.
    ///
    /// One name, written once, so that a scale weighing findings and a caller
    /// reading them are talking about the same thing.
    #[must_use]
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per violation; the length is the enum's"
    )]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::UnknownWord {
                ..
            } => "unknown_word",
            Self::OccasionalWord {
                ..
            } => "occasional_word",
            Self::UnlistedWord {
                ..
            } => "unlisted_word",
            Self::Misspelled {
                ..
            } => "misspelled",

            Self::AgreementMismatch {
                ..
            } => "agreement_mismatch",
            Self::GovernmentMismatch {
                ..
            } => "government_mismatch",
            Self::SelectionalMismatch {
                ..
            } => "selectional_mismatch",
            Self::NoSenseFits {
                ..
            } => "no_sense_fits",
            Self::ContradictsKnowledge {
                ..
            } => "contradicts_knowledge",
            Self::UnsupportedClaim {
                ..
            } => "unsupported_claim",
            Self::StrandedIndefinite {
                ..
            } => "stranded_indefinite",
            Self::AspectMismatch {
                ..
            } => "aspect_mismatch",
            Self::BrokenFuture {
                ..
            } => "broken_future",
            Self::WrongPreposition {
                ..
            } => "wrong_preposition",
            Self::MisplacedInfinitive {
                ..
            } => "misplaced_infinitive",
            Self::StrayCopula {
                ..
            } => "stray_copula",
            Self::FalsePair {
                ..
            } => "false_pair",
            Self::LeadingClitic {
                ..
            } => "leading_clitic",
            Self::CommandedClause {
                ..
            } => "commanded_clause",
            Self::TenseDiscord {
                ..
            } => "tense_discord",
            Self::MeasuredResult {
                ..
            } => "measured_result",
            Self::IdleCorrelate {
                ..
            } => "idle_correlate",
            Self::StrandedPreposition {
                ..
            } => "stranded_preposition",
            Self::DoubledDenial {
                ..
            } => "doubled_denial",
            Self::UnsharedFiller {
                ..
            } => "unshared_filler",
            Self::BorrowedNoun {
                ..
            } => "borrowed_noun",
            Self::AssertedExistence {
                ..
            } => "asserted_existence",
            Self::PredicatelessClause {
                ..
            } => "predicateless_clause",
            Self::ToldAndAsked {
                ..
            } => "told_and_asked",
            Self::DeniedPassive {
                ..
            } => "denied_passive",
            Self::ReflexiveObject {
                ..
            } => "reflexive_object",
            Self::UnheldPrepositional {
                ..
            } => "unheld_prepositional",
            Self::MisspeltPreposition {
                ..
            } => "misspelt_preposition",
            Self::MisgovernedNoun {
                ..
            } => "misgoverned_noun",
            Self::DoublyTied {
                ..
            } => "doubly_tied",
            Self::UnansweredPointer {
                ..
            } => "unanswered_pointer",
            Self::PassiveGerund {
                ..
            } => "passive_gerund",
            Self::PartedPhrase {
                ..
            } => "parted_phrase",
            Self::UnpairedCoordination {
                ..
            } => "unpaired_coordination",
            Self::AdvisedAgainst {
                ..
            } => "advised_against",
            Self::UnanchoredClause {
                ..
            } => "unanchored_clause",
            Self::MissingComma {
                ..
            } => "missing_comma",
            Self::MissingDash {
                ..
            } => "missing_dash",
            Self::DanglingGerund {
                ..
            } => "dangling_gerund",
            Self::Misdeclined {
                ..
            } => "misdeclined",
            Self::TimeDiscord {
                ..
            } => "time_discord",
            Self::VernacularForm {
                ..
            } => "vernacular_form",
            Self::DoubledStop {
                ..
            } => "doubled_stop",
            Self::MarkedReadingOnly {
                ..
            } => "marked_reading_only"
        }
    }

    /// How much weight the violation carries.
    #[must_use]
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per violation; the length is the enum's"
    )]
    pub const fn severity(&self) -> Severity {
        match self {
            Self::MissingDash {
                ..
            }
            | Self::OccasionalWord {
                ..
            }
            | Self::UnlistedWord {
                ..
            } => Severity::Doubt,
            Self::Misspelled {
                ..
            }
            | Self::UnknownWord {
                ..
            }
            | Self::AgreementMismatch {
                ..
            }
            | Self::GovernmentMismatch {
                ..
            }
            | Self::SelectionalMismatch {
                ..
            }
            | Self::NoSenseFits {
                ..
            }
            | Self::StrandedIndefinite {
                ..
            }
            | Self::DanglingGerund {
                ..
            }
            | Self::Misdeclined {
                ..
            }
            | Self::TimeDiscord {
                ..
            }
            | Self::AspectMismatch {
                ..
            }
            | Self::BrokenFuture {
                ..
            }
            | Self::WrongPreposition {
                ..
            }
            | Self::MisplacedInfinitive {
                ..
            }
            | Self::StrayCopula {
                ..
            }
            | Self::FalsePair {
                ..
            }
            | Self::LeadingClitic {
                ..
            }
            | Self::CommandedClause {
                ..
            }
            | Self::TenseDiscord {
                ..
            }
            | Self::MeasuredResult {
                ..
            }
            | Self::IdleCorrelate {
                ..
            }
            | Self::StrandedPreposition {
                ..
            }
            | Self::DoubledDenial {
                ..
            }
            | Self::UnsharedFiller {
                ..
            }
            | Self::BorrowedNoun {
                ..
            }
            | Self::AssertedExistence {
                ..
            }
            | Self::PredicatelessClause {
                ..
            }
            | Self::ToldAndAsked {
                ..
            }
            | Self::DeniedPassive {
                ..
            }
            | Self::ReflexiveObject {
                ..
            }
            | Self::UnheldPrepositional {
                ..
            }
            | Self::MisspeltPreposition {
                ..
            }
            | Self::MisgovernedNoun {
                ..
            }
            | Self::DoublyTied {
                ..
            }
            | Self::UnansweredPointer {
                ..
            }
            | Self::PassiveGerund {
                ..
            }
            | Self::PartedPhrase {
                ..
            }
            | Self::UnpairedCoordination {
                ..
            }
            | Self::UnanchoredClause {
                ..
            }
            | Self::MissingComma {
                ..
            }
            | Self::VernacularForm {
                ..
            }
            | Self::DoubledStop {
                ..
            }
            | Self::ContradictsKnowledge {
                ..
            } => Severity::Fatal,
            Self::MarkedReadingOnly {
                mark, ..
            } => mark.severity(),
            Self::UnsupportedClaim {
                ..
            }
            | Self::AdvisedAgainst {
                ..
            } => Severity::Note
        }
    }
}

/// The category two words failed to agree in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum AgreementCategory {
    /// Case.
    Case,
    /// Number.
    Number,
    /// Gender.
    Gender,
    /// Person.
    Person,
    /// Degree of comparison.
    Degree
}

/// What the checker decided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Status {
    /// The phrase reads, with everything supported.
    Accepted,
    /// The phrase reads, but something in it is unsupported or marked.
    Doubtful,
    /// The phrase does not read.
    Rejected
}

/// One reading of the phrase that survived every gate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reading {
    /// The senses chosen for the content words, in the order the words appear.
    pub senses:      Vec<SenseId>,
    /// The frame the predicate was read under, when the phrase has a predicate.
    pub frame_sense: Option<SenseId>
}

/// The outcome of checking one phrase.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Verdict {
    /// The readings that survived, best first. Empty when the phrase was
    /// rejected.
    pub readings:   Vec<Reading>,
    /// Everything the checker found wrong.
    pub violations: Vec<Violation>
}

impl Verdict {
    /// A verdict with nothing wrong and one reading.
    #[must_use]
    pub fn accepted(reading: Reading) -> Self {
        Self {
            readings:   std::vec![reading],
            violations: Vec::new()
        }
    }

    /// A verdict with no surviving reading.
    #[must_use]
    pub const fn rejected(violations: Vec<Violation>) -> Self {
        Self {
            readings: Vec::new(),
            violations
        }
    }

    /// A verdict that carries the readings that survived and the violations
    /// found.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::{
    ///     morphology::WordForm,
    ///     verdict::{Reading, Verdict, Violation}
    /// };
    ///
    /// let verdict = Verdict::checked(
    ///     std::vec![Reading {
    ///         senses:      Vec::new(),
    ///         frame_sense: None
    ///     }],
    ///     std::vec![Violation::UnknownWord {
    ///         form: WordForm::parse("бббб")?
    ///     }]
    /// );
    ///
    /// assert!(!verdict.readings.is_empty());
    /// assert!(!verdict.violations.is_empty());
    /// # Ok::<(), rusem::error::CoreError>(())
    /// ```
    #[must_use]
    pub const fn checked(readings: Vec<Reading>, violations: Vec<Violation>) -> Self {
        Self {
            readings,
            violations
        }
    }

    /// The decision the violations and the readings add up to.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusem::verdict::{Reading, Status, Verdict};
    ///
    /// let verdict = Verdict::accepted(Reading {
    ///     senses:      Vec::new(),
    ///     frame_sense: None
    /// });
    ///
    /// assert_eq!(verdict.status(), Status::Accepted);
    /// ```
    #[must_use]
    pub fn status(&self) -> Status {
        if self.readings.is_empty()
            || self
                .violations
                .iter()
                .any(|violation| violation.severity() == Severity::Fatal)
        {
            Status::Rejected
        } else if self
            .violations
            .iter()
            .any(|violation| violation.severity() == Severity::Doubt)
        {
            Status::Doubtful
        } else {
            Status::Accepted
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sense::SemanticClass;

    fn form(text: &str) -> WordForm {
        WordForm::parse(text).expect("valid form")
    }

    fn reading() -> Reading {
        Reading {
            senses:      Vec::new(),
            frame_sense: None
        }
    }

    #[test]
    fn clean_verdict_is_accepted() {
        assert_eq!(Verdict::accepted(reading()).status(), Status::Accepted);
    }

    #[test]
    fn selectional_mismatch_rejects() {
        let verdict = Verdict {
            readings:   std::vec![reading()],
            violations: std::vec![Violation::SelectionalMismatch {
                predicate:       form("пить"),
                predicate_sense: SenseId::new(1).expect("non-zero"),
                role:            SemanticRole::Patient,
                filler:          form("кирпич"),
                expected:        Constraint::OfClass(std::vec![SemanticClass::Liquid])
            }]
        };

        assert_eq!(verdict.status(), Status::Rejected);
    }

    #[test]
    fn occasional_word_only_raises_doubt() {
        let verdict = Verdict {
            readings:   std::vec![reading()],
            violations: std::vec![Violation::OccasionalWord {
                form:         form("кибербуряк"),
                motivated_by: form("буряк")
            }]
        };

        assert_eq!(verdict.status(), Status::Doubtful);
    }

    #[test]
    fn no_reading_means_rejected() {
        assert_eq!(Verdict::rejected(Vec::new()).status(), Status::Rejected);
    }
}
