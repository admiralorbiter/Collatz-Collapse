# Phase 7: Size-Change Termination & Language Invariants

## Executive Summary & Audit Status

Phase 7 generalizes program-termination analysis across multi-word Collatz macrostep trajectories using Size-Change Termination (SCT), $\omega$-automata language invariants, and 2-adic linear-form fixed-point dynamics.

## Executive Summary & Phase 7.3 Realignment

Phase 7 generalizes program-termination analysis across multi-word Collatz macrostep trajectories using symbolic return dynamics, 2-adic fixed-point dynamics, path-complete graph Lyapunov rankings, disjunctive transition invariants, and Size-Change Termination (SCT).

> **Phase 7.2 Status Update (July 2026):**
> *Phase 7.2 successfully discovered a sound noncommuting guarded-branching core at state $Q_1$ with based closed walks $u = [1,1,2]$ and $v = w_1 w_2 = [1,1,2,1,2,2]$. Both $uv$ and $vu$ possess exact positive-integer path cylinders, proving genuine noncommuting branching ($uv \neq vu$). Phase 7X is merged into Phase 7.3 as its mathematical and verification engine to synthesize a switching-sensitive invariant for $(u, v)$.*

---

## 1. Verified Branching Target & Interaction Algebra

### 1.1 Non-Equivalence to Single Periodic Words & Genuine Branching
Target $Q_1$ supports two based closed walks:
1. **$u = [1,1,2]$**: $a_u = 27, b_u = 16, c_u = 19, d_u = -11$. Normalized linear form $L_u(n) = 11n + 19$.
2. **$v = [1,1,2,1,2,2]$**: $a_v = 729, b_v = 512, c_v = 881, d_v = -217$. Normalized linear form $L_v(n) = 217n + 881$.

Their affine commutator constant is:
$$\Delta_{u,v} = d_u c_v - d_v c_u = (-11)(881) - (-217)(19) = -5568 = -2^6 \cdot 87, \qquad \kappa_{u,v} = v_2(\Delta_{u,v}) = 6$$

This constant measures the exact order defect between compositions:
$$b_u b_v \big(F_{uv}(n) - F_{vu}(n)\big) = 8192 \left(-\frac{5568}{8192}\right) = -5568$$

### 1.2 Symbolic Return Language at $Q_1$
By topological conjugacy of the 2-adic extension of Collatz to the binary shift (Bernstein-Lagarias, Laarhoven-de Weger), finite valuation words define symbolic cylinders. Words $u$ and $v$ are **candidate return words** to $Q_1$. Phase 7.3 studies the **induced return language** $L_{Q_1} \subseteq \{u,v\}^*$.

---

## 2. Four Competing Proof Architectures (Phase 7.3D)

Termination analysis for the $(u,v)$ switching language evaluates four competing proof architectures in order:
1. **State-Indexed / Path-Complete Graph Lyapunov Rankings (`path_complete_ranking_v1`)**: Attach state-dependent features $V_{Q_i}(n)$ and edge inequalities covering all accepted switching words.
2. **Disjunctive Transition Invariants (`disjunctive_transition_invariant_v1`)**: Prove that the transitive closure of the switching relation is covered by a finite union of well-founded relations $R_1 \cup \dots \cup R_m$.
3. **Lexicographic & Multiphase Rankings**: Lexicographic tuples $(L_u(n), x)$ or multiphase phase-transition rankings.
4. **Size-Change Termination (SCT)**: Classical SCT closure over universally verified feature relations.

---

## 3. Lean 4 Formalization & Theorem Scope

In `lean/Milestone70.lean` and `lean/Phase73.lean`:
1. **Generic Affine Kernel**: Formalize same-form eigenidentity, cross-form identity, affine commutator identity, and common-center criterion.
2. **Instance Verification**: Formally check quantified identities for $u=[1,1,2]$ and $v=[1,1,2,1,2,2]$ over their exact path cylinders.

---

## 4. Phase 7.3 Sub-Phase Roadmap

1. **Phase 7.3A**: Generic Affine Interaction & Symbolic Theorem Kernel (Rust/Python/Lean parallel gate).
2. **Phase 7.3B**: Minimal Single-Coordinate Ultrametric Register Machine ($L_u(n) = 11n+19$, Feature CEGAR).
3. **Phase 7.3C**: Symbolic Return-Language & Entropy Probe ($s \in \{u,v\}^{\le 12}$, spectral radius, topological entropy).
4. **Phase 7.3D**: Four Competing Proof Systems on Target A ($u/v$).
5. **Phase 7.3E**: Target Expansion (Minimal Core $\to$ Target B $\to$ Target C).

