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

### 2.1 JSON Schema Rules
1. **String-Encoded BigInts:** All arbitrary-precision integers (`c_k`, `r_k`, `threshold`, `exceptions`) must be serialized as base-10 strings (e.g., `"12345678901234567890"`). Floating-point or numeric JSON representations are strictly prohibited to prevent IEEE-754 rounding errors.
2. **Informational Field Stripping:** Fields like `growth_debt_float` or `heuristic_score` may exist for reporting, but `collatz-verify` ignores them completely.

### 2.2 Descent Certificate Schema (`descent_v1.json`)
```json
{
  "schema_version": "descent_v1",
  "valuation_word": [1, 1, 2, 1, 3],
  "total_twos": 8,
  "odd_steps": 5,
  "starting_residue": "23",
  "modulus_exponent": 8,
  "constant": "211",
  "descent_threshold": "15",
  "checked_exceptions": ["1", "3", "5", "7", "9", "11", "13"]
}
```

### 2.3 Cycle Certificate Schema (`cycle_v1.json`)
```json
{
  "schema_version": "cycle_v1",
  "valuation_word": [1, 2],
  "total_twos": 3,
  "odd_steps": 2,
  "starting_integer": "1",
  "intermediate_values": ["1", "1"],
  "is_nontrivial": false
}
```

### 2.4 Infeasible Prefix Certificate Schema (`infeasible_v1.json`)
```json
{
  "schema_version": "infeasible_v1",
  "valuation_word": [1, 1, 1, 1, 1, 1],
  "total_twos": 6,
  "odd_steps": 6,
  "starting_residue": "63",
  "modulus_exponent": 6,
  "constant": "364",
  "rejection_reason": "exceeds_minimal_counterexample_bound",
  "intermediate_step_index": 4,
  "bound_threshold": "12"
}
```

---

## 3. Verifier Invariants (`collatz-verify`)

When `collatz-verify` inspects a `descent_v1` certificate, it executes the following **6 exact invariant checks**:

```text
Step 1: Recompute Total Valuation
        A_k = sum(valuation_word)
        Assert A_k == total_twos and 2^{A_k} == 2^{modulus_exponent}

Step 2: Recompute Affine Constant c_k
        c_0 = 0
        c_{i+1} = 3 * c_i + 2^{A_i}
        Assert computed c_k == parse_bigint(constant)

Step 3: Verify Closed-Form Starting Residue
        Compute inverse = (3^k)^{-1} mod 2^{A_k}
        Assert parse_bigint(starting_residue) == (-c_k * inverse) mod 2^{A_k}

Step 4: Verify Multiplicative Contraction
        Assert 2^{A_k} > 3^k

Step 5: Verify Exact Integer Threshold B
        Assert parse_bigint(descent_threshold) == floor(c_k / (2^{A_k} - 3^k)) + 1

Step 6: Verify Checked Exceptions
        For each e in checked_exceptions:
            Assert e < B
            Assert e mod 2^{A_k} == starting_residue
            Run concrete odd_step(e) for k steps and assert result < e or reaches 1
```

If all 6 steps pass using checked arbitrary-precision arithmetic, `collatz-verify` prints `VALID`.

---

## 4. SAT Bit-Blasting & LRAT Proof Logging

For bounded impossibility results (e.g., proving no minimal-counterexample-feasible valuation word of length $k$ exists within a search space):

```text
┌────────────────────────────────────────┐
│ collatz-sat (CNF Encoder)              │
│ - Encodes valuation choice & arithmetic│
└───────────────────┬────────────────────┘
                    │ Emits CNF Formula
                    ▼
┌────────────────────────────────────────┐
│ External SAT Solver (e.g., CaDiCaL)    │
└───────────────────┬────────────────────┘
                    │ Emits LRAT Proof File
                    ▼
┌────────────────────────────────────────┐
│ Independent LRAT Checker (drat-trim)   │
│ - Verifies boolean UNSAT proof         │
└────────────────────────────────────────┘
```

> **Trust Boundary Requirement:** An LRAT proof guarantees that the boolean CNF formula is UNSAT. However, it does not guarantee that the CNF encoding accurately represents Collatz arithmetic. Therefore, every SAT proof bundle must contain:
> 1. `formula.cnf` (Dimacs format)
> 2. `proof.lrat` (LRAT proof file)
> 3. `encoding_spec.md` (Formal description mapping boolean variables to bit-vector arithmetic)
> 4. `problem_metadata.json`

---

## 5. Uncertified SMT Policy

SMT solvers (e.g., Z3) are immensely useful for candidate generation and abstract domain concretization. However, because SMT proof logging formats (Alethe/CVC5) are non-standard across solvers:
* **Uncertified SMT Policy:** No raw output from an SMT solver may be classified as a "Certificate".
* An SMT solver result is treated purely as a search heuristic until its output model is translated into an exact `collatz-cert` JSON certificate or verified by `collatz-verify`.

---

## 6. Lean 4 Formalization Bridge

To provide the ultimate level of mathematical certainty, certificates generated by `collatz-cert` can be imported into the Lean 4 interactive theorem prover:

1. **Lean Certificate Parser Macro:** Parses `descent_v1.json` strings directly into Lean 4 terms.
2. **Lean Standard Theorem:**
```lean
theorem descent_certificate_holds 
  (val_word : List Nat) (A_k k : Nat) (r c B : Nat)
  (h_inv : (3^k * r + c) % 2^A_k = 0)
  (h_contract : 2^A_k > 3^k)
  (h_thresh : B = c / (2^A_k - 3^k) + 1) :
  ∀ (n : Nat), n % 2^A_k = r ∧ n ≥ B → odd_collatz_map_k val_word n < n := by
  decide
```
3. Only key high-value certificates are exported to Lean 4, keeping formal proof maintenance minimal and focused.
