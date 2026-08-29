# Remediation plan

Ten defects of the engine, what each one should look like instead, and the
steps that get there. Ordered by what a mistake costs: the first four let a
wrong answer ship without anything going red, the rest cost work rather than
correctness.

**Scope: the engine only** — `rusem-core`, `rusem-morph`, `rusem-derive`,
`rusem-stress`, `rusem-syntax`, `rusem-knowledge`, `rusem-store`,
`rusem-verify`, `rusem`, `rusem-build`. The two surfaces are out of scope and
stay untouched: they are the outermost ring, they hold `rusem` and nothing
else, and there is no point shaping a console around an engine that is still
being taken apart. They get rebuilt last, over whatever the engine ends up
exposing.

## 0. The criterion every fix is measured against

A mistake must be **impossible or loud, never silent**.

The engine is built so that every fact carries the source it came from — that
is the whole point of `Evidenced<T>`. The defects below are all the same
failure of that principle at the level of the *machinery* rather than the data:
a rule that is not registered, a layer that is not wired, a variant that is not
described, a model whose relation set does not match — none of them fails. They
all quietly answer something else.

So each fix is judged by one question: **after it, does forgetting still
compile?** If yes, the fix is not done.

---

## 1. The rules are a trait

### What is there now

Three registries, none of them enforced.

| Where | How many | Shape | Registered by |
| --- | --- | --- | --- |
| `rules/svod/` | 357 files, 195 directories, **§ 1 … § 203, every paragraph of the code, no gaps** | `pub const PARAGRAPH`, `pub const POINT`, `pub struct Asked`, `pub enum Written`, `pub fn required(&Asked) -> Option<Written>` | **nothing** — not a workspace member, no crate path-depends on it, no `include!` reaches it |
| `crates/rusem-verify/src/svod/` | 9 files, **7 paragraphs** (§ 140, 149, 151, 153, 157, 164, 166 — all punctuation) | `pub fn <name>(&[Word]) -> Vec<usize>` | a hand-written chain of seven `violations.extend(…)` lines in `svod::found` |
| `crates/rusem-verify/src/gates/` | 33 modules | a free `pub fn` per gate, **each with a different signature** | two hand-written chains: `fn agreement` (`checker.rs:2640`, ~30 calls) and `Verifier::attempt` (`checker.rs:496`, 8 calls) |

**196 of the 203 paragraphs are written, tested against the book, and
unreachable from any binary.** They are not "not done" — they are done and
disconnected. `cargo check` does not see them, `cargo clippy` does not lint
them, nothing can score them, and if `WordForm` changed under them nothing
would say so.

### Why a trait, specifically

Four reasons, in order of force. The first is the one that makes it not a
matter of taste.

**1. A trait is the only thing that lets 357 rules exist in one collection.**

Each staged rule declares its *own* question and its *own* answer:

```rust
/// rules/svod/hard-sign-before-vowels/foreign.rs
pub struct Asked { pub following_first: char }
pub enum Written { HardSign }
pub fn required(asked: &Asked) -> Option<Written>;

/// rules/svod/ne-separately/verbs.rs
pub enum PartOfSpeech { Verb, Gerund }
pub enum Written { Separately }
pub fn required(part: PartOfSpeech) -> Option<Written>;
```

These two functions have no type in common. Rust cannot put them in the same
`Vec`, cannot iterate them, cannot count them. There is no `fn` pointer type
that fits both, no enum that could hold them without being rewritten every time
a rule is added. A trait object is the minimum construct in the language that
supplies the missing shared type. That is not a design preference; it is the
reason the 357 files are sitting outside the build in the first place — **there
was nowhere to put them.**

**2. A trait is where the citation stops being decoration and becomes a
requirement.**

`PARAGRAPH` and `POINT` are `pub const`s that nothing reads. They can be wrong,
duplicated, or missing and nothing notices — one file already has no public
function at all. As a required trait method:

```rust
fn cites(&self) -> Citation;
```

a rule that does not name its paragraph does not compile. And because the
method returns the citation to the *caller*, the violation can carry it: today
`Violation::MissingComma { before }` says a comma is missing and cannot say
which paragraph asks for it, even though `svod::Paragraph` exists and
`comma::clauses` knows the answer. The reader gets «§ 140» instead of a bare
assertion, which is the difference between a checker and an oracle.

**3. A trait splits the two ways a rule can be wrong, so tests can say which.**

A rule does two separate jobs:

| Job | Today | Where it belongs |
| --- | --- | --- |
| **State** the rule: given this question, this answer | `required(&Asked) -> Option<Written>` — pure, no engine types, testable against the book's own examples | stays exactly as it is |
| **Find** the question inside a phrase: which word, which position, is this the case the rule is about | does not exist for 196 paragraphs; is fused into the gate function for the rest | the trait's `found()` |

Keeping them apart is worth more than it looks. `required()` is checkable
against the book by a person who does not know the engine. `found()` is
checkable against real phrases by a person who does not know the book. Fused
together — which is what every `gates/*.rs` file is today — a failing phrase
says only "something in this gate is wrong", and the 200-line gate has to be
read whole. Split, the failing test names the half.

**4. Object safety keeps it free.**

`&'static dyn Rule` in a `const` slice: no allocation, no lazy init, no
registry type, one pointer per rule. The dispatch cost is one indirect call per
rule per phrase, against gates that already walk the word list.

### The shape

```rust
/// Where a rule is written down, so a finding can name it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Citation {
    /// The paragraph of the code of 1956.
    pub paragraph: u16,
    /// The point within the paragraph, or zero when the paragraph is whole.
    pub point:     u16
}

/// What a rule must be given before it can speak.
///
/// A rule that needs the government table and is run without one must not
/// quietly find nothing; it must be reported as unavailable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Needs {
    /// The rule reads the reading settled on for each word.
    pub readings:   bool,
    /// The rule reads the dependency tree.
    pub tree:       bool,
    /// The rule asks the lexicon for senses.
    pub lexicon:    bool,
    /// The rule reads the table of what nouns were seen governing.
    pub government: bool,
    /// The rule reads the corpus frequencies of readings.
    pub usage:      bool
}

/// Everything a rule is allowed to look at.
///
/// One struct rather than a signature per gate: the checker builds it once and
/// hands the same borrow to every rule, so adding a rule never changes a call
/// site and never widens what the others can see.
pub struct Sight<'a> {
    /// The words of the phrase, as tokenised.
    pub words:      &'a [Word],
    /// The reading settled on for each word, where one was settled.
    pub selected:   &'a [Option<&'a Analysis>],
    /// The dependency structure, when a parser was loaded.
    pub tree:       Option<&'a Tree>,
    /// What the checker worked out once for the whole phrase.
    pub known:      &'a Known,
    /// The senses behind the words, when knowledge was loaded.
    pub lexicon:    Option<&'a dyn Lexicon>,
    /// What the nouns were seen governing, when the table was loaded.
    pub government: Option<&'a Government>,
    /// The readings a corpus was seen using, when the table was loaded.
    pub usage:      Option<&'a Usage>
}

/// One rule of the language, stated and searched for.
pub trait Rule: Sync {
    /// Where the rule is written down.
    fn cites(&self) -> Citation;

    /// How badly a finding of this rule reads.
    ///
    /// Declared here rather than inferred from the violation, so that two
    /// rules reporting the same violation may differ in how sure they are.
    fn weight(&self) -> Severity;

    /// What the rule must be given before it can speak.
    fn needs(&self) -> Needs;

    /// What the rule found, appended rather than returned, so that running
    /// two hundred rules allocates once.
    fn found(&self, sight: &Sight<'_>, into: &mut Vec<Violation>);
}
```

`needs()` is what makes defect § 2 below structurally impossible: a rule that
declares `government: true` and is run on an engine with no government table is
**reported as unavailable**, not silently satisfied. That is the difference
between «the phrase is clean» and «I could not check this».

### The registry

A `build.rs` in `rusem-verify` walks `src/rules/`, emits the `pub mod` lines and
the slice into `OUT_DIR`, and `lib.rs` includes it:

```rust
/// Every rule the engine knows, in citation order.
pub const RULES: &[&dyn Rule] = &[ … ];
```

Convention: every file under `src/rules/` exposes `pub static RULE: &dyn Rule`.

This is chosen over the two alternatives on purpose:

| Option | Verdict |
| --- | --- |
| hand-written slice in one file | Same class of defect as today, just narrower: forgetting the line still compiles. Rejected. |
| `linkme` / `inventory` distributed slice | Works, but the collection is link-section magic — invisible to a reader, dependent on the linker, and the crate's internals are `unsafe` where this workspace denies it. Rejected on auditability. |
| `build.rs` walking the directory | A rule file that exists is registered; a rule file that is not registered cannot exist. The generated slice is a plain `const` a person can print and read. **Chosen.** |

