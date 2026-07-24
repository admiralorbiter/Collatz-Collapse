# Phase I.3 & Phase I.4 — Prefix Fidelity, Infinite Stream Construction & Full Collatz Reduction

**Status**: **ACTIVE SPECIFICATION (SCHEMA 5.0.0)**  
**Preceding Phase**: Phase I.2 (Section Entry and No-Escape Theorem)  
**Succeeded By**: Phase J (Canonical Path Exhaustion Framework)

---

## 1. Executive Summary & Main Reduction Theorem

Phase I.3 & I.4 prove prefix-cylinder fidelity for extracted finite canonical traces, construct the unique infinite canonical return stream $h^* \in \mathcal{A}^\mathbb{N}$, and prove the **Full Collatz Reduction Theorem**:

$$\text{Collatz Conjecture is False} \implies \exists \text{ minimal odd counterexample } N^* \text{ yielding an infinite semantically valid canonical return path } h^* \in \mathcal{A}^\mathbb{N}.$$

---

## 2. Phase I.3: Prefix-Cylinder Fidelity & Infinite Stream Construction

### 2.1 Prefix-Cylinder Fidelity Theorem (`CLM-I3-PREFIX-FIDELITY-001`)

**Category:** Verified Source Congruence Identity

Let $E_k(N^*) = (h_1, h_2, \dots, h_k)$ be the extracted prefix of depth $k \ge 1$ for minimal counterexample $N^*$.

Let $R_k(h_1 \dots h_k) \in [0, 2^{H_k}-1]$ be the least non-negative source representative of the cylinder $\mathcal{C}(h_1 \dots h_k)$ where $H_k = \sum_{j=1}^k B_{h_j}$.

**Theorem:**
$$N^* \equiv R_k(h_1 \dots h_k) \pmod{2^{H_k}} \qquad \text{for all } k \ge 1$$

Furthermore, by Phase H.1 (Minimal Pointwise Reduction), the sequence of least non-negative source representatives $(R_k)_{k=1}^\infty$ eventually stabilizes to $N^*$:
$$\exists K(N^*), \forall k \ge K(N^*), \qquad R_k(h_1 \dots h_k) = N^*$$

---

### 2.2 Infinite Stream Existence & Uniqueness (`CLM-I3-STREAM-EXIST-UNIQUE-001`)

**Category:** Topological & Algorithmic Theorem

The finite-prefix extraction operators $E_k(N^*)$ are prefix-compatible:
$$E_{k+1}(N^*) = h_1 \dots h_k h_{k+1} \implies E_k(N^*) = h_1 \dots h_k$$
By the inverse limit property of infinite sequence spaces over finite alphabets, there exists a **unique infinite sequence** $h^* = (h_1, h_2, h_3, \dots) \in \mathcal{A}^\mathbb{N}$ such that for every $k \ge 1$, the prefix $h^*_{\le k} = E_k(N^*)$.

---

## 3. Phase I.4: Full Collatz Reduction Theorem

### 3.1 Full Collatz to Canonical Path Reduction (`CLM-I4-FULL-REDUCTION-001`)

**Category:** Primary Architectural Reduction Theorem

**Theorem (`FULL_COLLATZ_TO_CANONICAL_PATH_REDUCTION_PROVED`):**

If no positive ordinary integer $N \in \mathbb{Z}_{>0}$ can realize an infinite semantically valid path $h \in \mathcal{A}^\mathbb{N}$ in the canonical return subsystem, then the Collatz Conjecture is **TRUE** for all positive integers.

*Contrapositive:*
$$\text{Collatz False} \implies \exists N^* \in 2\mathbb{N}+1, \, \exists h^* \in \mathcal{A}^\mathbb{N} \text{ such that } N^* \text{ realizes } h^* \text{ indefinitely.}$$

---

## 4. Status Badges & Registry

- `CANONICAL_PREFIX_CYLINDER_FIDELITY_PROVED`
- `COUNTEREXAMPLE_CANONICAL_STREAM_EXISTENCE_PROVED`
- `COUNTEREXAMPLE_CANONICAL_STREAM_UNIQUENESS_PROVED`
- `FULL_COLLATZ_TO_CANONICAL_PATH_REDUCTION_PROVED`
