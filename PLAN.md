# Rusem Development Plan

## Purpose

This document defines the migration from the current word-level checker to a deterministic,
evidence-backed analyzer of Russian text. The work is split into small tasks so that each task
can be implemented independently by a limited-context coding agent.

The target is a total, closed semantic system for Russian. It must not answer `Unknown` for a
well-formed Russian expression. Existing explanatory dictionaries are primary linguistic input,
not prose that people must rewrite into a second dictionary. The analyzer must read a dictionary
gloss as Russian text by the same grammar and semantic algorithm used for ordinary text. It derives
meaning recursively from lexical definitions, morpheme meanings, grammar, context, and logic. The
target is a system that:

- preserves every grammatically and semantically possible interpretation;
- recursively explains every lexical sense through other senses and typed logical operations;
- derives compatibility or contradiction from definitions instead of corpus probability;
- represents an absent referent as a constrained variable, not as unknown meaning;
- classifies an unrecognized form by a proved derivational or structural failure;
- records where every imported and derived fact came from;
- produces the same ordered result for the same text, code, data manifest, norm profile, and
  analysis limits;
- separates descriptive linguistic analysis from normative validation.

## Current State

### What is already valuable

The repository contains a substantial formal model of Russian at the word level:

- `alphabet` models the Russian alphabet and letter properties;
- `phonetics` models syllables, sonority, and stress alternatives;
- `morphemics` models morphemes, segmentations, alternations, and derivational chains;
- `grammar` models parts of speech, grammatical categories, forms, declension, conjugation,
  agreement, and government;
- `lexis` models lexemes, paradigms, generation, and reverse reading of generated forms;
- `rules` contains executable fragments of the 1956 code;
- `law` checks consistency properties of generated paradigms;
- `sense`, `relation`, `frame`, and `syntax` define the beginnings of a semantic model;
- `evidence` defines source attribution for imported facts.

These parts should be retained and integrated. They should not be replaced by a monolithic
checker or by opaque model output.

### Codebase reuse contract

The existing domain core is the starting point, not a prototype to replace. New work must extend
the following types and functions in place or wrap them at orchestration boundaries. An issue may
replace one of them only after naming a concrete invariant that the existing type cannot express.

| Area | Existing implementation to retain | Required migration |
| --- | --- | --- |
| Russian writing | `alphabet::{Letter, Vowel, Consonant, Sign, Mark}` and the predicates in `src/alphabet.rs` | Keep as the only authority on Russian letters. The document lexer adds spans and token kinds around this layer; it must not duplicate letter classification. |
| Word-form normalization | `morphology::WordForm`, including dash/apostrophe folding and `ё/е` lookup folding | Keep as the normalized lookup key for one Russian word. Never use it as the source token because it intentionally loses case and typography. |
| Stress | `phonetics::stress::{Stress, Stressed, of_spelling, of_word, Table}` | Keep the multi-valued stress result and spelling-first/table-second order. Add token identity, evidence, and a runtime table adapter; do not create a second stress model. |
| Morpheme segmentation | `morphemics::cut::ways`, `root::{shapes, same}`, `Segmentation::tiling` | Keep generation of all cuts and the exact-cover invariant. Add dictionary-base checks, token links, evidence, and lattice alternatives; do not reimplement segmentation. |
| Word formation | `Morpheme`, `Filed`, `DerivationWay`, `DerivationStep`, `DerivationChain` | Keep these value types. Implement the missing search that connects cuts, root variants, known bases, and affix glosses into every valid chain. Analyze `Filed::gloss` through the same definition pipeline as dictionary glosses. |
| Adapter grammar | `GrammarTag` and `morphology::Analysis` | Keep only at adapter/import boundaries. `Option` here means that the adapter did not state a category; sentence reasoning must not use these options directly. |
| Canonical Russian forms | `grammar::form::{Form, Agreed, Bare, Counted, Adjectival}`, `grammar::form::verb::VerbForm`, and `form::read` | Keep as the canonical internal grammar. Its sum types encode most not-applicable categories, but the current constructors and tag reader do not reject every contradictory combination. Add strict boundary validation, then produce one hypothesis per valid `Form`; do not introduce a parallel `NormalizedTag`. |
| Paradigms | imported `morphology::Paradigm` and formal `lexis::Paradigm` | Keep both because they represent source observations and the language's validated cells respectively. Compare them explicitly rather than merging their meanings. |
| Generation and reverse reading | `lexis::paradigm::*`, `lexis::reading::{of, agreeing, verb}` | Reuse directly to generate forms, recover every cell, and validate imported morphology. Do not schedule these algorithms for reimplementation. |
| Local grammar constraints | `grammar::dependency::{agrees, predicate::said_of, governed::{admits, handed}}` | Reuse as pure compatibility predicates over `Form`. Add orchestration wrappers that attach token IDs and evidence and distinguish `compatible`, `conflict`, and `not applicable`; do not duplicate their linguistic tables. |
| Closed classes | `grammar::closed` and its adverb, asking, conjunction, copula, interjection, numeral, parenthetical, particle, predicative, preposition, and pronoun modules | Treat as mandatory grammar data. Reuse their inventories and semantic distinctions for clause construction, negation, questions, modality, coordination, pronoun constraints, and government. Dictionary lookup may enrich them but may not replace them. |
| Lexemes | `lexis::{Lexeme, Word}` | Keep the distinction between permanent lexeme properties and inflected `Form`. A token hypothesis links a source token to both a `Word`/lemma candidate and one `Form`. |
| Dictionary senses | `sense::{Sense, Idiom, Register, TimePlacement}` | Preserve exact `definition` and examples as source text. Replace the mandatory singular `Sense.class` truth with sourced class assertions. Add IDs for analyses of definition text; do not overwrite the original gloss with a generated graph. |
| Semantic relations | `relation::{Relation, RelationKind}` | Keep stable relation kinds, total `inverse`, and `is_transitive` as the only transitivity policy. Add evidence-bearing graph storage and one traversal implementation. |
| Frames | `frame::{Frame, Slot, SemanticRole, Constraint}` | Keep role inventory and lookup helpers. Replace invalid-state `SlotForm` with a sum type, validate frame construction, and replace boolean constraint evaluation with proof-bearing logical status. |
| Syntax | `syntax::{Relation, Tree}` | Keep as a compatibility view. Introduce full UD labels, validated parses, token IDs, and a packed forest; project one complete branch back to `Tree` for old consumers. |
| Sources | `evidence::{Source, SourceKind, Provenance, Evidenced}` | Keep as imported-fact provenance. Add an evidence DAG for derived facts with several premises. `Confidence` remains source metadata and must never choose semantic truth or delete a reading. |
| Norm rules | `rules::{Rule, Scope, Citation, Found}` and the twelve `svod` modules | Evolve rather than replace. Keep each paragraph's decision logic and add explicit applicability/outcome around `rule::asked`; sentence-level rules consume analysis hypotheses instead of private word facts. |
| Core laws | `law::mirrored::Mirrored`, `law::steady::Steady`, `law::spelled::Spelled`, and `law::held::Held` | Preserve their deterministic invariants. Their input contracts may migrate when unsupported fields leave `Word`; do not weaken their assertions to accommodate imported data. |
| Knowledge boundaries | traits in `ports.rs` | Evolve signatures to carry token-linked hypotheses and evidence. Implement immutable adapters for local packs. Do not create duplicate service traits with the same questions. |
| Runtime facade | `engine::Engine` and `ports::Checker` | Keep only as compatibility API. Move orchestration into `AnalysisPipeline`, then implement `check` as analyze, validate, and project. |

### Existing invariants that new code must preserve

1. `Letter` is the sole classification of a Russian letter.
2. `WordForm` is one normalized Russian word, not arbitrary document text.
3. A strictly validated `Form` describes only category combinations Russian can realize; direct
   enum construction and the current permissive tag reader do not yet guarantee this alone.
4. `Segmentation` covers every character exactly once, in order, and contains a root.
5. A formal paradigm may map one spelling to several cells and one cell to several spellings.
6. Morphological, stress, segmentation, sense, frame, and syntax ambiguity is represented as
   alternatives; source score or insertion order never makes one alternative true.
7. `RelationKind::is_transitive` is the only authority on relation closure.
8. A source gloss is retained byte-for-byte and every derived meaning points back to it.
9. Pure linguistic functions remain independent of files, network, models, and global state.
10. Imported facts use `Evidenced`; derived facts use the evidence DAG and cite every immediate
    premise.

### Required data-structure migration

The following changes are the intended ownership boundaries. Later issues must not invent
temporary parallel representations without updating this section first.

#### Stable knowledge identity versus dense runtime identity

The numeric `LemmaId`, `SenseId`, `MorphemeId`, `FrameId`, and `SourceId` in `id.rs` are compact
runtime references. A number assigned by file/import order is not a persistent identity. Add a
stable `SourceKey`, then source-scoped keys such as `LemmaKey { source_key, external_key }` and
`SenseKey { lemma_key, source_sense_key }`; build dense numeric IDs by sorting these keys
canonically inside one versioned pack. Reports serialize both the pack fingerprint and stable key
when an entity must survive a rebuild. Existing numeric ID types remain useful inside one loaded
manifest and must not be replaced with strings in hot graph edges.

#### Source token versus lookup form

`SurfaceToken` owns exact document bytes, `ByteSpan`, `CharSpan`, and `TokenKind`. A word token may
have several `Normalization` records, each producing a `WordForm` lookup key. `WordForm` remains
unchanged and never gains punctuation, whitespace, source case, or document offsets.

#### Adapter analysis versus canonical form hypothesis