### Steps

1. `crates/rusem-verify/src/rule.rs` — `Citation`, `Needs`, `Sight`, `Severity`
   re-export, the `Rule` trait. No behaviour, nothing calls it yet.
2. `crates/rusem-verify/build.rs` + the generated slice. Empty directory, empty
   slice, green build.
3. Move **one** existing gate — `gates/copula.rs` is the right first one: it is
   self-contained, needs only `words` and `selected`, and carries seven tests of
   its own — into `src/rules/copula/`, add `impl Rule`, delete its line from
   `fn agreement`. Its tests move with it and must pass unchanged.
4. Move the remaining 32 gates and the 9 punctuation modules the same way, a
   handful per commit. `fn agreement` and the gate block in `Verifier::attempt`
   shrink to a single loop over `RULES`.
5. `Verifier::attempt` becomes: build `Sight` once, loop `RULES`, skip and
   record any rule whose `needs()` the sight cannot meet, extend.
6. Move `rules/svod/` in under `src/rules/svod/<slug>/`, keeping every
   `required()` byte-for-byte and adding one `impl Rule` block per file. This is
   196 paragraphs of work and it is the bulk of the plan — but each file is
   independent, so it parallelises and every one of them can land alone.
7. A rule whose question cannot yet be found in a phrase gets its `found()`
   returning nothing and a `Needs` it cannot meet — so the registry can report
   it as *stated but not reachable* rather than leaving it as a file nobody
   compiles. **Being unfinished becomes a value the engine can state.**

### How it is verified

- `cargo test -p rusem-verify` compiles all 203 paragraphs. Today it compiles 7.
- A test asserts `RULES` holds no duplicate `Citation` and that every citation
  falls in 1..=203.
- Every gate's own tests move with it and pass unchanged. That is the safety
  argument for steps 3–5: the mechanism changes, the verdicts do not.
- `crates/rusem-verify/tests/phrases.rs` — 478 lines of real phrases against the
  checker — is the backstop, and it never touches a gate by name, so it holds
  across the whole migration.

---

## 2. The engine owns its own composition

### What is there now

`Engine::builder()` exposes twelve independent setters — `.dictionary()`,
`.parser()`, `.stress()`, `.morphemes()`, `.scale()`, `.government()`,
`.usage()`, `.packs_in()`, `.seed()`, `.valency()`, `.store()`, `.build()` —
and nothing inside `rusem` knows which of them constitute a whole engine. The
composition root has leaked out of the library: the knowledge *what this thing
is made of* lives in whoever calls the builder, and is therefore duplicated
once per caller and true nowhere.

The builder cannot even be asked what it went without. `Settings::present`
returns `None` for a file that is not there and the setter is simply not
called; the resulting `Engine` is indistinguishable from one that was never
meant to have that layer. Two examples of what that costs, both from doc
comments the code itself carries:

| Layer absent | What the engine then does | Who is told |
| --- | --- | --- |
| government table | `Government::default()` is empty, and per `Verifier::governing`'s own doc comment the checker «never reports a misgoverned noun» — `Violation::MisgovernedNoun` cannot fire | nobody |
| scale weights | `Scale::default()` has an empty map, `Scale::decide` returns `None` (`scale.rs:138`), the verdict falls back to `Verdict::status()` and the learned threshold is absent | nobody |
| usage table | reading ties break by the analyzer's order rather than corpus frequency, so a different reading is judged — the difference is in *what was checked*, not only in the answer | nobody |

An engine that silently checks less than it claims is worse than one that
refuses to start. Both are recoverable; only the second is noticeable.

### How it should be

Two changes, and they are the same change seen from either end.

**Composition belongs to the library.** `Settings` already lives in `rusem`;
the code that turns `Settings` into an `Engine` belongs beside it, not in
whoever holds the crate.

```rust
impl Engine {
    /// Assembles the engine from settings: every layer whose file is present,
    /// and an account of the ones that were not.
    ///
    /// This is the composition root. The builder stays public for a caller
    /// that wants an engine this function would not build, but nothing has to
    /// know the order of twelve setters to get a whole one.
    pub fn from_settings(settings: &Settings) -> Result<(Self, Assembled)>;
}
```

**Absence is a fact the engine carries, not a hole in it.**

