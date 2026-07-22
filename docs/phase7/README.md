# Phase 7X Research Packet
## Affine Interaction, Ultrametric Cancellation, and Symbolic Switching

This packet defines a new research direction for the Collatz Research Workbench before expanding the Phase 7 semantic graph further.

The central shift is:

> Stop treating deeper residue refinement as the only way to recover lost path information.  
> Model how affine macrosteps interact through exact fixed-point forms, commutator constants, 2-adic cancellation depth, and precision debt.

The packet separates established algebraic identities from conjectural research directions.

## Documents

1. `00_research_direction_overview.md`  
   Motivation, primary research questions, expected outcomes, and how this direction fits after Milestone 7.2.

2. `01_affine_interaction_theory.md`  
   Exact algebra for macrostep composition, precision debt, affine commutators, cross-linear-form identities, resonance, and exact-word forcing.

3. `02_ultrametric_switching_abstraction.md`  
   A proposed abstraction based on valuation regions and cancellation residues instead of raw residue splitting alone.

4. `03_phase7x_implementation_and_experiments.md`  
   Rust-oriented implementation plan and a sequence of falsifiable experiments.

5. `04_certificate_schemas_and_verification.md`  
   Proposed proof-object schemas and independent verification requirements.

6. `05_claims_registry_and_review_gates.md`  
   Candidate claims, theorem-status language, negative outcomes, and milestone gates.

7. `06_phase72_handoff_and_migration.md`  
   How to preserve the valid Phase 7.2 work while replacing the parts that overfit a manually chosen residue graph.

## Recommended milestone name

**Phase 7X — Affine Interaction and Ultrametric Fuel Pilot**

This should be treated as a research insertion before a broader Phase 7.3 graph expansion.

## Current benchmark pair

The initial benchmark remains:

\[
w_1=[1,1,2],\qquad
w_2=[1,2,2].
\]

Their affine data are:

| Word | \(k\) | \(A\) | \(a=3^k\) | \(b=2^A\) | \(c\) | \(d=b-a\) |
|---|---:|---:|---:|---:|---:|---:|
| \(w_1\) | 3 | 4 | 27 | 16 | 19 | \(-11\) |
| \(w_2\) | 3 | 5 | 27 | 32 | 23 | \(5\) |

The interaction constant is:

\[
\Delta_{1,2}=d_1c_2-d_2c_1=-348,
\qquad
v_2(\Delta_{1,2})=2.
\]

This one integer explains both:

- why the affine maps fail to commute; and
- why cross-fixed-point valuations require a cancellation layer.

## Trust rule

Search code may propose patterns. A claim enters the verified layer only if an independent verifier recomputes all arithmetic from the valuation words and proves the quantified divisibility or inclusion statement.