`morphology::Analysis { lemma, lemma_id, tag, predicted }` remains the answer of a morphology
adapter. Add a separate internal record with this ownership:

```rust
pub struct LemmaHypothesis {
    pub id: LemmaHypothesisId,
    pub filed: Option<LemmaId>,
    pub word: Word,
    pub derivation: Option<DerivationChain>,
    pub evidence: EvidenceSetId,
}

pub struct FormHypothesis {
    pub id: FormHypothesisId,
    pub token: TokenId,
    pub lookup: WordForm,
    pub lemma: LemmaHypothesisId,
    pub form: Form,
    pub predicted: bool,
    pub evidence: EvidenceSetId,
}
```

If an adapter cannot provide `LemmaId`, lexical lookup or derivational analysis creates an unfiled
`LemmaHypothesis`; absence from a dictionary is not loss of identity inside the document. If
permanent `Lexeme` properties are still incomplete, the candidate remains at the adapter boundary
and causes a typed incomplete-data result; it is not represented by fake defaults inside `Word`.

`Readings::new` must stop making descending `Confidence` the semantic order. Add a complete
canonical key over lemma stable key, `Form`, predicted marker, and provenance. Keep `best()` only
for the legacy facade and prohibit its use in `AnalysisPipeline`.

#### Lexical word properties

`lexis::Word` should contain only the lemma spelling and permanent `Lexeme` structure needed by
formal generation. Move `native`, `proper`, and `parts` out of default-valued fields:

- origin becomes sourced alternatives such as native, borrowing, and unresolved conflict;
- proper-name status becomes a sourced lexical or named-entity hypothesis;
- morphemic structure becomes references to complete `Segmentation` hypotheses.

The current builder methods `borrowed`, `named`, `prefixed`, and `bare` remain temporarily for laws
and legacy tests. New analysis code builds a compatibility `rules::Facts` view from evidence-backed
hypotheses and must not call `Word::new` to manufacture positive facts.

#### Dictionary source record versus derived semantic assertions

Keep `Sense.definition` and `Idiom.definition` as exact imported text. Split the current mandatory
`Sense.class` and optional derived orientation from the source record into multi-valued assertions:

```rust
pub struct SenseAnalysis {
    pub sense: SenseId,
    pub gloss_document: DocumentId,
    pub readings: ChoiceGroupId,
    pub definition_graphs: Vec<SemanticGraphId>,
    pub class_claims: Vec<EvidenceSetId>,
    pub orientation_claims: Vec<EvidenceSetId>,
}
```

Each class/orientation claim points to its typed value through the evidence graph. Conflicting
dictionary classifications coexist and produce `InconsistentDefinitions`; later import order never
overwrites an earlier source.

#### Imported evidence versus derived proof

Keep `Evidenced<T>` for one imported fact stated at one source locator. Do not stretch it to derived
facts because it can cite only one `Provenance`. Add `EvidenceNode { claim, operation, premises,
sources, grade }` and intern nodes by canonical content. `Confidence` remains attached to imported
or model-proposed candidates; only logical compatibility and explicit norm rules may eliminate a
hypothesis.

#### Dependency compatibility and replacement

Do not add validation fields to public `syntax::Tree`. Introduce:

- `DependencyRelation { base, subtype }` so `nsubj:pass` and `obl:tmod` round-trip;
- `DependencyEdge { head: TokenId, dependent: TokenId, relation, evidence }`;
- validated `DependencyParse` with private storage and constructor checks;
- `DependencyForest` over shared edges and choice groups.

`Tree` remains the legacy projection of one complete parse until `Checker` consumers migrate.

#### Frame validity and constraint results

Keep the public `frame::SlotForm { case, preposition, infinitive, clause }` and current `Frame`
solely for source compatibility and the existing `Verdict` API. Add an internal `SlotRealization`
sum type whose variants are `Nominal`, `Infinitive`, `Clause`, and `Adverbial`, plus a validated
`SemanticFrame` with private slots. New reasoning uses only `SemanticFrame`; keep `SemanticRole`.

The public `verdict::Violation::GovernmentMismatch` continues to expose `frame::SlotForm`. The
legacy projection converts representable `SlotRealization` values into it. The new semantic and
frame layers must not depend on the compatibility type.

Keep pure compatibility helpers where useful, but replace `Constraint::holds_for -> bool` at the
semantic boundary with a result containing `LogicalStatus`, supporting paths, opposing paths, and
missing-data errors. In particular, `Not` may negate only a proved status, and `KindOf` may not pass
solely because two unrelated senses share one coarse `SemanticClass`.

#### Descriptive and normative reports

`AnalysisReport` owns the document, complete packed hypotheses, semantic graphs, definition graph
references, conflicts, evidence DAG, and data manifest. It contains no `Violation` or acceptance
status. `ValidationReport` references hypothesis IDs from an immutable `AnalysisReport` and owns
rule outcomes and edit suggestions. Existing `verdict::{Verdict, Reading, Violation}` remains only
the lossy compatibility projection and must not be used as internal pipeline state.

### What the runtime actually does

The only end-to-end path is `Engine::check` in `src/engine.rs`. It currently:

1. tokenizes a phrase with a private tokenizer;
2. calls an external `Morphology` implementation for each accepted word;
3. reports an empty analysis as `UnknownWord`;
4. reports exclusively predicted analyses as `UnlistedWord`;
5. creates empty semantic `Reading` placeholders;
6. selects the first non-predicted morphological analysis;
7. derives stress only from spelling;
8. guesses proper-name status from capitalization;
9. assumes every word is native Russian;
10. assumes morphemic structure is unknown;
11. runs twelve registered spelling rules;
12. returns a global list of violations.

The runtime does not use the declared `Syntax`, `Lexicon`, `SemanticNetwork`,
`FrameInventory`, `WordFormation`, or `DerivationalNest` ports.

### Principal defects

1. A sentence reading is represented as an empty list of senses rather than a set of linked
   token, morphology, syntax, sense, frame, and semantic assignments.
2. Morphological ambiguity is declared in the model but discarded by the engine.
3. Violations are global and cannot be attached to one interpretation.
4. One fatal finding rejects every otherwise valid interpretation.
5. Tokenization loses punctuation tokens, numbers, foreign fragments, whitespace, and source
   offsets.
6. Adapter-facing `GrammarTag` uses `None` for both unstated and not-applicable categories, but the
   canonical `grammar::form::Form` already represents not-applicable categories structurally. The
   missing piece is a hypothesis boundary that completes tags into every valid `Form` before
   sentence reasoning.
7. `syntax::Tree` does not enforce tree invariants.
8. UD relation subtypes are discarded.
9. `Constraint::holds_for` collapses supported, refuted, conflicting, and unknown states into a
   `bool`.
10. Negating an unknown constraint can incorrectly produce success.
11. `Constraint::KindOf` may accept unrelated senses merely because their coarse classes match.
12. Fallible knowledge ports cannot be represented honestly by the current boolean
    `SenseFacts` interface.
13. `SlotForm` can represent contradictory nominal, infinitive, and clause states at once.
14. Frame invariants are not protected.
15. Imported evidence is discarded before the verdict is built.
16. Derived findings cannot cite all premises from which they were derived.
17. `Confidence` does not define a complete deterministic order.
18. Data-derived semantic classes are represented as singular facts despite observable noisy
    classifications in the generated dictionary pack.
19. The current rule registry covers only twelve executable rules while `rules/map.jsonl`
    describes 357 rule records.
20. Descriptive analysis, normative spelling, acceptability, and semantic truth are mixed in
    one verdict model.

## Architectural Decisions

### Start as a modular monolith

Keep one crate while the domain contracts are being corrected. Splitting into many crates now
would freeze unstable APIs and repeat the coordination cost of the former twelve-crate design.

Consider splitting only after the following boundaries are stable:

- document model;
- evidence and truth model;
- lexical hypothesis model;
- syntax forest;
- semantic graph;
- normative rule interface;
- immutable data-store interface.

### Separate semantic analysis from normative validation

The descriptive API must not contain `Accepted`, `Rejected`, `Violation`, or any other
normative decision.

```rust
pub trait Analyzer {
    fn analyze(&self, text: &str, limits: AnalysisLimits) -> Result<AnalysisReport>;
}

pub trait Validator {
    fn validate(
        &self,
        analysis: &AnalysisReport,
        profile: &NormProfile,
    ) -> Result<ValidationReport>;
}
```

`Analyzer` must completely explain what each reading means, which readings are internally
consistent, and which readings are logically contradictory. `Validator` answers how the
consistent readings relate to one explicitly selected orthographic, grammatical, or stylistic
norm.

### Preserve ambiguity in a packed lattice

Do not construct an unrestricted Cartesian product and do not silently prune alternatives.
Store shared assignments once and represent alternatives through choice groups.

A more specific hypothesis refines a less specific hypothesis by adding assignments or
constraints. Combining incompatible assignments creates an explicit contradiction with a proof.
Reaching a resource limit is an execution failure and must not produce a valid partial semantic
answer.

### Use closed definitional logic

The semantic system must be closed over the configured Russian lexicon. Every admitted lexical
sense must have a machine-readable definition. Recursive expansion must eventually reach a
finite semantic basis or a grounded recursive component.

Semantic compatibility must not contain an `Unknown` value:

```rust
pub enum LogicalStatus {
    Satisfied,
    Violated,
    InconsistentDefinitions,
}
```

Normative evaluation must use a separate result:

```rust
pub enum RuleOutcome {
    Pass,
    Fail,
    NotApplicable,
}
```

Required laws:

- every admitted sense has a complete definition graph;
- every definition node is represented by sense references, relations, variables, logical
  operations, or a recursive component;
