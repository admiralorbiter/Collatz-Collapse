# Project Overview & Executive Research Roadmap

## 1. Project Mission & Purpose

The **Collatz Research Workbench in Rust** is an experimental computing platform designed to investigate the Collatz conjecture ($3x+1$ problem) through rigorous mathematical machinery, automated program verification, and proof-producing search algorithms.

Rather than running blind numerical searches to test ever-larger individual integers, this platform focuses on producing **mathematically interpretable, machine-verifiable artifacts**:

1. **Exact residue-class descent certificates:** Proving that entire infinite families of integers ($n \equiv r \pmod{2^m}$) descend below their starting values.
2. **Adversarial parity-pattern construction:** Synthesizing valuation words that maximize growth debt and test the boundary between ordinary integers and 2-adic trajectories.
3. **Counterexample-guided abstraction refinement (CEGAR):** Finding dangerous abstract trajectory cycles and systematically eliminating false paths.
4. **Transition invariants & ranking functions:** Synthesizing piecewise Lyapunov functions and relation covers that guarantee contraction across macrosteps.
5. **Finite cycle certificates:** Searching for exact non-trivial cycle equations or proving their impossibility within bounded parameter spaces.

---

## 2. Core Research Principles

### 2.1 Separate Search Heuristics from Certificate Verification
The platform enforces a strict boundary between search logic and verification logic:
- **Search Logic (`collatz-sieve`, `collatz-cegar`):** Employs floating-point scores, randomized exploration, heuristic beam search, SMT solvers, and parallel execution.
- **Verification Logic (`collatz-cert`, `collatz-verify`):** Uses **only** exact integer arithmetic (`BigUint`, `u128`), exact modular arithmetic, and deterministic verification algorithms. It contains zero solver or floating-point dependencies.

### 2.2 Prefer Symbolic Families Over Isolated Integers
Checking individual starting values adds empirical evidence but no structural coverage. Proving that every integer in a congruence class $n \equiv r \pmod{2^m}$ descends adds reusable mathematical structure. We prioritize results of the form:
$$n \equiv r \pmod{2^m} \quad \Longrightarrow \quad S^k(n) < n$$

### 2.3 Produce Reusable Mathematical Artifacts
Every computation must output a verifiable theorem-shaped object:
- **Observation:** Formally logged experimental data (e.g., maximum growth debt reached at depth $k$).
- **Candidate:** A dangerous valuation prefix remaining feasible under minimal counterexample constraints.
- **Certificate:** A self-contained JSON artifact verified by an independent, minimal verifier binary (`collatz-verify`).
- **Theorem Candidate:** A general pattern suggested by a family of certificates.

---

## 3. The 9-Phase Development Roadmap

