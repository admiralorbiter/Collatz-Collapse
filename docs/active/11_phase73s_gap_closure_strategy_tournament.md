# Phase 7.3S — Gap-Closure Strategy Tournament & Shortcut Audit

**Status**: **ACTIVE SPECIFICATION (SCHEMA 4.0.0)**  
**Preceding Phase**: Phase 7.3D-R2A (`VERIFIED_LOCAL_ZERO_LIFT_TRANSITION` & `SOUND_ACCELERATED_UNRANKED`)  
**Completed Sub-Phases**:
- `Phase 7.3S.0`: Active specifications and evidence report models (`[COMPLETE]`).
- `Phase 7.3S.1`: Corpus & Theorem-Backed Generators (`[COMPLETE / FROZEN]`).
  - Badges: `VERIFIED_CORPUS_SCHEMA`, `GHOST_ATLAS_COORDINATE_AUDIT_PASSED`, `VERIFIED_PERIODIC_FIXED_POINT_GENERATOR`, `VERIFIED_BOUNDED_EXTREMAL_SEARCH_INFRASTRUCTURE`, `VERIFIED_EXTREMAL_TABLE_H36_J2`.
- `Phase 7.3S.1B`: Zero-Tail & Large-Gap Stress Audit (`[COMPLETE]`).
  - Badges: `VERIFIED_ZERO_TAIL_INVARIANT_AUDIT`, `VERIFIED_SINGLE_GAP_NO_CHEAP_PRECISION`, `FIXED_PERIOD_GHOST_ZERO_TAIL_BOUND_PROVED`, `NO_CHEAP_PRECISION_OBSERVED_THROUGH_J32`.
- `Phase 7.3S.1C`: Proof Boundaries & Discrepancy Resolution (`[COMPLETE / FROZEN]`).
  - Badges: `DISCREPANCY_2_8_0_RESOLVED`, `CORRECTED_DEPTH3_J8_TABLE`, `EVENTUALLY_PERIODIC_PREIMAGE_FORMULA_VERIFIED`, `PLAIN_MODULAR_CEGAR_UNSOUNDNESS_DEMONSTRATED`.
- `Phase 7.3S.2A`: Exact Precision-Aware Cylinder Calculus & Backward Fixed Point (`[ACTIVE / IMPLEMENTED]`).
  - Badges: `EXACT_PRECISION_AWARE_CYLINDER_CALCULUS`, `BACKWARD_ZERO_LIFT_FIXED_POINT_PROBE_J32`, `REACHABLE_ENDPOINT_INTERSECTION_ANALYSIS`.
  - Unsoundness Proof: Plain $D \bmod 2^m$ abstraction collapses quotient $n = (D - C_j)/M_j$. Replaced by Exact Backward Cylinder Transformer $\text{Pre}_j([s]_m)$ and Coinductive Greatest Fixed Point $\mathcal{E}_\infty = \nu X. \Phi_J(X)$.

---

## 1. Executive Summary & Strategy Architecture

### 1.1 Structural Foundations
1. **Eventual-Zero Endpoint Reduction Theorem**: $\Lambda_{u,j} = 0 \iff D_u \equiv C_j \pmod{M_j}$, yielding exact return map $D_{uj} = F_j(D_u) = \frac{Q_j D_u + \beta_j}{M_j}$.
2. **Eventually Periodic Ghost Theorem**: For $u w^\omega$, ghost $z_w^* = -p/q \implies x = -P/R$ with odd $R = q Q_u$. Preimage $x \in \mathbb{Z}_2$, proving $Z(u w^r) = O_{u,w}(1)$ and uniform bound $Z \le K(U,P,J)$.
3. **Exact Cylinder Calculus**:
   - Forward: $\text{Post}_j([r]_p) = [D_j + Q_j t]_{p - B_j}$ (Acyclic in $p$ as $p \to p - B_j < p$).
   - Backward: $\text{Pre}_j([s]_m) = \left[ C_j + M_j \cdot (Q_j^{-1} (s - D_j) \bmod 2^m) \right]_{m + B_j}$ (Exact inverse $\text{Post}_j(\text{Pre}_j([s]_m)) = [s]_m$, lies 100% inside branch $j$ guard).
4. **Coinductive Formulation**: Endpoints supporting infinite zero-lift paths form greatest fixed point $\mathcal{E}_\infty = \nu X. \Phi_J(X)$ where $\Phi_J(X) = \bigcup_{j=0}^J \text{Pre}_j(X)$. Target: prove $I \cap \mathcal{E}_\infty = \emptyset$ for canonical initial endpoints $I$.

---

## 2. Staged Roadmap

```text
Phase 7.3S.1C: Proof Boundaries & Discrepancy Resolution (FROZEN)
Phase 7.3S.2A: Exact Precision-Aware Cylinder Calculus & Backward Fixed Point (ACTIVE)
Phase 7.3S.2B: Coinductive Predecessor Tree & Reachability Exclusion (QUEUED)
Phase 7.3S.3: Deterministic Rational Rigidity Orbit Probe
Phase 7.3S.4: Invariant Falsification on Enriched Corpus
Phase 7.3S.5: Evidence Report Synthesis & Human Strategy Selection
```
