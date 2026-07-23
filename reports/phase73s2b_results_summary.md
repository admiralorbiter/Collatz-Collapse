# Phase 7.3S.2B — Exact Verification Results Summary Report

**Schema Version**: `4.3.0`  
**Git Commit**: `fd16e0e5b89b61a4fe6247df5ba9567749c1b82a`  
**Table Hash (`j=0..32`)**: `b6c8fe576f2ce75e`  
**Primary Engine**: [crates/collatz-cegar/src/backward_approximant_engine.rs](file:///c:/Users/admir/Github/Collatz-Collapse/crates/collatz-cegar/src/backward_approximant_engine.rs)  
**Binary Trie Reduction**: [crates/collatz-cegar/src/cylinder_trie_reduction.rs](file:///c:/Users/admir/Github/Collatz-Collapse/crates/collatz-cegar/src/cylinder_trie_reduction.rs)  
**Reachability Probe**: [crates/collatz-cegar/src/bounded_reachability_probe.rs](file:///c:/Users/admir/Github/Collatz-Collapse/crates/collatz-cegar/src/bounded_reachability_probe.rs)  
**Independent Python Oracle**: [scripts/phase73s2b_backward_tree_oracle.py](file:///c:/Users/admir/Github/Collatz-Collapse/scripts/phase73s2b_backward_tree_oracle.py)

---

## 1. Frozen Badges & Verified Status Matrix

| Status Badge | Verdict | Mathematical Meaning & Scope |
| :--- | :---: | :--- |
| `DISCREPANCY_2_8_0_RESOLVED` | 🟢 **FROZEN** | Corrected $B(2,8,0)=67$. Proven that prior $B=84$ record belonged to `(2,2,8,0)`. |
| `CORRECTED_DEPTH3_J8_TABLE` | 🟢 **FROZEN** | True 3-gap max: Word `(0, 3, 1)` with $Z=11, B=43, \ell=32, Z/B=0.2558$. |
| `EVENTUALLY_PERIODIC_PREIMAGE_FORMULA_VERIFIED` | 🟢 **FROZEN** | Exact preimage $x = -P/R$ ($R = q Q_u$ odd) verified in $\mathbb{Z}_2$. |
| `PLAIN_MODULAR_CEGAR_UNSOUNDNESS_DEMONSTRATED` | 🟢 **FROZEN** | Plain $D \bmod 2^m$ collapses quotient $n = (D - C_j)/M_j$. |
| `EXACT_CYLINDER_TRANSFORMERS_J0_TO_J32_VERIFIED` | 🟢 **FROZEN** | $\text{Pre}_j$ and $\text{Post}_j$ exact round-trip verified for $j=0 \dots 32$. |
| `BACKWARD_APPROXIMANTS_J2_DEPTH3_VERIFIED` | 🟢 **FROZEN** | Predecessor sets $E_0 \dots E_3$ generated for $J_{\text{tail}}=2$. |
| `CYLINDER_COUNTS_J2_DEPTH3_1_3_9_27_VERIFIED` | 🟢 **FROZEN** | Exact reduced cylinder counts: $|E_0|=1, |E_1|=3, |E_2|=9, |E_3|=27$. No subsumption or sibling compression occurred through depth 3. |
| `BOUNDED_PREFIX_ZERO_LIFT_EXCLUDED_U2_JPRE2_JTAIL2` | 🟢 **FROZEN** | Proven for $U=2, J_{\text{pre}}=2, J_{\text{tail}}=2$ that no prefix admits zero lifts. |
| `FIRST_EMPTY_APPROXIMANT_U2_JPRE2_JTAIL2_N1` | 🟢 **FROZEN** | Intersection $I_{2,2} \cap E_1 = \emptyset$ is empty at depth $n=1$ ($T_{2,2,2} = 0$). |
| `DIRECT_FORWARD_AND_BACKWARD_INTERSECTION_AGREE` | 🟢 **FROZEN** | Direct forward replay and backward cylinder engines produce 100% identical reachability exclusions. |

---

## 2. Predecessor Set Approximants $E_n = \Phi_J^n(E_0)$

### 2.1 Approximant Level Statistics ($J_{\text{tail}}=2$)

| Level | Reduced Cylinder Count | Precision Distribution | Sample Cylinders $[r]_p$ | Descending Chain Invariant |
| :---: | :---: | :---: | :---: | :---: |
| **$E_0$** | **1** | $p = 0$ | `[0]_0` | Base Set ($\mathbb{Z}_2$) |
| **$E_1$** | **3** | $p \in \{9, 13, 17\}$ | `[342]_9`, `[7392]_13`, `[86208]_17` | $E_1 \subseteq E_0$ 🟢 |
| **$E_2$** | **9** | $p \in \{18, 22, 26, 30, 34\}$ | `[9130176]_26`, `[9539899584]_34`, `[350179520]_30` | $E_2 \subseteq E_1$ 🟢 |
| **$E_3$** | **27** | $p \in \{27, 31, 35, 39, 43, 47, 51\}$ | `[28530397376]_35`, `[5940150227136]_43`, `[83962319040]_39` | $E_3 \subseteq E_2$ 🟢 |

---

## 3. Descending-Chain Reachability Exclusion Verdict

### 3.1 Initial Endpoints Setup
- **Non-empty Prefix Word Count**: `prefix_word_count = 12` ($3 + 3^2 = 12$ words for $1 \le |u| \le 2, j_i \le 2$).
- **Distinct Endpoint Count**: `distinct_endpoint_count = 12` (all 12 prefix endpoints $D_u$ are distinct).
- **Initial Canonical Endpoints Set**: $I_{U=2, J_{\text{pre}}=2} = \{ D_u : 1 \le |u| \le 2, j_i \le 2 \}$.
- **First Empty Approximant**: **$n = 1$**. Evaluated intersection $I_{2,2} \cap E_1 = \mathbf{\emptyset}$.
- **Direct Forward Maximum Tail Depth**: $T_{2,2,2} = \max_{D \in I_{2,2}} T(D) = \mathbf{0}$.

### 3.2 Descending-Chain Exclusion Deduction
Since $I_{2,2}$ is finite and $E_{n+1} \subseteq E_n$:
$$(I_{2,2} \cap E_1 = \emptyset) \implies (I_{2,2} \cap E_n = \emptyset \quad \forall n \ge 1)$$

**Theorem Verdict**: No canonical prefix of length $1 \le |u| \le 2$ over gap alphabet $0 \le j \le 2$ can ever produce even a single zero-lift continuation with $j \le 2$.

---

## 4. Verification & Replay Matrix (100% Passed)

- **Rust Verification Suite**: 4/4 tests passed cleanly (`phase73s2b_tree_audit.rs`).
- **Python Independent Oracle**: 100% match on all tree levels $E_0 \dots E_3$.
- **Adversarial Mutation Suite**: 8 explicit mutations were **100% caught and rejected**.
