# Phase 7.3 Research Packet: Affine Interaction, Ultrametric Cancellation, and Symbolic Switching

This packet defines the mathematical, symbolic, and verification engine for **Phase 7.3**, integrated directly following the completion of **Phase 7.2**.

The central shift is:

> Phase 7.2 produced a sound noncommuting guarded-branching core ($u, v$), but raw residue states do not yet provide a compact switching invariant.  
> Rather than relying on raw residue splitting alone, Phase 7.3 models how closed-walk macrosteps interact through exact fixed-point forms, commutator constants, 2-adic cancellation depth, symbolic return-language dynamics, path-complete graph Lyapunov rankings, and disjunctive transition invariants.

## Documents

1. `00_research_direction_overview.md`  
   Motivation, primary research questions, symbolic 2-adic shift conjugacy, cancellation gate $x=6, U \equiv 1 \pmod{16}$, and Phase 7.3 sub-phase roadmap.

2. `01_affine_interaction_theory.md`  
   Exact algebra for macrostep composition, precision debt, affine commutators, cross-linear-form identities, resonance, and exact-word forcing.

3. `01_exact_q1_register_machine.md`  
   Exact $Q_1$ integer register machine ($n = 32k + 7$), generic quotient return theorem ($\eta_p$), $u/v$ transition rules, positive image progressions vs 2-adic target cylinders, and inverse-guard recursions.

4. `02_ultrametric_switching_abstraction.md`  
   Phase 7.3B-2 ultrametric cancellation machine ($L_u(n) = 32(11k+3)$, $x = v_2(L_u(n)) \ge 5$, $x' = x-4, U' = 27U$ under $u$; $x' = \gamma - 3, U' = \frac{729U+87}{2^\gamma}$ under $v$).

5. `phase73a_closeout.md`  
   Formally frozen closeout report for Phase 7.3A (Generic Affine Interaction & Cross-Form Theorem Kernel).

6. `phase73b_1_closeout.md`  
   Formally frozen closeout report for Phase 7.3B-1 (Exact $Q_1$ Quotient Reference Machine & Finite Language Uniqueness Theorem).

7. `phase73b_2_closeout.md`  
   Formally frozen closeout report for Phase 7.3B-2 (Ultrametric Cancellation Register Machine & 5-Stage Refinement Ladder).

8. `04_certificate_schemas_and_verification.md`  
   Proof-object schemas (`macrostep_data_v1`, `affine_interaction_v1`, `cross_form_cylinder_recovery_v1`, `quotient_register_transition_v1`, `guarded_return_classification_v1`, `phase73a_verification_report_v1`, `phase73b_verification_report_v1`).

7. `05_claims_registry_and_review_gates.md`  
   Candidate claims, theorem-status language, negative outcomes, and milestone review gates (Gates 7.3A through 7.3E).

8. `06_phase72_handoff_and_migration.md`  
   Documentation of the verified Phase 7.2 noncommuting branching core and entry criteria for Phase 7.3.

## Benchmark Levels

### Primary Phase 7.3 Switching Benchmark
The primary switching system at $Q_1 = \{ n \equiv 7 \pmod{32} \}$ uses based closed walks:
$$u=[1,1,2],\qquad v=[1,1,2,1,2,2]$$

| Word | $K$ | $A$ | $a=3^K$ | $b=2^A$ | $c$ | $d=b-a$ | $Q_1$ Return Guard | Quotient Rule $k'$ |
|---|---:|---:|---:|---:|---:|---:|---|---|
| $u$ | 3 | 4 | 27 | 16 | 19 | $-11$ | $k \equiv 7 \pmod{16}$ | $(27k + 3)/16$ |
| $v$ | 6 | 9 | 729 | 512 | 881 | $-217$ | $k \equiv 61 \pmod{512}$ | $(729k + 75)/512$ |

$$\Delta_{u,v} = d_u c_v - d_v c_u = (-11)(881) - (-217)(19) = -5568 = -2^6 \cdot 87, \qquad \kappa_{u,v} = v_2(\Delta_{u,v}) = 6$$

## Trust Rule

Search code may propose patterns. A claim enters the verified layer only if an independent verifier recomputes all arithmetic from the valuation words and proves the quantified divisibility or inclusion statement.
