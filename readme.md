# Collatz Research Workbench in Rust

## Project purpose

Build an experimental platform for investigating the Collatz conjecture through:

1. exact trajectory computation;
2. adversarial parity-pattern construction;
3. symbolic residue-class analysis;
4. automated descent certificates;
5. counterexample-guided abstraction refinement;
6. piecewise ranking-function synthesis;
7. cycle-certificate searches;
8. investigation of the boundary between positive integers and 2-adic trajectories.

The initial goal is **not** to claim a proof or search blindly through larger integers. The goal is to create machinery that produces mathematically interpretable results:

* a candidate counterexample;
* a finite cycle certificate;
* a proof that a family of integers descends;
* a dangerous abstract trajectory;
* a demonstration that an apparent trajectory is arithmetically impossible;
* or a precise description of why the current abstraction is insufficient.

---

# 1. Research principles

## 1.1 Separate evidence from certificates

Every result should be classified as one of:

### Observation

A pattern found experimentally.

Example:

> Among all valuation prefixes searched to depth 60, the most expansive realizable prefix had these valuations.

### Candidate

Something that appears dangerous but has not been proved to continue indefinitely.

### Certificate

A finite object that an independent program can verify using exact arithmetic.

Examples:

* a nontrivial cycle;
* a residue-class descent certificate;
* an impossible valuation-prefix certificate;
* a valid ranking function over a finite abstraction.

### Theorem candidate

A general pattern suggested by many certificates but not yet proved universally.

---

## 1.2 Keep proof logic separate from search logic

The search program may use:

* floating-point scores;
* heuristics;
* beam search;
* randomized exploration;
* estimated logarithms;
* machine learning;
* parallel execution.

The certificate verifier should use only:

* exact integer arithmetic;
* exact modular arithmetic;
* exact inequalities;
* deterministic algorithms.

The searcher may be complicated. The verifier should be small enough to audit.

---

## 1.3 Prefer symbolic families over isolated integers

Checking another trillion starting values adds evidence.

Proving that every integer in a residue class descends adds reusable mathematical structure.

Prioritize results of the form:

[
n\equiv r\pmod{2^m}
\quad\Longrightarrow\quad
S^k(n)<n
]

over results of the form:

[
\text{This one very large } n \text{ eventually reached }1.
]

---

# 2. Mathematical representation

## 2.1 Ordinary Collatz map

[
T(n)=
\begin{cases}
n/2,&n\equiv0\pmod2,\
3n+1,&n\equiv1\pmod2.
\end{cases}
]

This representation is useful for:

* ordinary stopping time;
* total stopping time;
* peak value;
* direct validation;
* comparison with published computations.

---

## 2.2 Odd-only Collatz map

For an odd positive integer (n), define:

[
a=v_2(3n+1),
]

where (v_2(x)) is the largest exponent (a) such that (2^a\mid x).

Then define:

[
S(n)=\frac{3n+1}{2^a}.
]

The result is the next odd number in the trajectory.

This should be the primary representation for symbolic experiments.

---

## 2.3 Valuation words

For an odd-only trajectory, record:

[
a_i=v_2(3n_i+1).
]

A finite valuation word is:

[
(a_0,a_1,\ldots,a_{k-1}).
]

Define:

[
A_0=0,
\qquad
A_k=\sum_{i=0}^{k-1}a_i.
]

After (k) odd steps, the trajectory has the exact form:

[
n_k=\frac{3^k n_0+c_k}{2^{A_k}},
]

where:

[
c_0=0,
]

and:

[
c_{i+1}=3c_i+2^{A_i}.
]

This recurrence should be one of the central data structures in the project.

---

## 2.4 Growth debt

Define:

[
D_k=k\log_2 3-A_k.
]

The multiplicative part of the (k)-step transformation is:

[
\frac{3^k}{2^{A_k}}=2^{D_k}.
]

Interpretation:

* (D_k>0): multiplicative expansion;
* (D_k<0): multiplicative contraction;
* (D_k\approx0): near-critical behavior.

Use floating point for search ranking, but compare the exact integers

[
3^k
\quad\text{and}\quad
2^{A_k}
]

when generating certificates.

---

