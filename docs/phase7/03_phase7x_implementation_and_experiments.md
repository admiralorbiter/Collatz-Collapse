# Phase 7.3 Implementation and Experiment Plan

## 1. Milestone Title

**Phase 7.3 — Affine Interaction, Ultrametric Register Machine, and Symbolic Return Invariants**

## 2. Primary Objective

Build and verify a compact, switching-sensitive ultrametric invariant for the verified Phase 7.2 noncommuting branching core at state $Q_1$:
- $u = [1,1,2]$
- $v = w_1 w_2 = [1,1,2,1,2,2]$

Evaluate four competing proof systems to classify whether the $(u, v)$ switching language admits a verified termination invariant.

## 3. Staged Implementation Architecture (Phase 7.3A through 7.3E)

```text
crates/collatz-affine/src/
    macrostep_data.rs
    affine_interaction.rs
    path_composition.rs

crates/collatz-abstract/src/
    precision_debt.rs
    cancellation_register_machine.rs
    feature_cegar.rs
    symbolic_return_language.rs

crates/collatz-cegar/src/
    graph_lyapunov_synthesis.rs
    disjunctive_invariant_synthesis.rs
    lexicographic_ranking_synthesis.rs
    sct_closure.rs

crates/collatz-cert/src/
    phase73_schemas.rs

crates/collatz-verify/src/
    verify_based_switching_core.rs
    verify_finite_switching_language.rs
    verify_path_complete_ranking.rs
    verify_disjunctive_transition_invariant.rs
    verify_language_growth.rs
```

## 4. Experiment 7.3A — Generic Affine Identity Kernel

### Goal
Implement and independently verify generic affine macrostep data, cross-form identities, commutator constants, and exact broad/exact resonance recovery across Rust, Python oracle, and Lean 4.

### Instantiation
- $u = [1,1,2]$ ($a_u=27, b_u=16, c_u=19, d_u=-11$)
- $v = [1,1,2,1,2,2]$ ($a_v=729, b_v=512, c_v=881, d_v=-217$)
- Denominator: $b_u b_v = 16 \cdot 512 = 8192$
- Multiplier: $a_u a_v = 27 \cdot 729 = 19683$
- Interaction constant: $\Delta_{u,v} = (-11)(881) - (-217)(19) = -5568 = -2^6 \cdot 87$, $v_2(\Delta_{u,v}) = 6$
- Order defect: $F_{uv}(n) - F_{vu}(n) = \frac{-5568}{8192} = -\frac{87}{128}$

---

## 5. Experiment 7.3B — Minimal Single-Coordinate Ultrametric Register Machine

### Goal
Construct a deterministic cancellation register machine for Target A ($u/v$ core) starting with a single reference coordinate:
$$L_u(n) = 11n + 19, \qquad x = v_2(L_u(n))$$

### Feature CEGAR Protocol
- **Initial Register State**: $(x, \text{res})$, $x \in \{<6, 6, >6\}$.
- **Resonance Layer ($x=6$)**: $L_u(n) = 2^6 U$ ($U$ odd, $U \equiv 1 \pmod{16}$).
- **Transition Laws**:
  - $u$: $x \mapsto x - 4$.
  - $v$: Requires $x=6, U \equiv 1 \pmod{16}$, yields $x' = v_2(729U + 87) - 3$.
- **Refinement Trigger**: Add $L_v(n) = 217n + 881$ or deeper $U$-bits only if a concrete counterexample proves single-coordinate state is insufficient.

---

## 6. Experiment 7.3C — Symbolic Return-Language & Entropy Probe

### Goal
Enumerate all binary switching words $s \in \{u,v\}^{\le L}$ for $L = 10 \dots 12$ to analyze the induced return language at $Q_1$.

### Measurements
- Number of admissible switching words of length $r$.
- Number of words with nonempty positive-integer return cylinders.
- Spectral radius of adjacency matrix and topological entropy.
- Fraction of prefixes allowing both $u$ and $v$ extensions.

---

## 7. Experiment 7.3D — Four Competing Proof Systems on Target A

Attempt to prove or disprove switching termination using four proof architectures in order:
1. **State-Indexed / Path-Complete Graph Lyapunov Rankings** (`path_complete_ranking_v1`)
2. **Disjunctive Transition Invariants** (`disjunctive_transition_invariant_v1`)
3. **Lexicographic & Multiphase Rankings**
4. **Size-Change Termination (SCT)**

### Result Statuses
- `TERMINATED_PATH_COMPLETE_RANKING`
- `TERMINATED_DISJUNCTIVE_INVARIANT`
- `TERMINATED_LEXICOGRAPHIC`
- `TERMINATED_MULTIPHASE`
- `TERMINATED_SCT`
- `SOUND_UNRANKED`
- `REFINEMENT_LIMIT`

---

## 8. Experiment 7.3E — Target Expansion (A $\to$ B $\to$ C)

Only after Target A is fully classified:
1. **Target B**: Add $Q_2$ self-loop.
2. **Target C**: Add $Q_3$ and complete 3-state SCC.

---

## 9. Lean 4 Formalization (`lean/Phase73.lean`)

1. Macrostep affine definitions and oddness of $d_p, c_p$.
2. Same-form eigenidentity: $b_p H_p(F_p(n)) = a_p H_p(n)$.
3. Cross-form identity: $b_q H_p(F_q(n)) = a_q H_p(n) + \Delta_{p,q}$.
4. Affine commutator identity: $b_p b_q (F_{q,p}(n) - F_{p,q}(n)) = \Delta_{p,q}$.
5. Common-center criterion: $\Delta_{p,q} = 0 \iff x_p^* = x_q^*$.
6. Instantiated $(u,v)$ arithmetic proofs over exact cylinders.