- a missing definition is a database-build error, not an analysis result;
- a missing analyzer capability is an engine error, not semantic uncertainty;
- ambiguity is a finite ordered set of complete interpretations;
- an omitted or context-dependent referent is a typed constrained variable;
- contradictory source definitions remain visible as `InconsistentDefinitions`;
- semantic rejection requires a derivation proving incompatible constraints;
- definitions determine literal semantic possibility independently from corpus frequency.

For example, the literal alignment in `камень думает` must be resolved entirely from definitions:

```text
камень
  -> physical object
  -> inanimate
  -> lacks a cognitive subject capability

думать
  -> cognitive event
  -> requires a thinking subject
  -> thinking subject must have cognitive capability

subject(думать, камень)
  -> requires cognitive-capable(камень)
  -> definition entails not cognitive-capable(камень)
  -> logical contradiction
```

This conflict does not require statistical plausibility or external observation. It is a proof from
the literal meanings of `камень` and `думать` and from the predicate's subject restriction. It does
not reject other branches: a figurative/personification interpretation survives when its own
definition or explicit mapping licenses the subject.

### Analyze dictionary definitions as Russian text

A dictionary gloss is not a passive string and is not manually translated into a second semantic
language. It enters the same tokenizer, morphology, syntax, sense, and composition pipeline as any
other Russian text. Sense references found in the gloss recursively load and analyze their own
glosses. Offline pack building and runtime lazy reading are two execution modes of this one
algorithm and must produce the same canonical graph.

The analyzer bootstraps the dictionary as one graph rather than requiring one article to be fully
understood before the next article can be read:

1. import every headword and numbered sense as an addressable node with its exact gloss;
2. analyze morphology and grammar of every gloss and retain all compatible readings;
3. connect words in each gloss to candidate sense nodes instead of expanding them into copied text;
4. propagate grammatical, lexical, contextual, and logical constraints through the whole graph;
5. repeat propagation until no node or edge changes, retaining every still-compatible reading.

Lazy runtime analysis performs the same operations on the reachable subgraph. Encountering a node
already being visited creates a cycle edge; it does not restart expansion.

Recursive definitions are represented as graph cycles and evaluated to a fixed point. A cycle such
as `существо`, `существовать`, and `существование` is not expanded forever and is not silently
discarded. The graph retains the cycle together with every constraint entering it from grammar,
morphemes, other definitions, and dictionary metadata.

The algorithm may use a finite set of typed logical operations to execute relations expressed by
Russian definitions. These operations are implementation anchors, not a manually authored
replacement lexicon and not aliases for particular dictionary words.

The initial logical-operation families must cover at least:

- entity, event, state, property, and relation;
- identity, existence, part, kind, possession, and attribution;
- animate, cognitive, physical, abstract, substance, and information capabilities;
- agent, experiencer, patient, instrument, location, source, goal, and content roles;
- cause, change, creation, destruction, movement, perception, cognition, and communication;
- space, time, quantity, comparison, negation, modality, and logical composition.

Logical operations are not dictionary words. They represent logical and grammatical relations
discovered while analyzing Russian definitions. The dictionary text and its recursive links remain
inspectable source evidence for the resulting graph.

### Make determinism part of every contract

Every public collection must define one complete canonical order. At minimum, ordering keys
must include:

1. source span start;
2. source span end;
3. entity-kind rank;
4. normalized form;
5. stable lemma, sense, or frame key;
6. complete grammatical tuple;
7. provenance key;
8. hypothesis identifier.

Never expose `HashMap` iteration order. Never use adapter insertion order as a tie-breaker.
Never assign persistent IDs from import order.

## Target Pipeline

```text
Source text
  -> lossless document model
  -> tokenization and sentence-boundary hypotheses
  -> morphological hypotheses
  -> stress and morphemic hypotheses
  -> dependency forest
  -> multiword expression and idiom hypotheses
  -> lexical-sense hypotheses
  -> frame and semantic-role hypotheses
  -> context constraints
  -> semantic graphs and propositions
  -> AnalysisReport
  -> NormProfile and normative rules
  -> ValidationReport
  -> legacy Verdict projection
```

### Actual implementation dependency order

Phase numbers group related contracts; they are not permission to execute a task before its inputs
exist. Coding issues must use the following dependency order:

1. Retain the existing alphabet, phonetics, morphemics, `Form`, paradigms, closed classes, local
   dependencies, relations, frames, rules, laws, and ports as the domain foundation.
2. Build the lossless document model and evidence DAG so every later hypothesis has stable token
   identity, source spans, and premises.
3. Convert adapter `GrammarTag` answers into token-linked canonical `Form` hypotheses, enriched and
   checked by existing formal paradigms and reverse reading.
4. Build the packed hypothesis lattice and syntax forest. Apply `agrees`, `said_of`, and `admits`
   while edges are proposed, recording rejected edges as conflicts.
5. Attach raw dictionary sense candidates, closed-class meanings, idioms, frame candidates, and
   morpheme/derivation candidates without selecting one by score.
6. Compose syntax, closed-class meaning, senses, frames, scope, and context into semantic graphs
   whose lexical nodes may still be unresolved `SenseId` references.
7. Run Task 4A.7 over all imported dictionary glosses with that same pipeline and build the global
   candidate sense-definition graph; do not recursively copy definition text.
8. Run Task 4A.7a to lower compositional gloss graphs into typed definition expressions.
9. Run Tasks 4A.8 through 4A.11 in order: validate references, identify recursive components,
   compute the fixed point, and canonicalize complete reachable definition graphs.
10. Run Task 4A.11a to compose affix glosses with the structural derivation chains from Task 3.10.
11. Re-run or incrementally refine ordinary-text hypotheses against the completed reachable
   definition graph. Eliminate only proved conflicts and retain all unresolved alternatives.
12. Validate the completed descriptive report under a selected norm profile and project compatible
   output to `Verdict`.

Consequently, Tasks 4A.0 through 4A.6 define imports and data contracts early, but Tasks 4A.7
through 4A.15, including suffixed tasks, require the exact prerequisites written on each task. In
particular, Task 4A.7
requires Tasks 1.7, 3.1 through 3.9, 5.1 through 5.7, 6.1 through 6.9, 7.1 through 7.7, 8.1 through
8.9, and 9.1 through 9.15; Task 3.10 is independent structural work consumed later by Task 4A.11a.
A new session must not implement dictionary semantic expansion before those prerequisites exist.

## Phase 0: Migration Contract

### Task 0.1: Create the domain glossary

Define `text`, `document`, `sentence`, `surface token`, `word form`, `lexeme`, `sense`,
`hypothesis`, `claim`, `finding`, and `violation`. No term may denote both a descriptive and a
normative concept.

Done when every planned public type uses terms from the glossary consistently.

### Task 0.2: Classify current public APIs

List every public type and function under `src`. Mark each one as:

- retained;
- internally refactored without an API change;
- compatibility-only;
- deprecated after migration;
- replaced immediately because it admits invalid state.

Done when no public item has an unspecified migration path.

### Task 0.3: Define determinism

Specify that output is a pure function of text, code version, data manifest, norm profile, and
analysis limits. Define which timestamps, paths, process state, and import order are forbidden
from affecting output.

Done when every external input to analysis is represented by an argument or manifest entry.

### Task 0.4: Inventory data artifacts

Classify every item under `data` and `rules` as source material, generated pack, trained model,
runtime index, fixture, or log. Record its schema, source revision, generator, and consumers.

Done when no runtime capability depends on an unclassified local file.

### Task 0.5: Record legacy behavior

Describe the exact current behavior of `Engine::check`, including all twelve registered rules,
token cleanup, unknown-word behavior, and status aggregation. This is a compatibility contract,
not the target design.

Done when the old path can later be projected from the new reports without reading the old
implementation.

### Baseline fact: closed classes are present

The current source already contains the closed-class constants and imports under
`grammar::closed`. No recovery task is required. Every later issue must run the ordinary build
checks before editing and must not redesign these inventories as a workaround for an unrelated
failure.

## Phase 1: Lossless Document Model

### Task 1.1: Add `ByteSpan`

Create a validated half-open byte range. Construction must reject inverted bounds and bounds
that are not valid for the owning document.

Done when invalid spans cannot be built through the public API.

### Task 1.2: Add `CharSpan`

Create a character-position span and deterministic conversion from a document byte span.

Done when reports can serve byte-oriented and human-oriented callers without recomputing
offsets inconsistently.

### Task 1.3: Add stable document-local identifiers

Add `DocumentId`, `SentenceId`, and `TokenId` newtypes. IDs must be assigned by canonical source
order, not by processing completion order.

Done when every later assignment can point to an exact document object.

### Task 1.4: Add `TokenKind`

Represent at least word, number, punctuation, whitespace, symbol, emoji, foreign fragment, and
malformed fragment.

Done when no source fragment must be discarded merely because it is not a Russian word.

### Task 1.5: Add `SurfaceToken`

Store token ID, exact source span, exact source text, and token kind. Normalized forms must not
replace source text.

Done when source reconstruction from ordered tokens is byte-identical.

### Task 1.6: Add explicit normalization records

Record case folding, Unicode dash folding, apostrophe folding, Unicode normalization, and
`ё/е` alternatives as evidence-backed transformations.

Done when a caller can distinguish what the author wrote from every normalized lookup key.

### Task 1.7: Implement the lossless lexer

Replace the private engine tokenizer with a lexer that covers the complete source text. Keep
whitespace and punctuation tokens.

Done when every byte belongs to exactly one ordered surface token.

### Task 1.8: Add sentence-boundary hypotheses