## 2.5 Exact descent threshold

Suppose a valuation prefix produces:

[
n_k=\frac{3^k n+c_k}{2^{A_k}}
]

and:

[
2^{A_k}>3^k.
]

Then:

[
n_k<n
]

whenever:

[
n>
\frac{c_k}{2^{A_k}-3^k}.
]

Define the exact integer threshold:

[
B=
\left\lfloor
\frac{c_k}{2^{A_k}-3^k}
\right\rfloor+1.
]

Every starting value following that valuation prefix and satisfying (n\ge B) descends below itself after (k) odd steps.

This gives a natural machine-verifiable certificate.

---

## 2.6 Minimal-counterexample constraint

If (n_0) were the smallest counterexample, its trajectory could never fall below (n_0).

Therefore every prefix must satisfy:

[
n_j\ge n_0.
]

Using the exact affine expression:

[
\frac{3^j n_0+c_j}{2^{A_j}}\ge n_0.
]

Equivalently:

[
c_j\ge
\left(2^{A_j}-3^j\right)n_0.
]

When (2^{A_j}>3^j), this gives an upper bound:

[
n_0\le
\frac{c_j}{2^{A_j}-3^j}.
]

This is extremely useful for pruning. A valuation prefix may force its starting integer to be simultaneously:

* inside a specific congruence class;
* above a positivity bound;
* and below a descent-derived upper bound.

If no integer satisfies all three, the prefix cannot belong to a minimal counterexample.

---

# 3. Proposed Cargo workspace

```text
collatz-lab/
├── Cargo.toml
├── README.md
├── crates/
│   ├── collatz-core/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── ordinary.rs
│   │   │   ├── odd_map.rs
│   │   │   ├── valuation.rs
│   │   │   ├── affine.rs
│   │   │   └── arithmetic.rs
│   │
│   ├── collatz-prefix/
│   │   ├── src/
│   │   │   ├── tree.rs
│   │   │   ├── extend.rs
│   │   │   ├── scoring.rs
│   │   │   └── realizability.rs
│   │
│   ├── collatz-cert/
│   │   ├── src/
│   │   │   ├── descent.rs
│   │   │   ├── cycle.rs
│   │   │   ├── verify.rs
│   │   │   └── schema.rs
│   │
│   ├── collatz-abstract/
│   │   ├── src/
│   │   │   ├── state.rs
│   │   │   ├── graph.rs
│   │   │   ├── scc.rs
│   │   │   ├── concretize.rs
│   │   │   └── refine.rs
│   │
│   ├── collatz-ranking/
│   │   ├── src/
│   │   │   ├── difference_constraints.rs
│   │   │   ├── potentials.rs
│   │   │   ├── cycle_mean.rs
│   │   │   └── exactify.rs
│   │
│   ├── collatz-solver/
│   │   ├── src/
│   │   │   ├── z3_model.rs
│   │   │   └── constraints.rs
│   │
│   ├── collatz-report/
│   │   ├── src/
│   │   │   ├── markdown.rs
│   │   │   ├── jsonl.rs
│   │   │   └── summary.rs
│   │
│   └── collatz-cli/
│       └── src/main.rs
│
├── experiments/
│   ├── prefix-search/
│   ├── residue-cover/
│   ├── cegar/
│   ├── ranking-functions/
│   ├── two-adic/
│   └── cycle-search/
│
├── certificates/
├── reports/
└── benches/
```

You can begin with only:

* `collatz-core`;
* `collatz-prefix`;
* `collatz-cert`;
* `collatz-cli`.

The other crates can be added when their boundaries become clear.

---

# 4. Suggested Rust dependencies

```toml
[workspace.dependencies]
num-bigint = "..."
num-traits = "..."
rayon = "..."
serde = { version = "...", features = ["derive"] }
serde_json = "..."
clap = { version = "...", features = ["derive"] }
tracing = "..."
tracing-subscriber = "..."
proptest = "..."
criterion = "..."
petgraph = "..."
z3 = { version = "...", optional = true }
```

Use current compatible versions rather than copying fixed versions from this outline.

## Core dependency roles

