# CEGAR, Abstract Interpretation & Proof Synthesis

## 1. Abstract Domains & Relational Constraints

### 1.1 Candidate Abstract Domains
Abstract interpretation over-approximates Collatz trajectory behavior over finite abstract states. The workbench combines multiple domain abstraction layers:

```text
Abstract State S = ( ResidueDomain(2^k), 
                     ResidueDomain(3^j), 
                     ValuationInterval[a_min, a_max], 
                     GrowthDebtInterval[D_min, D_max],
                     SignedRepresentativeDomain )
```

### 1.2 Relational Abstract Domains & Widening Safeguards
> **Soundness Risk:** Standard interval widening over affine coefficients ($n \mapsto \frac{3^k n + c_k}{2^{A_k}}$) causes exponential loss of precision, quickly admitting false infinite expansion paths.

To maintain precision, `collatz-abstract` uses **Relational Abstract Domains** (Octagons / Polyhedra and Congruence-Interval domains):
* **Galois Connection $(\alpha, \gamma)$:** Maps concrete integer sets $X \subseteq \mathbb{N}^+$ to abstract domain elements $\alpha(X)$ and vice versa.
* **Relational Widening $(\nabla)$:** Widening is applied strictly to linear difference constraints between trajectory variables (e.g., $n_k - n_0 \le c$), preserving modular congruences while bounding growth debt.

---

## 2. Abstract Transition Graphs & Maximum Cycle Mean

### 2.1 Transition Graph Construction
Transitions in the abstract state graph represent candidate macrosteps. When an abstract state does not uniquely specify the next valuation $a$, edges are added for all arithmetically admissible valuations.

### 2.2 Critical Cycle Extraction (Exact Integer Karp's Algorithm)
Each edge $e = (u, v)$ with valuation $a$ is assigned weight $w(e) = \log_2 3 - a$.

To eliminate 100% of floating-point rounding risks near critical boundaries ($\lambda^* \approx 0$), the workbench translates Karp's Maximum Cycle Mean condition ($\text{mean} \ge 0$) into a purely symbolic, exact integer comparison:
$$3^{|C|} \ge 2^{\sum_{e \in C} a_i} \iff \lambda^* \ge 0$$

* If $3^{|C|} < 2^{\sum a_i}$ ($\lambda^* < 0$): Every abstract cycle is strictly contracting.
* If $3^{|C|} \ge 2^{\sum a_i}$ ($\lambda^* \ge 0$): The cycle represents a potential counterexample candidate or a spurious abstract loop.

### 2.3 Explicit Intermediate Positivity Guards
To isolate positive integer trajectories ($\mathbb{N}^+$) from full 2-adic extensions (such as the 2-adic fixed point $-1/3$), concretization enforces explicit intermediate step positivity constraints:
$$n_i \ge 1 \qquad \forall i \in \{0, 1, \ldots, k\}$$

Spurious abstract loops violating positivity guards are immediately pruned, and a verified certificate is emitted.


---

## 3. The CEGAR Loop & Craig Interpolation Refinement

```text
                    ┌───────────────────────────────┐
                    │   Build Abstract State Graph  │
                    └───────────────┬───────────────┘
                                    │
                                    ▼
                    ┌───────────────────────────────┐
                    │  Extract Critical Cycle (SCC) │
                    └───────────────┬───────────────┘
                                    │
                                    ▼
                    ┌───────────────────────────────┐
                    │  Attempt Exact Concretization │
                    └───────────────┬───────────────┘
                                    │
                    ┌───────────────┴───────────────┐
                    ▼                               ▼
       ┌─────────────────────────┐     ┌─────────────────────────┐
       │   Real Positive Cycle   │     │  Spurious Trace Found   │
       │   or Counterexample!    │     │  (2-Adic Impostor / B)  │
       └─────────────────────────┘     └────────────┬────────────┘
                                                    │
                                                    ▼
                                       ┌─────────────────────────┐
                                       │   Craig Interpolation   │
                                       │   Refinement Predicate  │
                                       └────────────┬────────────┘
                                                    │
                                                    └────────────────────┘
```

### 3.1 Concretization Check
For a critical abstract cycle $(a_0, a_1, \ldots, a_{k-1})$:
1. Compute closed-form starting residue: broad class $r_k \equiv -c_k (3^k)^{-1} \pmod{2^{A_k}}$ or exact cylinder $r_k \equiv (2^{A_k}-c_k) (3^k)^{-1} \pmod{2^{A_k + 1}}$.
2. Check if $2^{A_k} > 3^k$ and compute threshold $B = \lfloor \frac{c_k}{2^{A_k} - 3^k} \rfloor + 1$.
3. Check minimal-counterexample upper bounds for all intermediate steps.