```rust
/// What the assembly opened and what it went without.
///
/// Returned rather than logged: the engine must not choose how it is reported,
/// only refuse to be silent about it.
#[derive(Debug)]
pub struct Assembled {
    /// The layers that were opened, with the paths they came from.
    pub loaded:  Vec<(&'static str, PathBuf)>,
    /// The layers that were absent, each with what the engine cannot do
    /// without it.
    pub missing: Vec<(&'static str, &'static str)>
}
```

`missing` is the half that matters. The doc comment on `Settings::present`
today describes dropping a layer silently as a feature. It is not: `Evidenced`
exists so that every fact names its source, and an engine that cannot say
*which checks it did not run* has a hole in exactly that principle — the
absence of evidence is itself evidence, and it is currently discarded.

`Engine::sources()` should report it beside the dictionaries: «government table
absent — misgoverned nouns are not reported».

### Steps

1. `Assembled` in `crates/rusem/src/engine.rs`, and `Engine::from_settings`
   returning `(Engine, Assembled)`.
2. Every layer that can be absent gets its `missing` entry naming the capability
   lost. Writing that sentence is the design work — a layer whose absence has no
   describable consequence should not be optional.
3. `Engine::sources()` reports `Assembled` alongside the sources.
4. The builder's own setters stay, unchanged and public. This adds a whole
   answer; it does not take away the parts.

Once § 1 lands, `Needs` closes this from the other side: a rule states what it
requires, and an engine assembled without it reports the rule as unavailable
rather than as satisfied. `Assembled` says what the engine lacks; `Needs` says
what that costs, rule by rule.

---

## 3. Violations describe themselves

### What is there now

`crates/rusem/src/engine.rs:1292` — `fn describe`, 296 lines, one arm per
violation kind, ending at line 1588 with:

```rust
other => format!("{other:?}")
```

`Violation` has 52 variants and the match has 48 arms. The four that fall
through today print their Rust `Debug` rendering as the Russian sentence a
person reads. A 53rd variant will compile fine and do the same.

There is a second copy of the same knowledge in `rusem-core`:
`Violation::kind()` (`verdict.rs:603`) gives the machine key and
`Violation::severity()` (`verdict.rs:755`) gives the fatality. Three
independent tables over the same enum, in two crates, kept in step by hand.

### How it should be

Delete the catch-all. That single edit turns the defect from silent to
compile-time — `match` on an enum without a wildcard is exhaustive by
construction, and a 53rd variant then *cannot* be added without writing its
sentence.

The message itself belongs with the other two tables, in `rusem-core`, as a
third method on `Violation`:

```rust
impl Violation {
    /// The stable machine key.
    pub fn kind(&self) -> &'static str;
    /// How badly it reads.
    pub fn severity(&self) -> Severity;
    /// How it is said in Russian, given a way to name a sense.
    ///
    /// The naming is passed in rather than looked up: `rusem-core` must not
    /// know what a knowledge base is, and the caller already has one.
    pub fn describe(&self, name: &dyn Fn(SenseId) -> String) -> String;
}
```

One `match` per method, all three beside the enum they cover, so adding a
variant means writing three arms in one file and the compiler asks for all
three. The `&dyn Fn` keeps the layering intact: `rusem-core` depends on nothing
and must stay that way; the facade passes `|sense| self.reference(sense)`.

Once § 1 lands, the rule's `Citation` joins the sentence: «пропущена запятая
перед *что* — § 140».

### Steps

1. Move the 48 arms from `crates/rusem/src/engine.rs` into
   `Violation::describe` in `crates/rusem-core/src/verdict.rs`.
2. Delete `other => format!("{other:?}")`. Fix what the compiler then names —
   that list is the bug.
3. `Engine::describe` becomes `violation.describe(&|sense| self.reference(sense))`.
4. A test walks a constructed instance of every variant and asserts the message
   is non-empty and holds no `{` or `}`.

---

## 4. Model files carry a header

### What is there now

`crates/rusem-syntax/src/parser.rs:501`:

```rust
if bytes.len() != ACTIONS * BUCKETS * 4 { return None; }
```

where `ACTIONS = 1 + 2 * Relation::ALL.len()` and `Relation::ALL` is
`[Self; 14]` (`crates/rusem-core/src/syntax.rs:57`). That is
`29 × 262144 × 4 = 30 408 704` bytes — exactly the size of `data/ru.weights`.