| Crate        | Purpose                                         |
| ------------ | ----------------------------------------------- |
| `num-bigint` | Exact arbitrary-precision integers              |
| `num-traits` | Generic numeric traits                          |
| `rayon`      | Parallel search across independent branches     |
| `serde`      | Reproducible certificate and result formats     |
| `clap`       | Command-line interface                          |
| `proptest`   | Property-based correctness testing              |
| `criterion`  | Performance regression testing                  |
| `petgraph`   | Abstract-state graphs and SCC analysis          |
| `z3`         | Optional SMT-based concretization and synthesis |
| `tracing`    | Structured logs and experiment diagnostics      |

Use `u64` or `u128` for fast execution when safe, then promote to `BigUint` before overflow.

Do not allow unchecked overflow in research results.

---

# 5. Core data structures

## 5.1 Odd step

```rust
pub struct OddStep<N> {
    pub from: N,
    pub to: N,
    pub valuation: u32,
}
```

## 5.2 Affine prefix

```rust
pub struct AffinePrefix {
    pub valuations: Vec<u32>,
    pub odd_steps: usize,
    pub total_twos: u64,
    pub constant: BigUint,
    pub residue: BigUint,
    pub modulus_exponent: u64,
}
```

Interpret this as:

[
n_k=
\frac{3^k n_0+c_k}{2^{A_k}}.
]

The prefix should also contain the congruence class of starting values that realize its exact valuation word.

An exact valuation word of total valuation (A_k) determines a starting congruence class at a corresponding power-of-two precision. The implementation should derive and verify this incrementally rather than assume it.

---

## 5.3 Descent certificate

```rust
pub struct DescentCertificate {
    pub valuations: Vec<u32>,
    pub residue: BigUint,
    pub modulus: BigUint,
    pub odd_steps: usize,
    pub total_twos: u64,
    pub constant: BigUint,
    pub threshold: BigUint,
    pub checked_exceptions: Vec<BigUint>,
}
```

The independent verifier checks:

1. the valuation word is exactly realizable by the residue class;
2. the affine recurrence is correct;
3. (2^{A_k}>3^k);
4. the threshold was computed correctly;
5. every member of the class above the threshold descends;
6. every smaller positive representative was checked explicitly.

---

## 5.4 Search node

```rust
pub struct PrefixNode {
    pub prefix: AffinePrefix,
    pub growth_debt: f64,
    pub minimum_relative_height: f64,
    pub smallest_representative: BigUint,
    pub minimal_counterexample_feasible: bool,
    pub score: f64,
}
```

Only `score` and heuristic fields may use floating point.

All feasibility decisions should be recomputed exactly.

---

# 6. Experiment 0: Establish a trustworthy core

## Research question

Can the implementation compute ordinary and odd-only trajectories exactly and consistently?

## Implementation

Implement:

```rust
fn collatz_step(n: &BigUint) -> BigUint;
fn odd_step(n: &BigUint) -> OddStep<BigUint>;
fn valuation_two(n: &BigUint) -> u64;
fn stopping_time(n: &BigUint) -> Option<u64>;
fn trajectory_prefix(n: &BigUint, limit: usize) -> Vec<BigUint>;
```

Also implement optimized versions for `u64` and `u128`.

## Required tests

### Known trajectory

Verify the trajectory beginning at (27).

### Ordinary versus odd-only equivalence

For randomly generated odd inputs:

1. run one odd-only step;
2. run ordinary steps until the next odd value;
3. confirm that the results and valuation agree.

### Affine-prefix identity

For random starting values and random prefix lengths, verify:

[
2^{A_k}n_k=3^k n_0+c_k.
]

### Forced-growth family

For:

[
n=2^{k+1}-1,
]

verify that the first (k) odd valuations are all (1).

### Cross-representation agreement

Confirm that `u128` and `BigUint` implementations agree wherever the `u128` calculation does not overflow.

## Success criterion

No research experiment begins until all core identities are covered by unit tests and property tests.

---

# 7. Experiment 1: Adversarial valuation-prefix search

## Research question

What are the most counterexample-like finite valuation words that are exactly realizable by positive integers?

## Search object

Search over valuation words:

[
(a_0,a_1,\ldots,a_{k-1}),
\qquad a_i\ge1.
]

