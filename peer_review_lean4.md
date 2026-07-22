# Phase 5.5 Implementation Plan & Gate Checklist: Peer Review
## Perspective: Formal Verification & Proof Assistant (Lean 4) Expert

### 1. Exact Lean 4 Contraction Lemma Formulation
The current structural base lemmas presented in `docs/05_certificates_and_verify.md` require refinement to fully align with standard Lean 4 natural number arithmetic (avoiding subtraction of naturals where `a - b = 0` if `b > a`).

**Current Formulation:**
$$(2^{A_k} - 3^k) \cdot (n_0 - 1) \ge c_k \implies S^k(n_0) < n_0$$

**Critique & Recommendation:**
In Lean 4, natural number subtraction saturates at zero. It is much more robust to express inequalities strictly via addition and multiplication to avoid pathological edge cases with zero. 

The contraction condition for $S^k(n_0) < n_0$ should be established directly using the affine step equation $2^{A_k} S^k(n_0) = 3^k n_0 + c_k$. 

For $S^k(n_0) \le n_0 - 1$ (which means $S^k(n_0) < n_0$ in integers):
$2^{A_k} (n_0 - 1) \ge 2^{A_k} S^k(n_0) = 3^k n_0 + c_k$
$2^{A_k} n_0 - 2^{A_k} \ge 3^k n_0 + c_k$
$(2^{A_k} - 3^k) n_0 \ge c_k + 2^{A_k}$

However, the user prompt specifically specifies the formulation to audit as:
`c_k < (2^{A_k} - 3^k) * n_0 => P_w(n_0) < n_0`
This is a stronger and cleaner formulation. In Lean 4, this should be written without subtraction on the LHS if possible, i.e., $c_k + 3^k n_0 < 2^{A_k} n_0$.

**Floor Threshold Corollary:**
The corollary is given as:
`n_0 >= floor(c_k / (2^{A_k} - 3^k)) + 1 => c_k < (2^{A_k} - 3^k) * n_0`

**Critique & Recommendation:**
In Lean 4, `floor` and division can introduce unnecessary complexities in `Nat`. We can translate the `floor` condition purely multiplicatively. The statement $n_0 \ge \lfloor c_k / (2^{A_k} - 3^k) \rfloor + 1$ is mathematically equivalent to $n_0 \cdot (2^{A_k} - 3^k) > c_k$.
Lean 4 Formulation recommendation:
```lean
lemma floor_threshold_soundness (n₀ c_k A_k k : Nat) (h_contract : 3^k < 2^{A_k}) :
  c_k < (2^{A_k} - 3^k) * n₀ → S_k n₀ < n₀
```
And its corollary mapping the integer floor threshold $B = \lfloor c_k / (2^{A_k} - 3^k) \rfloor + 1$:
```lean
lemma floor_threshold_implies_descent (n₀ c_k A_k k B : Nat) (h_contract : 3^k < 2^{A_k}) 
  (h_B : B * (2^{A_k} - 3^k) > c_k) :
  n₀ ≥ B → c_k + 3^k * n₀ < 2^{A_k} * n₀
```
This perfectly isolates the floating-point/division logic in the search phase from the pure integer proof in the verifier.

### 2. `cover_v1` Manifest Example Fix
**Issue Identified:**
In `docs/05_certificates_and_verify.md`, the `cover_v1` JSON manifest example uses `"valuation_word": [1, 1]` with `"total_twos": 2`. This is invalid as a contracting leaf because for $A_k = 2$ and $k = 2$, we have $2^{A_k} = 4$ and $3^k = 9$. Thus, $2^2 \not> 3^2$, violating the Multiplicative Contraction invariant (Step 4 of the Verifier Invariants).

**Resolution:**
The invalid `[1, 1]` leaf must be replaced with a valid contracting leaf, specifically `[2, 2]`.
For `[2, 2]`:
- $k = 2$
- $A_k = 4$
- $2^4 = 16 > 9 = 3^2$ (Valid Contraction)
- Affine Constant $c_k = 5$
- Starting Residue: $n_0 = 3 \pmod{16}$

**Action Item:** Update the `docs/05_certificates_and_verify.md` to reflect this valid leaf in the JSON schema example.

```json
{
  "schema_version": "cover_v1",
  "total_leaves": 1,
  "max_modulus_exponent": 4,
  "total_scaled_measure": "1",
  "is_exact_cover": true,
  "merkle_root_hash": "4a8b1c9d8e7f6a5b",
  "leaves": [
    {
      "valuation_word": [2, 2],
      "total_twos": 4,
      "starting_residue": "3",
      "modulus_exponent": 4,
      "valuation_semantics": "terminal_at_least"
    }
  ]
}
```

### 3. Independent Python Reference Oracle Architecture (`scripts/reference_oracle.py`)

As a Formal Verification Expert, relying solely on a Rust verifier (`collatz-verify`) introduces a single point of failure (compiler bugs, subtle integer overflow bugs if `num-bigint` has edge cases, etc.). The introduction of `scripts/reference_oracle.py` is an excellent strategic choice.

**Recommended Architecture for the Python Oracle:**
1. **Zero-Dependency Philosophy:** The Python oracle should use ONLY the Python Standard Library (`json`, `sys`, `math`, `argparse`). Python natively supports arbitrary-precision integers, eliminating the need for external math crates.
2. **Independent Parsing:** It must parse the `descent_v1`, `tail_descent_v1`, and `cover_v1` schemas entirely independently of the Rust serialization definitions.
3. **Exact Invariant Reproduction:**
   The script should implement the 6 verifier invariant checks defined in the specs:
   - Calculate $A_k$ from `valuation_word` and verify `modulus_exponent`.
   - Calculate $c_k$ using a native Python loop and `assert computed_c_k == int(cert['constant'])`.
   - Perform the modular inverse calculation natively (e.g., `pow(3**k, -1, 2**A_k)` available in Python 3.8+) and verify `starting_residue`.
   - Verify contraction: `assert 2**A_k > 3**k`.
   - Verify threshold: `B = (c_k // (2**A_k - 3**k)) + 1` and check `descent_threshold == str(B)`.
   - **Exception Simulation:** Loop through $0 < e < B$ where $e \equiv n_0 \pmod{2^m}$ and natively simulate the Collatz steps, asserting $S^k(e) < e$ or reaches 1.

**Outcome:**
Providing this independent Python oracle serves as a definitive cross-validation anchor. If a theorem formulated in Lean 4 aligns with the mathematical specification, and both the Rust verifier and the independent Python oracle independently arrive at `VALID` for a given certificate, the structural integrity of the proof is practically unassailable short of a full machine-checked formalization.
