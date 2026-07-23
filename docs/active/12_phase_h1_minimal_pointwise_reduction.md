# Phase H.1: Minimal Pointwise Reduction

## 1. Executive Summary & Purpose

Phase H.1 establishes the foundational, pointwise reduction connecting infinite 2-adic valuation paths to ordinary non-negative integers $\mathbb{N}^+$.

The primary result is the **Minimal Pointwise Equivalence Chain**:
$$\text{represented by } N \in \mathbb{Z}_{\ge 0} \iff (R_n)_{n=1}^\infty \text{ is bounded} \iff (R_n)_{n=1}^\infty \text{ eventually stabilizes} \iff (\lambda_n)_{n=1}^\infty \text{ is eventually zero.}$$

---

## 2. Status Badges & Registry

- `CANONICAL_RETURN_CONVENTION_V1_FROZEN`
- `CANONICAL_RETURN_DEFINITION_FINGERPRINT_PASSED`
- `PHASE_H0C_ARITHMETIC_CORE_INTERACTION_COMPLETE`
- `PHASE_H1_MINIMAL_POINTWISE_REDUCTION_COMPLETE`
- `H2A_SELECTOR_AXIOMS_FROZEN`

---

## 3. Precision Schedule & Realization Hypotheses

### 3.1 Precision Schedule $H_n$
The source precisions form a strictly increasing, unbounded schedule of natural numbers:
$$H_0 < H_1 < H_2 < \dots < H_n < \dots, \qquad \lim_{n \to \infty} H_n = \infty$$
This cofinality guarantees that the inverse limit $\varprojlim \mathbb{Z} / 2^{H_n} \mathbb{Z} \cong \mathbb{Z}_2$ is well-defined and determines a unique 2-adic element.

### 3.2 Definition of Path Realization
An ordinary integer $N \in \mathbb{Z}_{\ge 0}$ **realizes** an infinite canonical valuation path $w = (h_1, h_2, \ldots)$ if and only if for every finite prefix $w_{\le n}$:
1. $N$ lies in the exact source cylinder of $w_{\le n}$.
2. All internal canonical guards of $w_{\le n}$ are semantically valid.
3. $N \equiv r_n(w) \pmod{2^{H_n}}$ using the closed-form exact cylinder congruence.

---

## 4. Six Separated Theorem Claims

### 4.1 Projective Compatibility Theorem (`CLM-H1-PROJ-COMPAT-001`)

**Category:** Verified Algebraic Identity

For any semantically valid infinite path $w$, the sequence of source residues $r_n(w) \pmod{2^{H_n}}$ satisfies:
$$r_{n+1} \equiv r_n \pmod{2^{H_n}} \quad \text{for all } n \ge 0$$

### 4.2 Least Representative Decomposition & Monotonicity (`CLM-H1-DECOMP-MONO-001`)

**Category:** Verified Algebraic Identity

Let $R_n(w) = r_n \pmod{2^{H_n}} \in [0, 2^{H_n}-1]$ be the least non-negative representative.
There exists a unique non-negative **lift block** $\lambda_{n+1} \in \mathbb{Z}_{\ge 0}$ such that:
$$R_{n+1} = R_n + \lambda_{n+1} \cdot 2^{H_n}, \qquad 0 \le \lambda_{n+1} < 2^{H_{n+1}-H_n}$$
Consequently, the representative sequence is strictly monotone non-decreasing:
$$R_{n+1} \ge R_n \quad \text{for all } n \ge 0$$

### 4.3 Boundedness and Stabilization Equivalence (`CLM-H1-BOUNDED-STABLE-001`)

**Category:** Verified Finite Theorem

A monotone non-decreasing sequence of natural numbers $(R_n)_{n=0}^\infty$ is bounded ($\sup_n R_n < \infty$) if and only if it is eventually constant:
$$\sup_n R_n < \infty \iff \exists K \in \mathbb{N}, \forall n \ge K, \quad R_n = R_K = N$$

**Corollaries for Phase H:**
- Unbounded representative growth $R_n(w) \to \infty$ provably implies $w$ is **not realizable** by any ordinary non-negative integer.

### 4.4 Fixed-Integer Natural Characterization (`CLM-H1-INTEGER-CHAR-001`)

**Category:** Verified Finite Theorem

The inverse limit element $x_\infty = \lim_{n \to \infty} r_n \in \mathbb{Z}_2$ is a genuine non-negative ordinary integer $N \in \mathbb{Z}_{\ge 0}$ if and only if the sequence of least representatives $(R_n(w))_{n=0}^\infty$ eventually stabilizes:
$$\exists K \in \mathbb{N}, \forall n \ge K, \quad R_n(w) = R_K(w) = N$$

### 4.5 Zero Lift-Block Tail Characterization (`CLM-H1-CARRY-CHAR-001`)

**Category:** Verified Algebraic Identity

Sequence stabilization $R_n(w) = N$ for all $n \ge K$ is strictly equivalent to the eventual vanishing of all lift blocks:
$$\forall j \ge K + 1, \quad \lambda_j = 0$$

### 4.6 Fixed-Source Subsystem Reduction Theorem (`CLM-H1-FIXED-SOURCE-REDUCE-001`)

**Category:** Domain-Scoped Certificate

If a positive ordinary integer $N \in \mathbb{N}^+$ realizes an infinite canonical path $w$, then:
$$\sup_n R_n(w) = N < \infty$$
and the lift blocks have zero tail $\lambda_j = 0$ for all $j \ge K(N)$ where $2^{H_{K(N)}} > N$.

> **Subsystem Coverage Note (`CLM-H1-SUBSYSTEM-COVERAGE-001`):** Subsystem coverage—whether every hypothetical Collatz counterexample produces such a path—is maintained as a separate, independently audited domain-scoped claim.

---

## 5. Analytical Test Classes & Verified Results

### 5.1 Ordinary Natural Integer ($N = 13$)
- Schedule $H_n = [2, 4, 8, 12, 16]$.
- $R_0 = 1, R_1 = 13, R_2 = 13, R_3 = 13, R_4 = 13$.
- Lift blocks: $\lambda_1 = 3$, $\lambda_2 = 0, \lambda_3 = 0, \lambda_4 = 0$ (Zero Lift Tail).
- Stabilizes to $N = 13$ at stage $K = 1$.

### 5.2 Negative Ordinary Integer ($N = -1$)
- Schedule $H_n = [4, 8, 12, 16]$.
- $R_n = 2^{H_n} - 1 \to [15, 255, 4095, 65535]$.
- Unbounded growth $R_n \to \infty$, non-zero lift blocks $\lambda_n > 0$.
- Does not stabilize to any natural integer.

### 5.3 Non-Natural 2-Adic Rationals (Pole $-1/3$ and Periodic Core $-26/217$)
- $3R_n \equiv -1 \pmod{2^{H_n}} \implies R_n \to \infty$ unboundedly.
- Core $-26/217 \implies 217 R_n \equiv -26 \pmod{2^{H_n}} \implies R_n \to \infty$ unboundedly.
- Demonstrates that non-natural 2-adic fixed points and ghost periodic orbits fail ordinary integer realizability.