Each node stores:

* exact affine data;
* exact starting residue;
* modulus precision;
* smallest positive representative;
* growth debt;
* whether a minimal counterexample remains possible;
* whether the prefix already proves descent.

## Extending a prefix

Begin with a prefix defining:

[
n_0\equiv r\pmod{2^m}.
]

To append valuation (a):

1. lift the residue class to the required higher power of (2);
2. test candidate lifts;
3. retain the unique lift, if any, for which the next valuation is exactly (a);
4. update (A_k), (c_k), and the affine formula;
5. verify the result with direct evaluation of the smallest representative.

A direct lifting algorithm is preferable initially. Optimize the modular algebra later.

## Initial branching

Start with:

```text
a ∈ {1, 2, 3, 4, 5, 6, 7, 8}
```

Larger valuations generally create strong contraction and can initially be grouped into an overflow or terminal category.

Do not assume the cap is mathematically valid. It is a search parameter.

## Candidate scores

Experiment with several independent scores.

### Raw growth debt

[
D_k=k\log_2 3-A_k.
]

### Minimum-height score

Prefer prefixes for which no intermediate trajectory falls far below the starting value.

### Minimal-counterexample feasibility

Immediately prune a prefix when no starting integer in its congruence class can satisfy:

[
n_j\ge n_0
]

for every prefix position.

### Ordinary-integer score

Prefer residue classes whose smallest positive representative has a binary structure unlike obvious approximations to negative 2-adic integers.

This is heuristic only.

## Search strategies

Implement these in order:

1. depth-first enumeration;
2. beam search;
3. best-first search;
4. parallel subtree search;
5. optional Monte Carlo tree search.

## Output

For each depth, save:

* highest growth debt;
* longest non-descending prefix;
* smallest realizing integer;
* residue and modulus;
* valuation word;
* peak-to-start ratio;
* reason pruned;
* whether the branch resembles a negative 2-adic trajectory.

## What would be interesting?

* A valuation family that remains minimal-counterexample-feasible much longer than the all-(1) family.
* A recurring structural motif among the surviving prefixes.
* Evidence that every highly expansive prefix forces a starting residue increasingly close to a negative integer.
* A finite upper bound on how long certain classes of low-complexity valuation words can remain feasible.

---

# 8. Experiment 2: Symbolic residue-class descent certificates

## Research question

How much of the positive integers can be certified by symbolic descent arguments rather than direct enumeration?

## Basic procedure

For each valuation prefix:

1. calculate (k), (A_k), and (c_k);
2. test whether:

[
2^{A_k}>3^k;
]

3. if so, compute:

[
B=
\left\lfloor
\frac{c_k}{2^{A_k}-3^k}
\right\rfloor+1;
]

4. prove that every realizing integer (n\ge B) satisfies:

[
S^k(n)<n;
]

5. enumerate the finitely many positive members of the residue class below (B);
6. verify that each reaches a smaller value or reaches 1.

If all checks pass, the entire residue class is certified.

## Prefix trie

Represent valuation words as a trie.

Each node has one of these states:

```rust
enum NodeStatus {
    Unexplored,
    CertifiedDescent,
    Infeasible,
    NeedsRefinement,
    Dangerous,
}
```

Stop expanding certified or infeasible nodes.

Expand only unresolved nodes.

## Primary measurements

Track:

* percentage of explored prefixes certified;
* number of residue classes certified;
* depth distribution of certificates;
* maximum threshold;
* number of unresolved spines;
* growth rate of the unresolved set;
* entropy of the surviving branches.

## Important limitation

A finite-prefix trie may continually follow a dangerous branch whose infinite limit corresponds to a negative or nonordinary 2-adic integer.

This experiment therefore leads directly into the realizability and CEGAR experiments.

## Possible research result

A useful intermediate result would look like:

> Every valuation prefix outside a precisely characterized family has a descent certificate of depth at most (d).

The remaining family would then become the new mathematical target.

---

# 9. Experiment 3: Counterexample-guided abstraction refinement

## Research question

Can we construct a finite over-approximation of Collatz behavior, discover dangerous abstract paths, and repeatedly eliminate paths that no positive integer can realize?

