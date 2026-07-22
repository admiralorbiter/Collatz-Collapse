# Phase 7X Certificate Schemas and Verification

## 1. Design principles

Every proof object must distinguish:

- arithmetic identity;
- domain semantics;
- path validity;
- graph classification;
- termination algebra;
- bounded search scope.

No boolean field such as `"verified": true` is trusted without recomputation.

## 2. `destination_precision_v1`

Purpose: prove the exact source precision needed to determine a target modulus.

```json
{
  "schema_version": "destination_precision_v1",
  "word": [1, 1, 2],
  "odd_steps_k": 3,
  "total_valuation_a": 4,
  "source_residue": "7",
  "current_source_exponent": 5,
  "target_exponent": 6,
  "required_source_exponent": 10,
  "additional_bits": 5
}
```

Verifier recomputes:

\[
M_{\mathrm{required}}=\max(M_{\mathrm{current}},A+q).
\]

It must also verify necessity by showing that if \(M-A<q\), two quotient values produce distinct target residues.

## 3. `affine_commutator_v1`

```json
{
  "schema_version": "affine_commutator_v1",
  "left_word": [1, 1, 2],
  "right_word": [1, 2, 2],
  "left": {
    "k": 3,
    "A": 4,
    "a": "27",
    "b": "16",
    "c": "19",
    "d": "-11"
  },
  "right": {
    "k": 3,
    "A": 5,
    "a": "27",
    "b": "32",
    "c": "23",
    "d": "5"
  },
  "delta": "-348",
  "delta_v2": 2,
  "common_fixed_point": false
}
```

Verifier recomputes all fields from the words.

## 4. `cross_linear_form_transition_v1`

```json
{
  "schema_version": "cross_linear_form_transition_v1",
  "reference_word": [1, 1, 2],
  "transition_word": [1, 2, 2],
  "identity": {
    "left_multiplier": "32",
    "transition_multiplier": "27",
    "delta": "-348"
  },
  "broad_required_v2": 5,
  "exact_required_v2": 6
}
```

The displayed identity string is informational. Structured integers are authoritative.

Verifier checks:

\[
b_qH_p(F_q(n))=a_qH_p(n)+\Delta.
\]

## 5. `resonance_cylinder_v1`

```json
{
  "schema_version": "resonance_cylinder_v1",
  "reference_word": [1, 1, 2],
  "transition_word": [1, 2, 2],
  "interaction_v2_kappa": 2,
  "semantics": "exact_word",
  "normalized_odd_residue": "5",
  "normalized_modulus_exponent": 4,
  "recovered_source_residue": "43",
  "recovered_source_exponent": 6
}
```

Verifier must:

1. recompute \(\Delta\) and \(\kappa\);
2. solve the normalized congruence;
3. map back to \(n\)-coordinates;
4. compare to direct exact-cylinder modular inversion.

## 6. `common_fixed_point_family_v1`

```json
{
  "schema_version": "common_fixed_point_family_v1",
  "family_id": "CF-001",
  "words": [],
  "common_fixed_point": {
    "numerator": "C",
    "denominator": "D"
  },
  "common_form": {
    "alpha": "D",
    "beta": "-C"
  },
  "zero_case": {
    "positive_integer_fixed_point": false
  }
}
```

Verifier checks pairwise \(\Delta=0\) and every family identity.

## 7. `path_cylinder_v1`

```json
{
  "schema_version": "path_cylinder_v1",
  "path_words": [
    [1, 1, 2],
    [1, 2, 2]
  ],
  "composite": {
    "k": 6,
    "A": 9,
    "c": "881"
  },
  "source_semantics": "exact_guarded_path",
  "source_residue": "1959",
  "source_exponent": 14,
  "intermediate_guards": [],
  "target_guard": {
    "residue": "7",
    "exponent": 5
  }
}
```

Verifier must recompute every intermediate image universally, not only test the representative.

## 8. `ultrametric_cancellation_v1`

```json
{
  "schema_version": "ultrametric_cancellation_v1",
  "reference_word": [],
  "transition_word": [],
  "kappa": 0,
  "source_region": "resonant",
  "normalized_source_residue": "0",
  "normalized_source_bits": 0,
  "additional_cancellation_v2": 0,
  "target_feature_v2": 0
}
```

This schema is accepted only when the verifier proves the complete quantified transition.

## 9. `phase7x_discovery_outcome_v1`

Always emitted.

```json
{
  "schema_version": "phase7x_discovery_outcome_v1",
  "scope": {
    "max_word_length": 0,
    "max_step_valuation": 0,
    "max_path_length": 0,
    "max_residue_exponent": 0
  },
  "outcome": "SOUND_UNRANKED",
  "artifacts": []
}
```

Allowed outcomes:

- `COMMON_CENTER_FAMILY_FOUND`
- `NEAR_COMMUTING_TARGETS_FOUND`
- `CANCELLATION_AUTOMATON_TERMINATED`
- `PHASE6D_COLLAPSE`
- `PATH_INCOMPATIBLE`
- `SOUND_UNRANKED`
- `NO_RECURRENT_COMPONENT`
- `REFINEMENT_LIMIT`

## 10. Independent validity layers

Every final report must show:

```text
Layer 1: Macrostep arithmetic
Layer 2: Interaction identities
Layer 3: Cylinder and path semantics
Layer 4: Abstract-state soundness
Layer 5: Recurrent-language classification
Layer 6: Termination algebra
Layer 7: Claim-scope validity
```

Each layer returns:

- `VALID`
- `INVALID`
- `UNRESOLVED`
- `NOT_APPLICABLE`

## 11. Lean trust boundary

Lean should prove generic identities and selected quantified instances.

Rust and Python should remain responsible for:

- artifact parsing;
- bounded enumeration;
- graph construction;
- SCC extraction;
- certificate routing.

The final theorem must disclose exactly which parts were checked by Lean and which were checked by independent executable verifiers.