Represent ambiguous boundaries rather than committing irreversibly at abbreviations, initials,
decimal points, and ellipses.

Done when boundary ambiguity can enter the hypothesis lattice.

### Task 1.9: Add a legacy token view

Project document tokens into the narrow `rules::Writing` view needed by the current twelve
rules. Do not let the compatibility view become the new document API.

Done when old rules can run without the old tokenizer.

## Phase 2: Logic, Failures, and Evidence

### Task 2.1: Implement `LogicalStatus`

Add `Satisfied`, `Violated`, and `InconsistentDefinitions`. Implement total logical composition
without an unknown state.

Done when every pair of statuses has defined conjunction, disjunction, and negation behavior.

### Task 2.2: Implement `RuleOutcome`

Separate success, failure, and non-applicability.

Done when missing rule requirements produce a typed engine or database error rather than a rule
outcome.

### Task 2.3: Add typed non-semantic failures

Represent invalid input, incomplete definition database, unsupported engine capability, adapter
failure, invalid imported record, and reached execution limit as typed failures outside the
semantic result.

Done when the engine cannot return a successful `AnalysisReport` after skipping a required stage.

### Task 2.4: Extend source identity

Add schema version, source revision, content fingerprint, and external locator semantics to
source metadata.

Done when two different revisions cannot accidentally share an evidence identity.

### Task 2.5: Add `EvidenceId` and `EvidenceNode`

An evidence node must contain a claim, operation, immediate premises, source references, and an
evidence grade.

Done when a derived fact can cite all immediate inputs rather than one arbitrarily chosen
source.

### Task 2.6: Add closed evidence operations

Represent at least imported, normalized, definition-expanded, derived-by-rule, matched, composed,
and contradicted operations.

Done when evidence serialization never relies on an unstructured explanatory string.

### Task 2.7: Add `EvidenceSet`

Allow several supporting and contradicting sources for one claim. Do not overwrite an earlier
source with a later import.

Done when source conflicts produce `LogicalStatus::InconsistentDefinitions` and remain
inspectable.

### Task 2.8: Separate grade from confidence

Keep numeric confidence as source metadata if needed. Use a discrete `EvidenceGrade` for policy
and ordering.

Done when floating-point comparison cannot decide semantic truth.

### Task 2.9: Add `DataManifest`

Record every pack and trained artifact used by one analysis, including schema, revision,
fingerprint, and normalization profile.

Done when an `AnalysisReport` is reproducible from its declared inputs.

## Phase 3: Unified Lexical and Morphological Hypotheses

Implementation order inside this phase is Task 3.1 for type declarations, Task 3.4 for raw adapter
answers, Task 3.2 for strict conversion, Task 3.3 for structurally validated candidates, Task 3.6
for paradigm boundaries, Task 3.7 for formal comparison and final `FormHypothesis` construction,
then Task 3.5 for canonical collection ordering. Tasks 3.8 through 3.12 consume that result.

### Task 3.1: Add token-linked `FormHypothesis`

Declare `LemmaHypothesisId`, `FormHypothesisId`, `LemmaHypothesis`, `TokenAnalysis`,
`ValidatedFormCandidate`, and `FormHypothesis` with the ownership specified in Required
data-structure migration. This task declares records and validating constructors only; it does not
run an adapter or create hypotheses from text.

Done when structurally invalid token/lemma/evidence links cannot be constructed through public APIs
and no record duplicates another phase type. `TokenAnalysis` deliberately permits any unchanged
adapter `GrammarTag`; Tasks 3.2 and 3.3, not its constructor, diagnose linguistic contradictions.

### Task 3.2: Confine `GrammarTag` to adapter boundaries

Keep `GrammarTag` as the normalized adapter wire format already documented in `grammar.rs`. Convert
it with a new strict wrapper around `grammar::form::read::form`. The wrapper must reject stated
categories that the selected form would silently drop, short active participles, and permanent
gender/animacy/aspect/transitivity values inconsistent with the selected `Lexeme`. An impossible
tag is an invalid imported record. A tag missing a category needed to identify a `Form` remains an
incomplete source record and cannot enter sentence reasoning; formal paradigm comparison happens
only in Task 3.7.

Prerequisite: Task 3.4.

Done when no syntax, frame, semantic, or normative API accepts `GrammarTag`.

### Task 3.3: Validate adapter readings against formal forms

For every `morphology::Analysis`, perform strict tag-to-`Form` validation from Task 3.2 and retain
all structurally valid results. This task checks the adapter's stated categories only; comparison
against generated formal paradigms belongs to Task 3.7. Record invalid combinations as evidence
conflicts and do not create a second category hierarchy.

Output: `ValidatedFormCandidate`, not final `FormHypothesis`.

Prerequisite: Task 3.2.

Done when every surviving morphological hypothesis contains an existing canonical `Form` and every
rejected adapter reading names the violated form invariant.

### Task 3.4: Add `TokenAnalysis`

Define `TokenAnalysis` as the raw adapter wrapper: token ID, normalized lookup form, unchanged
`morphology::Analysis`, provenance, grade, and predicted status. It exists before strict conversion
and must not be consumed by syntax or semantics. `FormHypothesis` from Task 3.1 is the canonical
result after Tasks 3.2, 3.3, and 3.7.

Prerequisite: Task 3.1.

Done when no reading is detached from its source token.

### Task 3.5: Preserve every morphological reading

Remove first-reading selection from the new path. Sort alternatives by a complete canonical key.

Done when equal-confidence adapter output produces the same order regardless of insertion order.

Prerequisite: Task 3.7.

### Task 3.6: Clarify the two paradigm models

Keep imported paradigms as source observations and `lexis::paradigm::Paradigm` as the canonical
formal paradigm. Add an explicit comparison adapter between them.

Done when their difference is represented rather than hidden by a conversion.

### Task 3.7: Enrich readings from formal paradigms

Use existing `lexis::paradigm` generation, `Paradigm::cells_of`, and `lexis::reading::of` to support
or refute imported form assignments. This task integrates existing algorithms; it does not rewrite
declension, conjugation, generation, or reverse reading.

Done when formal morphology adds support or a recorded conflict to every structurally valid result
from Task 3.3 without silently deleting external alternatives.

Output: final token-linked `FormHypothesis` values.

Prerequisites: Tasks 3.3 and 3.6.

### Task 3.8: Connect the stress inventory

Wrap existing `phonetics::stress::{of_spelling, of_word, Stressed, Table}` in a token-linked stage.
Return all existing stress variants with source evidence. Preserve spelling-first/table-second
lookup; spelling-derived stress is evidence, not an overriding semantic answer.

Prerequisites: Task 4.9 stress adapter plus Tasks 3.1 through 3.7.

Done when multisyllabic words can receive table-backed stress hypotheses.

### Task 3.9: Connect the morpheme inventory

Call existing `morphemics::cut::ways`, root-shape logic, and `Segmentation::tiling`. Return every
validated segmentation as a token-linked alternative with evidence. Do not add another segment or
tiling type.

Prerequisites: Task 4.9 morpheme adapter plus Tasks 3.1 through 3.7.

Done when every admitted Russian word has one or more complete segmentations or a proved
indecomposable lexical-root analysis.

### Task 3.10: Implement derivational motivation

Implement the missing search over existing `DerivationChain`, `DerivationStep`, `DerivationWay`,
segmentations, known bases, alternations, and derivational nests. This task is structural only: it
returns every valid chain and does not analyze affix meaning.

Prerequisites: Task 4.9 morpheme and derivational-nest adapters plus Task 3.9. Semantic composition
of `Filed::gloss` with base senses belongs to Task 4A.11a after dictionary definition expansion.

Done when a word absent from the lexical index can receive every structurally valid derivation
chain; Task 4A.11a later composes their meanings.

### Task 3.11: Classify forms absent from the lexical index

Separate malformed fragment, foreign word, dictionary gap, predicted Russian form, motivated
formation, misdeclined known lemma, and unexplained form.

Done when every absent form receives a structural classification or a proved rejection as a
Russian word form.

### Task 3.12: Remove unsupported defaults

Reduce `lexis::Word` to lemma spelling plus permanent `Lexeme`. Move `native`, `proper`, and
`rules::Parts` into separate evidence-backed lexical, named-entity, and segmentation hypotheses.
Add a `LawInput` or equivalent borrowed view containing `&Word` plus explicit resolved lexical
properties, and change `Law::broken` and `law::spelled::Spelled` to consume it. First migrate
existing laws and rule fixtures to this input, then remove the fields and the `borrowed`, `named`,
`prefixed`, and `bare` mutators. Build the old `rules::Facts` view at the validation boundary.
Remove capitalization-derived `proper` from analysis.

Done when every positive lexical property follows from a definition, derivation, or formal rule.

## Phase 4: Runtime Knowledge Layer

### Task 4.1: Define a versioned pack envelope

Every pack must declare format magic, schema version, source metadata, generator version, and
content fingerprint.

Done when incompatible artifacts fail with a named reason rather than being partially read.

### Task 4.2: Implement stable entity keys

Add stable `SourceKey`, `LemmaKey`, `SenseKey`, `MorphemeKey`, and `FrameKey` values from source
namespaces and external record keys. Keep existing numeric IDs as dense pack-local references and
assign them by canonical stable-key order. Never present a numeric ID without its pack fingerprint
as persistent cross-build identity.

Done when rebuilding identical source revisions in a different file order produces identical
stable keys and dense IDs, while adding a record may renumber dense IDs without changing stable
keys.

### Task 4.3: Implement `SourceCatalog`

Resolve every provenance reference through one immutable source registry.

Done when a record with an unknown source is reported as invalid rather than silently accepted.

### Task 4.3a: Migrate knowledge-port result contracts