```text
Phase 1: Progressive Core & Exact Trajectory Engine
   ├── collatz-core (Ordinary/Odd steps, exact arithmetic)
   ├── collatz-affine (Affine recurrences & modular inversion)
   ├── collatz-cert (Certificate schemas & collatz-verify binary)
   └── collatz-cli (Deterministic command-line interface)
        │
Phase 2: Modern Verification Sieves & Ablation Study
   ├── DescentSieve, Mod9PreimageSieve, PathMergingSieve
   ├── OddEvenEvenSieve, MinimalCounterexampleSieve
   └── Kinematic vs. Minimality classification & Roaring Bitmaps
        │
Phase 3: Symbolic Residue Cover & Certificate Engine
   ├── Valuation-prefix congruence reconstruction
   ├── Exact descent threshold B computation & finite exception checking
   └── Automated JSON certificate generation & verification
        │
Phase 3.5: Proof Audit & Certificate Hardening
   ├── Strict schema validation (deny_unknown_fields) & DoS parsing guards
   ├── Quantified tail descent certificates (tail_descent_v1)
   ├── Canonical disjoint cover manifest export (cover_v1)
   ├── Deprecation of naive mod-9 sieving & simulation witness enhancement
   └── Formalization of 4 Lean 4 structural base lemmas
        │
Phase 4: Adversarial Search & Science of the Unresolved Set [COMPLETE]
   ├── Multi-objective diversity beam search (growth debt, entropy, -1/3 pole match)
   ├── Sequential Importance Sampling (SIS) with exponential tilting theta* = 0.287
   ├── Kramer (2026) dual-adic Z2 x Z3 real drift diagnostics & BigInt signed representatives
   ├── Krasikov–Lagarias-inspired macrostep linear potential certificates (Delta V < 0)
   └── Automata DFA sampled-prefix observation: E = V - 1 (0 cycles on 500 samples)
        │
Phase 4.5: Claims Audit & Proof-Object Refinement Gate [COMPLETE]
   ├── Central Theorem-Status Registry & documentation claims audit
   ├── Negative-Binomial 2-adic baseline & 2.26% audit gap decomposition
   ├── Kraft-McMillan verifier invariants (0 <= mu_exact <= 1, 0 <= Mass_broad <= 2)
   ├── Adversarial claims test suite & Serde DoS defense security bounds
   └── Phase 5 miniature CEGAR vertical slice prototype
        │
Phase 5: Macrostep CEGAR & Linear Ranking Invariant Synthesis [COMPLETE]
   ├── Relational Abstract Domains (ResidueDomain(2^m) + Interval[N_min, N_max])
   ├── Staged Relational Widening (nabla) holding congruence modulus fixed
   ├── Exact Symbolic Integer Karp Cycle Mean Engine (3^|C| >= 2^A <=> lambda* >= 0)
   ├── Explicit Positivity Guards (n_i >= 1) rejecting 2-adic attractors (-1/3)
   ├── Automated Certificate Emission (descent_v1, tail_descent_v1, infeasible_subsumption_v1)
   └── collatz-cegar crate & NegativeRefinementLemmaJson cutoff artifacts
        │
Phase 6: Macrocycle Invariant Synthesis & 2-Adic Fixed-Point Dichotomy [COMPLETE]
   ├── Phase 6A: Valuation-Preserving Modulus Refinement & Tail-Descent Invariants
   ├── Phase 6B: Residue-Weighted Discrete Scalar Invariant Synthesis & Graph Contraction
   ├── Phase 6C: Certificate Schema Formalization & Lean 4 Export Pipeline
   └── Phase 6D: Periodic 2-Adic Fixed-Point & Finite-Fuel Dichotomy Invariants
        │
Phase 7: Full 2-Adic Language Invariant Synthesis & System-Level Integration [IN PROGRESS]
   ├── Phase 7.1: Bounded Semantic Refinement & Invalid Target Audit [COMPLETE]
   ├── Phase 7.2: Guarded Benchmark Graph & Noncommuting Branching Discovery [COMPLETE]
   ├── Phase 7.3-0: Semantic Normalization & Sequence Composition Conventions
   ├── Phase 7.3A: Exact Integer Register Reference Semantics & Periodic Exclusion
   ├── Phase 7.3B: Proved Quotient Abstraction & 3-Adic Branch History Signal
   ├── Phase 7.3C: Infinite Symbolic Dynamics, Transducer & 2-Adic Fractal Geometry
   ├── Phase 7.3D: Reordered Proof Hierarchy & Certificate Schema Enforcements
   ├── Phase 7.3E: Target Expansion Discipline (Target A -> Target B -> Target C)
   └── Phase 7.4: Expanded Word Libraries & Interaction Spectrum at Scale
        │
Phase 8: Diophantine Cycle Bounds & SAT/LRAT Proof Production
   ├── collatz-diophantine (Simons & de Weger linear forms in logarithms)
   ├── Boolean SAT bit-blasting & LRAT UNSAT proof generation
   └── Lean 4 certificate import macros & formally verified kernel
        │
Phase 9: Dual 2-Adic/3-Adic Coupling & High-Performance GPU Execution
   ├── Dual (p, q)-adic endpoint compatibility diagnostics
   └── CUDA/OpenCL kernels & deterministic residue-shard partitioning

```

---

## 4. Definition of Useful Progress

A successful outcome does not require solving the Collatz conjecture outright. High-value deliverables include:
* A symbolic explanation or reduction of existing computational sieves.
* Discovering new infinite residue classes equipped with machine-checked descent certificates.
* Proving that every valuation prefix outside a formally specified regular grammar admits a descent certificate of bounded depth.
* A CEGAR dataset classifying why false divergent abstract paths fail under concrete arithmetic.
* Synthesis of a transition invariant cover for a large subsystem using a small set of well-founded relations.
* An LRAT-certified proof of bounded impossibility for specific valuation-word families.
* A Lean 4 verified theorem certifying a residue class descent.

---

## 5. Central Theorem-Status Registry

To enforce scientific discipline and prevent over-broad claims, all project statements are strictly categorized according to the following registry:

| Category | Description | Examples in Workbench |
| :--- | :--- | :--- |
| **Definition** | Formal terms, representations, or metric choices | `TerminalAtLeast` vs `ExactWord` semantics, Growth debt $D_k$ |
| **Known Theorem** | Established results from literature | Terras stopping time (1976), Tao almost-all bound (2022) |
| **Verified Finite Theorem** | Machine-verified theorems produced by `collatz-verify` | Depth 20 canonical broad cover measure (90.2621%), `tail_descent_v1` |
| **Domain-Scoped Certificate** | Verified properties over explicit, independently checkable domains | Krasikov–Lagarias-inspired potential decrease over macrosteps $M$ |
| **Empirical Observation** | Sampled computational measurements | Sampled DFA prefix acyclicity ($V=2349, E=2348$), SIS weights |
| **Conjecture** | Unproven mathematical hypotheses | General Collatz conjecture, global regular language non-existence |
| **Planned Capability** | Features scheduled for future execution | Phase 5 Polyhedral CEGAR, Phase 8 LRAT proof generation |
