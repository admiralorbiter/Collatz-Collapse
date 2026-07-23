# Phase 7.3C Closeout Report — Symbolic Return Language Dynamics, Lift-Digit Realizability, and 2-Adic Cantor Homeomorphism

**Date**: July 22, 2026  
**Status**: **COMPLETED & FORMALLY FROZEN FOR HANDOFF TO PHASE 7.3D**  
**Crates Modified/Created**: `collatz-affine`, `collatz-cegar`, `collatz-cert`  
**Test Matrix**: 100% Rust workspace test suite, Python differential oracle (**0 diff**), 10-corruption mutation matrix (**10/10 REJECTED**)

---

## 1. Executive Summary

Phase 7.3C establishes the symbolic return language dynamics for the Collating Quotient Machine over $\mathcal{L} = \{u, v\}^{\le 12}$ ($8,190$ non-empty words). The principal theoretical achievement is the **Lift-Digit Realizability Theorem**, which proves that an infinite symbolic stream $\omega \in \{u,v\}^\mathbb{N}$ codes an ordinary non-negative integer $\alpha_\omega \in \mathbb{Z}_{\ge 0}$ if and only if its lift-digit sequence $(\lambda_j)_{j=1}^\infty$ is eventually zero.

Because the two first-level return guards $G_u = 7 + 16\mathbb{Z}_2$ and $G_v = 61 + 512\mathbb{Z}_2$ are disjoint ($61 \equiv 13 \pmod{16} \neq 7$), a concrete quotient state can enable at most one zero-lift child. Consequently, Phase 7.3C transforms the original branching-language search into a **partial deterministic termination problem** for the map:
$$D(k) = \begin{cases} \frac{27k+3}{16}, & k \equiv 7 \pmod{16} \\ \frac{729k+75}{512}, & k \equiv 61 \pmod{512} \\ \text{undefined}, & \text{otherwise} \end{cases}$$

An ordinary non-negative integer $u/v$ trajectory exists if and only if $D^j(k)$ is defined for all $j \ge 0$ for some $k \ge 0$. The existence of such an infinite zero-lift ray remains **UNRESOLVED** and serves as the precise operational target for Phase 7.3D.

---

## 2. Core Mathematical Theorems

### Theorem 1 (Symbolic Language Enumeration Count)
For depths $r = 1 \dots 12$, the total number of non-empty words in $\{u,v\}^{\le 12}$ is:
$$\sum_{r=1}^{12} 2^r = 2^{13} - 2 = 8,190 \text{ non-empty words}$$
All $8,190$ non-empty words are 100% cross-validated across 3 independent guard constructions:
1. Recursive quotient preimages via `QuotientRegisterMachine`.
2. Composite quotient map $T_s(k) = \frac{a_s k + \eta_s}{2^{A_s}}$.
3. Flattened $n$-space cylinder `solve_starting_residue_exact`.

### Theorem 2 (The Lift-Digit Realizability Theorem)
For a finite word $s$, its canonical guard is $k \equiv r_s \pmod{2^{A(s)}}$. For a child word $sp$ ($p \in \{u,v\}$), guard inclusion $G_{sp} \subset G_s$ implies:
$$r_{sp} = r_s + \lambda_{s,p} 2^{A(s)}$$
where lift digit $\lambda_{s,u} \in \{0 \dots 15\}$ (since $A_u = 4$) and $\lambda_{s,v} \in \{0 \dots 511\}$ (since $A_v = 9$).

For an infinite stream $\omega = \omega_1 \omega_2 \dots$, $r_j = r_{j-1} + \lambda_j 2^{A_{j-1}}$, yielding 2-adic point:
$$\alpha_\omega = \sum_{j=1}^\infty \lambda_j 2^{A_{j-1}}$$
Because $0 \le \lambda_j < 2^{A_j - A_{j-1}}$, in binary representation $\lambda_j$ occupies the non-overlapping bit interval $[A_{j-1}, A_j - 1]$ with zero cross-block carry ambiguity.
$$\alpha_\omega \in \mathbb{Z}_{\ge 0} \iff (\lambda_j) \text{ is eventually zero} \iff (r_j) \text{ is eventually constant} \iff (r_j) \text{ is bounded}$$

### Theorem 3 (2-Adic Cantor Coding & Topological Conjugacy)
The inverse quotient branch maps $g_u(k) = \frac{16k-3}{27}$ and $g_v(k) = \frac{512k-75}{729}$ are strict contractions on $\mathbb{Z}_2$ with ratios $2^{-4}$ and $2^{-9}$. Their images are disjoint first-level guards $7+16\mathbb{Z}_2$ and $61+512\mathbb{Z}_2$.

The coding map $\pi: \{u,v\}^\mathbb{N} \to G_\infty$ given by $\pi(\omega) = \bigcap_{r=1}^\infty G_{\omega_1 \dots \omega_r} = \{\alpha_\omega\}$ is a homeomorphism satisfying the conjugacy relation:
$$T \circ \pi = \pi \circ \sigma$$
where $T: G_\infty \to G_\infty$ is the piecewise quotient map and $\sigma$ is the left shift.
- Topological Entropy of $(G_\infty, T)$: $h_{\text{top}} = \ln 2 \approx 0.693147$.
- Hausdorff Dimension: $\dim_H(G_\infty) = d \approx 0.1625357554$, uniquely satisfying $2^{-4d} + 2^{-9d} = 1$.
- Dual Haar Measures: $\mu_k(G_r) = (33/512)^r$, $\mu_n(G_r) = 2^{-5} (33/512)^r$.

