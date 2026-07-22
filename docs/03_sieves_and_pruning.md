# Computational Sieves & Tree Pruning Framework

## 1. The Four-Category Sieve Taxonomy

To avoid conflating algebraic constraints, state subsumption, and minimal counterexample bounds, all sieves in the workbench are classified into a rigorous 4-category taxonomy:

```text
                                ┌─────────────────────────┐
                                │     Prefix Sieve        │
                                └────────────┬────────────┘
                                             │
     ┌──────────────────────┬───────────────┴───────────────┬──────────────────────┐
     ▼                      ▼                               ▼                      ▼
┌──────────────┐   ┌─────────────────┐             ┌──────────────────┐   ┌──────────────────┐
│ Category 1:  │   │   Category 2:   │             │   Category 3:    │   │   Category 4:    │
│ Transition   │   │ Subsumption &   │             │ Minimal Counter- │   │ Search           │
│ Infeasibility│   │ Deduplication   │             │ example Exclusion│   │ Diagnostics      │
├──────────────┤   ├─────────────────┤             ├──────────────────┤   ├──────────────────┤
│ Modulo/edge  │   │ State already   │             │ Valid path in Z2 │   │ Heuristic        │
│ constraints  │   │ covered/subsumed│             │ but cannot be in │   │ prioritization   │
│ incompatible │   │ in search DAG.  │             │ least N+ counter-│   │ score.           │
│ with state.  │   │ Examples:       │             │ example.         │   │ Example:         │
│              │   │ - PathMerging   │             │ Examples:        │   │ - TwoAdic        │
│              │   │ - OddEvenEven   │             │ - DescentSieve   │   │   Impostor       │
│              │   │                 │             │ - MinimalCounter │   │                  │
└──────────────┘   └─────────────────┘             └──────────────────┘   └──────────────────┘
```

> [!NOTE]
> **Semantic Emptiness Clarification**: No finite valuation word, by itself, is semantically empty over positive integers $\mathbb{N}^+$, because every finite valuation word yields a non-empty residue class modulo $2^{A_k+1}$ containing infinitely many positive odd integers. Category 1 applies strictly to transition/edge constraints incompatible with full source/target state configurations.

---

## 2. Trait Specification & Concurrency Guidelines

### 2.1 The `PrefixSieve` Trait
Every sieve implements the thread-safe `PrefixSieve` trait (`collatz-sieve`):

```rust
pub enum SieveResult {
    Keep,
    Reject { reason: RejectionReason },
    Refine { requested_bits: u32 },
}

pub trait PrefixSieve: Send + Sync {
    fn name(&self) -> &'static str;
    fn evaluate(&self, state: &PrefixState) -> SieveResult;
}
```

### 2.2 Strict Concurrency & Memory Rules
1. **Thread Safety (`Send + Sync`):** Sieves are evaluated in parallel across subtrees using `rayon::iter::ParallelIterator`.
2. **Zero Hot-Loop Allocations:** Sieve implementations must never perform heap allocations inside `evaluate`. Buffer workspaces must be passed in or allocated on the stack.
3. **Interior Mutability:** State updates (such as statistics tracking) must use atomic counters (`AtomicUsize`) or lock-free concurrent structures (`DashMap`), avoiding mutex bottlenecks.

---

## 3. Sieve Descriptions & Mechanics

### 3.1 Subsumption & Transition Constraint Sieves

#### Mod-9 Preimage Sieve Deprecation Note
> [!WARNING]
> **Deprecated in Phase 3.5**: The naive `Mod9PreimageSieve` testing residue mod 9 against LSB-first residue representatives $r \pmod{2^{A_k}}$ has been removed. Because $\gcd(2^{A_k}, 9) = 1$, the CRT guarantees that integers $n_0 \equiv r \pmod{2^{A_k}}$ cover all residues modulo 9 uniformly. Mod-9 exclusion is valid strictly under predecessor subsumption or minimal-counterexample logic (e.g. Angeltveit 2026).