Change `Lexicon::lemmas_of` to return evidence-bearing lemma candidates. Replace
`Lexicon::senses_of` and `Lexicon::sense` results with the source-preserving `SenseRecord` defined
by Task 4.5. Replace
`SemanticNetwork::is_kind_of -> Result<bool>` with a proof-bearing query result or remove it in
favor of the central path query. Replace `frame::SenseFacts` `Option`/`bool` answers with fallible,
evidence-bearing assertions that distinguish absence, contradiction, and adapter failure. Update
`FrameInventory::frames_of` to return evidence-bearing validated `SemanticFrame` values from Task
8.2. Update the forwarding implementations in `ports.rs` in the same task.

Prerequisites: Tasks 2.1 through 2.7, Tasks 4.1 through 4.3, Task 4.5, and Task 8.2.

Done when every knowledge answer used to eliminate a hypothesis carries evidence or a typed
failure and no semantic port reduces it to `bool`.

### Task 4.4: Implement runtime `Lexicon`

Read the explanatory dictionary pack and implement all declared lexicon queries.

Done when forms resolve to lemmas, lemmas to ordered senses, idioms to entries, and every answer
carries evidence.

Prerequisites: Tasks 4.3a and 4.5.

### Task 4.5: Make semantic classification multi-valued

Split the current `Sense` shape into a source-preserving `SenseRecord` and derived
`SenseAssertions`. `SenseRecord` retains ID/key, lemma, source sense number, part of speech, exact
definition, register/domain marks, figurative mark, and examples. Move mandatory singular
`Sense.class` and derived `orientation` out of the source record into multi-valued evidence-backed
assertions. Keep a compatibility conversion only for old consumers that can represent one
uncontested class.

Prerequisites: Tasks 2.7 and 4.2. This contract must complete before Tasks 4.3a and 4.4 update and
implement `Lexicon`.

Done when noisy generated classifications can coexist with corrected source-backed assertions.

### Task 4.6: Implement runtime `SemanticNetwork`

Store relations between senses, produce inverse edges deterministically, and expose evidence
paths.

Done when taxonomy queries return truth, path, and evidence rather than a bare boolean.

Prerequisite: Task 4.3a.

### Task 4.7: Centralize graph traversal

Implement one bounded traversal algorithm over the semantic-network port. Use existing
`RelationKind::inverse` when traversing the reverse direction and `RelationKind::is_transitive` as
the sole closure policy. Relation kinds and depth policies must not be restated in storage adapters.

Done when no backend maintains a separate taxonomy algorithm.

### Task 4.8: Implement runtime `FrameInventory`

Read FrameBank and dictionary-derived frames, preserving rejected and dangling records in an
integrity report.

Done when frame lookup is deterministic and every slot refers to valid domain entities.

Prerequisites: Tasks 4.3a, 8.1, and 8.2. Phase numbering is thematic; a runtime inventory returning
validated `SemanticFrame` values cannot be implemented before the frame model is valid.

### Task 4.9: Implement runtime stress, morpheme, and derivational adapters

Load existing stress and morpheme data through immutable indexed adapters and implement the
existing `WordFormation` and `DerivationalNest` ports. Add a diagnostic stress importer before
constructing `phonetics::stress::Table`: the current table reader silently skips malformed lines,
which is unacceptable for the integrity report. Runtime lookup must use the validated index rather
than reading source files per request.

Prerequisite: Task 4.3a.

Done when stress, segmentation, known-base, and derivational-nest queries are deterministic and
evidence-bearing, and every malformed or skipped source record appears in the integrity report.

### Task 4.10: Add a knowledge integrity report

Report dangling relation targets, duplicate external keys, contradictory metadata, malformed
frames, unresolved classes, and skipped records.

Done when no importer discards a record without an explicit report entry.

## Phase 4A: Executable Definition System

This phase is the semantic foundation of Rusem. Dictionary definitions must stop being passive
prose attached to a sense. They are Russian input to the same analyzer that they help construct.
No task in this phase may require a person to rewrite individual glosses as formal definitions.

### Task 4A.0: Import explanatory dictionaries as the definition source

Definitions are read from existing Russian explanatory dictionaries (Ozhegov, Ushakov,
Yefremova, MAS), not written by hand. Build a deterministic importer that reads each dictionary's
source format into a raw definition record: headword, sense number, exact gloss text, register and
domain marks, grammatical metadata, and source citation. Listing and licensing each chosen source
is part of this task.

Done when every chosen dictionary is imported losslessly into raw records with per-record source
citation.

### Task 4A.1: Define the semantic type system

Add distinct types for entity, event, state, property, relation, quantity, time, location, and
proposition. Define which semantic operations accept and produce each type.

Done when an ill-typed definition such as using a physical entity as a time interval is rejected
while building the definition database.

### Task 4A.2: Define the logical-operation inventory

Create a small, versioned inventory of semantic predicates and operations. Every operation must
have a stable ID, argument types, arity, logical meaning, and Russian rendering template.

Done when operations execute relations derived from text rather than replace dictionary words.

### Task 4A.3: Add definition variables

Represent typed variables, identity, binding, quantification, and role references inside
definitions.

Done when a verb definition can state requirements about its subject, object, and event without
referring to surface token positions.

### Task 4A.4: Add definition expressions

Support operation application, sense reference, conjunction, disjunction, negation, implication,
role restriction, equality, existential binding, and universal restriction.

Done when a lexical definition is a validated expression tree rather than free-form prose.

### Task 4A.5: Add capability constraints

Model capabilities such as animate, cognitive, perceptive, movable, consumable, liquid, physical,
informational, and volitional as definition-level predicates and implications.

Done when predicate argument requirements can be derived from definitions instead of hardcoded
word lists.

### Task 4A.6: Add lexical definition records

Associate every `SenseId` with its imported Russian glosses and source evidence. This is the raw
record available before semantic composition; it contains no `SemanticGraphId`, class claim, or
completed analysis.

Prerequisites: Tasks 4A.0 and 4.2.

Done when every exact source gloss is addressable by stable sense key and source citation.

### Task 4A.7: Analyze dictionary glosses with the core pipeline

Feed every imported gloss through the ordinary Russian analysis pipeline. Resolve words in a gloss
to their dictionary senses, recursively analyze referenced glosses, and apply grammar and context
constraints while hypotheses are created. Build sense nodes and candidate edges; do not expand
cycles or claim a completed canonical definition in this task.

Do not require a referenced sense to have been analyzed first. Build sense nodes and candidate
edges before semantic propagation. A recursive lookup creates a cycle edge rather than recursive
text expansion.

Prerequisites: Task 4A.6 plus Tasks 1.7, 3.1 through 3.9, 5.1 through 5.7, 6.1 through
6.9, 7.1 through 7.7, 8.1 through 8.9, and 9.1 through 9.15.

Done when a selected raw dictionary article becomes a semantic graph without a hand-authored
formal definition and every recursive reference is a finite graph edge.

### Task 4A.7a: Lower gloss graphs into definition expressions

Convert each compositional semantic graph produced by Task 4A.7 into the typed definition language
from Tasks 4A.1 through 4A.5. The lowering rules are structural and shared by every gloss:

- entity, event, state, property, and relation nodes become typed variables or operation
  applications;
- semantic-role edges bind operation arguments by role rather than surface position;
- conjunction, disjunction, negation, implication, modality, and quantifier graph nodes become the
  corresponding definition expressions with preserved scope;
- lexical nodes become `SenseId` references and recursive references remain graph edges;
- every lowered node cites the source semantic node and all syntax/sense/frame evidence that built
  it.

If a semantic graph construct has no lowering rule, return a typed missing-capability failure and
do not emit a partial definition. Alternative semantic graphs become alternative definition
expressions; lowering never chooses one.

Prerequisites: Tasks 4A.1 through 4A.7 and Tasks 9.1 through 9.16.

Done when every supported gloss semantic graph round-trips between its canonical graph and typed
definition expression without losing roles, scope, alternatives, recursion, or evidence.

### Task 4A.8: Validate definition closure

Traverse all sense references and logical operations. Reject dangling source references, type
errors, unbound variables, and invalid arity. Preserve recursive components instead of requiring
every path to end in a manually selected base concept.

Done when every runtime sense belongs to a finite inspectable graph whose recursion is explicit.

Prerequisite: Task 4A.7a.

### Task 4A.9: Analyze recursive definition components

Compute strongly connected components in the definition graph. Merge pure mutual renaming into an
explicit equivalence component; retain all incoming grammatical, morphemic, lexical, and logical
constraints.

Done when `A means B; B means A` terminates as an explicit equivalence component and grounded
recursive structures retain their base conditions and transitions.

Prerequisite: Task 4A.8.

### Task 4A.10: Implement fixed-point expansion

Evaluate grounded recursive components to a canonical least fixed point. Memoize expansion by
sense ID and definition revision.

Done when recursive definitions terminate deterministically without arbitrary depth cutoffs.

Prerequisite: Task 4A.9.

### Task 4A.11: Implement complete sense expansion

Expand a sense into a canonical graph of dictionary sense references, logical operations, typed
variables, constraints, and retained recursive components.

Done when every admitted sense can be explained by its source gloss and recursive graph without
infinite expansion or an unresolved source reference.

Support eager offline indexing and lazy reachable-subgraph analysis through the same
implementation. Populate `SenseAnalysis` records only here, after graph IDs and canonical
definition graphs exist.

Prerequisites: Task 4A.10 and Task 9.16.

Done additionally when eager and lazy execution serialize the same canonical reachable graph.

### Task 4A.11a: Compose derivational meaning

