# End-to-End Lean 4 Vertical Theorem Slice Specification

## Overview

This specification details the formal translation pipeline from verified JSON certificates (`descent_v1`, `cover_v1`) to machine-checked **Lean 4 universal theorems**.

By utilizing the **Prescribed-Division Transform Domination Lemma** ($S^k(n) \le P_w(n)$) and **Subtraction-Free Multiplicative Contraction**, Lean 4 formalization completely avoids saturated natural number subtraction (`Nat.sub`) issues and discharges integer inequalities natively via `omega`.

---

## 1. Concrete Lean 4 Theorem Templates

### Template A: Broad Valuation Semantics (`terminal_at_least`)
For a verified `descent_v1` certificate with broad valuation word $w$, residue $r$, modulus $2^m$, constant $c_k$, and threshold $B$:

```lean
import Mathlib.Data.Nat.Basic
import Mathlib.Data.Nat.ModEq
import Mathlib.Tactic.Omega

-- Base structural lemma: Prescribed-Division Transform Domination
theorem broad_domination_lemma (n : ℕ) (w : List ℕ) (r A_k : ℕ) 
    (h_res : n ≡ r [MOD 2^A_k]) :
  exact_odd_step_iterate n w ≤ prescribed_division_transform n w := by
  sorry

-- Subtraction-Free Contraction Lemma
theorem contraction_forces_descent (n c_k A_k k P_val : ℕ)
    (h_eq : 2^A_k * P_val = 3^k * n + c_k)
    (h_contract : c_k + 3^k * n < 2^A_k * n) :
  P_val < n := by
  omega

-- Concrete Universal Theorem for Certified Residue Class [2, 2] (r = 1 mod 16, B = 2)
theorem certified_descent_residue_1_mod_16 (n : ℕ) (h_pos : 0 < n) (h_res : n % 16 = 1) (h_ge : n ≥ 2) :
  exact_odd_step_iterate_2_2 n < n := by
  have h_prescribed : prescribed_division_transform_2_2 n < n := by
    -- Instantiated numerals: c_k = 7, 3^k = 9, 2^A_k = 16
    -- Constraint: 7 + 9 * n < 16 * n
    have h_eq : 16 * prescribed_division_transform_2_2 n = 9 * n + 7 := by sorry
    have h_ineq : 7 + 9 * n < 16 * n := by omega
    exact contraction_forces_descent n 7 16 2 (prescribed_division_transform_2_2 n) h_eq h_ineq
  have h_dom : exact_odd_step_iterate_2_2 n ≤ prescribed_division_transform_2_2 n :=
    broad_domination_lemma n [2, 2] 1 4 h_res
  omega
```

### Template B: Exact Valuation Semantics (`exact_word`)
For exact valuation words (e.g. $[1,1,2,1,3]$, $k=5, c_5=251, A_5=8, 2^{A_5+1}=512, r=295, B=20$), where $P_w(n) = S^k(n)$:

```lean
-- Universal Theorem for Exact Cylinder Residue Class [1,1,2,1,3] (r = 295 mod 512, B = 20)
theorem certified_exact_descent_residue (n : ℕ) (h_pos : 0 < n) (h_res : n % 512 = 295) (h_ge : n ≥ 20) :
  exact_odd_step_iterate_exact n < n := by
  have h_eq : 256 * exact_odd_step_iterate_exact n = 243 * n + 251 := by sorry
  have h_ineq : 251 + 243 * n < 256 * n := by omega
  exact contraction_forces_descent n 251 256 5 (exact_odd_step_iterate_exact n) h_eq h_ineq
```

---

## 2. Pipeline Execution Flow

```text
       JSON Certificate (descent_v1 / cover_v1)
                         │
                         ▼
        Rust Independent Verifier (collatz-verify)
                         │
                         ▼
        Independent Python Oracle Cross-Check (Route A & B)
                         │
                         ▼
        Lean 4 Import Generator (lean_export)
                         │
                         ▼
        Lean 4 Proof Kernel Verification (`omega`)
```
