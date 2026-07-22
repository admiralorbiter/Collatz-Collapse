# Experimental Suite & Reporting Standards

## 1. Core Experiments Overview (Experiments 0–7)

The experimental suite is structured into eight core experiments, progressing from basic arithmetic correctness to integrated search and formal proof production.

---

### Experiment 0: Establish a Trustworthy Core
* **Goal:** Verify exact ordinary and odd-only step equivalence, arbitrary-precision consistency (`u128` vs `num-bigint`), and affine recurrence identities.
* **Key Tests:**
  1. Trajectory verification for $n = 27$.
  2. Ordinary vs odd-only step equivalence over random odd integers.
  3. Affine prefix identity check: $2^{A_k} n_k = 3^k n_0 + c_k$.
  4. Forced growth family: $n = 2^{k+1} - 1$ produces $k$ consecutive valuations $a_i = 1$.
  5. Closed-form modular inversion verification: $n_0 \equiv -c_k (3^k)^{-1} \pmod{2^{A_k}}$.
* **CLI Command:** `collatz test core`

---

### Experiment 1: Adversarial Valuation-Prefix Search
* **Goal:** Find valuation words that maximize growth debt $D_k = k \log_2 3 - A_k$ while remaining minimal-counterexample feasible.
* **Search Mechanics:** Diversity-preserving beam search over valuations $a_i \in \{1, 2, \ldots, 8\}$.
* **Outputs:** Longest non-descending prefixes, smallest positive representatives, residue classes, growth debt curves.
* **CLI Command:** `collatz prefix search --depth 50 --beam 10000 --objective minimal-counterexample`

---

### Experiment 2: Symbolic Residue-Class Descent Cover
* **Goal:** Determine what percentage of odd starting integers are certified by finite residue-class descent arguments up to depth $k$.
* **Data Structure:** Prefix Trie tracking node status (`CertifiedDescent`, `Infeasible`, `NeedsRefinement`, `Dangerous`).
* **Metrics:** Cumulative percentage certified, depth distribution of certificates, maximum threshold $B$, unresolved spine density.
* **CLI Command:** `collatz cert generate --max-depth 40 --output certificates/`

---

### Experiment 3: Counterexample-Guided Abstraction Refinement (CEGAR)
* **Goal:** Build abstract transition graphs, extract dangerous cycles, concretize exact affine data, and eliminate spurious paths using Craig interpolation.
* **Metrics:** Graph states, edges, dangerous SCC count, spurious paths eliminated per refinement iteration.
* **CLI Command:** `collatz abstract cegar --config experiment3.toml`

---

### Experiment 4: Piecewise Ranking-Function Synthesis
* **Goal:** Synthesize residue-weight potential functions $V(n) = \log_2 n + w[n \bmod 2^k]$ and difference constraints to prove multi-step contraction.
* **Stages:**
  1. Floating-point numerical exploration.
  2. Rational interval constraint solving.
  3. Certificate extraction and exact verification.
* **CLI Command:** `collatz ranking synth --modulus 64 --output ranking_cert.json`

---

### Experiment 5: Positive Integers vs. 2-Adic Impostors
* **Goal:** Classify why dangerous infinite valuation patterns correspond to negative 2-adic integers or the $-1/3$ pole rather than positive integers $\mathbb{N}^+$.
* **Analysis:** Tracks nearest signed representatives, binary Hamming weights, runs of 1-bits, and distance from $-1/3 \pmod{2^{A_k}}$.
* **CLI Command:** `collatz two-adic analyze --pattern "1,1,1,1,2"`

---

### Experiment 6: Cycle-Certificate Search
* **Goal:** Search for exact nontrivial integer cycles or prove their non-existence within bounded valuation lengths $k$.
* **Filter Pipeline:**
  1. Multiplicative condition $2^{A_k} > 3^k$.
  2. Divisibility check $(2^{A_k} - 3^k) \mid c_k$.
  3. Exact integer division and odd-only trajectory validation.
* **CLI Command:** `collatz cycle search --max-odd-steps 25`

---

### Experiment 7: Integrated Adversarial Search
* **Goal:** Combine growth debt scoring, minimal counterexample bounds, 2-adic impostor diagnostics, and CEGAR refinement signals into a multi-objective search pipeline.
* **CLI Command:** `collatz search integrated --config integrated.toml`

---

## 2. High-Value Side Experiments (Experiments A–G)

| Experiment | Title | Core Objective | Primary Deliverable |
| :--- | :--- | :--- | :--- |
| **Experiment A** | Sieve Ablation Study | Measure individual and combined sieve efficacy | Empirical sieve performance matrix |
| **Experiment B** | Critical Abstract Cycle Finder | Karp's algorithm on abstract state graphs | Automated spurious trace extraction |
| **Experiment C** | Residue-Weight Potentials | Solve linear difference constraints over $V(n)$ | Subsystem ranking functions |
| **Experiment D** | Binary-plus-Ternary Value | Benchmark $2^k$ vs $2^k \times 3^j$ abstraction cost | Refinement state space efficiency report |
| **Experiment E** | Grammar Inference | Infer formal languages for surviving prefixes | Regular grammar / Automaton for open spines |
| **Experiment F** | E-Graph Compression | Canonicalize certificates using `egg` | Compressed macrostep lemma library |
| **Experiment G** | SAT Impossibility Proof | Bit-blast bounded prefix search to CNF | LRAT-verified UNSAT proof bundle |

---

## 3. Output Formats & Directory Standards

Every experiment run must output structured, reproducible artifacts into a dedicated results directory:

```text
reports/experiment_run_2026_07_21/
├── metadata.json          # Commit hash, compiler version, CLI flags, seed, timestamp
├── config.toml            # Exact experiment configuration copy
├── results.jsonl          # JSON Lines stream of search nodes / certificates
├── summary.md             # Human-readable Markdown execution summary
└── checksums.txt          # SHA-256 checksums of all output files
```

### JSON Lines Schema (`results.jsonl`)
```json
{
  "experiment": "adversarial-prefix",
  "depth": 40,
  "valuation_word": [1, 1, 2, 1],
  "total_twos": 5,
  "constant": "47",
  "starting_residue": "23",
  "modulus_exponent": 6,
  "growth_debt": 1.33985,
  "status": "minimal_counterexample_feasible",
  "reason": null
}
```
