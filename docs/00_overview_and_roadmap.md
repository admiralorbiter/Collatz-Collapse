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
Phase 4: Adversarial Valuation-Prefix Search
   ├── Diversity-preserving beam search & paradoxical sequence diagnostics
   └── 2-Adic signed representative tracking & -1/3 pole singularity checks

        │
Phase 5: Counterexample-Guided Abstraction Refinement (CEGAR)
   ├── Relational abstract domains (Octagons/Polyhedra)
   ├── Maximum cycle mean analysis on transition graphs
   └── Craig interpolation for spurious trace refinement
        │
Phase 6: Automated Transition Invariants & SyGuS Synthesis
   ├── Piecewise ranking functions & linear difference constraints
   └── Well-foundedness bounds & path-complete Lyapunov graph covers
        │
Phase 7: Word-Based Automata & Equality Saturation
   ├── LSB-first binary transducers & regular residue languages
   └── e-graph macrostep canonicalization (`collatz-egraph`)
        │
Phase 8: Proof Production (SAT/LRAT & Lean 4 Formalization)
   ├── Boolean SAT bit-blasting & LRAT UNSAT proof generation
   └── Lean 4 certificate import macros & formally verified kernel
        │
Phase 9: High-Performance GPU & Sharded Distributed Execution
   └── Optional CUDA/OpenCL kernels & deterministic residue-shard partitioning
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