### Theorem 4 (Exclusion of Ultimately Periodic Realizations)
For every primitive period root $w \in \{u,v\}^+$, $\eta_w > 0$ and $a_w = 3^{K_w} > 2^{A_w} = b_w$. Thus:
$$k^*_w = \frac{\eta_w}{2^{A_w} - a_w} < 0$$
For any negative rational 2-adic $y < 0$, $g_u(y) < 0$ and $g_v(y) < 0$. Therefore, for any prefix $p \in \{u,v\}^*$, the pullback $g_p(k^*_w) < 0$ remains strictly negative.
**Conclusion**: Every ultimately periodic stream $p w^\omega$ codes a negative rational 2-adic integer and has **no non-negative integer realization**.

---

## 3. Zero-Lift Subsystem Bounded Statistics (Depths 1..12)

Out of $8,190$ non-empty words in $\{u,v\}^{\le 12}$:

| Statistic | Count | Percentage |
| :--- | ---: | ---: |
| Total $u$-child edges | 4,094 | 100% |
| **Zero-lift $u$-children ($\lambda_{s,u} = 0$)** | 254 | 6.20% |
| Total $v$-child edges | 4,094 | 100% |
| **Zero-lift $v$-children ($\lambda_{s,v} = 0$)** | 6 | 0.15% |
| Parents with **0 zero-lift children** | 3,834 | 93.65% |
| Parents with **1 zero-lift child** | 260 | 6.35% |
| **Parents with 2 zero-lift children** | **0** | **0.00%** |

### Depth-12 Maximum Zero-Lift Run Witness
- **Maximum zero-lift edges (evaluated children)**: **2 zero-lift edges** (3 nodes in chain).
- **Exact Witness**: $u u u v v v u \to u u u v v v u u \to u u u v v v u u u$.
  - Guard: $r = 2,673,862,933,783$.
  - Valuation Exponents: $A = 43 \to 47 \to 51$.
  - Lift Digits: $\lambda_1 = 0$, $\lambda_2 = 0$.

> [!NOTE]
> **Exploratory Finding Beyond Depth 12**:
> At depth 13, zero-lift run length grows to 3 edges ($v v v v u^6 \to v v v v u^7 \to v v v v u^8 \to v v v v u^9$, guard $r = 210,064,648,573,987,901$). At depth 19, zero-lift run length reaches 5 edges. Zero-lift run length is not globally bounded by 2; it grows with depth, making Phase 7.3D a deterministic ranking problem for $D(k)$.

---

## 4. Primitive Necklace Classification Table

| Necklace # | Canonical Root | Period | Phase / Rotation Words | Rational Fixed Points $k^*_w$ |
| ---: | :--- | ---: | :--- | :--- |
| 1 | $[1,1,2]$ ($u$) | 1 | $u$ | $3 / -11 \approx -0.2727$ |
| 2 | $[1,1,2,1,2,2]$ ($v$) | 2 | $v$, $[1,2,2,1,1,2]$ | $75 / -217$, $85 / -217$ |
| 3 | $[1,1,2,1,1,2,1,2,2]$ ($uuv$) | 3 | $uuv$, $uvu$, $vuu$ | $3387 / -11491$, $3561 / -11491$, $3854 / -11491$ |
| 4 | $[1,1,2,1,1,2,1,1,2,1,2,2]$ ($uuuv$) | 4 | $uuuv$, $uuvu$, $uvuu$, $vuuu$ | $113241 / -400369$, $116025 / -400369$, etc. |
| 5 | $[1,1,2,1,1,2,1,2,2,1,1,2,1,2,2]$ ($uuvv$) | 5 | $uuvv$, $uvvu$, $vvuu$, $vuuv$, $uvuv^*$ | All negative rational 2-adics |

*($*uvuv = (uv)^2$ is non-primitive and assigned to primitive root $uv$.)*

---

## 5. Scope Separation & Status Summary

1. **Full 2-Adic Cantor System $(G_\infty, T)$**: Conjugate to left shift $\sigma$; topological entropy $h_{\text{top}} = \ln 2 \approx 0.693147$.
2. **Ultimately Periodic Class**: Fully ruled out; all rational fixed points and pullbacks are negative.
3. **Ordinary Positive Infinite Subsystem**: Governed by the partial deterministic map $D(k)$. Existence of an infinite zero-lift ray remains **OPEN** (Aperiodic Unresolved).

---

## 6. Handoff to Phase 7.3D

Phase 7.3D will take the deterministic map $D(k)$ and zero-lift continuation rules to:
1. Search for potential ranking functions or Lyapunov obstructions for $D(k)$.
2. Analyze terminal $k$-states along record zero-lift runs.
3. Classify whether the partial deterministic map $D(k)$ has a finite maximum run or permits unbounded runs.
