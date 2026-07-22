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
* **Principle:** Detects when two distinct valuation prefixes converge to identical affine residues modulo $2^m$ and $3^j$.
* **Mechanism:** Maintains a concurrent lookup table of active state signatures. If a prefix arrives at a state known to have been explored or certified by another branch, it is pruned immediately.

#### Odd-Even-Even Sieve (`OddEvenEvenSieve`)
* **Principle:** Analyzes valuation word patterns containing isolated or recurring valuation configurations.
* **Mechanism:** Leverages structural congruence relations to eliminate valuation sub-words that force contradictory modular equations.

---

### 3.2 Minimality Sieves (Counterexample Bounds in $\mathbb{N}^+$)

#### Descent Sieve (`DescentSieve`)
* **Principle:** Evaluates whether $2^{A_k} > 3^k$.
* **Mechanism:** If $2^{A_k} > 3^k$, computes threshold $B = \lfloor \frac{c_k}{2^{A_k} - 3^k} \rfloor + 1$. If the smallest positive representative $r_k \ge B$, the prefix is certified as descending and pruned from further expansion.

#### Minimal-Counterexample Sieve (`MinimalCounterexampleSieve`)
* **Principle:** Applies intermediate step lower bounds $n_j \ge n_0$.
* **Mechanism:** For each intermediate step $j$ where $2^{A_j} > 3^j$, enforces $n_0 \le \frac{c_j}{2^{A_j} - 3^j}$. If $r_k > \frac{c_j}{2^{A_j} - 3^j}$, the prefix cannot belong to a minimal counterexample and is rejected.

#### 2-Adic Impostor Sieve (`TwoAdicImpostorSieve`)
* **Principle:** Identifies prefixes approaching the $-1/3$ singularity or negative 2-adic integers.
* **Mechanism:** Tracks the nearest signed representative of $r_k \pmod{2^{A_k}}$ to zero. If the 2-adic limit forces a negative integer (e.g., $-1, -5/7$), the prefix is flagged as a 2-adic impostor.

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