There are no magic bytes, no version, no record of which relations the file was
trained against. **Renaming or reordering any of the 14 relations without
changing their number loads the old 30 MB file cleanly and mislabels every
dependency arc**, and every gate that reads the tree then judges the wrong
structure. Adding or removing one at least fails loudly.

### How it should be

The file should state what it is. Thirty-two bytes in front of the weights:

```rust
/// The header a model file opens with.
///
/// The fingerprint is over the relation names in their fixed order, so a file
/// trained before a relation was renamed is refused rather than misread.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    /// `RUSEMPS1`, so a wrong file is refused before it is measured.
    pub magic:       [u8; 8],
    /// The format revision, bumped when the layout changes.
    pub revision:    u16,
    /// How many relations the model was trained over.
    pub relations:   u16,
    /// How many buckets the features were hashed into.
    pub buckets:     u32,
    /// A digest of the relation names, in order.
    pub fingerprint: [u8; 16]
}
```

`rusem-store` already does exactly this — magic `RUSEMBS1` and a strict version
field, per its own notes. The parser model is the one binary artefact in the
project that does not, and it is the one where a mismatch is undetectable.

The fingerprint is the part that matters. Magic and length catch a wrong file;
only a digest over `Relation::ALL`'s names catches a *right-sized file from a
different relation set*, which is the failure that ships bad answers.

### Steps

1. `Header` in `crates/rusem-syntax/src/parser.rs`, with a `const fn` computing
   the fingerprint from `Relation::ALL`.
2. `rusem-build parser` writes it.
3. `Parser::from_bytes` reads it and returns a *reason* rather than `None`:
   wrong magic, wrong revision, wrong relation set, wrong length. Today every
   failure is the same silent `None`, which the engine turns into an
   unexplained `CoreError`.
4. Regenerate `data/ru.weights`, or write a one-shot that prepends the header to
   the existing file — the weights themselves are unchanged.
5. A test asserts a file whose fingerprint does not match is refused, built by
   flipping one byte of the digest.

---

## 5. The store is chosen, not guessed

### What is there now

`Builder::packs_in` in `crates/rusem/src/engine.rs` ends with:

```rust
self.store = stored.first().cloned();
```

Three separate hazards in one line. A caller that does
`.store("/path/x.store").packs_in(dir)` where `dir` holds no `.store` file gets
`self.store = None` — the explicit choice is overwritten by the scan, and the
engine loads every pack into memory instead, which is the exact cost
`rusem-store` exists to avoid. No error, no log. A directory holding two
`.store` files silently uses whichever sorts first. And which of the two
knowledge backends answered is not recorded anywhere the caller can read.

This matters more than it reads, because the two backends are **not proven
equivalent** — see § 6.

### How it should be

`packs_in` fills the store only when nothing was set:

```rust
if self.store.is_none() {
    self.store = match stored.as_slice() {
        [] => None,
        [only] => Some(only.clone()),
        many => return Err(TooManyStores(many.len()).into())
    };
}
```

Two stores in a directory is an operator mistake, not a coin flip. And the
chosen backend goes into the `Assembled` report from § 2, so `rusem sources`
says which one answered.

### Steps

1. The guard and the `many` arm above.
2. A `StoreChoice` line in `Assembled`.
3. Tests: explicit store survives `packs_in`; two stores is an error; no store
   falls back to packs and says so.

---

## 6. One taxonomy climber

### What is there now

`crates/rusem-knowledge/src/index.rs:325,372` and
`crates/rusem-store/src/read.rs:343,386` each hard-match

```rust
RelationKind::Hypernym | RelationKind::InstanceOf
```

and each carries its own `MAX_ASCENT` / `CLASS_ASCENT` / `CONSTRAINT_ASCENT`.
`crates/rusem/src/base.rs` fans every port call to one or the other depending
on whether a `.store` file happened to be in the directory. Nothing checks the
two agree.

A divergence shows up as **the same question answered differently depending on
a file's presence** — combined with § 5, depending on a file's presence that
nobody chose.

### How it should be

The climb is one algorithm over a graph, and it should exist once. It does not
belong to either backend: both of them merely *supply edges*.

Put it in `rusem-core` beside the ports, as a free function over the
`SemanticNetwork` port both backends already implement:

```rust
/// Climbs the taxonomy from a sense, breadth first, to a bounded height.
///
/// The relations that ascend are named here and nowhere else: a new one is
/// added by editing this list, and both backends follow without being told.
pub fn ascend(
    network: &dyn SemanticNetwork,
    from: SenseId,
    height: Ascent
) -> Result<Vec<SenseId>>;

/// How far a climb may go, and why it stops there.
#[derive(Debug, Clone, Copy)]
pub enum Ascent {
    /// Six steps, for semantic checks over the whole connected graph.
    Whole,
    /// One step, for collecting classes before the upper ontology blurs.
    Class,
    /// Three steps, covering real genus chains without the noise above them.
    Constraint
}
```

Then both backends delete their climbers and their constants. The ascending
relation kinds are named once, so § 7 of the relation checklist — «two files,
two sites each, they must stay in sync» — stops existing.

### Steps

1. `ascend` and `Ascent` in `crates/rusem-core/src/relation.rs`.
2. Delete both climbers, route `Base::Loaded` and `Base::Stored` through
   `ascend`.
3. A differential test: build a store from the seed pack, ask both backends the
   same hundred questions, assert equal answers. This test should exist
   regardless of this fix — it is the only thing that would have caught the
   drift.

---

## 7. Word lists: closed classes stay, open lexicon leaves

### What is there now

67 `const NAME: &[&str]` lists across `crates/rusem-verify/src/`, 17 of them in
`checker.rs`. They carry no source, no confidence, no `Evidenced` wrapper —
which is the one thing the architecture is otherwise built to prevent.

My earlier note called all 67 misplaced. That was too broad, and the correction
matters because it changes the work: **most of them are not lexicon at all.**

### The criterion

| Kind | Test | Belongs |
| --- | --- | --- |
| **Closed class** | Can a person write down every member, and would the list still be complete in fifty years? Conjunctions, particles, prepositions, enclitics, pronouns. | **In code.** A closed class *is* grammar. Putting «и, да, или» in a pack does not make it better sourced, only harder to read and possible to load without. |
| **Open lexicon** | Could a dictionary add a member tomorrow? Nouns, verbs, adjectives, measure words, names. | **In a pack**, with the source that vouches for it. |

By that test:

| Stays in code (closed) | Moves to a pack (open) |
| --- | --- |
| `ENCLITICS` (ли, же, бы, ль, б), `JOINING`, `COORDINATING`, `SUBORDINATING`, `DENYING_WORDS` (не, нет, ни, без, безо), `TURNED` (ся, сь), `POINTING`, `STANDING`, `UNCHANGING` (его, её, их), `FROZEN_CHOICE`, `COUNTING` (два … полтора), `CARRYING`, `OPENING`, `RELATIVES`, `SUBORDINATING`, `WILL` (буду … будут) | `MEASURES`, `PLURAL_ONLY`, `AFTERWARD_NOUNS`, `COMMON_GENDER`, `INDECLINABLE_MASCULINE`, `EITHER_GENDER`, `WALKING_GERUNDS`, `NAMING`, `PHASE_VERBS`, `HOLLOW`, `STANDING_IN`, `VERNACULAR_FORMS`, `VERNACULAR_LEMMAS`, `JOINING` in `gates/copula.rs` (the nine copular verbs), `PREPOSITION_CASES` |

Roughly a third of the 67 move. The other two thirds are correctly where they
are and should be left alone.

### The defect the census turned up on the way

`JOINING` is defined **four times, with four different contents**:

| File | Members |
| --- | --- |
| `checker.rs` | и, или, либо, да |
| `gates/shared.rs` | и, да, но, а |
| `gates/coordinated.rs` | и, да, или, либо, но |
| `gates/subject.rs` | и, да, или, либо, ни |

Four answers to «what is a coordinating conjunction», under one name, in one
crate. At most one of them is right and nothing says which. Whatever else is
done, this is a bug today.

### How it should be

```rust
/// The closed classes of Russian function words.
///
/// A closed class is grammar, not lexicon: it is written here rather than
/// loaded from a pack because it cannot grow, and because a checker that
/// cannot find its conjunctions is not degraded but broken.
pub mod closed {
    /// The conjunctions that join equals.
    pub const COORDINATING: &[&str] = &[…];
    /// The conjunctions that hang a clause under another.
    pub const SUBORDINATING: &[&str] = &[…];
    /// The particles that lean on the word before them.
    pub const ENCLITIC: &[&str] = &[…];
    /// The words that deny.
    pub const DENYING: &[&str] = &[…];
}
```

One definition per class, in `crates/rusem-verify/src/closed.rs`, every gate
importing from there. The open lists become records in a pack, reaching the
checker as an `Evidenced` table the way `Government` and `Usage` already do —
and under § 1, a rule that needs that table *declares* it, so an engine without
it says so instead of passing everything.