Analyze each lexical affix `Filed::gloss` through the completed dictionary-definition pipeline and
compose its graph with every base-sense graph along each structural `DerivationChain` from Task
3.10. Preserve alternative segmentations, base words, and senses as separate choice groups.

Prerequisites: Tasks 3.10 and 4A.11.

Done when an unfiled but structurally motivated word receives derived semantic alternatives without
hardcoded affix meaning or a guessed single base.

### Task 4A.12: Implement definitional subsumption

Decide whether one expanded definition entails another, contradicts it, or is a stricter subtype.

Done when `камень` can be proved to be an inanimate physical object through its definition chain.

Prerequisite: Task 4A.11.

### Task 4A.13: Implement predicate requirement extraction

Derive argument types, capabilities, semantic roles, and cardinality constraints from a
predicate's expanded definition and frames.

Done when `думать` yields a requirement for a cognitive-capable thinking subject without a
special case for the lemma.

Prerequisites: Tasks 4A.11, 4A.12, and 8.2.

### Task 4A.14: Implement definitional contradiction proofs

Combine an argument's expanded definition with a predicate role requirement and produce the
minimal incompatible predicate pair and their derivation paths.

Done when the literal `камень думает` alignment produces a proof that the subject is definitionally
inanimate and lacks the capability required by the cognitive event without deleting independently
licensed figurative alignments.

Prerequisite: Task 4A.13.

### Task 4A.15: Implement recursive human explanation

Render a semantic graph into Russian explanations at configurable detail levels. Preserve sense
IDs and graph links so rendering does not become the source of semantics.

Done when a caller can request a short definition, expanded definition, or operation-level proof
for the same analysis.

Prerequisite: Task 4A.14.

### Task 4A.16: Define totality boundaries

Specify exactly which inputs are admitted as Russian expressions. A well-formed admitted input
must always yield complete interpretations or contradiction proofs. A rejected input must carry
a proof such as invalid encoding, no valid tokenization, or no valid morphological derivation.

During migration, publish an explicit capability manifest naming the constructions not yet
implemented in Phase 13. Encountering one is a typed missing-capability failure, never `Unknown`
and never a successful partial meaning. The final project completion criterion requires removing
those exclusions for well-formed Russian rather than redefining ordinary Russian as inadmissible.

Done when `Unknown` is not a possible successful semantic result.

## Phase 5: Packed Hypothesis Lattice

### Task 5.1: Add hypothesis identifiers and nodes

Each node records parent hypotheses, one added assignment or constraint, evidence dependencies,
and a canonical key.

Done when a complete interpretation can be traced to its incremental derivation.

### Task 5.2: Add typed assignments

Represent tokenization, morphology, stress, segmentation, dependency, MWE, sense, frame, slot,
and semantic assignments as closed variants.

Done when assignment compatibility can be checked without string keys.

### Task 5.3: Add `ChoiceGroup`

Group mutually exclusive alternatives without copying all shared assignments.

Done when one ambiguous token does not duplicate the rest of the sentence state.

### Task 5.4: Add explicit conflicts

Record incompatible assignments and the evidence that created them.

Done when elimination can explain both conflicting alternatives.

### Task 5.5: Implement hypothesis refinement

Define a partial order in which a child adds information without changing established parent
assignments. Grammar rules, agreement, and compatibility checks apply when a child is proposed.
An incompatible proposal creates a compact conflict record with its premises, but no expandable
child hypothesis; the engine does not build a complete branch and filter it afterward.

Done when refinement is monotonic.

### Task 5.6: Implement compatible meet

Combine compatible constraints and produce an explicit bottom/conflict for incompatible ones.
Rule application during combination is constrain-first, not generate-then-filter: the search
space is bounded by the rules themselves at combination time. Preserve the failed meet as evidence
for elimination without retaining it as a branch eligible for further expansion.

Done when combination never silently chooses one side.

### Task 5.7: Intern equivalent nodes

Use canonical keys to share equivalent partial hypotheses.

Done when storage growth follows unique assignments rather than the full Cartesian product.

### Task 5.8: Add explicit analysis limits

Include maximum nodes, paths, graph depth, and work units in `AnalysisLimits`.

Done when limits are arguments and appear in the report.

### Task 5.9: Report truncation

When a limit is reached, return a typed execution-limit error with consumed and required work
metadata. Do not return the completed prefix as a successful semantic report.

Done when a successful report is always semantically complete for the admitted input.

## Phase 6: Dependency Forest

### Task 6.1: Preserve complete UD relations

Represent base relation and subtype separately. Do not fold `nsubj:pass`, `obl:tmod`, or
`nmod:poss` into one coarse label.

Done when imported and predicted UD labels round-trip without semantic loss.

### Task 6.2: Add `DependencyEdge`

An edge must reference token hypotheses, relation, direction, and evidence.

Done when edges cannot point to an unrelated tokenization branch.

### Task 6.3: Add validated `DependencyParse`

Protect root count, bounds, acyclicity, reachability, and tokenization compatibility.

Done when invalid parses cannot be constructed through the public API.

### Task 6.4: Replace the syntax-port input

Accept lexical hypotheses rather than parallel `forms` and `readings` arrays.

Done when every parser input analysis is unambiguously tied to one token.

### Task 6.4a: Generate the complete dependency candidate set

Implement deterministic candidate generation independently from model scores. For every compatible
tokenization and sentence-boundary hypothesis, propose every root and every head, dependent, and
complete UD relation combination not structurally impossible by token identity or boundary. Parts
of speech, canonical `Form`, closed-class behavior, punctuation, agreement, predication, and
government are constraints applied later by Task 6.6; full clause structure is derived later by
Task 6.7. Candidate generation must not assume either result.

A trained parser may prioritize this finite candidate stream but may not add semantic truth, remove
a candidate, or define completeness. Candidate generation must expose counters so analysis limits
measure proposed and retained work separately.

Prerequisites: Tasks 1.8, 3.1 through 3.9, 5.1 through 5.7, and Tasks 6.1 through 6.4.

Done when a sentence receives the same candidate edge set with no parser model, with a parser model,
and under different candidate traversal orders.

### Task 6.5: Add `DependencyForest`

Store alternative heads and labels in packed form.

Done when equally supported parses survive together.

Prerequisites: Tasks 6.1 through 6.4a.

### Task 6.6: Apply grammatical compatibility

Use existing `grammar::dependency::agrees`, `predicate::said_of`, and `governed::admits` as pure
constraints while dependency edges are proposed. Wrap each result with token IDs and premises. An
applicable missing canonical `Form` is an incomplete lexical hypothesis, not agreement failure.

Prerequisite: Task 6.5.

Done when grammar either refines the forest or reports an inconsistent source record.

### Task 6.7: Add clause structure

Represent main, coordinated, subordinate, relative, infinitive, participial, and gerund clauses.

Prerequisite: Task 6.6.

Done when clause-dependent rules no longer infer clause boundaries from adjacent words.

### Task 6.8: Add coordination structure

Represent conjunction type, members, shared dependents, and coordination scope.

Done when a filler may be shared only through an explicit coordination hypothesis.

### Task 6.9: Add ellipsis placeholders

Represent omitted predicates and participants as typed symbolic variables carrying every
constraint derivable from morphology, syntax, semantics, and context.

Done when predicateless surface clauses receive complete symbolic analyses without invented
lexical content.

### Task 6.10: Version parser models

Add magic, schema revision, feature fingerprint, relation fingerprint, dimensions, and checksum
to `ru.weights`.

A trained model may order the deterministic candidate stream for performance, but it may not add,
remove, or originate an edge and may not define the exhaustive set of syntactically possible
parses. Deterministic candidate generation, grammar constraints, and the packed forest establish
completeness; model scores remain metadata.

Done when a reordered relation inventory cannot silently load an incompatible model.

### Task 6.11: Define canonical parse order

Order complete and partial parses independently of traversal and hash-table order.

Done when equivalent forests serialize identically.

## Phase 7: Senses, Multiword Expressions, and Idioms

### Task 7.1: Add `SenseChoice`

Tie every sense candidate to token or MWE span, lemma hypothesis, part of speech, source, and
evidence.

Done when a sense cannot be selected for an incompatible lemma or part of speech.

### Task 7.2: Build an idiom index

Index idioms by lemma alternatives while preserving required word forms and permitted syntactic
variation.

Done when idiom lookup does not require a single preselected morphological reading.

### Task 7.3: Represent overlapping MWEs

Put overlapping candidates into explicit conflict groups.

Done when choosing one expression does not silently erase another.

### Task 7.4: Add grammatical MWEs

Index the compound prepositions, conjunctions, particles, correlates, and semantic distinctions
already present under `grammar::closed`. Add only missing analytical forms and source-span matching;
do not duplicate the closed-class word lists in an MWE store.

Done when their internal words are not incorrectly analyzed as independent free participants.

### Task 7.5: Add register and domain facts

Carry register and subject domain into hypotheses without rejecting them in the descriptive
layer.

Done when norm policy, not the lexicon, decides whether a marked reading is acceptable.

### Task 7.6: Implement evidence-backed taxonomy queries

Return truth, shortest admissible paths, conflicting paths, and traversal limits.

Done when a selectional decision can cite the exact semantic chain.

### Task 7.7: Integrate central semantic traversal

Route taxonomy and definition queries through the traversal from Task 4.7. Preserve
`RelationKind::is_transitive`; never add local closure rules for synonyms, antonyms, causes, or
instance links.

Done when all sense-selection constraints share one evidence-bearing path implementation and no
consumer implements its own graph walk.

## Phase 8: Frames and Semantic Roles

### Task 8.1: Add valid `SlotRealization` state

