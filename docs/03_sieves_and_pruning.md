# Computational Sieves & Tree Pruning Framework

## 1. Sieve Taxonomy: Kinematic vs. Minimality

To avoid conflating algebraic impossibilities with bounds constraints, all sieves in the workbench are classified into one of two mathematical categories:

```text
                               ┌─────────────────────────┐
                               │     Prefix Sieve        │
                               └────────────┬────────────┘
                                            │
                    ┌───────────────────────┴───────────────────────┐
                    ▼                                               ▼
     ┌─────────────────────────────┐                 ┌─────────────────────────────┐
     │      Kinematic Sieves       │                 │      Minimality Sieves      │
     ├─────────────────────────────┤                 ├─────────────────────────────┤
     │ Rule out valuation sequences│                 │ Rule out valuation sequences│
     │ that are algebraically      │                 │ that are valid in Z2 but    │
     │ impossible in Z2.           │                 │ impossible for a minimal    │
     │                             │                 │ counterexample in N+.       │
     │ Examples:                   │                 │ Examples:                   │
     │ - Mod9PreimageSieve         │                 │ - DescentSieve              │
     │ - PathMergingSieve          │                 │ - MinimalCounterexampleSieve│
     │ - OddEvenEvenSieve          │                 │ - TwoAdicImpostorSieve      │
     └─────────────────────────────┘                 └─────────────────────────────┘
```

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

### 3.1 Kinematic Sieves (Algebraic Impossibilities in $\mathbb{Z}_2$)

#### Mod-9 Preimage Sieve (`Mod9PreimageSieve`)
* **Principle:** Inspects the residue class $n_0 \pmod 9$ alongside $n_0 \pmod{2^{A_k}}$.
* **Mechanism:** $3n+1 \pmod 9$ takes values strictly in $\{1, 4, 7\}$. This constrains how valuation transitions interact with ternary residues.
* **Certificate:** Returns an algebraic proof showing no starting value in the modular intersection can yield the proposed valuation step.

#### Path-Merging Sieve (`PathMergingSieve`)
* **Principle:** Detects when two distinct valuation prefixes merge into identical abstract residue states modulo $2^m$ and $3^j$.
* **Mechanism:** Maintains thread-local LRU caches flushing to a global concurrent table to prevent lock contention across Rayon threads. Emits `infeasible_subsumption_v1` certificates containing the target valuation word to form a Directed Acyclic Graph (DAG) of certificates.
* **Concurrency:** Uses lock-free lookup to avoid mutex bottlenecks in `rayon::iter::ParallelIterator` loops.

#### Odd-Even-Even Sieve (`OddEvenEvenSieve`)
* **Principle:** Analyzes valuation word patterns containing isolated or recurring valuation configurations.
* **Mechanism:** Leverages structural congruence relations to eliminate valuation sub-words that force contradictory modular equations.

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
