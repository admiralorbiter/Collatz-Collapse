# Phase 7: Size-Change Termination & Language Invariants

## Executive Summary & Audit Status

Phase 7 generalizes program-termination analysis across multi-word Collatz macrostep trajectories using Size-Change Termination (SCT), $\omega$-automata language invariants, and 2-adic linear-form fixed-point dynamics.

> **Audit Status Note (July 2026):**
> *Following an expert red-team audit, the Phase 7 theorem claim is currently undergoing repair. Hand-authored 2-state static residue guards and cross-feature weak relations were invalidated. Prototype infrastructure is complete, but target SCC formalization requires history-sensitive trace refinement (Phase 7D).*

---

## 1. Multi-Word Target SCC Selection & Distinction from Phase 6D

### 1.1 Non-Equivalence to Single Periodic Words
Phase 6D canonicalization handles primitive cyclic rotations of single words (e.g. $[1, 2] \sim [2, 1]$).
To avoid trivial single-word reduction, target SCC `SCC-M70-001` is selected across **two distinct primitive cyclic necklace classes**:

1. **Class 1 (`CLASS_1_1_2`):** Primitive word $[1, 1, 2]$, total valuation $A_1 = 4$, length $k_1 = 3$. Affine constant $c_1 = 19$, 2-adic fixed point $x_1^* = -19/11$, normalized fixed-point linear form $L_1(n) = 11n + 19$.
2. **Class 2 (`CLASS_1_2_2`):** Primitive word $[1, 2, 2]$, total valuation $A_2 = 5$, length $k_2 = 3$. Affine constant $c_2 = 23$, 2-adic fixed point $x_2^* = 23/5$, normalized fixed-point linear form $L_2(n) = 5n - 23$.

Because $A_1 = 4 \neq A_2 = 5$, `CLASS_1_1_2` and `CLASS_1_2_2` are provably non-isomorphic cyclic necklace classes.

### 1.2 Exact Arithmetic Cylinders & Image Residues
- **Edge $E_{12}: Q_1 \xrightarrow{[1, 1, 2]} Q_2$:** Source cylinder $n \equiv 7 \pmod{32}$, target image $F_1(7) = 13 \equiv 13 \pmod{16}$.
- **Edge $E_{21}: Q_2 \xrightarrow{[1, 2, 2]} Q_1$:** Source cylinder $n \equiv 43 \pmod{64}$, target image $F_2(43) = 37 \equiv 37 \pmod{32}$.

---

## 2. Size-Change Termination (SCT) Framework

### 2.1 Feature Vector & Well-Founded Domains
Termination analysis is conducted over fixed-point valuation features on positive natural numbers:
$$\mathbf{v}(n) = \big(v_2(L_1(n)), v_2(L_2(n))\big)$$

### 2.2 Own-Feature Valuation Drops
- **$E_{12}$ Own-Feature Drop:** $2^{A_1} L_1(F_1(n)) = 3^{k_1} L_1(n) \implies 16 L_1(F_1(n)) = 27 L_1(n)$. Valuation drops by 4 ($A_1 = 4$).
- **$E_{21}$ Own-Feature Drop:** $2^{A_2} L_2(F_2(n)) = 3^{k_2} L_2(n) \implies 32 L_2(F_2(n)) = 27 L_2(n)$. Valuation drops by 5 ($A_2 = 5$).

---

## 3. Lean 4 Formalization & Theorem Scope

In `lean/Milestone70.lean`, the theorem proves non-zero positivity of fixed-point linear forms:
```lean
theorem linear_forms_positive_on_guards (n : ℕ) (h_Q1 : guard_Q1 n) (h_Q2 : guard_Q2 n) :
    0 < L1 n ∧ 0 < L2 n
```
Full SCC termination formalization in Lean 4 will be established once history-sensitive trace refinement (Phase 7D) synthesizes a sound lexicographic ranking function $V(q, n)$.

---

## 4. Audit Findings & Repair Roadmap

1. **Exact Replay Required:** Hand-authored state guards are strictly replaced by mechanically generated starting cylinders.
2. **Trace Partitioning (Phase 7D):** Deterministic modular transitions between cyclic macrosteps require trace memory variables to preserve unconsumed valuation excess.
3. **Artifact-Driven Verification:** Python reference oracle (`scripts/reference_oracle.py`) dynamically audits CLI certificate files and computes real SHA-256 digests.
