-- Formal Lean 4 Theorem Suite for Phase 7.3A Generic Affine Interaction Algebra
import Mathlib.Data.Int.Basic
import Mathlib.Data.Rat.Basic
import Mathlib.Tactic.Ring
import Mathlib.Tactic.Omega
import Mathlib.Tactic.Linarith

namespace Phase73A

-- 1. Same-Form Eigenidentity over Z
theorem same_form_identity (aP bP cP dP n : ℤ) (hdP : dP = bP - aP) :
    dP * (aP * n + cP) - bP * cP = aP * (dP * n - cP) := by
  subst hdP
  ring

-- 2. Cross-Form Identity over Z
theorem cross_form_identity (aP bP cP aQ bQ cQ dP dQ Delta n : ℤ)
    (hdP : dP = bP - aP) (hdQ : dQ = bQ - aQ) (hDelta : Delta = dP * cQ - dQ * cP) :
    dP * (aQ * n + cQ) - bQ * cP = aQ * (dP * n - cP) + Delta := by
  subst hdP
  subst hdQ
  subst hDelta
  ring

-- 3. Affine Commutator Identity over Z
theorem affine_commutator_identity (aP bP cP aQ bQ cQ dP dQ Delta : ℤ)
    (hdP : dP = bP - aP) (hdQ : dQ = bQ - aQ) (hDelta : Delta = dP * cQ - dQ * cP) :
    (aP * cQ + bQ * cP) - (aQ * cP + bP * cQ) = -Delta := by
  subst hdP
  subst hdQ
  subst hDelta
  ring

-- 4. Delta Antisymmetry over Z
theorem delta_antisymmetric (dP cP dQ cQ : ℤ) :
    (dQ * cP - dP * cQ) = -(dP * cQ - dQ * cP) := by
  ring

-- 5. Common-Center Cross Product Equality Criterion over Z
theorem delta_zero_iff_cross_products_equal (dP cP dQ cQ : ℤ) :
    (dP * cQ - dQ * cP = 0) ↔ (dP * cQ = dQ * cP) := by
  omega

-- 6. Same Rational Fixed Point Criterion over Q
theorem same_rational_center_iff_delta_zero (cP dP cQ dQ : ℚ) (hdP : dP ≠ 0) (hdQ : dQ ≠ 0) :
    (cP / dP = cQ / dQ) ↔ (dP * cQ - dQ * cP = 0) := by
  constructor
  · intro h
    have h_cross : cP * dQ = cQ * dP := (div_eq_div_iff hdP hdQ).mp h
    linarith
  · intro h
    have h_cross : dP * cQ = dQ * cP := by linarith
    have h_cross2 : cP * dQ = cQ * dP := by linarith
    exact (div_eq_div_iff hdP hdQ).mpr h_cross2

end Phase73A
