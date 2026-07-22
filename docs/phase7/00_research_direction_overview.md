# Phase 7.3 Research Direction Overview
## Affine Interaction, Ultrametric Fuel, and Symbolic Return Dynamics

## 1. Why this direction exists

Phase 7.2 completed with a major discovery: destination-aware refinement successfully produced a **sound noncommuting guarded-branching target** at state $Q_1$ with based closed walks:
- $u = [1,1,2]$
- $v = w_1 w_2 = [1,1,2,1,2,2]$

Both $uv$ and $vu$ possess exact positive-integer path cylinders, confirming genuine noncommuting branching ($uv \neq vu$).

However, raw residue states do not yet provide a compact switching invariant. Each macrostep consumes 2-adic bits, while static residue partitions forget information later divisions expose.

Phase 7.3 merges the ultrametric affine interaction machinery to serve as the mathematical, symbolic, and verification engine for this verified $(u, v)$ branching target.

## 2. Primary research question

> Can the switching behavior of the verified $(u, v)$ branching core be represented by a compact, switching-sensitive ultrametric invariant (or affine register machine) that tracks 2-adic cancellation depth and symbolic return dynamics, rather than relying on ever-deeper raw residue partitions?

## 3. Secondary research questions

1. **Symbolic Return Dynamics**: Can the switching language $s \in \{u,v\}^*$ be modeled as an induced return language to $Q_1$?
2. **Finite Return Language**: Does every word $s \in \{u,v\}^*$ have a nonempty positive-integer cylinder returning to $Q_1$ at block boundaries?
3. **Graph Lyapunov Rankings**: Can a state-indexed or path-complete graph Lyapunov function (`path_complete_ranking_v1`) certify stability over accepted words?
4. **Disjunctive Transition Invariants**: Can a finite union of well-founded relations (`disjunctive_transition_invariant_v1`) cover the transitive closure of the switching relation?
5. **Feature CEGAR**: Is a single reference coordinate $L_u(n) = 11n + 19$ sufficient, or does feature CEGAR demand adding $L_v(n) = 217n + 881$?
6. **Language Entropy**: What are the growth rate, spectral radius, and topological entropy of the admissible $u/v$ switching language?

## 4. Mathematical Objects & Benchmark Parameters

### Benchmark Switching Pair $(u, v)$ at $Q_1$
- $u = [1,1,2]$: $a_u = 27, b_u = 16, c_u = 19, d_u = -11$. Linear form $L_u(n) = 11n + 19$.
- $v = [1,1,2,1,2,2]$: $a_v = 729, b_v = 512, c_v = 881, d_v = -217$. Linear form $L_v(n) = 217n + 881$.

### Commutator Data
$$\Delta_{u,v} = d_u c_v - d_v c_u = (-11)(881) - (-217)(19) = -5568 = -2^6 \cdot 87, \qquad \kappa_{u,v} = v_2(\Delta_{u,v}) = 6$$

### Affine Commutator Identity
$$b_u b_v \big(F_{uv}(n) - F_{vu}(n)\big) = 8192 \left(-\frac{5568}{8192}\right) = -5568$$

### Concrete Switching Coordinate & Cancellation Gate
Let $x = v_2(L_u(n))$:
- **$u$-transition**: $x \mapsto x - 4$.
- **$v$-transition**: Supported on resonance layer $x = 6$, $L_u(n) = 2^6 U$ ($U$ odd, $U \equiv 1 \pmod{16}$):
  $$x' = v_2(729 U + 87) - 3$$

---

## 5. Established versus Proposed Results

### Established Algebraic Identities (Lean 4 Formalization Target)
1. **Destination Precision**: $M \ge A + q_t$ is necessary and sufficient for full source cylinder determinism modulo $2^{q_t}$.
2. **Same-Form Eigenidentity**: $b_p H_p(F_p(n)) = a_p H_p(n)$.
3. **Cross-Form Identity**: $b_q H_p(F_q(n)) = a_q H_p(n) + \Delta_{p,q}$.
4. **Affine Commutator Identity**: $b_p b_q (F_{q,p}(n) - F_{p,q}(n)) = \Delta_{p,q}$.
5. **Common-Center Criterion**: $\Delta_{p,q} = 0 \iff x_p^* = x_q^*$.

### Competing Proof Architectures (Phase 7.3D)
1. **Path-Complete Graph Lyapunov Rankings** (`path_complete_ranking_v1`)
2. **Disjunctive Transition Invariants** (`disjunctive_transition_invariant_v1`)
3. **Lexicographic & Multiphase Rankings**
4. **Size-Change Termination (SCT)**

---

## 6. Phase 7.3 Sub-Phase Roadmap

```text
Phase 7.3A: Generic Affine Interaction & Symbolic Theorem Kernel
Phase 7.3B: Minimal Single-Coordinate Ultrametric Register Machine (L_u(n) = 11n+19)
Phase 7.3C: Symbolic Return-Language & Entropy Probe (s ∈ {u,v}^≤12)
Phase 7.3D: Four Competing Proof Architectures on Target A (u/v core)
Phase 7.3E: Target Expansion (Minimal Core -> Target B -> Target C)
```
