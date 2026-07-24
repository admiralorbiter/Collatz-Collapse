# Active Roadmap and Phase Overview — Verified Collatz Formalization & Obstruction-Discovery Program

## Project Scope & Retrospective Verdict Notice

> [!CAUTION]
> **Project Scope Update**: The project is formally designated as the **Verified Collatz Formalization and Obstruction-Discovery Program**.
> All phase expansions beyond Phase I.Z-G are halted. All focus is dedicated to the Gate 0–8 Zero-Echo Reconstruction plan and the Adaptive CEGAR-Refinement Program.
> The headline theorems `minimal_odd_counterexample_returns_to_q1_infinitely_often` and `no_minimal_counterexample_avoiding_tail` are marked as **Compiled but Mathematically Untrusted** pending reconstruction of the avoidance graph proof.

---

## 864-State Diagnostic Census & Retrospective Verdict Findings

Following the zero-echo reconstruction of **Gates 0–6** (where actual shifted height contraction, linear time growth, 3-adic residue compiler correctness, eventual least-representative equality, full ternary compression, and 864-state trajectory projection were derived 100% in Lean 4 without `sorry`), we executed a diagnostic Tarjan SCC census over the 864-state quotient space:

```
=== 864-STATE AVOIDANCE GRAPH DIAGNOSTIC CENSUS ===
States (Total): 864 (810 avoiding, 54 Q1)
Boolean edges: 7,992
SCC count: 271
Largest SCC size: 540 (1 Giant Recurrent SCC containing ~92.3% of reachable states)
Cyclic SCCs: 1
Reachable states: 585
Reachable SCCs: 46
Reachable cyclic SCCs: 1 (The 540-state giant SCC is fully reachable)
====================================================
```

### Strategic Verdict & CEGAR Pivot:
1. **Do NOT Formalize the 864-State Partition**: The current quotient over-approximation collapses into a single giant 540-state cyclic SCC due to mod-32 non-determinism at $r=21 \pmod{32}$ and path-splicing across abstract cycles.
2. **Freeze Top-Level Contradiction DAG**: Before certifying refined graphs, state the exact top-level contradiction proof connecting $y_m = \mu_m$ (and leading ternary zeros) to a formal contradiction $\bot$.
3. **CEGAR Cycle-Lifting Harness**: Implement a diagnostic harness in Rust to test concrete liftability of simple cycles in the giant SCC, identifying path-splicing and carry/valuation failures to refine only the necessary predicates.

---

## Six-Stage Adaptive Reconstruction Roadmap

- **Stage 1 — Kernel Freeze & Snapshot for Gates 0–6**: Complete (`lean/OddPrefixWitness.lean`, `Gate2OrbitBridge.lean`, `Gate3TimeGrowth.lean`, `Gate4ResidueBridge.lean`, `Gate5TailCompression.lean`, `Gate6AvoidanceReflection.lean` verified without `sorry`).
- **Stage 2 — Top-Level Contradiction DAG Formalization**: Active (`Gate2EDagContradiction.lean` - formalizing $\neg\text{Collatz} \implies \bot$ via recurrent compression and avoiding graph elimination).
- **Stage 3 — CEGAR Cycle-Lifting Diagnostic Harness**: Active (`src/zero_lift_search.rs` - concrete path-splicing & cycle-lifting analysis).
- **Stage 4 — Controlled Precision & Valuation-Drift Sweep**: Open (Parameter sweep over binary precision $a$, 3-adic compiler $b$, valuation cutoff $K$, and block length $L$).
- **Stage 5 — Adaptive Quotient Design & State Refinement**: Open (`lean/Gate7AdaptiveQuotient.lean` - Valuation Cylinders $\times$ Compiler Carry State $\times$ Compression Guard).
- **Stage 6 — Verified Refined Graph Certification & Elimination**: Open (`lean/Gate7SCCPartition.lean`, `Gate8AvoidanceGraph.lean` - Lean 4 verified parent tree certificates and elimination).

---

## Verified Kernel & Gate Sequence (Gates 0–8)

| Gate ID | Reconstruction Objective | Target Symbol / Signature | Status |
|---|---|---|---|
| **Gate 0** | **Definition-Fidelity Audit** | Audit `OddPrefixWitness`, `UniversalOddPrefixCertificateAt`, and `semanticPrefixTime` | **PASSED** |
| **Gate 1** | **Shifted Height Contraction & Prefix Bound** | `odd_step_shifted_height_step` proved; `odd_prefix_shifted_height_bound_of_witness` derived without `h_bound` | **PASSED** |
| **Gate 2A** | **Generic Orbit Prefix Witness** | `oddStep_exact`, `oddStep_pos`, `oddStep_odd`, `v2_3x_plus_1_pos`, `oddOrbit_state_pos`, `oddOrbit_state_odd`, `oddOrbit_step_exact`, `oddOrbit_prefix_has_odd_prefix_witness` | **PASSED** |
| **Gate 2B** | **Semantic Endpoint Identification** | `semantic_return_state_eq_oddOrbit_prefix` ($y_m = \text{oddOrbit}(M, T_m)$) | **PASSED** |
| **Gate 2C** | **Source Facts from hreal** | `semantic_realizer_source_q1`, `semantic_realizer_source_pos`, `semantic_realizer_source_odd` | **PASSED** |
| **Gate 2D** | **Category B Witness & Shifted Height** | `semantic_prefix_has_odd_prefix_witness` & `semantic_prefix_shifted_height_bound` | **PASSED** |
| **Gate 3** | **Time Growth & Exponential Domination** | `semantic_word_time_pos`, `semantic_prefix_time_ge_index`, `nat_le_two_pow`, `two_pow_mono`, `initial_height_below_two_power_of_large_index`, `semantic_prefix_time_exponential_domination` | **PASSED** |
| **Gate 4** | **Residue Congruence Bridge** | `semantic_prefix_endpoint_residue_lt`, `semantic_return_state_endpoint_mod`, `semantic_return_state_endpoint_congruence` | **PASSED** |
| **Gate 5** | **Tail Compression & Least Representative** | `sufficient_time_forces_endpoint_lt_three_power`, `recurrent_tail_eventually_endpoint_eq_least_representative`, `semantic_return_endpoint_compression`, `semantic_return_endpoint_has_leading_ternary_zeros` | **PASSED** |
| **Gate 6** | **Avoidance Edge Reflection & Trajectory Projection** | `concrete_state_space_cardinality` (864), `two_pow_mod_nine_depends_on_mod_six`, `AvoidingStepWitness`, `avoidingEdgeSpec_implies_finiteConditions`, `avoidingEdgeB_overapproximates`, `actual_avoiding_transition_projects_to_boolean_edge` | **PASSED** |
| **Gate 7** | **Refined Adaptive SCC Partition** | Bounded-rank parent tree certified SCC partition over refined adaptive quotient space | **NEXT TARGET** |
| **Gate 8** | **Avoidance Graph Elimination** | `all_true_avoiding_components_eliminated` with closed certificates | OPEN |
