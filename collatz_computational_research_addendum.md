# Computational Research Addendum for the Rust Collatz Workbench

**Research status:** July 2026  
**Purpose:** Extend the original Rust experiment outline with relevant research domains, computational techniques, optimization strategies, and resources that may be useful now or in later phases.

---

## Executive summary

The strongest direction is not simply to calculate more Collatz trajectories. Existing projects already verify enormous finite ranges with highly optimized CPU, GPU, sieve, and distributed-computing techniques. A more distinctive and potentially useful project would combine four layers:

1. **A fast exact arithmetic kernel**
   - Odd-only Collatz steps
   - Macrostep tables
   - Residue-class sieves
   - Path merging
   - CPU-first implementation with an optional GPU backend later

2. **A symbolic search layer**
   - Valuation words
   - Affine trajectory formulas
   - Residue-class descent certificates
   - Minimal-counterexample feasibility pruning

3. **A program-verification layer**
   - Counterexample-guided abstraction refinement
   - Abstract interpretation
   - Transition invariants
   - Piecewise or lexicographic ranking functions
   - Syntax-guided synthesis of candidate invariants

4. **A certification layer**
   - Small deterministic Rust verifier
   - Exact arithmetic only
   - Optional SAT proof logs such as LRAT
   - Eventual export of important results to Lean

The most promising immediate addition to the roadmap is to reproduce and modularize the **sieve ideas used in modern computational verification**, then redirect those tools toward **symbolic family certificates** instead of only extending the checked numerical range.

A good working principle is:

> Use computation to discover a compact theorem-shaped object, then verify that object independently.

---

# 1. What existing computational work changes in our plan

## 1.1 High-performance brute-force verification is already sophisticated

David Barina's convergence-verification work replaces exponentially large precomputed tables with much smaller lookup structures and uses optimized CPU and OpenCL implementations. The later project introduced additional sieving and distributed processing and reported verification through \(2^{71}\).

Resources:

- [Convergence verification of the Collatz problem](https://link.springer.com/article/10.1007/s11227-020-03368-x)
- [Improved verification limit for the convergence of the Collatz conjecture](https://link.springer.com/article/10.1007/s11227-025-07337-0)
- [Reference implementation and distributed project](https://github.com/xbarin02/collatz)

### Implication for this project

We should not begin by trying to beat the current verification bound. Instead, we should:

- reproduce a small, trustworthy version of the known algorithm;
- use it as a performance and correctness baseline;
- extract its sieves as reusable symbolic components;
- study which residue families each sieve eliminates;
- generate certificates describing why a whole family can be ignored.

This lets existing verification work become infrastructure rather than competition.

---

## 1.2 Recent verification work organizes several distinct sieves

A 2026 preprint by Vigleik Angeltveit describes an algorithm intended to scale from verification below \(2^N\) to below \(2^{N+1}\) in less than twice the time. It discusses several named sieves:

- descent sieve;
- mod-\(9\) preimage sieve;
- path-merging sieve;
- odd-even-even sieve;
- rational approximation and precomputed bitvectors;
- a CPU/GPU division of labor.

Resource:

- [An improved algorithm for checking the Collatz conjecture for all \(n<2^N\)](https://arxiv.org/abs/2602.10466)

This is a recent preprint rather than a settled final word, but it is highly relevant as an engineering reference.

### Implication for this project

Add a general sieve interface rather than hard-coding one pruning rule:

```rust
pub trait PrefixSieve {
    fn evaluate(&self, state: &PrefixState) -> SieveResult;
}

pub enum SieveResult {
    Keep,
    Reject { certificate: RejectionCertificate },
    Refine { requested_precision: u32 },
}
```

Initial implementations:

```text
DescentSieve
Mod9PreimageSieve
PathMergingSieve
OddEvenEvenSieve
MinimalCounterexampleSieve
TwoAdicImpostorSieve
```

The key research question becomes:

> Which combinations of independently understandable sieves make the unresolved symbolic tree shrink fastest?

That is more informative than a single opaque optimized kernel.

---

## 1.3 Random enormous inputs answer a different question

Algorithms can efficiently test individual inputs with billions of decimal digits by batching many ordinary iterations into large arithmetic operations.

Resource:

- [Numerical verification of the Collatz conjecture for billion-digit random numbers](https://arxiv.org/abs/2502.16743)

This is useful for arbitrary-precision implementation ideas, but it does not solve the universal-quantifier problem. Testing one gigantic input is mathematically weaker than certifying an infinite residue class.

### Implication for this project

Support huge individual inputs as a diagnostic and benchmarking feature, but do not make them the central research objective.

---

# 2. Automated termination research

Collatz can be treated as a tiny deterministic program whose universal termination is unknown. This connects the project directly to program-analysis research.

## 2.1 String rewriting and matrix interpretations

Yolcu, Aaronson, and Heule constructed a string-rewriting system in a mixed binary-ternary representation whose termination is equivalent to the Collatz conjecture. They used automated termination methods, matrix interpretations, and SAT to prove nontrivial weakened forms.

Resource:

- [An Automated Approach to the Collatz Conjecture](https://arxiv.org/abs/2105.14697)

### Why this matters

This confirms that automated termination is not merely an analogy. There is already a precise Collatz encoding suitable for termination tools.

### Proposed experiment

Create a Rust representation of their rewriting system and use it for two purposes:

1. reproduce one of the weakened automated results;
2. compare rewriting-system states with the valuation-prefix states in our workbench.

The most useful output would be a translation such as:

```text
valuation prefix
    ↕
mixed binary-ternary rewrite trace
    ↕
residue-class affine transformation
```

Different proof systems may see different regularities in the same trajectory family.

---

## 2.2 Transition invariants rather than one decreasing quantity

Podelski and Rybalchenko showed that termination can be characterized using a **disjunctively well-founded transition invariant**: a finite union of well-founded relations can be enough, even when no single obvious measure decreases everywhere.

Resources:

- [Transition Invariants](https://swt.informatik.uni-freiburg.de/berit/papers/transition-invariants.pdf)
- [A Complete Method for the Synthesis of Linear Ranking Functions](https://link.springer.com/chapter/10.1007/978-3-540-24622-0_20)

### Why this matters

The initial outline searched for piecewise ranking functions. Transition invariants give that idea a stronger theoretical basis.

Instead of requiring:

\[
V(T(n))<V(n),
\]

we might seek a finite set of relations:

\[
R_1,R_2,\ldots,R_k
\]

such that every sufficiently long Collatz transition belongs to at least one relation that is well-founded.

### Concrete Collatz templates

- ordinary descent: \(m<n\);
- bit-length descent;
- descent within one residue class;
- reduced growth debt;
- reduced distance to a certified residue;
- lexicographic descent in \((\log n,\text{residue penalty})\);
- descent after a macrostep rather than one step.

### Proposed implementation

Add a crate:

```text
collatz-transition-invariants/
```

with a trait such as:

```rust
pub trait WellFoundedRelation {
    fn contains(&self, from: &SymbolicState, to: &SymbolicState) -> bool;
    fn certificate(&self) -> RelationCertificate;
}
```

Then search for a finite cover of all abstract transitions.

This may be a better target than one universal Lyapunov function.

---

## 2.3 Counterexample-guided abstraction refinement

Counterexample-guided abstraction refinement, or CEGAR, repeatedly:

1. builds a finite abstraction;
2. finds a dangerous abstract trace;
3. checks whether it is real;
4. refines the abstraction if the trace is spurious.

Resource:

- [Counterexample-Guided Abstraction Refinement](https://link.springer.com/chapter/10.1007/10722167_15)

### Why Collatz is a natural CEGAR problem

A coarse abstraction might say that a valuation \(a=1\) transition can repeat forever. Exact concretization then shows that the infinite all-\(1\) pattern approaches the 2-adic integer \(-1\), not a positive ordinary integer.

That gives a textbook spurious counterexample:

```text
Abstract model:
    indefinitely expanding path exists

Concrete arithmetic:
    no positive integer realizes the infinite path

Refinement:
    track enough binary/sign information to exclude it
```

### New refinement dimensions to test

Do not refine only by adding more low binary bits. Compare:

- additional bits modulo \(2^k\);
- residues modulo \(3^j\);
- recent valuation history;
- affine constant \(c_k\) buckets;
- signed representative nearest zero;
- distance from \(-1 \pmod{2^k}\);
- minimal-counterexample upper and lower bounds;
- path-merging identity;
- bit-length slope;
- paradoxical-prefix status.

Track which refinement destroys each spurious path. The resulting dataset may reveal what information a successful proof must preserve.

---

## 2.4 Abstract interpretation and widening

Abstract interpretation formalizes sound approximation of infinite state spaces. Widening and narrowing are methods for forcing fixpoint computations to converge in infinite abstract domains.

Resource:

- [Abstract Interpretation Frameworks](https://pcousot.github.io/publications/CousotCousot-JLC-n2--3-p103--179-1992.pdf)

### Candidate abstract domains for Collatz

Build several small domains rather than one enormous state type:

```text
ResidueDomain(2^k)
ResidueDomain(3^j)
ValuationInterval
GrowthDebtInterval
AffineCoefficientDomain
BitLengthDifferenceDomain
SignedRepresentativeDomain
KnownDescentDomain
```

Then combine them with reduced products.

### Possible widening rule

Suppose a symbolic search repeatedly sees affine transformations:

\[
n \mapsto \frac{3^k n+c}{2^A}.
\]

A widening could generalize several concrete constants \(c\) into an interval or congruence family while preserving a sound upper bound on the resulting value.

The challenge is to avoid widening away exactly the arithmetic detail that separates positive integers from 2-adic impostors. That tradeoff itself is an experiment worth measuring.

---

# 3. Automated invariant and ranking-function synthesis

## 3.1 Syntax-guided synthesis

Syntax-guided synthesis, or SyGuS, asks a solver to find an expression from a supplied grammar that satisfies a logical specification.

Resources:

- [Syntax-Guided Synthesis](https://sygus-org.github.io/assets/pdf/Journal_SyGuS.pdf)
- [SyGuS language standard](https://arxiv.org/abs/2312.06001)

### Application to Collatz

Provide a grammar for ranking-function candidates:

```text
V(n, r, h) ::=
    log2(n)
  | bit_length(n)
  | residue_weight[r]
  | history_weight[h]
  | V + V
  | rational * V
  | max(V, V)
  | min(V, V)
```

The specification can require that every transition in a finite symbolic abstraction decreases one of the selected components.

The solver does not prove Collatz by itself. It proposes a compact candidate, which the exact verifier then checks on the abstraction and, when possible, lifts to a symbolic theorem.

### Practical solver strategy

Keep Rust as the orchestration and certificate layer, but invoke an external solver through:

- SMT-LIB;
- SyGuS text format;
- a subprocess interface;
- or a solver library if integration is stable.

Do not make the research repository depend on one solver API.

---

## 3.2 ICE learning for invariants

ICE learning synthesizes invariants from:

- positive examples;
- negative examples;
- implication examples.

Resource:

- [ICE: A Robust Framework for Learning Invariants](https://madhu.cs.illinois.edu/CAV14ice.pdf)

### Collatz mapping

- **Positive example:** a state that must belong to the candidate invariant.
- **Negative example:** a known terminal, impossible, or excluded state.
- **Implication example:** if state \(x\) belongs, then its successor \(T(x)\) must belong.

This fits naturally with counterexample-guided refinement.

### Proposed use

Train or enumerate a simple classifier over features such as:

```text
n mod 2^k
n mod 3^j
v2(3n + 1)
recent valuation suffix
growth debt bucket
distance from starting value
signed 2-adic approximation class
```

The learner proposes a candidate inductive set. The exact teacher checks:

1. inclusion of required states;
2. exclusion of forbidden states;
3. closure under all symbolic transitions.

Machine learning is acceptable here because it proposes an invariant. It is not trusted as the proof.

---

## 3.3 Path-complete Lyapunov functions

Path-complete Lyapunov functions use several potential functions connected by a labeled directed graph to certify stability of switched systems.

Resource:

- [Path-Complete Graphs and Common Lyapunov Functions](https://arxiv.org/abs/1612.03983)

### Collatz mapping

Each odd-only valuation \(a\) acts like a mode:

\[
n\mapsto \frac{3n+1}{2^a}.
\]

The sequence of modes is constrained by arithmetic. A path-complete graph can represent which potential is required to decrease after each mode or short valuation word.

### Important adaptation

Standard switched-system work often considers arbitrary switching. Collatz switching is not arbitrary; only arithmetically realizable valuation sequences count.

Therefore use a **constrained path-complete graph** whose language is produced by the residue automaton.

This suggests a layered certificate:

```text
Residue automaton:
    determines legal valuation labels

Path-complete potential graph:
    proves decline along every legal labeled path
```

That combination is more tailored to Collatz than a generic average-drift argument.

---

## 3.4 Maximum cycle mean

Once an abstract transition graph has an edge weight such as:

\[
w(a)=\log_2 3-a,
\]

a dangerous abstract strongly connected component is one containing a cycle with nonnegative average weight.

### Proposed use

For each abstraction:

1. construct the legal transition graph;
2. calculate the maximum cycle mean;
3. extract a critical cycle;
4. concretize it;
5. if spurious, refine the abstraction;
6. if real but finite, derive the corresponding cycle equation;
7. if all cycle means are negative, generate a graph certificate.

The critical cycle is a much more informative counterexample than a generic long path.

---

# 4. Automata, symbolic dynamics, and word-based representations

## 4.1 Regular model checking

Regular model checking represents infinite sets of configurations with finite automata and transition relations with finite-state transducers.

Resource:

- [A Survey of Regular Model Checking](https://link.springer.com/chapter/10.1007/978-3-540-28644-8_3)

### Why it may fit

Integers can be represented as binary or mixed-base words. Collatz steps involve:

- parity inspection;
- multiplication by three;
- adding one;
- shifting;
- carry propagation;
- possible length change.

These operations are natural candidates for transducer representations.

### Main technical obstacle

Length-changing arithmetic and unbounded carry behavior may make the simplest automata closure arguments fail. That is useful information, not a reason to avoid the experiment.

### Proposed experiment

Build a transducer for one accelerated odd step over least-significant-bit-first binary strings.

Then ask:

- Is the set of numbers reaching a certified region in at most \(k\) macrosteps regular?
- Does repeated preimage computation stabilize under widening?
- What automaton states correspond to the surviving dangerous residue classes?
- Can unresolved valuation words be represented by a compact grammar?

A finite automaton for unresolved prefixes would make structural motifs explicit.

---

## 4.2 The 2-adic conjugacy viewpoint

Bernstein and Lagarias studied a conjugacy between the Collatz map on the 2-adic integers and a shift map.

Resources:

- [The \(3x+1\) Conjugacy Map](https://websites.umich.edu/~lagarias/doc/bernstein.pdf)
- [Lagarias's overview of the \(3x+1\) problem](https://arxiv.org/abs/2111.02635)

### Computational opportunity

The parity or valuation word can be treated as an infinite symbolic sequence. Finite prefixes determine increasingly precise 2-adic residues.

Our search should record not just the smallest positive representative, but also:

```text
residue modulo 2^k
nearest signed representative
distance from -1 modulo 2^k
eventual-one density
prefix compression complexity
automaton state
```

A dangerous branch may look expansive only because it converges to a negative or nonordinary 2-adic object.

### New experiment: ordinary-integer realizability pressure

For a finite positive integer, sufficiently high binary digits are all zero. Define a heuristic pressure score measuring how many forced high bits are one as the valuation prefix grows.

Search for a theorem-shaped statement:

> Sustained nonnegative growth debt forces an unbounded number of high binary digits to be one.

This would exclude ordinary positive integers while allowing nonordinary 2-adic paths.

The score itself is not a proof, but it may reveal the correct invariant.

---

## 4.3 Mixed binary-ternary representations

The rewriting-system work is important because Collatz is not purely binary:

- divisibility and halving are binary;
- multiplication by three introduces ternary structure;
- Tao's almost-all result uses mixing on 3-adic cyclic groups.

Resource:

- [Almost all orbits of the Collatz map attain almost bounded values](https://arxiv.org/abs/1909.03562)

### Proposed addition

Track both:

```text
n mod 2^k
n mod 3^j
```

but do so selectively. A full product domain grows quickly.

Use CEGAR to add ternary precision only when a spurious path survives binary refinement.

This makes “binary versus ternary compatibility” a measured engineering question rather than a broad philosophical idea.

---

## 4.4 Paradoxical sequences

Recent work studies finite trajectories whose parity behavior suggests multiplicative growth while the exact affine remainder changes the actual comparison. It derives conditions connecting these “paradoxical” finite sequences to the broader conjecture.

Resource:

- [Paradoxical behavior in Collatz sequences](https://arxiv.org/abs/2502.00948)

This is a recent preprint and should be treated as a research lead rather than unquestioned infrastructure.

### Why it may help

Our existing growth-debt score focuses on:

\[
\frac{3^k}{2^{A_k}}.
\]

Paradoxical behavior explicitly studies the discrepancy between that coefficient and the exact affine trajectory.

### Proposed data fields

```rust
pub struct AffineDiagnostics {
    pub multiplicative_growth: Rational,
    pub additive_remainder: BigUint,
    pub exact_relative_change: Ordering,
    pub paradoxical_prefix: bool,
}
```

Then search separately for:

- coefficient-expanding but exact-descending prefixes;
- coefficient-contracting but exact-nondescending prefixes;
- valuation words that maximize additive-remainder influence.

This may improve search scoring and prevent the system from overvaluing misleading prefixes.

---

# 5. Symbolic rewriting and e-graphs

## 5.1 Equality saturation

An e-graph stores many equivalent expressions compactly. Equality saturation repeatedly applies sound rewrite rules without immediately choosing one normal form.

Resources:

- [egg: Fast and Extensible Equality Saturation](https://arxiv.org/abs/2004.03082)
- [Rust `egg` documentation](https://docs.rs/egg/)

### Potential Collatz uses

Use an e-graph to simplify and compare symbolic expressions such as:

\[
\frac{3^k n+c_k}{2^{A_k}},
\]

nested odd steps, cycle equations, and descent inequalities.

Possible rewrite rules:

```text
pow2(a + b)  <-> pow2(a) * pow2(b)
pow3(a + b)  <-> pow3(a) * pow3(b)
(3*x + 1) / 2^a followed by another macrostep
common factor extraction
affine composition
residue-conditioned simplification
```

### What e-graphs are good for

- avoiding duplicate algebraic forms;
- discovering cheaper equivalent certificate expressions;
- canonicalizing macrostep transformations;
- finding repeated symbolic motifs across different valuation prefixes;
- simplifying generated SMT or SAT constraints.

### What they are not

An e-graph is not automatically a proof of termination. Each rewrite must be sound, and extracted inequalities still require exact verification.

### Suggested crate

```text
collatz-egraph/
```

Use it after the affine-prefix implementation is stable, not in the first milestone.

---

# 6. SAT, SMT, and proof-producing search

## 6.1 SAT as a combinatorial search engine

SAT is suitable when a bounded problem can be encoded with Boolean decisions:

- choice of valuation at each depth;
- path through a finite abstraction;
- residue-bit assignments;
- existence of a dangerous cycle;
- selection of ranking-function pieces;
- residue-cover optimization.

For Rust:

- [Varisat](https://docs.rs/varisat/latest/varisat/) is a Rust CDCL solver with proof-related support.
- [RustSAT](https://arxiv.org/abs/2505.15221) provides Rust infrastructure and unified interfaces for several SAT and MaxSAT solvers.

### Recommendation

Use RustSAT or a small internal CNF builder as the integration boundary. Allow external state-of-the-art solvers rather than requiring the project to use only a solver written in Rust.

---

## 6.2 Proof logging

A solver result is much more useful when it emits a proof that an independent checker can validate.

Resources:

- [The DRAT format and DRAT-trim checker](https://arxiv.org/abs/1610.06229)
- [Efficient Certified RAT Verification and LRAT](https://arxiv.org/abs/1612.02353)

LRAT adds hints that make proof checking simpler and suitable for formally verified checkers.

### Project policy

For any SAT-based claim of impossibility:

```text
formula.cnf
solver-proof.lrat
problem-metadata.json
exact-domain-description.md
```

The repository should contain enough information to reconstruct what the Boolean variables mean.

### Best use cases

- no dangerous cycle exists in a finite abstraction;
- no valuation word of length \(k\) satisfies all specified constraints;
- a selected finite family of residue templates covers all abstract states;
- no ranking function exists within a chosen finite grammar.

The last type is especially useful: it proves a limitation of the attempted method, not a limitation of mathematics.

---

## 6.3 SMT and SyGuS certificates

SMT is convenient for:

- mixed integer inequalities;
- bit-vectors;
- modular arithmetic;
- arrays of residue weights;
- quantified template constraints.

However, SMT proof production and independent checking are less uniform than SAT proof logging.

Resource:

- [Alethe: Towards a Generic SMT Proof Format](https://arxiv.org/abs/2107.02354)

### Recommendation

Use SMT initially as a candidate generator. Translate final results into:

- exact Rust-checkable inequalities;
- SAT where practical;
- or Lean statements.

Keep the trusted base small.

---

# 7. Formal verification

## 7.1 A small Rust verifier first

The first trusted layer should remain a separate Rust binary that checks:

- affine-prefix recurrence;
- exact valuations;
- modular-lifting claims;
- descent thresholds;
- residue-class coverage;
- cycle equations;
- ranking inequalities;
- graph coverage.

This is practical and keeps development fast.

## 7.2 Lean later

Lean uses a small trusted kernel and supports combining interactive mathematics with automation.

Resource:

- [The Lean Theorem Prover](https://lean-lang.org/papers/system.pdf)

### Suggested progression

1. Implement search in Rust.
2. Emit a compact JSON certificate.
3. Verify it with an independent Rust binary.
4. Formalize the certificate semantics in Lean.
5. Import only particularly valuable results into Lean.

Do not formalize the whole search engine.

### Candidate first Lean theorem

A good first formalization would be:

> For every odd positive integer in residue class \(r \pmod{2^m}\), the supplied valuation word is exact and the resulting macrostep is below the starting value above threshold \(B\).

That theorem is local, reusable, and directly connected to the certificate format.

---

# 8. Optimization domains and engineering techniques

## 8.1 Tiered arithmetic

Use several arithmetic tiers:

```text
u64 fast path
u128 fast path
arbitrary-precision fallback
```

Potential arbitrary-precision backends:

- `num-bigint` for a simple pure-Rust baseline;
- [`rug`](https://docs.rs/rug/latest/rug/) for GMP-backed integers and rationals when benchmarking shows that large-integer arithmetic is a bottleneck.

Do not assume one backend is universally faster. Benchmark the actual workload:

- repeated multiply-by-three and shifts;
- modular reduction by powers of two and three;
- affine-prefix composition;
- large exponentiation;
- serialization.

## 8.2 Macrosteps

Precompute transformations for short parity or valuation blocks:

\[
n\mapsto \frac{3^k n+c}{2^A}.
\]

A macrostep record can contain:

```rust
pub struct MacroStep {
    pub length: u16,
    pub odd_steps: u16,
    pub total_twos: u16,
    pub multiplier_pow3: u16,
    pub additive_constant: u128,
    pub required_residue: u128,
    pub modulus_bits: u16,
}
```

Use `BigUint` when the table width exceeds the safe fixed-width range.

Macrosteps reduce branching overhead and allow vectorized or GPU-friendly evaluation.

## 8.3 Path merging

Different starting values often enter the same later state or certified region. Avoid recomputing a shared tail.

At several scales:

- memoize stopping information for small values;
- detect equal symbolic affine states;
- merge residue-tree nodes with identical future behavior;
- hash-cons automaton and e-graph states;
- use path-merging sieves before launching expensive analysis.

## 8.4 Compressed bitsets

Residue filters and precomputed admissibility tables are often naturally represented as bitvectors.

For sparse or mixed-density sets, benchmark:

- raw `Vec<u64>` bitsets;
- [`roaring`](https://docs.rs/roaring/latest/roaring/) compressed bitmaps;
- sorted integer vectors;
- perfect-hash or finite-state representations.

Roaring bitmaps are designed for compressed set operations, but raw bitsets may be faster for dense, fixed-size sieve tables. Measure both.

## 8.5 SIMD

Potential SIMD targets:

- bulk residue testing;
- batched macrosteps;
- bitset intersections;
- hash/checksum calculation;
- valuation extraction from machine words.

Keep the scalar implementation as the reference and add SIMD behind a feature flag.

## 8.6 GPU

GPU work is justified only after:

- the scalar algorithm is correct;
- profiling identifies a regular bulk kernel;
- memory transfer does not dominate;
- certificates are generated or cross-checked on the CPU.

Good GPU candidates:

- evaluating millions of independent residue representatives;
- fixed-width macrostep application;
- sieve-bitvector generation;
- bulk critical-path scoring.

Poor GPU candidates:

- highly irregular BigUint trajectories;
- branch-heavy symbolic tree refinement;
- SMT or e-graph logic;
- certificate verification.

Follow the example of existing verification projects: keep preprocessing, work partitioning, and independent checks on the CPU.

## 8.7 Deterministic parallelism

Rayon is suitable for independent branch search, but result ordering and checkpointing should be deterministic.

Use:

- stable node identifiers;
- deterministic random seeds;
- sorted result emission;
- per-shard checksums;
- immutable experiment configuration;
- resumable work units.

A distributed search should partition by exact residue-prefix ranges rather than by timing-dependent work stealing when reproducibility matters.

## 8.8 External-memory search

If the symbolic frontier becomes too large:

- store nodes as fixed-width records;
- sort and deduplicate by symbolic state;
- use append-only JSONL only for summaries, not the hot frontier;
- use memory-mapped binary files or an embedded key-value store;
- checkpoint by depth or residue shard;
- retain certificates but discard reconstructible search intermediates.

The core state should be designed for compact binary serialization from the beginning.

---

# 9. Search methods worth comparing

## 9.1 Depth-first search

Best for finding a single deep valuation prefix with low memory. It may, however, spend too long in one structurally repetitive branch.

## 9.2 Beam search

Best for counterexample-like prefix discovery with bounded memory. Run several beams with different scoring functions and preserve diversity by residue class and motif. Absence from a beam is never a proof.

## 9.3 Best-first search

Useful for extracting the strongest current adversarial prefix and combining exact bounds with heuristic scores. The main risk is priority-queue growth.

## 9.4 Branch and bound

Use exact bounds derived from:

- descent thresholds;
- minimal-counterexample inequalities;
- coefficient growth;
- additive remainder;
- residue compatibility;
- cycle divisibility.

This should be the main mathematically meaningful search technique.

## 9.5 Monte Carlo tree search

Useful only as a candidate generator. It may discover unusual valuation motifs but provides no coverage guarantee.

## 9.6 Cube and conquer

Partition a large SAT or symbolic search into independent cubes:

```text
prefix constraints
residue-bit assignments
valuation suffix families
```

Each cube can be solved separately and produce its own certificate. This is attractive for distributed experiments and later formal proof import.

---

# 10. Proposed revised workspace

```text
collatz-lab/
├── crates/
│   ├── collatz-core/
│   ├── collatz-affine/
│   ├── collatz-prefix/
│   ├── collatz-sieve/
│   ├── collatz-cert/
│   ├── collatz-verify/
│   ├── collatz-abstract/
│   ├── collatz-transition-invariants/
│   ├── collatz-ranking/
│   ├── collatz-sat/
│   ├── collatz-sygus/
│   ├── collatz-automata/
│   ├── collatz-egraph/
│   ├── collatz-report/
│   └── collatz-cli/
├── experiments/
│   ├── baseline-verification/
│   ├── sieve-comparison/
│   ├── symbolic-residue-cover/
│   ├── adversarial-prefix/
│   ├── cegar/
│   ├── ranking-synthesis/
│   ├── automata/
│   ├── two-adic-realizability/
│   └── cycle-search/
├── certificates/
├── formal/
│   └── lean/
├── reports/
└── references/
```

Do not create every crate immediately. The structure indicates eventual boundaries.

---

# 11. Revised experimental sequence

## Phase 1: Reproducible kernel

Deliver:

- ordinary and odd-only steps;
- `u64`, `u128`, and BigUint paths;
- affine macrosteps;
- property tests;
- benchmark suite;
- deterministic CLI;
- independent reference implementation.

New requirement: benchmark at least two arbitrary-precision backends before choosing one for large jobs.

## Phase 2: Reproduce known sieve concepts

Deliver:

- descent sieve;
- mod-\(9\) preimage sieve;
- path-merging sieve;
- odd-even-even sieve;
- bitvector representation;
- performance comparison.

For each sieve, generate a Markdown report covering its rule, soundness argument, residue classes eliminated, overlap, runtime, memory, and certificate format.

## Phase 3: Symbolic residue certificates

Deliver:

- valuation-prefix congruence reconstruction;
- exact affine formula;
- descent threshold;
- finite-exception checking;
- certificate verifier;
- residue-cover trie.

Research question: how much of the search space can be eliminated by small local certificates?

## Phase 4: Adversarial prefix search

Deliver:

- several independent scores;
- minimal-counterexample feasibility;
- paradoxical-prefix diagnostics;
- diversity-preserving beam search;
- 2-adic signed-representative analysis.

Research question: what structural motifs survive all currently known simple sieves?

## Phase 5: CEGAR

Deliver:

- finite abstraction;
- maximum-cycle-mean analysis;
- dangerous SCC extraction;
- exact concretization;
- reason-coded spurious counterexamples;
- automatic refinement.

Research question: which state information is repeatedly required to eliminate fake expanding paths?

## Phase 6: Ranking and transition-invariant synthesis

Deliver:

- linear difference constraints;
- residue-weight potentials;
- path-complete multi-potential graphs;
- lexicographic functions;
- SyGuS templates;
- exact certificate extraction.

Research question: can every transition in a large certified subsystem be covered by a small finite union of well-founded relations?

## Phase 7: Automata and e-graphs

Deliver:

- binary transducer prototype;
- regular-language representation of selected residue families;
- e-graph canonicalization of affine macrosteps;
- grammar extraction from surviving prefixes.

Research question: does the unresolved set have low descriptive complexity, or does it require increasing state?

## Phase 8: Proof production

Deliver:

- LRAT proof output for finite SAT claims;
- formally specified certificate schema;
- first Lean residue-class theorem;
- reproducible proof bundle.

## Phase 9: GPU and distributed execution

Only begin after profiling confirms that exhaustive fixed-width kernels are limiting progress. Require exact overflow guards, CPU cross-checks, per-shard checksums, and reproducible job manifests.

---

# 12. High-value experiments that are relatively easy

## Experiment A: Sieve ablation study

Implement each sieve independently and in combinations. Measure:

```text
nodes eliminated
time per node
memory
unique eliminations
certificate size
unresolved density
```

This establishes which ideas are actually carrying the search.

## Experiment B: Critical abstract cycle finder

For a chosen modulus and valuation cap:

1. build the abstract graph;
2. weight edges by \(\log_2 3-a\);
3. find the maximum-mean cycle;
4. derive its exact congruence conditions;
5. classify it as positive realizable, negative integer, nonordinary 2-adic, finite-prefix only, or impossible;
6. repeat after refinement.

This is a concrete CEGAR prototype with clear outputs.

## Experiment C: Ranking weights by residue

Try:

\[
V(n)=\log_2 n+w[n\bmod 2^k].
\]

For each \(k\):

1. construct all sound abstract transitions;
2. solve the difference constraints numerically;
3. extract the obstructing cycle if infeasible;
4. refine or increase \(k\);
5. rationalize and exactly verify any solution.

Even a negative result maps the limits of simple residue-weight potentials.

## Experiment D: Binary-plus-ternary refinement value

Run identical CEGAR experiments with:

```text
mod 2^k only
mod 2^k × mod 3
mod 2^k × mod 9
mod 2^k × mod 27
```

Measure whether ternary state reduces spurious counterexamples enough to justify its state-space cost.

## Experiment E: Valuation-word grammar inference

Collect surviving prefixes and infer repeated suffixes, forbidden substrings, small deterministic automata, substitution motifs, and entropy by depth. Verify every inferred forbidden pattern symbolically.

## Experiment F: Certificate compression with e-graphs

Generate many residue-class descent certificates, place their affine expressions into an e-graph, and extract shared lemmas or minimal expressions.

Question:

> Can thousands of certificates be represented as a small number of symbolic macrostep identities?

## Experiment G: SAT proof of bounded impossibility

Encode:

> There is no minimal-counterexample-feasible valuation word of length \(k\) satisfying constraint family \(C\).

Produce an LRAT proof. This does not settle Collatz, but it validates the full proof-producing pipeline.

---

# 13. What machine learning could usefully do

Machine learning should not predict “converges” versus “does not converge.” All known training examples converge, and prediction would not certify anything.

Useful roles:

- rank search branches;
- cluster unresolved valuation motifs;
- propose invariant templates;
- select CEGAR refinement predicates;
- infer small automata;
- estimate which sieve should run first;
- compress similar certificate families;
- identify anomalous affine-remainder behavior.

Every learned output must pass an exact symbolic checker.

A good interface is:

```text
learner proposes
verifier checks
counterexample becomes new training data
```

This is closer to ICE or CEGIS than ordinary supervised prediction.

---

# 14. What not to prioritize

- **Do not start with a GPU implementation.** The first bottleneck is likely conceptual and symbolic, not arithmetic throughput.
- **Do not test random huge integers as the central experiment.** That generates evidence but little reusable structure.
- **Do not rely on floating-point comparisons for certificates.** Use floating point for prioritization only.
- **Do not make one giant state structure.** Use composable abstract domains and measure the value of each added dimension.
- **Do not trust solver output without proof artifacts.** At minimum, independently re-evaluate the proposed model or certificate.
- **Do not claim novelty too early.** Many Collatz ideas have been rediscovered repeatedly.

Background resources:

- [The \(3x+1\) Problem: An Overview](https://arxiv.org/abs/2111.02635)
- [Annotated bibliography, 1963–1999](https://arxiv.org/abs/math/0309224)
- [Annotated bibliography, 2000–2009](https://arxiv.org/abs/math/0608208)
- [Lagarias's Collatz resource page](https://dept.math.lsa.umich.edu/~lagarias/3x%2B1.html)

---

# 15. Recommended reading order

## Read immediately

1. [Convergence verification of the Collatz problem](https://link.springer.com/article/10.1007/s11227-020-03368-x) — computational baseline and lookup-table strategy.
2. [Improved verification limit for the convergence of the Collatz conjecture](https://link.springer.com/article/10.1007/s11227-025-07337-0) — distributed and sieve improvements.
3. [An improved algorithm for checking all \(n<2^N\)](https://arxiv.org/abs/2602.10466) — newer sieve taxonomy and CPU/GPU organization.
4. [An Automated Approach to the Collatz Conjecture](https://arxiv.org/abs/2105.14697) — termination-rewriting representation.
5. [Transition Invariants](https://swt.informatik.uni-freiburg.de/berit/papers/transition-invariants.pdf) — finite relation covers instead of one decreasing measure.

## Read during the CEGAR phase

6. [Counterexample-Guided Abstraction Refinement](https://link.springer.com/chapter/10.1007/10722167_15)
7. [Abstract Interpretation Frameworks](https://pcousot.github.io/publications/CousotCousot-JLC-n2--3-p103--179-1992.pdf)
8. [ICE: A Robust Framework for Learning Invariants](https://madhu.cs.illinois.edu/CAV14ice.pdf)
9. [Syntax-Guided Synthesis](https://sygus-org.github.io/assets/pdf/Journal_SyGuS.pdf)

## Read during ranking and control experiments

10. [A Complete Method for the Synthesis of Linear Ranking Functions](https://link.springer.com/chapter/10.1007/978-3-540-24622-0_20)
11. [Path-Complete Graphs and Common Lyapunov Functions](https://arxiv.org/abs/1612.03983)

## Read during symbolic representation work

12. [The \(3x+1\) Conjugacy Map](https://websites.umich.edu/~lagarias/doc/bernstein.pdf)
13. [A Survey of Regular Model Checking](https://link.springer.com/chapter/10.1007/978-3-540-28644-8_3)
14. [egg: Fast and Extensible Equality Saturation](https://arxiv.org/abs/2004.03082)
15. [Paradoxical behavior in Collatz sequences](https://arxiv.org/abs/2502.00948)

## Read during proof-production work

16. [The DRAT format and DRAT-trim checker](https://arxiv.org/abs/1610.06229)
17. [Efficient Certified RAT Verification](https://arxiv.org/abs/1612.02353)
18. [The Lean Theorem Prover](https://lean-lang.org/papers/system.pdf)

---

# 16. Recommended Rust technology map

| Need | Initial choice | Later or alternative choice |
|---|---|---|
| Fixed-width arithmetic | `u64`, `u128` | custom SIMD kernels |
| Arbitrary precision | `num-bigint` | `rug`/GMP after benchmarks |
| Parallel search | Rayon | distributed residue shards |
| Graphs and SCCs | Petgraph | custom compact CSR graph |
| Dense sieve bits | `Vec<u64>` | SIMD word operations |
| Sparse residue sets | `roaring` | sorted arrays or custom hybrid |
| SAT construction | RustSAT or internal CNF builder | external state-of-the-art solver |
| Pure Rust SAT | Varisat | mainly for integration and experiments |
| SMT/SyGuS | text protocol to external solver | proof-producing pipeline |
| E-graphs | `egg` | `egglog` for relational fixpoint experiments |
| Serialization | Serde | compact binary certificate format |
| Property testing | Proptest | fuzzing plus differential testing |
| Benchmarking | Criterion | hardware performance counters |
| Formalization | independent Rust verifier | Lean |

The exact crate versions should be pinned when implementation begins.

---

# 17. The most promising combined research program

1. **Build a fast known-good finite verifier.** Reproduce enough of modern sieve-based verification to trust the arithmetic and understand the optimization landscape.
2. **Convert every optimization into a symbolic rule.** A sieve should be able to say why it removed a family.
3. **Construct the unresolved symbolic transition graph.** States represent residue and history information; edges represent possible valuation macrosteps.
4. **Extract the most dangerous cycle or path.** Use maximum cycle mean or branch and bound.
5. **Concretize it exactly.** Determine whether it represents a positive integer family, a finite-prefix illusion, a negative integer, a nonordinary 2-adic integer, or an impossible conjunction.
6. **Refine only where needed.** Record which predicate eliminated the spurious path.
7. **Synthesize a finite relation cover.** Use ranking functions, transition invariants, path-complete potentials, SyGuS, or ICE.
8. **Emit a certificate.** Use exact Rust verification, SAT proof logs, and later Lean.

This approach connects computational number theory with program verification while preserving a small trusted core.

---

# 18. Definition of useful progress

A useful result does not need to solve Collatz. Strong outcomes include:

- a faster or simpler exact sieve;
- a symbolic explanation of an existing sieve;
- a new infinite residue family with a descent certificate;
- a compact grammar for unresolved valuation words;
- a CEGAR dataset classifying spurious divergent paths;
- evidence that a particular abstraction can never prove termination;
- a ranking function for a large subsystem;
- a transition-invariant cover using a small number of relations;
- a theorem linking sustained growth debt to nonordinary 2-adic digit structure;
- an LRAT-certified bounded impossibility result;
- a Lean-verified residue-class certificate;
- a reproducible Rust platform useful to other researchers.

The project should continually ask:

> Did this computation merely run, or did it produce a reusable mathematical object?

---

# 19. Suggested immediate implementation changes

Add these tasks to the original first milestones:

1. Create a benchmark crate comparing `u128`, `num-bigint`, and `rug`.
2. Read and implement a simplified version of Barina's lookup-table method.
3. Implement the descent and path-merging sieves first.
4. Define a generic sieve trait and certificate type.
5. Add mod-\(3^k\) state to the symbolic representation, but keep it optional.
6. Add a maximum-cycle-mean analysis to the abstract graph.
7. Store the nearest signed representative of every \(2^k\) residue.
8. Add paradoxical-prefix diagnostics separating coefficient growth from exact growth.
9. Build a small SAT encoding for bounded valuation-word feasibility.
10. Require every search experiment to emit configuration, exact seed, Git commit, checksums, summary Markdown, and any independently checkable certificates.

The recommended first substantial experiment is the **sieve ablation study**, followed by the **critical abstract cycle CEGAR prototype**.