## Initial abstract state

Start with:

```rust
pub struct AbstractState {
    pub residue_mod_two: u64,
    pub two_exponent: u32,
    pub recent_valuations: Vec<u8>,
    pub debt_bucket: i32,
}
```

Later add:

* residue modulo (3^s);
* bit-length category;
* distance from descent threshold;
* sign classification of the associated 2-adic representative;
* affine constant category.

## Abstract transitions

When the available modulus does not determine the next valuation uniquely, include every possible transition.

This intentionally over-approximates the true system.

The abstract graph may therefore contain false trajectories.

## Dangerous abstract structures

Search for strongly connected components containing:

* no descent state;
* no known terminal state;
* a cycle with nonnegative total growth debt;
* or an indefinitely repeatable low-valuation pattern.

## CEGAR loop

```text
Build abstraction
    ↓
Find dangerous SCC or path
    ↓
Extract abstract valuation word
    ↓
Attempt exact concretization
    ↓
Real positive trajectory?
 ┌───────────────┴───────────────┐
Yes                             No
 ↓                               ↓
Candidate counterexample       Explain failure
or real cycle                   and refine abstraction
```

## Concretization checks

Given an abstract path:

1. derive its exact valuation constraints;
2. compute its nested power-of-two residue class;
3. determine whether a positive representative exists;
4. apply minimal-counterexample inequalities;
5. check any cycle equation;
6. optionally submit remaining constraints to Z3.

## Refinement options

When a path is spurious, determine why:

* insufficient binary precision;
* hidden ternary constraint;
* valuation-history mismatch;
* positive-integer condition lost;
* affine constant ignored;
* path corresponds only to a negative integer;
* path exists 2-adically but not among ordinary integers.

Add only the information required to prevent that specific false path.

## Primary metric

Measure how the dangerous abstract state space changes with refinement:

```text
iteration
states
edges
dangerous SCCs
largest dangerous SCC
spurious paths eliminated
new state variables introduced
```

## Valuable negative result

Even failure can produce a meaningful result:

> No abstraction using only (m) binary residue bits and (h) valuation-history entries can prove contraction because this explicit family of indistinguishable states remains.

That describes the minimum information a successful finite-state argument would require.

---

# 10. Experiment 4: Piecewise ranking-function synthesis

## Research question

Can a finite family of potentials prove descent even though ordinary size does not decrease at every step?

## First model

Let each abstract residue state (r) receive a weight (w_r).

Define:

[
V(n)=\log_2 n+w_r.
]

For a transition from state (r) to state (s) with valuation (a), the asymptotic change is approximately:

[
\Delta V
========

\log_2 3-a+w_s-w_r.
]

Seek weights satisfying:

[
\log_2 3-a+w_s-w_r\le-\varepsilon.
]

This is a system of difference constraints.

## Graph interpretation

Each transition carries weight:

[
g_e=\log_2 3-a.
]

A potential exists when every relevant directed cycle has sufficiently negative total gain.

This connects the problem to:

* maximum cycle mean;
* Bellman-Ford difference constraints;
* path-dependent Lyapunov functions;
* stability of constrained switched systems.

## Implementation stages

### Stage A: Numerical exploration

Use floating-point weights to determine whether a promising potential appears to exist.

### Stage B: Rational interval bounds

Replace (\log_2 3) with rigorous rational upper and lower bounds.

### Stage C: Exact certificate

Export:

* states;
* transitions;
* rational potential weights;
* rational (\varepsilon);
* exact inequality checks.

### Stage D: Multi-step edges

Replace individual odd steps with certified macro-transitions:

[
n\longmapsto
\frac{3^k n+c}{2^A}.
]

This may expose contraction hidden across several steps.

### Stage E: Multiple ranking functions

Allow a lexicographic or piecewise potential:

[
(V_1,V_2,\ldots,V_q).
]

A transition may increase one component as long as an earlier component decreases.

## Success criteria

A successful experiment need not cover every integer.

Meaningful results include:

* a ranking function for a large residue subsystem;
* identification of the exact SCCs preventing a global potential;
* proof that no single-state-weight potential can work at a given modulus;
* discovery that adding ternary state information eliminates the obstruction.

