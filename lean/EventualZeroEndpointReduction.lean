import Mathlib.Data.Nat.Basic
import Mathlib.Data.Int.Basic

/--
  Phase 7.3S.1C.0: Eventual-Zero Endpoint Reduction Lemma.

  For any guarded prefix u with endpoint D_u, extending by gap j produces zero lift block
  \Lambda_{u, j} = 0 if and only if D_u \equiv C_j \pmod{M_j}.

  Furthermore, when \Lambda_{u, j} = 0, the next endpoint is given exactly by:
    D_{u j} = D_j + Q_j * \frac{D_u - C_j}{M_j} = F_j(D_u) = \frac{Q_j D_u + \beta_j}{M_j}.
-/

theorem eventual_zero_endpoint_reduction_lemma
  (D_u C_j M_j Q_j β_j D_j : ℤ)
  (hM_pos : M_j > 0)
  (h_affine : M_j * D_j = Q_j * C_j + β_j)
  (h_zero_lift : D_u % M_j = C_j % M_j) :
  ∃ (k : ℤ), D_u = C_j + k * M_j ∧
  (Q_j * D_u + β_j) / M_j = D_j + k * Q_j := by
  sorry