### Steps

1. `closed.rs` with one definition per class. Resolve the four `JOINING`s into
   one — deliberately, reading each gate to see which membership it actually
   needs, and splitting into two named classes if two are genuinely needed.
2. Point every gate at it. Probes must not move; if they do, one of the four
   lists was load-bearing and the difference is a finding.
3. A pack record kind for lexical tables, a `rusem-build` subcommand that emits
   it from the dictionary, and a `Lexical` table on `Verifier` beside
   `Government` and `Usage`.
4. Move the open lists over one at a time, the gate tests green after each.

---

## 8. A library does not touch the process

### What is there now

`crates/rusem-morph/src/analyzer.rs:151` — `hush()` calls
`std::panic::set_hook`, once, lazily, on the first `analyze` call. The
replacement hook drops any panic whose `location().file()` contains the string
`"rsmorphy"` (`GUESSER`, line 137) and forwards the rest.

Two things are wrong, and the second is the architectural one.

**It reaches outside its layer.** The panic hook is process-global state.
`rusem-morph` is an adapter — the narrowest ring of the design, one port over
one dictionary — and it mutates a global belonging to whoever owns the process,
as a side effect of being asked for the morphology of a word. A host that
embeds `rusem` inside a larger program loses its own hook without ever calling
anything named like a hook.

**It hides the failure instead of representing it.** The panic is real: the
dictionary cannot read that form. `CoreError::AdapterFailure` exists for
precisely this and is what every other adapter failure becomes. Muting the
report converts a fact the engine could carry into nothing at all — the same
defect as § 2, one layer down.

### How it should be

The panic is caught where it happens and becomes a value, and no global is
touched by anyone:

```rust
impl OpenCorporaMorphology {
    /// Asks the dictionary, turning a panic from it into an error.
    ///
    /// The dictionary raises rather than returns on forms it cannot read. That
    /// is its failure, not the process's, so it is caught here and named:
    /// nothing outside this function learns that a panic was involved, and
    /// nothing global is touched to keep it quiet.
    fn asked(&self, form: &WordForm) -> Result<Vec<Parsed>> {
        std::panic::catch_unwind(AssertUnwindSafe(|| self.analyzer.parse(form)))
            .map_err(|_| CoreError::AdapterFailure { … })
    }
}
```

`catch_unwind` is safe code and the workspace sets no `panic = "abort"`, so this
needs nothing the lints forbid. The `hush` hook, `HUSHED`, and the `GUESSER`
substring match all disappear — and with them the string-matching on a file path
that would silence any other crate whose source path happened to contain
`rsmorphy`.

The gain is not only tidiness. Today an unreadable form is invisible; after
this it is an `AdapterFailure` the checker can count, report as a source of
uncertainty, or decide to treat as an unknown word. The engine gets a fact
where it had a silence.

### Steps

1. `catch_unwind` at the two call sites in `crates/rusem-morph/src/analyzer.rs`
   that reach `rsmorphy`, converting to `CoreError::AdapterFailure` with the
   form in the message.
2. Delete `hush`, `HUSHED` and `GUESSER`.
3. Decide, in `Verifier::read`, what an `AdapterFailure` on one word means: an
   unknown word, or a phrase the checker declines to judge. Today the question
   cannot be asked because the failure never arrives.

---

## 9. The manifests tell the truth

### What is there now

| Crate | Defect |
| --- | --- |
| `rusem-stress` | declares `rusem-core`, imports it **nowhere** in `src/` |
| `rusem-syntax` | declares an optional `serde` feature; `serde` appears **nowhere** in `src/` |
| `rusem-store`, `rusem-stress`, `rusem-syntax` | omit `publish = false` while every other crate sets it — yet all three path-depend on `rusem-core`, which *is* `publish = false`, with no `version` field. `cargo publish` fails on all three. |

The dependency graph in `ARCHITECTURE.md` had to draw dotted edges for
dependencies that do not exist. A manifest that overstates its dependencies makes the graph — the
one artefact people reason about the architecture from — wrong.

### Steps

1. Delete the dead edges.
2. Either set `publish = false` on the three, or, if they are meant to ship,
   give the path dependencies `version` fields and decide what `rusem-core`'s
   publication story is. The current state is neither.
3. `cargo machete` or `cargo +nightly udeps` in CI so this cannot come back.

---