---

# 11. Experiment 5: Positive integers versus 2-adic impostors

## Research question

Why do dangerous infinite valuation patterns frequently correspond to negative or nonordinary 2-adic starting values rather than positive integers?

## Basic representation

A valuation prefix determines increasingly precise low-order binary digits of its starting value.

Track:

* residue (r_k);
* modulus (2^{m_k});
* smallest nonnegative representative;
* signed representative nearest zero;
* Hamming weight;
* runs of leading or trailing (1) bits;
* distance from (0);
* distance from (-1\bmod 2^{m_k}).

## Reference example

The valuation pattern:

[
1,1,1,\ldots
]

forces residues approaching:

[
-1
]

in the 2-adic integers.

Finite prefixes are realized by positive numbers such as:

[
2^{k+1}-1,
]

but the infinite limiting object is negative.

## Experiment classes

Generate valuation words from:

* periodic sequences;
* eventually periodic sequences;
* substitution systems;
* finite automata;
* linear-feedback shift registers;
* low-complexity handcrafted rules;
* adversarial beam search.

For each word:

1. derive its residue sequence;
2. determine whether it approaches a recognizable ordinary integer;
3. test whether periodicity produces a cycle equation;
4. classify the candidate as positive, negative, nonintegral, or unresolved;
5. measure its long-term growth debt.

## Desired conjecture form

Look for statements such as:

> Every valuation word with sustained positive growth debt forces infinitely many high-order binary digits to be nonzero.

Since an ordinary positive integer has only finitely many nonzero binary digits, such a theorem would exclude divergent positive trajectories.

This experiment is speculative. Its value is in identifying a precise bridge between:

* valuation growth;
* binary digit structure;
* and ordinary positivity.

---

# 12. Experiment 6: Cycle-certificate search

## Research question

Can valuation words produce exact nontrivial integer cycles?

Suppose a cycle contains (k) odd values with valuations:

[
(a_0,\ldots,a_{k-1}).
]

Then:

[
n_k=n_0.
]

Using the affine expression:

[
n_0=
\frac{c_k}{2^{A_k}-3^k}.
]

A positive cycle requires:

[
2^{A_k}>3^k.
]

It also requires:

1. the denominator divides (c_k);
2. (n_0) is positive and odd;
3. every intermediate value is positive and odd;
4. every valuation is exact;
5. the cycle is not the familiar (1)-cycle in odd-only form.

## Search order

1. enumerate short valuation words;
2. reject using the comparison (2^{A_k}\le3^k);
3. use modular divisibility filters;
4. perform exact division only for survivors;
5. verify every proposed cycle independently.

## Main purpose

This is unlikely to be the easiest route to a new counterexample.

Its value is:

* validating affine and valuation machinery;
* producing finite certificates;
* recovering known negative cycles when signed integers are enabled;
* identifying arithmetic patterns that nearly satisfy cycle divisibility.

---

# 13. Experiment 7: Integrated adversarial search

## Research question

Can the different experimental signals be combined to construct the most plausible counterexample candidates?

## Candidate score

A combined score could include:

[
\text{score}
============

\alpha D_k
+\beta L_k
-\gamma R_k
-\delta C_k,
]

where:

* (D_k): growth debt;
* (L_k): length without dropping below the start;
* (R_k): evidence that the residue approaches a negative 2-adic integer;
* (C_k): arithmetic complexity or certificate proximity.

Run separate searches with different objectives rather than trusting one combined score.

## Candidate categories

Save candidates that maximize:

* total growth debt;
* non-descending length;
* peak-to-start ratio;
* survival under minimal-counterexample constraints;
* distance from known negative 2-adic patterns;
* resistance to residue-class descent certificates;
* persistence across abstraction refinements.

## Candidate lifecycle

Every candidate should eventually receive one status:

```text
Still running
Reached 1
Dropped below start
Entered certified residue class
Valuation prefix impossible
Corresponds to negative 2-adic behavior
Forms a cycle
Requires deeper analysis
```

Failures are research data. Record why every candidate died.

---

# 14. Command-line interface