#### Path-Merging Sieve (`PathMergingSieve`)
* **Principle:** Detects when two distinct valuation prefixes merge into identical abstract residue states.
* **Mechanism:** Maintains thread-local LRU caches flushing to a global concurrent table to prevent lock contention across Rayon threads. Emits `infeasible_subsumption_v1` certificates containing explicit simulation witnesses (source/target affine states $(A_k, c_k)$, residue inclusion offset $m$, and trajectory step alignment offset $j$).
* **Concurrency:** Uses lock-free lookup to avoid mutex bottlenecks in `rayon::iter::ParallelIterator` loops.

#### Odd-Even-Even Sieve (`OddEvenEvenSieve`)
* **Principle:** Analyzes valuation word patterns containing isolated or recurring valuation configurations.
* **Mechanism:** Leverages structural congruence relations to eliminate valuation sub-words that force contradictory modular equations under state bounds.

---

### 3.2 Minimality Sieves (Counterexample Bounds in $\mathbb{N}^+$)

#### Descent Sieve (`DescentSieve`)
* **Principle:** Evaluates whether $2^{A_k} > 3^k$.
* **Mechanism:** If $2^{A_k} > 3^k$, computes threshold $B = \lfloor \frac{c_k}{2^{A_k} - 3^k} \rfloor + 1$. If the smallest positive representative $r_k \ge B$, the prefix is certified as descending and pruned from further expansion. Emits `descent_v1` certificate.

#### Minimal-Counterexample Sieve (`MinimalCounterexampleSieve`)
* **Principle:** Applies intermediate step lower bounds $n_j \ge n_0$.
* **Mechanism:** For each intermediate step $j$ where $2^{A_j} > 3^j$, enforces $n_0 \le \frac{c_j}{2^{A_j} - 3^j}$. If $r_k > \frac{c_j}{2^{A_j} - 3^j}$, the prefix cannot belong to a minimal counterexample and is rejected. Emits `infeasible_minimality_v1` certificate.

#### 2-Adic Impostor Search Diagnostic (`TwoAdicImpostorSieve`)
* **Principle:** Identifies prefixes approaching the $-1/3$ singularity or negative 2-adic integers (e.g., $x \to -1$).
* **Mathematical Boundary:** In $\mathbb{N}^+$, proximity to a negative 2-adic integer alone is not sufficient to prune a formal proof because $2^{A_k}-1$ is a valid positive integer. Therefore, this sieve acts as a **Search Scoring Diagnostic** in Experiment 1, or triggers formal pruning strictly when the positive representative $2^{A_k}-1$ exceeds the minimal counterexample bound.

---

## 4. Bitset Data Structures: Roaring Bitmaps

For tracking residue class exclusions and precomputed admissibility tables across large powers of two ($2^{20}$ to $2^{32}$):
* Standard `HashSet<u64>` suffers from severe memory overhead and pointer indirection.
* Plain `Vec<u64>` bitsets become unwieldy for high modular powers.
* **Roaring Bitmaps (`roaring` crate):** Uses hybrid uncompressed bitsets, run-length encoding (RLE), and sparse arrays. It drastically compresses sparse modular exclusion bitsets while accelerating set operations (`union`, `intersection`) via SIMD vectorization.

---

## 5. Experiment A: Sieve Ablation Protocol

To quantify the exact efficacy of each sieve, **Experiment A (Sieve Ablation Study)** executes search benchmarks with sieves enabled individually and in combination.

### Measured Metrics
```text
Sieve Combination
├── Nodes Explored
├── Nodes Eliminated
├── Memory Footprint (MB)
├── Execution Time (s)
├── Unique Eliminations (nodes caught ONLY by this sieve)
└── Certificate Output Volume
```

The output of Experiment A produces an empirical ranking of sieves, determining the optimal pipeline order for deep adversarial searches.