### 3.2 Spurious Refinement via Craig Interpolation
When a trace is spurious (e.g., it is valid in 2-adics but violates $\mathbb{N}^+$ minimal counterexample bounds), an automated **Craig Interpolant** is generated:
* An interpolant $I$ separates the concrete state trajectory formulas $A$ from the positivity/threshold bounds $B$ ($A \implies I$ and $I \land B = \text{False}$).
* The interpolant $I$ yields a minimal modulo arithmetic constraint (e.g., adding $n \pmod{2^{A_k + m}}$ or $n \pmod{3^j}$ precision) that destroys the spurious path without bloating the state space.

### 3.3 Undecidability & Refinement Termination
> **Undecidability Warning:** Collatz-like maps are known to be universal (Turing-complete). Therefore, the CEGAR loop is not guaranteed to terminate and may generate infinite refinements.

**Cutoff Rules:**
* Maximum abstraction state limit ($N_{\text{max}} = 100,000$).
* Maximum refinement iterations ($IT_{\text{max}} = 500$).
* Emit a **Negative Refinement Lemma** if cutoff is reached: documenting the exact minimal state information required to eliminate the spurious path.

---

## 4. Transition Invariants & SyGuS Ranking Synthesis

### 4.1 Transition Invariants
Rather than searching for a single global Lyapunov function decreasing at every step, we construct a **disjunctively well-founded transition invariant** (Podelski & Rybalchenko):
$$R = R_1 \cup R_2 \cup \ldots \cup R_m$$

where each $R_i$ is a well-founded relation covering a subset of abstract macrostep transitions.

### 4.2 Syntax-Guided Synthesis (SyGuS) & Lower-Bound Enforcement
Candidates for piecewise ranking functions $V(n)$ are generated via SyGuS over the grammar:

```text
V(n, r) ::= log2(n) | bit_length(n) | weight[r] | V + V | rational * V | max(V, V)
```

> **Well-Foundedness Requirement:** The SyGuS solver must emit a proof of two coupled conditions:
> 1. **Strict Decrease:** $V(S(n)) - V(n) \le -\varepsilon$ for all legal transitions.
> 2. **Lower-Bound Certificate:** $V(n) \ge 0$ for all positive integers $n \in \mathbb{N}^+$ in the domain. A decreasing function without a lower bound does not prove termination.

### 4.3 Path-Complete Lyapunov Graphs
For switched valuation dynamics, individual potential functions $V_i$ are connected via a directed graph $G_L$:
$$V_j(S(n)) - V_i(n) \le -\varepsilon \qquad \forall (i, j) \in E(G_L)$$

This certifies stability across all arithmetically legal valuation paths.

---

## 5. Symbolic Relational Control States & Macrocycle Countdown Refinement

### 5.1 Elimination of False Infinite Self-Loops via Symbolic Control States
Single-step residue abstraction modulo 16 contains a false infinite self-loop $15 \to 15$ under valuation $a = 1$. Rather than building an infinite explicit state graph $(15, 0), (15, 1) \ldots$, `collatz-cegar` uses **2 finite symbolic control states**:
1. `MinusOneCountdownPositive(m)`: $r = 2^m - 1 \pmod{2^m}, \tau \ge 1$.
2. `MinusOneCountdownZero(m)`: $r = 2^m - 1 \pmod{2^m}, \tau = 0$.

#### Symbolic Transition Rules
- `(15, tau >= 2)` $\xrightarrow{a=1}$ `(15, tau >= 1)` ($\Delta \tau = -1$)
- `(15, tau = 1)` $\xrightarrow{a=1}$ `(15, tau = 0)` ($\Delta \tau = -1$)
- `(15, tau = 0)` $\xrightarrow{a=1} 7 \pmod{16}$ (**Exits self-loop to residue 7!**)

### 5.2 Relational Graph Manifest & Canonical SHA-256 Digest
* **State Inventory (9 Control States):** `Residue(1)`, `Residue(3)`, `Residue(5)`, `Residue(7)`, `Residue(9)`, `Residue(11)`, `Residue(13)`, `MinusOnePositive` ($\tau \ge 1$), `MinusOneZero` ($\tau = 0$).
* **Edge Count:** 36 legal transitions.
* **Deterministic SHA-256 Digest Function:** `compute_canonical_relational_graph_hash`.