```text
collatz trace <N>
collatz trace <N> --odd-only
collatz scan --start <N> --count <COUNT>
collatz prefix search --depth 50 --beam 10000
collatz prefix verify <FILE>
collatz cert generate --depth 40
collatz cert verify <FILE>
collatz cover build --max-depth 50
collatz abstract build --two-bits 12 --history 4
collatz abstract cegar --config experiment.toml
collatz ranking synth --graph graph.json
collatz cycle search --odd-steps 20
collatz report build <RESULT_DIRECTORY>
```

## Example initial experiment

```bash
cargo run --release -- \
  prefix search \
  --depth 40 \
  --max-valuation 8 \
  --beam 10000 \
  --objective minimal-counterexample \
  --output reports/prefix-depth-40.jsonl
```

These parameters are starting points, not mathematically privileged values.

---

# 15. Output format

Use JSON Lines for large experiment streams.

```json
{
  "experiment": "prefix-search",
  "version": 1,
  "depth": 40,
  "valuations": [1, 1, 2, 1],
  "total_twos": 5,
  "constant": "47",
  "residue": "23",
  "modulus": "64",
  "growth_debt": 1.33985,
  "status": "minimal-counterexample-feasible",
  "reason": null
}
```

Every output directory should also contain:

```text
metadata.json
config.toml
results.jsonl
summary.md
checksums.txt
git-commit.txt
```

Record:

* Git commit;
* compiler version;
* command;
* configuration;
* random seed;
* CPU/thread count;
* start and completion timestamps;
* certificate schema version.

---

# 16. Independent verifier

Create a separate binary:

```text
collatz-verify
```

It should not depend on:

* Rayon;
* Petgraph;
* Z3;
* search heuristics;
* ranking synthesis;
* experiment configuration.

It should accept a certificate file and return:

```text
VALID
```

or:

```text
INVALID: exact reason
```

Keep this verifier intentionally small.

A research result becomes much stronger when another person can verify it without rerunning the expensive search.

---

# 17. Testing strategy

## Unit tests

Test:

* valuation calculations;
* affine recurrence;
* modular lifting;
* threshold calculations;
* cycle formulas;
* certificate parsing.

## Property tests

Generate random odd integers and verify:

[
2^{A_k}n_k=3^k n_0+c_k.
]

Generate random valuation prefixes and compare:

* symbolic execution;
* concrete execution of realizing representatives.

Generate certificates, modify one field, and confirm that verification fails.

## Differential tests

Implement important calculations twice:

* optimized version;
* simple reference version.

Compare them over random inputs.

## Fuzzing targets

Fuzz:

* certificate deserialization;
* valuation-prefix extension;
* modular lifting;
* conversion between `u128` and `BigUint`;
* boundary conditions around exact powers of two.

## Benchmarks

Measure:

* odd steps per second;
* prefix extensions per second;
* BigUint promotion frequency;
* memory per search node;
* scaling across Rayon thread counts;
* SCC construction time;
* certificate-verification throughput.

---

# 18. Recommended milestone order

## Milestone 1: Exact trajectory engine

Deliver:

* ordinary map;
* odd-only map;
* arbitrary-precision arithmetic;
* trajectory CLI;
* property tests.

## Milestone 2: Affine valuation prefixes

Deliver:

* valuation-word representation;
* exact (A_k) and (c_k);
* congruence reconstruction;
* prefix verification.

## Milestone 3: Descent certificates

Deliver:

* exact descent threshold;
* residue-class certificates;
* independent verifier;
* JSON serialization.

This is the first stage capable of producing mathematically reusable results.

## Milestone 4: Adversarial prefix search

Deliver:

* beam search;
* exact pruning;
* candidate reports;
* parallel exploration.

## Milestone 5: Symbolic descent trie

Deliver:

* prefix tree;
* certified and unresolved branch classification;
* stubborn-spine reports.

## Milestone 6: Abstract graph and CEGAR

Deliver:

* residue-state abstraction;
* SCC search;
* abstract witness extraction;
* exact concretization;
* refinement loop.

## Milestone 7: Ranking-function synthesis

Deliver:

* difference constraints;
* maximum-cycle-mean analysis;
* numerical potentials;
* exact rational certificates.

