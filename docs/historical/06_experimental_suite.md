# Experimental Suite & Reporting Standards

## 1. Core Experiments Overview (Experiments 0–7)

The experimental suite is structured into eight core experiments, progressing from basic arithmetic correctness to integrated search and formal proof production.

---

## 3. Completed Experimental Benchmarks (Phases 1–3 Milestone)

The workbench has completed empirical benchmark runs across depths 10, 18, and 20 using the updated 2-adic Patricia Trie verifier engine and corrected dual valuation semantics:

| Depth Tier | Certificates Generated | Exact-Cylinder Lower Bound ($\mu_{\text{exact}}$) | Broad Union Measure ($\mu_{\text{union}}$) | Raw Overlap-Weighted Mass | Unresolved 2-Adic Measure | Execution Time |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Depth 10** | 2,080 | $1,376,755 / 2,097,152$ (**65.65%**) | $899,005 / 1,048,576$ (**85.74%**) | 1.3130 | 14.26% | 135.83 ms |
| **Depth 18** | 3,229,802 | $94,787,358,459 / 137,438,953,472$ (**68.97%**) | $61,716,044,541 / 68,719,476,736$ (**89.81%**) | 1.3793 | 10.19% | 36.11 s |
| **Depth 20** | **24,805,616** | **$1,524,876,280,571 / 2,199,023,255,552$ (69.34%)** | **$992,442,370,557 / 1,099,511,627,776$ (90.26%)** | **1.3869** | **9.74%** | **301.02 s** |

### Key Empirical Findings:
1. **90%+ Canonical 2-Adic Union Coverage:** As depth increases ($10 \to 18 \to 20$), the true set measure of covered odd integers accumulates to **90.26%** ($\le 1.0$), proving that over 90% of odd integers descend within 20 odd steps.
2. **Disjoint Cylinder Lower Bound:** The exact cylinder lower bound accumulates to **69.34%**, providing an unconditional lower bound over strictly disjoint 2-adic cylinders.
3. **Single-Threaded Verifier Throughput:** The pure Rust verifier `collatz-verify` achieves single-threaded throughput exceeding **82,000 to 138,984 certificates per second** with 100% exact integer arithmetic.
4. **Algorithmic Scaling:** Depth-First Search (DFS) and the Tail-Cutoff Lemma bounded total RAM consumption to **< 1 MB** even when evaluating 24.8 million symbolic certificates.

---

## 4. Phase 4 Experimental Suite Results & Science of the Unresolved Set

Following the Phase 3 depth 20 run, Phase 4 evaluated the 9.74% unresolved 2-adic frontier across five specialized experiments:

| Phase 4 Experiment | Target Domain | Command | Key Empirical Result |
| :--- | :--- | :--- | :--- |
| **Exp 4.1: Adversarial Beam Search** | High-debt valuation words | `collatz search --beam-width 200` | Max growth debt $D_{21} = +12.2842$ on all-ones prefix `[1...1]` |
| **Exp 4.2: Sequential Importance Sampling** | Rare-event growth paths ($\mathbb{E}[a_i] = \log_2 3$) | `collatz sis --samples 10000` | Sampled 10,000 "breathing trajectories" with likelihood weights |
| **Exp 4.3: Kramer Dual-Adic Diagnostics** | $\mathbb{Z}_2 \times \mathbb{Z}_3$ drift prediction | `collatz-sieve::kramer` | Tracked 3-adic endpoint residue $c_k \pmod 9$ & real drift |
| **Exp 4.4: Krasikov-Lagarias Potentials** | Macrostep rational invariants | `collatz potential --valuations ...` | Proven strict macrostep contraction ($\Delta V \le -19.64 < 0$) |
| **Exp 4.5: Automata DFA Extraction** | Regular grammar & cycles | `collatz dfa --samples 500` | **$V=2349, E=2348 \implies$ Acyclic DAG with ZERO pumpable cycles** |


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

---

## 4. Phase 5 CEGAR Experimental Suite

### Experiment 5.1: Macrostep Relational CEGAR Engine Benchmark
* **Goal:** Execute the full CEGAR loop on relational abstract state graphs (modulo $2^m$), extracting critical cycles via exact integer comparison $3^{|C|} \ge 2^A$, enforcing positivity guards $n_i \ge 1$, pruning abstract edges, and emitting verified JSON certificates.
* **CLI Command:** `collatz cegar --max-depth 20 --iterations 100`
* **Output Artifacts:** `reports/phase5_cegar_synthesis.md`, `NegativeRefinementLemmaJson` artifacts.

