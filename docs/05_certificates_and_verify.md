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



Concrete JSON certificates then only discharge closed arithmetic equality checks against these formal Lean lemmas.