## Milestone 8: 2-adic boundary experiments

Deliver:

* residue-sequence analysis;
* periodic-word classification;
* positive/negative/nonordinary indicators;
* theorem-candidate reports.

---

# 19. First research run

After completing Milestones 1–3, run this experiment:

## Objective

Find valuation prefixes that remain compatible with a minimal counterexample for as long as possible.

## Parameters

```toml
max_depth = 40
max_valuation = 8
beam_width = 10000
objective = "minimal-counterexample-feasible"
parallel = true
```

## For each prefix

1. reconstruct the exact starting residue;
2. update (A_k) and (c_k);
3. test exact realizability;
4. apply every minimal-counterexample inequality;
5. derive any descent threshold;
6. prune impossible or certified prefixes;
7. rank unresolved prefixes;
8. save the strongest survivors.

## Report sections

```markdown
# Prefix Search Report

## Configuration
## Number of Nodes Explored
## Number Pruned as Impossible
## Number Certified as Descending
## Number Remaining Dangerous
## Best Growth-Debt Prefix
## Longest Minimal-Counterexample-Feasible Prefix
## Residues Closest to Negative Integers
## Recurring Valuation Motifs
## Suggested Refinements
```

## Key question

Do the strongest surviving prefixes share a structure that can be described without listing every valuation?

That structural description is more important than the candidate numbers themselves.

---

# 20. Research questions the platform should answer

1. How quickly does the set of minimal-counterexample-feasible prefixes shrink?
2. Do dangerous prefixes converge toward negative 2-adic integers?
3. Are all low-complexity infinite valuation patterns incompatible with positive divergence?
4. Which residue information is required to distinguish real and spurious dangerous paths?
5. Can every dangerous abstract SCC be eliminated by finite refinement?
6. Can a piecewise potential prove descent outside one explicit exceptional family?
7. Does adding residues modulo powers of (3) materially improve certification?
8. Are the unresolved prefixes statistically rare but structurally similar?
9. Can the unresolved tree be represented by a finite grammar or automaton?
10. Does survival force increasing binary complexity?
11. Can a symbolic certificate cover infinitely many starting values at once?
12. What is the smallest abstraction for which every cycle has negative growth?

---

# 21. Questions that would alter the implementation plan

These do not need to be answered before beginning the core engine.

## Intended outcome

Is the primary goal:

* a serious research repository;
* a personal mathematical experiment;
* a public educational project;
* or an article documenting the attempt?

## Available compute

Will experiments run on:

* a normal desktop;
* a high-core workstation;
* cloud machines;
* or multiple distributed workers?

## Solver preference

Are native dependencies such as Z3 acceptable, or should the initial repository remain pure Rust?

## Interface preference

Should the system primarily produce:

* CLI output;
* Markdown reports;
* CSV/JSON data;
* an interactive web interface;
* or all of the above?

## Formal verification

Would a later Lean, Coq, or Isabelle verifier be desirable, or is an independent exact Rust verifier sufficient initially?

---

# 22. Definition of success

The project should not define success only as solving the Collatz conjecture.

Strong intermediate successes include:

* discovering a new symbolic descent family;
* building a compact certificate checker;
* identifying a previously unnoticed dangerous valuation structure;
* proving a class of abstract proof methods insufficient;
* connecting dangerous growth to negative 2-adic approximation;
* synthesizing a ranking function for a substantial subsystem;
* producing a reproducible CEGAR framework for arithmetic dynamics;
* or turning millions of computations into a small number of general symbolic rules.

The guiding principle should be:

> Every computation should either produce a certificate, eliminate a family of possibilities, or reveal exactly what information the current model is missing.

---

# References for proposed Rust tooling

* `num-bigint` provides arbitrary-precision signed and unsigned integer types.
* Rayon provides data-parallel iteration and task execution.
* Petgraph provides directed graph structures, algorithms, and Graphviz output.
* The Rust `z3` crate provides bindings to the Z3 SMT solver.
* Clap provides command-line argument parsing.
* Serde provides serialization and deserialization infrastructure.
* Proptest supports generated property tests and shrinking of failures.
* Criterion provides statistics-based Rust benchmarking.
