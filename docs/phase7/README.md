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

3. `02_ultrametric_switching_abstraction.md`  
   A proposed abstraction based on valuation regions, cancellation residues, and affine register machines instead of raw residue splitting alone.

4. `03_phase7x_implementation_and_experiments.md`  
   Rust/Python/Lean implementation plan and a sequence of falsifiable experiments for Phase 7.3A through 7.3E.

5. `04_certificate_schemas_and_verification.md`  
   Proposed proof-object schemas (`based_switching_core_v1`, `finite_switching_language_v1`, `path_complete_ranking_v1`, `disjunctive_transition_invariant_v1`, `language_growth_report_v1`).

6. `05_claims_registry_and_review_gates.md`  
   Candidate claims, theorem-status language, negative outcomes, and milestone review gates (Gates 7.3A through 7.3E).

7. `06_phase72_handoff_and_migration.md`  
   Documentation of the verified Phase 7.2 noncommuting branching core and entry criteria for Phase 7.3.

## Benchmark Levels

### Edge-Level Benchmark Pair
\[
w_1=[1,1,2],\qquad w_2=[1,2,2], \qquad \Delta_{w_1,w_2}=-348, \qquad v_2(\Delta)=2
\]
Used for generic formula and identity validation.

### Primary Phase 7.3 Switching Benchmark
The primary switching system at $Q_1$ uses based closed walks:
\[
u=w_1=[1,1,2],\qquad v=w_1 w_2=[1,1,2,1,2,2]
\]

| Word | \(k\) | \(A\) | \(a=3^k\) | \(b=2^A\) | \(c\) | \(d=b-a\) |
|---|---:|---:|---:|---:|---:|---:|
| \(u\) | 3 | 4 | 27 | 16 | 19 | \(-11\) |
| \(v\) | 6 | 9 | 729 | 512 | 881 | \(-217\) |

The interaction constant and 2-adic valuation depth are:
\[
\Delta_{u,v} = d_u c_v - d_v c_u = (-11)(881) - (-217)(19) = -5568 = -2^6 \cdot 87, \qquad \kappa_{u,v} = v_2(\Delta_{u,v}) = 6
\]

This constant measures the exact order defect between compositions:
\[
b_u b_v \big(F_{uv}(n) - F_{vu}(n)\big) = 8192 \left(-\frac{5568}{8192}\right) = -5568
\]

## Trust Rule

Search code may propose patterns. A claim enters the verified layer only if an independent verifier recomputes all arithmetic from the valuation words and proves the quantified divisibility or inclusion statement.