Keep public `frame::SlotForm` unchanged for current Rust and serialization compatibility. Add the
internal `SlotRealization` sum type with nominal, infinitive, clause, and adverbial variants;
nominal realization contains case and optional preposition. New frame reasoning uses only this
type.

Convert new variants back to the existing `SlotForm` only while projecting a representable legacy
`GovernmentMismatch`. Do not make the semantic layer depend on the compatibility type.

Done when a slot cannot simultaneously be nominal, infinitival, and clausal.

### Task 8.2: Validate frames

Add `SemanticFrame` as the validated internal replacement for the public compatibility `Frame`.
Protect frame identity, owning sense, role uniqueness rules, `SlotRealization` alternatives, and
required participant semantics with private fields and a validating constructor.

Prerequisite: Task 8.1.

Done when malformed imported frames cannot enter runtime reasoning as valid frames.

### Task 8.3: Extract participant candidates from syntax

Use dependency edges and clause structure, not linear windows.

Done when non-adjacent ordinary Russian dependencies remain visible.

### Task 8.4: Separate surface and semantic matching

Compute case/preposition/clause compatibility independently from semantic restrictions.

Done when a report can say whether grammar, semantics, both, or neither caused a mismatch.

### Task 8.5: Add `SlotMatch`

Store frame, slot, filler, surface result, semantic result, and evidence paths.

Done when every accepted or refused participant assignment is explainable.

### Task 8.6: Enumerate maximal frame alignments

Preserve every maximal compatible filler-to-slot alignment. Do not select the first frame or the
frame with an undocumented heuristic score.

Done when equal alignments remain alternatives in canonical order.

### Task 8.7: Model omitted participants

Distinguish optional, contextually implicit, syntactically elided, and truly missing required
participants.

Done when an absent surface noun does not automatically create a frame violation.

### Task 8.8: Handle voice

Project active, passive, reflexive, impersonal, and participial constructions onto semantic
roles.

Done when grammatical subject and semantic agent are not assumed identical.

### Task 8.9: Handle shared coordinated fillers

Allow one filler to satisfy several predicates only when syntax and slot realizations support
sharing.

Done when incompatible government produces a local conflict rather than a global guess.

## Phase 9: Compositional Semantics

### Task 9.1: Define the semantic graph

Add entity, event, state, property, reference, operator, and proposition nodes. Every node and
edge must point to source hypotheses and evidence.

Done when one sentence interpretation can be represented without flattening it into sense IDs.

### Task 9.2: Project nominal senses

Create entity or abstract-reference nodes from nominal hypotheses.

Done when different nominal senses create different semantic branches.

### Task 9.3: Project predicate senses

Create event, state, or property nodes tied to frame alignments.

Done when predicate meaning and participant mapping are represented together.

### Task 9.4: Compose attributes and nominal modifiers

Attach properties and relations according to dependency structure and sense compatibility.

Done when attachment ambiguity creates alternatives rather than nearest-word attachment.

### Task 9.5: Add negation scope

Reuse the denying-particle inventory and particle senses in `grammar::closed::particle`. Represent
negation as an operator over every proposition or predicate attachment licensed by syntax.

Done when a nearby `не` is not assumed to deny the wrong semantic object.

### Task 9.6: Add modality

Seed possibility, necessity, ability, desire, obligation, certainty, and doubt from the existing
predicative, parenthetical, particle, and parenthetical-sense inventories. Represent each as an
operator separately from asserted events.

Done when modal content is not extracted as an unconditional world claim.

### Task 9.7: Add question semantics

Reuse `grammar::closed::asking` and interrogative pronoun/adverb classes. Represent interrogative
scope and requested variables.

Done when questions are not treated as assertions.

### Task 9.8: Add conditional semantics

Represent antecedent and consequent without asserting either unconditionally.

Done when hypothetical clauses cannot trigger factual contradiction checks as plain claims.

### Task 9.9: Add temporal composition

Combine morphological tense, lexical time orientation, temporal modifiers, and clause relations.

Done when temporal disagreement is represented per hypothesis before normative evaluation.

### Task 9.10: Add aspectual composition

Represent completion, process, repetition, phase, and result-state relations.

Done when tense and aspect are no longer conflated.

### Task 9.11: Add quantification

Represent universal, existential, negative, numeric, collective, and approximate quantifiers.

Done when semantic claims preserve quantifier scope.

### Task 9.12: Add coordination operators

Reuse the existing conjunction inventory and coordination/subordination meanings under
`grammar::closed::conjunction`. Represent conjunction, disjunction, contrast, and paired
coordination without reducing them to one generic junction.

Done when `и` and `или` no longer produce the same graph shape.

### Task 9.13: Add reported speech and quotation

Represent speaker, reported proposition, and quotation boundaries.

Done when quoted or reported claims are not attributed directly to the document author.

### Task 9.14: Add symbolic references

Represent pronouns, omitted subjects, deixis, and context-dependent antecedents as typed variables
with identity, animacy, number, gender, person, deixis, and role constraints.

Done when lack of an explicit antecedent still produces a complete symbolic meaning without
fabricating a concrete referent.

### Task 9.15: Apply context constraints

Use preceding and following sentence hypotheses to add grammatical, referential, lexical, and
semantic constraints. Context removes a reading only when it creates a proved incompatibility; it
must not select a reading merely because it is statistically more likely.

Done when context can reduce an ambiguous reading set with an evidence path while genuinely
unresolved readings remain explicit alternatives.

### Task 9.16: Canonicalize semantic graphs

Define structural canonical keys and merge equivalent graphs.

Done when equivalent derivations share one semantic alternative while preserving all evidence.

### Task 9.17: Extract explicit claims

Create knowledge-network claims only from documented graph patterns and only with preserved
scope, modality, negation, and attribution.

Done when contradiction checking cannot consume a bare adjacent-word pattern.

## Phase 10: Normative Validation

### Task 10.1: Define `NormProfile`

Include orthographic revision, punctuation revision, `ё` policy, allowed registers, treatment of
borrowings, new words, and semantic-contradiction presentation policy.

Done when normative behavior has no hidden global defaults.

### Task 10.2: Define `NormRule`

Evolve the existing `rules::Rule`, `Scope`, `Citation`, `Facts`, and `Found` contracts. Keep each
implemented paragraph's `found` logic. Add explicit requirements, applicability, and evaluation
around it rather than introducing a second unrelated rule hierarchy.

Done when a missing prerequisite is observable and cannot compile into silent success.

### Task 10.3: Define `RuleEvaluation`

Store outcome, affected spans, affected hypotheses, evidence, and suggested edits.

Done when one rule can explain pass, fail, and non-applicability.

### Task 10.4: Migrate the twelve current rules

Adapt them to the new document and analysis views without rewriting their linguistic decision
logic.

Done when the compatibility projection can reproduce the intended legacy findings.

### Task 10.5: Separate word and span edits

Represent replacement of one word, insertion of punctuation, deletion, and replacement of a
multi-token span as distinct edit types.

Done when an interjection-comma correction is not stored as a replacement word containing two
words.

### Task 10.6: Implement agreement validation

Project agreement conflict records produced by Task 6.6 into norm findings under the selected
profile. Do not call `grammar::dependency::agrees` a second time. The descriptive layer owns the
linguistic compatibility decision; validation decides only whether and how that proved conflict is
reported as a norm violation.

Done when a mismatch is fatal only for hypotheses in which the relevant words are related and
their categories are known.

### Task 10.7: Implement government validation

Project government conflict records from Tasks 6.6 and 8.4 into norm findings. Preserve the
governing lemma or sense, preposition, case, syntactic relation, frame slot, and evidence paths
already recorded. Do not recompute closed-class or frame compatibility in validation.

Done when a finding names the exact failed expectation.

### Task 10.8: Implement selectional validation

Consume definitional contradiction proofs produced during semantic analysis. Normative validation
must not independently guess semantic compatibility.

Done when `SelectionalMismatch` always contains a minimal proof from expanded definitions.

### Task 10.9: Implement knowledge-claim validation

Distinguish definitionally satisfied claims, definitionally contradicted claims, and inconsistent
definition data. A missing required definition is a database failure, not a claim status.

Done when every contradiction includes an explicit opposing path.

### Task 10.10: Move register policy into validation

Marked, figurative, dialectal, obsolete, and technical readings remain descriptive facts.

Done when different profiles can evaluate the same analysis differently without rerunning
analysis.

### Task 10.11: Classify all mapped rules

For every record in `rules/map.jsonl`, assign one status:

- executable;
- blocked by an incomplete definition or linguistic implementation task;
- declarative but not yet detected in text;
- dictionary-only;
- not mechanically decidable;
- obsolete for the selected norm revision.

Done when rule coverage is measurable without pretending all 357 records execute.

### Task 10.12: Implement rules by families

Create reusable evaluators for rule families such as stress-dependent spelling, morpheme-boundary
spelling, paradigm comparison, separate/together writing, hyphenation, punctuation, and
syntactic agreement.

Done when adding a rule record usually means adding data and a focused detector rather than a
new bespoke architecture.

### Task 10.13: Deduplicate findings

Merge findings with the same rule and source span while preserving every supporting hypothesis
and evidence path.

Done when ambiguity does not flood the report with duplicate messages.

### Task 10.14: Order findings canonically

Order by span, citation, finding kind, edit, and evidence key.

Done when output order is independent of rule execution order.

## Phase 11: Result Models

### Task 11.1: Add `AnalysisReport`

It must contain the lossless document, packed complete hypotheses, semantic graphs, contradiction
proofs, evidence graph, data manifest, and analysis limits.

Done when descriptive analysis requires no `Verdict` type.

