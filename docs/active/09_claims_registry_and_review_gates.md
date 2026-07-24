# Claims Registry and Review Gates — Verified Collatz Formalization & Obstruction-Discovery Program

## Four-Category Single-Source-of-Truth Ledger

### Category A: Proved Definition-Faithful Theorems (Lean 4 Verified Without Sorry)
- `collatz_false_implies_minimal_counterexample_exists` (Proved from well-foundedness of $\mathbb{N}$)
- `minimal_counterexample_is_odd` (Proved via $N/2 < N$)
- `odd_step_shifted_height_step` ($2(s_{\text{next}}+1) \le 3(s_{\text{curr}}+1)$)
- `odd_prefix_shifted_height_at` ($2^j(x_j+1) \le 3^j(M+1)$ by induction on $j \le T$)
- `odd_prefix_shifted_height_bound_of_witness` ($2^T(y+1) \le 3^T(M+1)$ at $j=T$)
- `oddStep_exact` ($2^{v_2(3x+1)} \text{oddStep}(x) = 3x+1$)
- `oddStep_pos` ($0 < \text{oddStep}(x)$)
- `oddStep_odd` ($\text{Odd}(\text{oddStep}(x))$)
- `v2_3x_plus_1_pos` ($1 \le v_2(3x+1)$)
- `oddOrbit_state_pos` ($0 < \text{oddOrbit}(M, n)$)
- `oddOrbit_state_odd` ($\text{Odd}(\text{oddOrbit}(M, n))$)
- `oddOrbit_step_exact` ($2^{v_2(3x_n+1)} \text{oddOrbit}(M, n+1) = 3 \text{oddOrbit}(M, n) + 1$)
- `oddOrbit_prefix_has_odd_prefix_witness` (`OddPrefixWitness M T (oddOrbit M T)`)
- `semantic_realizer_source_pos_from_q1` ($0 < M$ for $M \equiv 7 \pmod{32}$)
- `semantic_realizer_source_odd_from_q1` ($\text{Odd}(M)$ for $M \equiv 7 \pmod{32}$)
- `oddOrbit_add` ($\text{oddOrbit}(\text{oddOrbit}(M, T), U) = \text{oddOrbit}(M, T + U)$)
- `semantic_word_time_pos` ($1 \le w.\text{word}.\text{length}$)
- `semantic_prefix_time_ge_index` ($m \le T_m$)
- `nat_le_two_pow` ($n \le 2^n$)
- `two_pow_mono` ($a \le b \implies 2^a \le 2^b$)
- `initial_height_below_two_power_of_large_index` ($M + 1 \le 2^{T_m}$ for $M + 1 \le m$)
- `semantic_prefix_time_exponential_domination` ($\exists m_0 = M+1, \forall m \ge m_0, M + 1 \le 2^{T_m}$)
- `semantic_prefix_endpoint_residue_lt` ($\mu_m < 3^{T_m}$)
- `concrete_state_space_cardinality` ($\text{Fintype.card ConcreteAvoidingQuotientState} = 864$)
- `two_pow_mod_nine_depends_on_mod_six` ($2^k \equiv 2^{k \pmod 6} \pmod 9$)
- `avoidingEdgeSpec_implies_finiteConditions` ($\text{AvoidingEdgeSpec}(s,t) \implies \text{AvoidingFiniteConditions}(s,t)$)
- `avoidingEdgeB_overapproximates` ($\text{AvoidingEdgeSpec}(s,t) \implies \text{avoidingEdgeB}(s,t) = \text{true}$)

### Category B: Proved Actual-Orbit Bridges (Lean 4 Verified Without Sorry)
- `semantic_realizer_source_q1` ($M \equiv 7 \pmod{32}$)
- `semantic_realizer_source_pos` ($0 < M$)
- `semantic_realizer_source_odd` ($\text{Odd}(M)$)
- `semantic_return_state_eq_oddOrbit_prefix` ($y_m = \text{oddOrbit}(M, T_m)$)
- `semantic_prefix_has_odd_prefix_witness` (`OddPrefixWitness M T_m y_m`)
- `semantic_prefix_shifted_height_bound` ($2^{T_m} (y_m + 1) \le 3^{T_m} (M + 1)$ with zero witness arguments)
- `semantic_return_state_endpoint_mod` ($y_m \pmod{3^{T_m}} = \mu_m$)
- `semantic_return_state_endpoint_congruence` ($(y_m : \text{ZMod } 3^{T_m}) = (\mu_m : \text{ZMod } 3^{T_m})$)
- `sufficient_time_forces_endpoint_lt_three_power` ($y_m < 3^{T_m}$ for $m \ge M+1$)
- `recurrent_tail_eventually_endpoint_eq_least_representative` ($y_m = \mu_m$ for $m \ge M+1$ with zero open hypotheses)
- `semantic_return_endpoint_compression` ($2^K y_m < 3^{T_m}$ eventually for any $K \ge 0$)
- `semantic_return_endpoint_has_leading_ternary_zeros` ($y_m < 3^{T_m - L}$ eventually for any $L \ge 0$)
- `actual_avoiding_step_projects_to_witness` (Actual trajectory step projects to `AvoidingStepWitness`)
- `actual_avoiding_transition_projects_to_boolean_edge` (`avoidingEdgeB s t = true` directly from `RealizesAvoidingItinerary α M` with zero open premises)

### Category C: Valid Reduction Lemmas with Open Premises / Bridges
- `minimal_counterexample_has_universally_certified_tail` (Requires actual-tail itinerary bridge)

### Category D: Reclassified Echo / Schema Wrappers
- Legacy theorems with $P \implies P$, `False` premise, or `Finset.univ` schema wrappers.

---

## Active Review Gates

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
| **Gate 6** | **Avoidance Edge Over-Approximation & Trajectory Projection** | `concrete_state_space_cardinality` (864), `two_pow_mod_nine_depends_on_mod_six`, `AvoidingStepWitness`, `avoidingEdgeSpec_implies_finiteConditions`, `avoidingEdgeB_overapproximates`, `actual_avoiding_transition_projects_to_boolean_edge` | **PASSED** |
| **Gate 7** | **Refined Adaptive SCC Partition** | Bounded-rank parent tree certified SCC partition over refined adaptive quotient space | **NEXT TARGET** |
| **Gate 8** | **Avoidance Graph Elimination** | `all_true_avoiding_components_eliminated` with closed certificates | OPEN |
