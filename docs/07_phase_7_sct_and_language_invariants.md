# Phase 7: Size-Change Termination & Language Invariants

## Executive Summary & Phase 7.3 Realignment

Phase 7 generalizes program-termination analysis across multi-word Collatz macrostep trajectories using exact integer reference register semantics, proved quotient abstractions, 2-adic IFS fractal geometry, residue-lifting transducers, and reordered termination proof systems.

> **Phase 7.2 Status Update (July 2026):**
> *Phase 7.2 successfully discovered a sound noncommuting guarded-branching core at state $Q_1 = 7 + 32\mathbb{N}_0$ with based closed walks $u = [1,1,2]$ and $v = w_1 w_2 = [1,1,2,1,2,2]$. Both $uv$ and $vu$ possess exact positive-integer path cylinders ($1767 \pmod{16384}$ and $1959 \pmod{16384}$ respectively under left-to-right composition), proving genuine noncommuting branching ($uv \neq vu$). Phase 7.3 builds reference semantics and symbolic invariants for this verified $(u, v)$ branching target.*

---

## 1. Verified Branching Target & Interaction Algebra

### 1.1 Non-Equivalence to Single Periodic Words & Genuine Branching
Target $Q_1$ supports two based closed walks:
1. **$u = [1,1,2]$**: $a_u = 27, b_u = 16, c_u = 19, d_u = -11$. Linear form $L_u(n) = 11n + 19$.
2. **$v = [1,1,2,1,2,2]$**: $a_v = 729, b_v = 512, c_v = 881, d_v = -217$. Linear form $L_v(n) = 217n + 881$.

Their affine commutator constant is:
$$\Delta_{u,v} = d_u c_v - d_v c_u = (-11)(881) - (-217)(19) = -5568 = -2^6 \cdot 87, \qquad \kappa_{u,v} = v_2(\Delta_{u,v}) = 6$$

This constant measures the exact order defect between compositions:
$$b_u b_v \big(F_{uv}(n) - F_{vu}(n)\big) = 8192 \left(-\frac{5568}{8192}\right) = -5568$$

---

## 2. Exact Integer Reference Register Machine ($k$-Machine)

Every state in $Q_1$ is parameterized by $n = 7 + 32k$ ($k \ge 0$). The exact partial register machine governing based returns to $Q_1$ is:

$$\text{U-return: guard } k \equiv 7 \pmod{16}, \quad k' = \frac{27k + 3}{16}$$
$$\text{V-return: guard } k \equiv 61 \pmod{512}, \quad k' = \frac{729k + 75}{512}$$

- Valuation word $v$ execution requires $x = v_2(L_u(n)) = 6$ and $U \equiv 1 \pmod{16}$.
- Based $v$-return requires $x = 6$ and $U \equiv 81 \pmod{256} \iff k \equiv 61 \pmod{512}$.

---

## 3. Structural Dynamics & 2-Adic Fractal Geometry

1. **Finite Return Language**: $L_{\text{finite}} = \{u,v\}^*$. Every finite macro word $s \in \{u,v\}^*$ has a positive guarded return cylinder.
2. **No Positive Ultimately Periodic Paths**: Every composite macrostep is real-expanding, forcing fixed points to be strictly negative 2-adic rationals. Thus $L_{\text{positive},\omega} \cap \{\text{ultimately periodic}\} = \emptyset$, and $L_{\text{positive},\omega}$ is non-$\omega$-regular.
3. **2-Adic IFS Geometry**: The set of 2-adic states supporting infinite switching has 2-adic Haar measure $\mu(X) = 0$ and 2-adic Hausdorff dimension $s \approx 0.1625357554$ ($2^{-4s} + 2^{-9s} = 1$).
4. **Residue-Lifting Transducer**: Transforms positive integer realization into evaluating eventual zero binary output on a residue-lifting transducer.

---

## 4. Reordered Proof Systems Hierarchy (Phase 7.3D)

Termination proof systems are evaluated in order of analytical strength for non-monotonic guarded switching:
1. **Direct Source-Height / Cylinder-Floor Bounds** (`source_height_bound_v1`)
2. **Monotonicity Constraints**
3. **Lexicographic & Multiphase Rankings** (using multi-coordinate tuple $(v_2(L_u(n)), v_3(11k+3), \text{phase})$)
4. **Disjunctive Transition Invariants** (`disjunctive_transition_invariant_v1`)
5. **Path-Complete Graph Lyapunov Rankings** (`path_complete_ranking_v1`)
6. **Classical SCT Projection**

---

## 5. Phase 7.3 Sub-Phase Roadmap

```text
Phase 7.3-0: Semantic Normalization & Composition Conventions
Phase 7.3A:  Algebra & Exact Reference Semantics (k-machine, equivalence, periodic exclusion)
Phase 7.3B:  Proved Quotient Abstraction & 3-Adic Signal (simulation theorem, U ≡ 81 mod 256)
Phase 7.3C:  Infinite Symbolic Dynamics & Transducer (finite full shift, transducer eventual zero, Haar 0, s ≈ 0.1625, M_r)
Phase 7.3D:  Reordered Proof Hierarchy & Certificate Schemas
Phase 7.3E:  Target Expansion Discipline (Target A -> Target B -> Target C)
```