### Task 11.2: Enforce successful-report totality

Make `AnalysisReport` constructible only after every mandatory stage has produced a complete
result. Invalid data, missing capability, and execution limits remain errors.

Done when every successful report is complete by construction.

### Task 11.3: Add `ValidationReport`

Store surviving hypotheses, eliminated hypotheses, rule evaluations, deduplicated findings,
unchecked rules, norm profile identity, and aggregate presentation status.

Done when validation never mutates or deletes the original `AnalysisReport`.

### Task 11.4: Record hypothesis elimination

Each elimination must name conflicting assignments or failed rules and their evidence.

Done when every removed interpretation has a machine-readable reason.

### Task 11.5: Add edit suggestions

Represent edits against source spans with preconditions. Do not store only a free-form corrected
string.

Done when several non-overlapping edits can be applied deterministically.

### Task 11.6: Add the legacy projection

Map representable findings and readings into the current `Verdict` API. Preserve unprojectable
details in the new reports rather than discarding them globally.

Done when old consumers can migrate independently from the engine internals.

## Phase 12: Engine Migration

### Task 12.1: Introduce `AnalysisPipeline`

Create an internal orchestrator with explicit immutable stage dependencies. Keep the current
`Engine` public API unchanged in this task.

Done when the new pipeline can be constructed without changing legacy behavior.

### Task 12.2: Route tokenization through the document model

Use the lossless lexer and legacy token projection for the current checker.

Done when the old tokenizer has no unique behavior.

### Task 12.3: Add `Engine::analyze`

Expose the descriptive report without invoking normative rules.

Done when morphology-only analysis works through the new API.

### Task 12.4: Add `Engine::validate`

Validate an existing report under an explicit profile.

Done when the same report can be evaluated under two profiles without recomputation.

### Task 12.5: Connect lexical enrichment stages

Connect stress, morphemes, derivation, and knowledge adapters one stage at a time. One adapter is
one task.

Done when construction fails with a typed configuration error if a mandatory adapter is absent.

### Task 12.6: Connect the syntax stage

Add dependency forests as mandatory input to sentence-level semantic composition.

Done when sentence analysis cannot report success without a complete syntax forest.

### Task 12.7: Connect senses and MWEs

Add lexical-sense and expression alternatives without selecting one globally.

Done when sentence readings contain actual linked sense assignments.

### Task 12.8: Connect frames and semantic composition

Add frame alignments, semantic roles, graphs, and claims incrementally.

Done when semantic findings can cite syntax, sense, frame, and relation evidence.

### Task 12.9: Reimplement `Checker::check` as a facade

Implement it as `analyze -> validate(default profile) -> legacy projection`.

Done when it no longer contains its own tokenizer, first-reading selection, or rule loop.

### Task 12.10: Retire placeholder readings

Mark the current empty `verdict::Reading` representation as compatibility-only and remove it
from internal reasoning.

Done when no analysis stage constructs an empty reading to indicate success.

### Task 12.11: Verify removal of unsupported defaults

Task 3.12 owns the actual removal of `native = true`, capitalization-derived proper-name status,
and unconditional unknown morphemic structure. This integration task only verifies that the final
facade cannot recreate those defaults.

Done when an end-to-end test proves these facts arise only from evidence-backed hypotheses.

### Task 12.12: Remove the legacy internal path

Delete old tokenizer and direct `rules::all()` execution only after the compatibility facade no
longer calls them.

Done when there is one end-to-end analysis path.

## Phase 13: Later Linguistic Coverage

These capabilities belong after the core semantic graph is stable:

1. named-entity recognition as hypotheses rather than capitalization rules;
2. abbreviation expansion;
3. dates, times, measures, currencies, and numeric expressions;
4. comparative and superlative constructions;
5. distributive and collective readings;
6. long-range discourse coreference beyond the context constraints required by Task 9.15;
7. discourse relations;
8. information structure and topicalization;
9. presupposition and implicature as explicitly non-factual layers;
10. metaphor and metonymy as marked alternative mappings;
11. terminology and domain-specific norm profiles;
12. historical and pre-reform profiles;
13. direct speech and nested quotation;
14. parenthetical and inserted constructions;
15. non-projective syntax;
16. productive neologisms and compounds;
17. foreign inclusions and transliteration;
18. typo hypotheses distinct from accepted orthographic forms.

## Work-Package Rules for Limited-Context Agents

Every implementation issue must follow this template:

```text
Goal:
One domain change only.

Prerequisites:
Exact completed task IDs.

Files allowed:
One primary module and its direct exports or consumers.

Inputs:
Existing types and invariants the implementation may rely on.

Outputs:
Exact new or changed types and signatures.

Forbidden shortcuts:
Defaults, silent pruning, successful partial output, insertion-order ties, and free-form evidence.

Definition of done:
One externally observable invariant.

Out of scope:
Every adjacent integration that belongs to a later task.
```

Good task example:

```text
Implement LogicalStatus in src/logical_status.rs.

Prerequisites:
Task 0.1.

Files allowed:
src/logical_status.rs and the export in src/lib.rs.

Outputs:
LogicalStatus::{Satisfied, Violated, InconsistentDefinitions}; total not, and, or operations.

Forbidden shortcuts:
Do not add an unknown state. Do not modify frame constraints.

Definition of done:
Every unary and binary operation has a deterministic result, and contradictory definitions stay
distinguishable from a violated semantic constraint.
```

Bad task example:

```text
Implement semantic analysis and connect it to Engine.
```

## Recommended First Vertical Slice

The first milestone should contain only:

1. validated source spans;
2. a lossless document and token model;
3. closed `LogicalStatus` without an unknown result;
4. an evidence DAG;
5. canonical ordering;
6. every morphological reading for each token;
7. strict conversion into token-linked canonical `Form` hypotheses;
8. a packed hypothesis lattice with recorded failed meets;
9. deterministic dependency candidate generation and a validated packed syntax forest;
10. imported unmodified articles from a real explanatory dictionary;
11. token/MWE sense candidates and the minimal frames needed by the examples;
12. compositional semantic graphs with negation and alternative scope;
13. recursive analysis of dictionary articles through that ordinary Russian pipeline and
    structural lowering of gloss graphs into typed definition expressions;
14. a minimal typed logical-operation inventory sufficient to execute the derived graph;
15. SCC and fixed-point expansion of recursive definition graphs;
16. frame alignment, definition requirement extraction, and contradiction proofs;
17. a punctuation insertion detector for ambiguous clause boundaries;
18. typed execution failures instead of successful partial reports;
19. a minimal complete `AnalysisReport`;
20. a compatibility view for the twelve existing rules;
21. a minimal `ValidationReport`;
22. projection back to the current `Verdict`.

This milestone may select a small set of articles from the imported dictionary for execution
speed, but their glosses must remain unmodified and no lexical meaning may be hand-authored. It
must perform one complete semantic derivation. Its acceptance examples are:

1. `камень думает`: both words are recursively expanded. The literal stone/think alignment derives
   an incompatible subject requirement and retains its proof as a conflict. A figurative or
   personification branch survives only if an imported sense, definition, or explicit mapping
   licenses it; the engine must not globally reject the surface sentence.
2. `казнить нельзя помиловать`: both words and the negation resolve without punctuation, the
   lattice keeps two complete interpretations with opposite senses, the punctuation rule names
   both valid insertion points as findings, and no interpretation is silently dropped.
3. An imported dictionary gloss is tokenized and analyzed as ordinary Russian, recursively follows
   the senses used in its explanation, terminates on cycles, and yields the same canonical graph
   in eager offline and lazy runtime modes.

Connecting the full local datasets before this slice is complete would preserve the current
conflation of lookup, heuristic selection, normative judgment, and semantic truth.

## Explicit Non-Goals

- Do not restore the former twelve-crate architecture verbatim.
- Do not restore a multi-thousand-line checker with hardcoded lexical lists.
- Do not treat a deterministic algorithm as permission to return one arbitrary reading.
- Do not treat corpus frequency as truth.
- Do not require hand-authored formal definitions before dictionary text can enter runtime.
- Do not return `Unknown`, a gap, or partial success for a well-formed admitted Russian input.
- Do not hardcode `камень`, `думать`, or other lexical pairs into compatibility checks.
- Do not use corpus likelihood as a substitute for a definitional proof.
- Do not let a trained model produce untraceable final facts.
- Do not put descriptive and normative decisions in the same type.
- Do not implement all 357 rule records as unrelated bespoke modules.
- Do not split the crate until the new domain boundaries are stable.
- Do not remove the legacy facade before the new pipeline can project its behavior.

## Completion Criterion

The project reaches its intended architectural goal when a caller can submit Russian text and
receive:

- a lossless structural representation of the original document;
- every retained morphological, stress, morphemic, syntactic, lexical, frame, and semantic
  alternative;
- explicit links between all alternatives;
- explicit conflicts and elimination reasons;
- semantic propositions with scope, modality, negation, time, quantification, and attribution;
- recursive explanations of every sense through source dictionary glosses, logical operations,
  and explicit fixed-point components;
- definitional compatibility or contradiction proofs for every predicate argument;
- source and derivation evidence for every returned fact;
- typed symbolic variables for referents omitted from the text;
- deterministic canonical ordering;
- a separate normative report under an explicitly identified Russian norm profile;
- compatibility output for existing `Verdict` consumers.

The engine must never return `Unknown` for a well-formed admitted Russian expression. It must
return complete interpretations, complete symbolic interpretations with constrained variables,
or definitionally proved contradictions. Missing definitions, missing mandatory stages, and
resource exhaustion are failures of the database or engine and cannot be successful linguistic
answers. That is the central invariant of the design.
