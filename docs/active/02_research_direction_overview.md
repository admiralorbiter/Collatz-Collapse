# Phase 7.3 Research Direction Overview
## Affine Interaction, Ultrametric Fuel, and Symbolic Return Dynamics

---

## 1. Why this direction exists

Phase 7.2 completed with a major discovery: destination-aware refinement successfully produced a **sound noncommuting guarded-branching target** at state $Q_1 = 7 + 32\mathbb{N}_0$ with based closed walks:
- $u = [1,1,2]$
- $v = w_1 w_2 = [1,1,2,1,2,2]$

Both $uv$ and $vu$ possess exact positive-integer path cylinders ($1767 \pmod{16384}$ and $1959 \pmod{16384}$ respectively under left-to-right composition), confirming genuine noncommuting branching ($uv \neq vu$).

However, raw residue states do not provide a compact switching invariant. Each macrostep consumes 2-adic bits, while static residue partitions forget information exposed by subsequent divisions.

Phase 7.3 replaces heuristic residue refinement with an **exact integer reference register machine** ($k$-machine) for $Q_1$, constructs proved quotient abstractions, and analyzes the infinite symbolic dynamics of $u/v$ switching.

---

## 2. Primary Research Questions

1. **Exact Reference Semantics**: What is the minimal exact integer register machine governing based returns to $Q_1$?
2. **Quotient Simulation**: Is the ultrametric single-coordinate abstraction a sound simulation quotient of the exact $k$-machine?
3. **Finite vs. Infinite Language**: Is the finite return language $L_{\text{finite}} = \{u,v\}^*$, and what structural constraints govern the infinite positive realization language $L_{\text{positive},\omega}$?
4. **Ultimately Periodic Exclusion**: Can any positive integer realize an ultimately periodic infinite $u/v$ switching sequence? (`CLM-P7X-PERIODIC-DIVERGENCE-001`: **No**, all fixed points $k^*_w < 0$ and pullbacks $g_p(k^*_w) < 0$ are negative rational 2-adics).
5. **2-Adic Cantor Coding & Conjugacy**: Is the piecewise quotient map $(G_\infty, T)$ conjugate to the full binary shift $(\{u,v\}^\mathbb{N}, \sigma)$ with $h_{\text{top}} = \ln 2$ and Hausdorff dimension $d \approx 0.1625357554$? (**Yes**).
6. **Lift-Digit Realizability & Deterministic $D(y)$ Engine**: Can positive non-negative integer realizability be reduced to an eventual-zero output property on a deterministic endpoint map $D(y)$? (`CLM-P7X-ZERO-LIFT-DETERMINISM-001`: **Yes**).
7. **Zero-Cycle-Free CEGAR Subsystem**: Does a sound finite overapproximation graph of endpoint $y_s$ without zero-lift cycles prove source-height divergence $M_r \to \infty$? (`CLM-P7X-ZERO-CYCLE-FREE-CEGAR-001`: **Yes**).

---

## 3. Mathematical Objects & Benchmark Parameters

### Benchmark Switching Pair $(u, v)$ at $Q_1$
- $u = [1,1,2]$: $a_u = 27, b_u = 16, c_u = 19, d_u = -11$. Linear form $L_u(n) = 11n + 19$.
- $v = [1,1,2,1,2,2]$: $a_v = 729, b_v = 512, c_v = 881, d_v = -217$. Linear form $L_v(n) = 217n + 881$.

### Commutator Data
$$\Delta_{u,v} = d_u c_v - d_v c_u = (-11)(881) - (-217)(19) = -5568 = -2^6 \cdot 87, \qquad \kappa_{u,v} = v_2(\Delta_{u,v}) = 6$$

### Affine Commutator Identity
$$b_u b_v \big(F_{uv}(n) - F_{vu}(n)\big) = 8192 \left(-\frac{5568}{8192}\right) = -5568$$

---

## 4. Exact Integer Reference Register Machine ($k$-Machine)

Every state in $Q_1$ is written as $n = 7 + 32k$ ($k \ge 0$). Direct substitution into $F_u(n)$ and $F_v(n)$ yields the exact, deterministic partial register machine:

$$\text{U-return: guard } k \equiv 7 \pmod{16}, \quad k' = \frac{27k + 3}{16}$$
$$\text{V-return: guard } k \equiv 61 \pmod{512}, \quad k' = \frac{729k + 75}{512}$$

### Key Distinction: Valuation Word vs. Based Return
- Executing valuation word $v$ requires $x = v_2(L_u(n)) = 6$ and $U \equiv 1 \pmod{16}$.
- Returning to $Q_1$ requires the stronger guard: $x = 6$ and $U \equiv 81 \pmod{256} \iff k \equiv 61 \pmod{512}$.
- All abstractions are verified against this exact $k$-machine reference semantics.

---

## 5. Infinite Dynamics & Structural Theorems

1. **Finite Full-Return Language Theorem**:
   $$\forall s \in \{u,v\}^*, \quad s \text{ has a nonempty positive guarded return cylinder in } Q_1. \implies L_{\text{finite}} = \{u,v\}^*$$

2. **Exclusion of Ultimately Periodic Switching**:
   No positive integer realizes an ultimately periodic infinite $u/v$ path ($p w^\omega$). Every composite macrostep $w$ has fixed point $k^*_w = \frac{\eta_w}{2^{A_w} - a_w} < 0$. All pullbacks $g_p(k^*_w) < 0$ preserve strict negativity (`CLM-P7X-PERIODIC-DIVERGENCE-001`).

3. **Lift-Digit Realizability Theorem**:
   An infinite stream $\omega$ codes a non-negative integer $\alpha_\omega \in \mathbb{Z}_{\ge 0}$ if and only if its lift sequence $(\lambda_j)$ is eventually zero (`CLM-P7X-LIFT-DIGIT-REALIZABILITY-001`).

4. **Deterministic Partial Endpoint Map $D(y)$**:
   $$\tau_p(s) = 0 \iff y_s \equiv g_p \pmod{2^{A_p}}$$
   where $y_s = T_s(r_s)$. Since $G_u = 7 \pmod{16}$ and $G_v = 61 \pmod{512}$ are disjoint, the zero-lift subsystem is partial and deterministic (`CLM-P7X-ZERO-LIFT-DETERMINISM-001`):
   $$D(y) = \begin{cases} \frac{27y+3}{16}, & y \equiv 7 \pmod{16} \\ \frac{729y+75}{512}, & y \equiv 61 \pmod{512} \\ \text{undefined}, & \text{otherwise} \end{cases}$$

---

## 6. Revised Sub-Phase Roadmap

```text
Phase 7.3-0: Semantic Normalization & Composition Conventions (PASSED)
Phase 7.3A:  Algebra & Exact Reference Semantics (PASSED)
Phase 7.3B:  Proved Quotient Abstraction & Ultrametric Machine (PASSED)
Phase 7.3C:  Symbolic Return Language, Cantor Homeomorphism & Lift Digits (PASSED)
Phase 7.3D:  Zero-Output Lasso Elimination for D(y) & Source-Height Divergence M_r -> ∞
Phase 7.3E:  Target Expansion Discipline (Target A -> Target B -> Target C)
```