## 10. The search runs the gates once, not twice

### What is there now

`crates/rusem-verify/src/checker.rs:2210` — when the settled reading carries a
fatal violation, `check` walks `combinations(&words)` twice, calling
`self.attempt` on every combination both times:

```rust
let mut best_coverage: Option<usize> = None;
for picks in combinations(&words) {
    let attempt = self.attempt(&words, &picks, questioning, &known)?;
    best_coverage = Some(best_coverage.map_or(attempt.matched, |held| held.max(attempt.matched)));
}

for picks in combinations(&words) {
    let attempt = self.attempt(&words, &picks, questioning, &known)?;
    if attempt.fatal() == 0 && best_coverage.is_none_or(|held| attempt.matched >= held) { … }
}
```

Bounded by `MAX_ATTEMPTS = 256` (`checker.rs:33`), so up to 512 full gate
batteries per phrase, each re-running ~40 gates and re-querying the lexicon.

### Why the obvious fix is wrong

The tempting version — one pass, keep the best as you go, break early — does
**not** preserve the result. The second loop's condition is
`attempt.matched >= best_coverage`, and `best_coverage` is the maximum over
*every* combination. It is not knowable until the last one has been seen, so a
single streaming pass would accept an early attempt that a later, better one
would have excluded. That is a verdict change, not an optimisation.

### How it should be

The waste is not the two passes; it is calling `attempt` twice for each
combination. Run the gates once, keep the results, then select over what was
kept:

```rust
let attempts: Vec<Attempt> = combinations(&words)
    .map(|picks| self.attempt(&words, &picks, questioning, &known))
    .collect::<Result<_>>()?;

let best_coverage = attempts.iter().map(|held| held.matched).max();

for attempt in attempts { … }
```

Exactly half the gate work, byte-identical selection, and the `?` moves out of
the loop body. The cost is holding up to `MAX_ATTEMPTS` `Attempt` values at
once instead of one — 256 structs, each a small vector of violations and a
reading, against 256 avoided runs of forty gates over the lexicon. That trade
is not close.

If the memory is judged too much, the correct shape is a single pass that keeps
every attempt whose `matched` ties the running maximum and discards the rest
when the maximum rises — same result, bounded by the number of ties rather than
by `MAX_ATTEMPTS`. Do that only if measurement says the simple version costs
something.

### Steps

1. Collect once, take the maximum from the collected attempts, select over
   them. No change to `attempt`, to `combinations`, or to the ordering.
2. Assert equality the honest way: for a corpus of phrases that reach the
   search, require the verdict of **every** one to be identical before and
   after — not a tally. A change here that moves one verdict is a bug, and
   tallies stay level while two phrases swap.

## Sequence

The order is not the numbering — it is what unblocks what, and what is safe to
do while the checker is being taken apart.

| Order | Item | Why here | Risk |
| --- | --- | --- | --- |
| 1 | § 9 manifests | Independent, minutes, makes the graph true before anyone reads it while working | none |
| 2 | § 3 violations describe themselves | Independent, and the compiler does the work | none |
| 3 | § 4 model header | Independent, and until it lands every parser change is a live hazard | regenerate one file |
| 4 | § 5 store chosen | Two lines, and § 6 needs a known backend to test against | none |
| 5 | § 6 one climber | Needs § 5. The differential test is worth having before the checker is rebuilt | medium — the test may find real drift |
| 6 | § 2 one assembly | Needs nothing; must land before § 1 so `Needs` has one place to be satisfied | low |
| 7 | § 8 panic hook | Independent, small | none |
| 8 | **§ 1 the rule trait, steps 1–5** | The mechanism, over the 42 rules that already exist | high — the gates’ own tests are the guard |
| 9 | § 7 word lists | Best done while the gates are already being moved, one file at a time | medium — resolving the four `JOINING`s will change a verdict somewhere |
| 10 | **§ 1 steps 6–7** | The 196 paragraphs. Parallel, independent, one file per commit | low per file, long |
| 11 | § 10 gates run once | Last, because it is the only change that can alter a verdict for a reason other than a rule. Equivalence is proven verdict by verdict | medium |

Steps 1–7 are a week of small, individually reversible commits. Step 8 is the
one that has to be done carefully. Step 10 is the project.

The two surfaces are deliberately absent from this table. They are rebuilt
after step 11, against the engine that exists by then rather than the one that
exists now.

Nothing here is created until this plan is approved — no issues, no branches.
