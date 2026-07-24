import Mathlib.Data.Int.Basic
import Mathlib.Data.Nat.Basic
import PhaseI1CounterexampleCapture
import OddPrefixWitness
import Gate2OrbitBridge
import Gate3TimeGrowth
import Gate4ResidueBridge
import Gate5TailCompression
import Gate6AvoidanceReflection

namespace Gate1AxiomAudit

open PhaseI1CounterexampleCapture

/-!
# Gate 1 through Gate 6 Axiom Audit: Pure First-Principles Derivation of Formal Surface

This file inspects the exact type and axiom dependencies of:
1. `odd_prefix_shifted_height_bound_of_witness` (Gate 1 Category A Headline)
2. `oddOrbit_prefix_has_odd_prefix_witness` (Gate 2A Category A Constructor)
3. `semantic_prefix_has_odd_prefix_witness` (Gate 2D Category B Witness Bridge)
4. `semantic_prefix_shifted_height_bound` (Gate 2D Category B Headline Orbit Bound)
5. `semantic_prefix_time_ge_index` (Gate 3B Category A Linear Growth)
6. `semantic_prefix_time_exponential_domination` (Gate 3 Category A Headline Domination)
7. `semantic_return_state_endpoint_mod` (Gate 4C Category B Residue Remainder Bridge)
8. `semantic_return_state_endpoint_congruence` (Gate 4D Category B Residue ZMod Corollary)
9. `sufficient_time_forces_endpoint_lt_three_power` (Gate 5A Category B Bound)
10. `recurrent_tail_eventually_endpoint_eq_least_representative` (Gate 5B Category B Headline Compression)
11. `semantic_return_endpoint_compression` (Gate 5C Category B Asymptotic Compression)
12. `semantic_return_endpoint_has_leading_ternary_zeros` (Gate 5C Category B Leading Zeros)
13. `concrete_state_space_cardinality` (Gate 6.0 Category A Cardinality Proof = 864)
14. `two_pow_mod_nine_depends_on_mod_six` (Gate 6.2 Category A Mod-9 Periodicity)
15. `avoidingEdgeSpec_implies_finiteConditions` (Gate 6.7 Category A Edge Completeness)
16. `avoidingEdgeB_overapproximates` (Gate 6.9 Category A Over-Approximation)
17. `actual_avoiding_transition_projects_to_boolean_edge` (Gate 6.11 Category B Trajectory Projection)
-/

#print OddPrefixWitness
#print odd_step_shifted_height_step
#print odd_prefix_shifted_height_at
#print odd_prefix_shifted_height_bound_of_witness
#print axioms odd_prefix_shifted_height_bound_of_witness

#print v2_3x_plus_1
#print oddStep
#print oddOrbit
#print oddStep_exact
#print oddStep_pos
#print oddStep_odd
#print v2_3x_plus_1_pos
#print oddOrbit_state_pos
#print oddOrbit_state_odd
#print oddOrbit_step_exact
#print oddOrbit_prefix_has_odd_prefix_witness
#print axioms oddOrbit_prefix_has_odd_prefix_witness

#print semantic_realizer_source_q1
#print semantic_realizer_source_pos
#print semantic_realizer_source_odd
#print semantic_return_state_eq_oddOrbit_prefix
#print semantic_prefix_has_odd_prefix_witness
#print semantic_prefix_shifted_height_bound
#print axioms semantic_prefix_shifted_height_bound

#print semantic_word_time_pos
#print semantic_prefix_time_ge_index
#print nat_le_two_pow
#print two_pow_mono
#print initial_height_below_two_power_of_large_index
#print semantic_prefix_time_exponential_domination
#print axioms semantic_prefix_time_exponential_domination

#print semanticPrefixEndpointResidueNat
#print semantic_prefix_endpoint_residue_lt
#print semantic_return_state_endpoint_mod
#print semantic_return_state_endpoint_congruence
#print axioms semantic_return_state_endpoint_congruence

#print sufficient_time_forces_endpoint_lt_three_power
#print recurrent_tail_eventually_endpoint_eq_least_representative
#print semantic_return_endpoint_compression
#print semantic_return_endpoint_has_leading_ternary_zeros
#print axioms semantic_return_endpoint_has_leading_ternary_zeros

#print concrete_state_space_cardinality
#print oddResidue32Value_odd
#print two_pow_mod_nine_depends_on_mod_six
#print AvoidingStepWitness
#print AvoidingEdgeSpec
#print AvoidingFiniteConditions
#print avoidingEdgeSpec_implies_finiteConditions
#print avoidingEdgeB_overapproximates
#print actual_avoiding_transition_projects_to_boolean_edge
#print axioms actual_avoiding_transition_projects_to_boolean_edge

end Gate1AxiomAudit
