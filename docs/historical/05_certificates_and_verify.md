# Certificate Specifications & Verification Engine

## 1. Verification Architecture & Trust Boundaries

The verification layer enforces strict separation between search heuristics and proof verification:

```text
┌─────────────────────────────────────────────────────────┐
│ SEARCH LAYER (collatz-sieve, collatz-cegar, solvers)     │
│ - Uses Rayon parallelism, floats, heuristics, SMT, Z3   │
│ - Complex, optimized, subject to solver bugs            │
└───────────────────────────┬─────────────────────────────┘
                            │ Emits JSON Certificate File
                            ▼
┌─────────────────────────────────────────────────────────┐
│ VERIFIER BINARY (collatz-verify)                        │
│ - Zero dependencies on solvers, floats, or Rayon        │
│ - Small, audited, pure-Rust exact integer engine         │
│ - Returns strictly VALID or INVALID (with exact reason) │
└─────────────────────────────────────────────────────────┘
```

---

## 2. JSON Certificate Schemas

### 2.1 JSON Schema Rules & Security Bounds
1. **Strict Structural Schema Matching:** All schemas enforce `#[serde(deny_unknown_fields)]` to reject malformed or extra unexpected fields.
2. **True DoS Defense Bounds:** Denial-of-Service (DoS) protection is enforced by explicit runtime bounds:
   - Maximum input file size limit (e.g. 10 MB).
   - Maximum arbitrary-precision string length (`MAX_DIGITS = 4096`).
   - Maximum valuation word length limits (`MAX_WORD_LEN = 1024`).
   - Hard iteration ceiling on exception loops (`MAX_EXCEPTIONS_CHECKED = 100_000`).
3. **Informational Field Stripping:** Fields like `growth_debt_float` or `heuristic_score` are excluded from proof validation.

### 2.2 Descent Certificate Schema (`descent_v1.json`)

#### Broad Valuation Class Example (`terminal_at_least`)
```json
{
  "schema_version": "descent_v1",
  "valuation_word": [1, 1, 2, 1, 3],
  "total_twos": 8,
  "odd_steps": 5,
  "starting_residue": "39",
  "modulus_exponent": 8,
  "constant": "251",
  "descent_threshold": "20",
  "checked_exceptions": [],
  "valuation_semantics": "terminal_at_least"
}
```

#### Exact Valuation Cylinder Example (`exact_word`)
```json
{
  "schema_version": "descent_v1",
  "valuation_word": [1, 1, 2, 1, 3],
  "total_twos": 8,
  "odd_steps": 5,
  "starting_residue": "295",
  "modulus_exponent": 9,
  "constant": "251",
  "descent_threshold": "20",
  "checked_exceptions": [],
  "valuation_semantics": "exact_word"
}
```

### 2.3 Tail Descent Certificate Schema (`tail_descent_v1.json`)
Certifies all infinite child valuations $a_k \ge a_{\text{crit}}$ in a single quantified analytical step:
```json
{
  "schema_version": "tail_descent_v1",
  "prefix_word": [1, 1, 2],
  "prefix_total_twos": 4,
  "prefix_constant": "13",
  "minimum_child_valuation": 3,
  "proof_bound": "1"
}
```

### 2.4 Canonical Cover Manifest Schema (`cover_v1.json`)
Exports the exact disjoint antichain of binary cylinders forming the certified union:
```json
{
  "schema_version": "cover_v1",
  "total_leaves": 1,
  "max_modulus_exponent": 4,
  "total_scaled_measure": "1",
  "is_exact_cover": true,
  "merkle_root_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  "leaves": [
    {
      "valuation_word": [2, 2],
      "total_twos": 4,
      "starting_residue": "1",
      "modulus_exponent": 4,
      "valuation_semantics": "terminal_at_least"
    }
  ]
}
```

### 2.5 Subsumption Certificate Schema with Simulation Witness (`infeasible_subsumption_v1`)
```json
{
  "schema_version": "infeasible_subsumption_v1",
  "valuation_word": [1, 1, 3],
  "total_twos": 5,
  "odd_steps": 3,
  "target_valuation_word": [2, 1, 2],
  "target_total_twos": 5,
  "source_constant": "23",
  "target_constant": "25",
  "residue_offset": "0",
  "step_offset": 2,
  "subsumption_reason": "state_signature_subsumed_in_dag"
}
```

---

## 3. Verifier Invariants & Security Bounds (`collatz-verify`)

When `collatz-verify` inspects a `descent_v1` certificate, it executes **6 exact invariant checks**:

```text
Step 0: Explicit DoS Security Bounds
        Assert modulus_exponent <= 4096
        Assert sum(valuation_word) <= 4096
        Assert for all a_i: 1 <= a_i <= 255
        Assert string length <= MAX_DIGITS (4096)

Step 1: Recompute Total Valuation & Verify Valuation Semantics Exponent
        A_k = sum(valuation_word)
        If valuation_semantics == "exact_word":
            Assert modulus_exponent == A_k + 1
        Else ("terminal_at_least"):
            Assert modulus_exponent == A_k

Step 2: Recompute Affine Constant c_k
        c_0 = 0
        c_{i+1} = 3 * c_i + 2^{A_i}
        Assert computed c_k == parse_bounded_biguint(constant)

Step 3: Verify Closed-Form Starting Residue
        If "exact_word":
            Assert starting_residue == (2^{A_k} - c_k) * (3^k)^{-1} mod 2^{A_k + 1}
        Else ("terminal_at_least"):
            Assert starting_residue == -c_k * (3^k)^{-1} mod 2^{A_k}

Step 4: Verify Multiplicative Contraction
        Assert 2^{A_k} > 3^k

Step 5: Verify Exact Integer Threshold B
        Assert descent_threshold == floor(c_k / (2^{A_k} - 3^k)) + 1

Step 6: Independent Exhaustive Exception Verification
        Independently construct E = { n : 0 < n < B, n ≡ starting_residue (mod 2^m), n odd }
        For each e in E (up to MAX_EXCEPTIONS_CHECKED = 100,000):
            Run concrete odd_step(e) for k steps and assert result < e or reaches 1
```

If all steps pass using checked arbitrary-precision arithmetic, `collatz-verify` prints `VALID`.

---

## 4. Lean 4 Structural Base Lemmas (Subtraction-Free `Nat` Form)

Rather than relying on `decide` to discharge universal natural-number statements, Lean 4 formalization avoids saturated subtraction issues (`Nat.sub` saturating at 0) by expressing all lemmas in subtraction-free multiplicative `Nat` arithmetic:

1. **Affine Recurrence Lemma (`affine_step_correctness`)**:
   $$\forall n_0, \quad 2^{A_k} P_w(n_0) = 3^k n_0 + c_k$$
2. **Prescribed-Division Domination Lemma (`broad_domination_lemma`)**:
   $$\forall n_0, \quad S^k(n_0) \le P_w(n_0)$$
3. **Residue Valuation Forcing Lemma (`residue_forces_valuation`)**:
   $$n_0 \equiv -c_k (3^k)^{-1} \pmod{2^{A_k}} \implies 2^{A_k} \mid (3^k n_0 + c_k)$$
4. **Subtraction-Free Contraction Lemma (`contraction_forces_descent`)**:
   $$c_k + 3^k n_0 < 2^{A_k} n_0 \implies P_w(n_0) < n_0$$
5. **Floor Threshold Soundness Lemma (`exact_threshold_soundness`)**:
   $$n_0 \ge \left\lfloor \frac{c_k}{2^{A_k} - 3^k} \right\rfloor + 1 \implies c_k + 3^k n_0 < 2^{A_k} n_0$$

Concrete JSON certificates then only discharge closed arithmetic equality checks against these formal Lean 4 lemmas.

---

## 5. Relational Countdown Certificate Schemas

### 5.1 Minus-One Countdown Certificate (`minus_one_countdown_v1.json`)
Operates as a **Kernel-Supported Symbolic Theorem Schema**:
```json
{
  "schema_version": "minus_one_countdown_v1",
  "modulus_exponent": 4
}
```
* **Verifier Rule:** Verifies the 2-adic symbolic derivation $S(n) + 1 = 3 \cdot 2^{m+\tau-1} u$ for exponent $m \le 16$, validating the loop rule ($\tau \ge 1 \implies S(n) \equiv 2^m - 1 \pmod{2^m}$) and exit rule ($\tau = 0 \implies S(n) \equiv 2^{m-1} - 1 \pmod{2^m}$).

### 5.2 Macrocycle Obstruction Certificate (`obstruction_cycle_v1.json`)
Documents verified non-contracting abstract cycles and their finite-fuel / infinite-realization status:
```json
{
  "schema_version": "obstruction_cycle_v1",
  "classification": "FiniteFuelMacrocycle",
  "cycle_length": 3,
  "vertex_sequence": ["7", "11", "9", "7"],
  "valuation_word": [1, 1, 2],
  "total_twos": 4,
  "odd_steps": 3,
  "constant": "19",
  "edge_realizable": true,
  "one_lap_realizable": true,
  "one_lap_witness": "231",
  "two_lap_witness": "743",
  "three_lap_witness": "41703",
  "verified_repetition_laps": 10,
  "all_finite_repetitions_proved": true,
  "finite_repetition_proof": {
    "claim_id": "CLM-MACROCYCLE-112-FINITE-REPETITION-001",
    "proof_artifact": "claims/verified/macrocycle_112_finite_repetition.json"
  },
  "infinite_positive_realization": "ruled_out",
  "infinite_realization_proof": {
    "claim_id": "CLM-MACROCYCLE-112-NO-POSITIVE-INFINITE-001",
    "proof_artifact": "claims/verified/macrocycle_112_no_positive_infinite.json"
  },
  "macrocycle_countdown": {
    "linear_coefficient": "11",
    "linear_constant": "19",
    "valuation_offset": 4,
    "valuation_drop_per_lap": 4,
    "normalized_decrement": 1,
    "definition": "floor((v2(11n+19)-4)/4)"
  },
  "primary_obstruction": "FiniteFuelMacrocycle (Cycle 7->11->9->7: 1-lap witness n=231 mod 256, 2-lap witness n=743 mod 4096, 3-lap witness n=41703 mod 65536)"
}
```

### 5.3 Verified Finite-Fuel Macrocycle Certificate (`finite_fuel_macrocycle_v2.json`)
```json
{
  "schema_version": "finite_fuel_macrocycle_v2",
  "valuation_word": [1, 1, 2],
  "odd_steps": 3,
  "total_twos": 4,
  "affine_constant": "19",
  "state_modulus_exponent": 4,
  "start_residue": "7",
  "return_residue": "7",
  "fixed_point_linear_form": {
    "alpha": "11",
    "beta": "19",
    "definition": "alpha*n + beta",
    "normalization": "positive_leading_coefficient"
  },
  "fixed_point": {
    "numerator": "-19",
    "denominator": "11",
    "positive_integer": false
  },
  "countdown": {
    "multiplier_kind": "expanding",
    "multiplier_numerator": "27",
    "multiplier_denominator": "16",
    "word_repetition_offset": 1,
    "return_state_offset": 4,
    "valuation_drop_per_lap": 4,
    "word_repetitions_definition": "floor((v2(alpha*n+beta)-1)/A)",
    "return_state_repetitions_definition": "floor((v2(alpha*n+beta)-m)/A)"
  },
  "one_lap_witness": "231",
  "finite_repetition_proof": {
    "claim_id": "CLM-FINITE-FUEL-4-3",
    "proof_artifact": "claims/verified/macrocycle_4_3_finite.json",
    "proof_hash": "e6717a61ec27e289bfad1615a19544c45b8492fe1d78216f9fef11fe9bbfe1bc"
  },
  "infinite_realization_proof": {
    "claim_id": "CLM-NO-POSITIVE-INFINITE-4-3",
    "proof_artifact": "claims/verified/macrocycle_4_3_no_infinite.json",
    "proof_hash": "2f65b4c1fc7ec6ec54a6524314c44d08ee732e737976e18507ceb2c3ee2ca1c7"
  }
}
```

### 5.4 Size-Change Termination Proof Object Schema (`size_change_scc_v1.json`)
Requires complete transition graphs, canonical edge ordering, and explicit vertex sets for verifier re-computation:
```json
{
  "schema_version": "size_change_scc_v1",
  "scc_id": "SCC-DEPTH20-CRITICAL-001",
  "feature_vector": ["v2_L1", "v2_L2", "bitlength"],
  "vertices": ["u1", "v1"],
  "transition_graphs": [
    {
      "source_node": "u1",
      "target_node": "v1",
      "valuation_word": [1, 2],
      "relations": [
        {"src_feature": "v2_L1", "relation": "decrease", "dst_feature": "v2_L1"},
        {"src_feature": "v2_L2", "relation": "non_increase", "dst_feature": "v2_L1"}
      ]
    }
  ],
  "canonical_edge_ordering": ["u1->v1"],
  "verifier_recomputation_required": true
}
```

### 5.5 Büchi Automaton Emptiness Proof Object Schema (`buchi_emptiness_scc_v1.json`)
Stores the complete state inventory, finite macrostep alphabet, transition table, accepting state set, and SCC decomposition:
```json
{
  "schema_version": "buchi_emptiness_scc_v1",
  "scc_id": "SCC-BUCHI-DEPTH20-001",
  "alphabet": ["M1_1_2", "M2_2_1", "M_tail_crit"],
  "states": ["q0", "q1", "q2"],
  "initial_state": "q0",
  "accepting_states": ["q1"],
  "transitions": [
    {"src": "q0", "symbol": "M1_1_2", "dst": "q1"},
    {"src": "q1", "symbol": "M2_2_1", "dst": "q2"}
  ],
  "reachable_states": ["q0", "q1", "q2"],
  "scc_decomposition": [["q0"], ["q1", "q2"]],
  "verifier_recomputation_required": true
}
```
