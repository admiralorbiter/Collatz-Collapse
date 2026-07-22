# Phase 7X Implementation and Experiment Plan

## 1. Milestone title

**Phase 7X — Affine Interaction and Ultrametric Fuel Pilot**

## 2. Primary objective

Build and compare three sound abstractions over a bounded macrostep library:

1. destination-aware raw residue refinement;
2. cross-form ultrametric cancellation refinement;
3. path-first symbolic transducer refinement.

Measure which representation best suppresses spurious SCCs while preserving machine-verifiable path semantics.

## 3. Proposed Rust modules

```text
crates/collatz-affine/src/
    macrostep_data.rs
    affine_interaction.rs
    path_composition.rs

crates/collatz-abstract/src/
    precision_debt.rs
    ultrametric_state.rs
    cancellation_refinement.rs
    interaction_graph.rs

crates/collatz-cegar/src/
    phase7x_discovery.rs
    path_first_graph.rs
    near_commuting_search.rs
    common_center_clusters.rs

crates/collatz-cert/src/
    phase7x_schemas.rs

crates/collatz-verify/src/
    verify_affine_interaction.rs
    verify_cross_form_transition.rs
    verify_common_center_family.rs
    verify_path_cylinder.rs
```

## 4. Core data structures

```rust
pub struct MacrostepData {
    pub valuation_word: Vec<u32>,
    pub odd_steps_k: u32,
    pub total_valuation_a: u32,
    pub multiplier_a: BigUint,   // 3^k
    pub divisor_b: BigUint,      // 2^A
    pub affine_constant_c: BigInt,
    pub fixed_form_d: BigInt,    // b - a
}
```

```rust
pub struct AffineInteraction {
    pub left_word_id: String,
    pub right_word_id: String,
    pub delta: BigInt,
    pub delta_v2: Option<u32>,   // None means delta = 0
    pub common_fixed_point: bool,
}
```

```rust
pub struct CancellationGuard {
    pub reference_word_id: String,
    pub transition_word_id: String,
    pub kappa: u32,
    pub required_total_v2: u32,
    pub normalized_residue: BigUint,
    pub normalized_modulus_exponent: u32,
}
```

## 5. Experiment 7X.1 — Generic identity verification

### Goal

Verify the generic algebra on every pair in a bounded word library.

### Checks

For each \(p,q\):

- recompute \(a,b,c,d\);
- recompute \(\Delta\);
- verify the commutator identity on symbolic coefficients;
- verify the cross-form identity;
- verify \(\Delta=0\) exactly matches equality of reduced fixed points.

### Output

```text
affine_interaction_matrix.json
```

### Success criterion

Rust verifier and Python oracle independently agree on every pair.

---

## 6. Experiment 7X.2 — Cross-form cylinder recovery

### Goal

Recover broad and exact source cylinders for \(q\) using a reference form \(H_p\).

### Procedure

For each ordered pair \(p\ne q\):

1. Compute:
   \[
   a_qH_p(n)+\Delta_{p,q}.
   \]
2. Solve:
   \[
   v_2(\cdot)\ge A_q
   \]
   for the broad cylinder.
3. Solve:
   \[
   v_2(\cdot)\ge A_q+1
   \]
   for the exact cylinder.
4. Compare to direct modular inversion.

### Success criterion

The two methods produce the same canonical residues for every ordered pair.

### Benchmark expectation

For \(w_1,w_2\), recover:

\[
w_2\text{ broad}: 11\bmod32,
\qquad
w_2\text{ exact}: 43\bmod64.
\]

---

## 7. Experiment 7X.3 — Interaction spectrum

### Goal

Identify structurally interesting word pairs.

### Library parameters

Freeze:

- maximum word length \(K_{\max}\);
- maximum individual valuation \(V_{\max}\);
- optional total valuation bound \(A_{\max}\);
- primitive-word policy;
- exact generation predicate.

### Metrics

For every pair:

- \(|\Delta|\);
- \(v_2(\Delta)\);
- shared fixed point;
- expansion/contracting kind;
- exact cylinder overlap;
- finite path compatibility.

### Reports

- common-center clusters;
- top near-commuting pairs;
- strongly separated pairs;
- pairs with large cancellation-depth variance.

---

## 8. Experiment 7X.4 — Common-center arbitrary switching

### Goal

Search for nontrivial families with:

\[
\Delta_{p,q}=0
\]

for all pairs.

### For each cluster

1. Reduce the common fixed point to \(C/D\).
2. Construct:
   \[
   H(n)=Dn-C.
   \]
3. Verify:
   \[
   b_pH(F_p(n))=a_pH(n)
   \]
   for every family member.
4. Classify the zero case.
5. Generate arbitrary-switching finite-fuel claims where sound.

### Negative outcome

If all clusters are trivial rotations, powers, or equivalent segmentations, report that classification explicitly.

---

## 9. Experiment 7X.5 — Ultrametric abstraction benchmark

### Goal

Compare state explosion and precision requirements.

### Run A: Raw residue CEGAR

Use destination-aware refinement only.

### Run B: Cancellation abstraction

Use:

- valuation regions relative to \(\kappa\);
- normalized odd residue on resonance surfaces;
- minimal concrete cylinder witness.

### Run C: Hybrid transducer

Use:

- control state;
- precision debt;
- carry/cancellation register;
- path-first validation.

### Metrics

| Metric | Run A | Run B | Run C |
|---|---:|---:|---:|
| States | | | |
| Edges | | | |
| Max residue exponent | | | |
| Refinement rounds | | | |
| Spurious SCCs | | | |
| Verified path cylinders | | | |
| Phase 6D collapses | | | |
| Unresolved components | | | |

---

## 10. Experiment 7X.6 — Path-first graph construction

### Goal

Prevent edgewise-valid but path-incompatible SCCs.

### Algorithm

1. Enumerate bounded path words.
2. Compose the affine map exactly.
3. Compute exact path cylinder.
4. Check intermediate guards.
5. Canonicalize source and target states.
6. Insert a summarized graph edge only after validation.

### Compare against

The ordinary edge-first graph over the same library and bounds.

### Success criterion

Every reported closed walk has a nonempty path certificate.

---

## 11. Experiment 7X.7 — Ranking synthesis

For each verified recurrent component:

1. Test Phase 6D composite reduction.
2. Test common-center fuel.
3. Generate exact cross-form transitions.
4. Attempt lexicographic ranking.
5. Attempt cancellation countdown.
6. Attempt multiphase ranking.
7. Attempt SCT only after universal relations are certified.

### Required result statuses

- `TERMINATED_PHASE6D`
- `TERMINATED_COMMON_CENTER`
- `TERMINATED_LEXICOGRAPHIC`
- `TERMINATED_MULTIPHASE`
- `SOUND_UNRANKED`
- `PATH_INCOMPATIBLE`
- `REFINEMENT_LIMIT`
- `NO_RECURRENT_COMPONENT`

---

## 12. Python oracle

Extend the independent oracle to recompute:

- macrostep affine data;
- \(\Delta\);
- \(v_2(\Delta)\);
- fixed-point equality;
- broad and exact resonance cylinders;
- path compositions;
- path-cylinder congruences;
- common-center family identities;
- ranking relations.

The oracle must not import Rust-generated derived fields as trusted facts.

## 13. Lean 4 plan

Suggested file:

```text
lean/Phase7XAffineInteraction.lean
```

Formalize in this order:

1. macrostep affine definitions;
2. oddness of \(d_p\) and \(c_p\);
3. same-form identity;
4. cross-form identity;
5. commutator identity;
6. broad divisibility equivalence;
7. exact-word parity equivalence;
8. common-center family theorem;
9. selected benchmark instantiations.

Ranking theorems should be added only after a concrete verified component exists.

## 14. Mutation tests

Reject at least:

- wrong sign in \(\Delta\);
- reversed composition order;
- using sign-normalized \(L\) without transforming \(\Delta\);
- claiming \(\Delta=0\) for unequal reduced fixed points;
- replacing \(A_q+1\) with \(A_q\) for exact-word forcing;
- treating \(v_2(0)\) as zero;
- dropping the resonance equality case;
- accepting a bounded test as a universal relation;
- merging states with different cancellation residues;
- constructing an SCC from edgewise validity alone;
- alphabet manifest omissions;
- hardcoded oracle statuses.

## 15. Deliverables

```text
docs/phase7x/
    README.md
    affine_interaction_theory.md
    ultrametric_abstraction.md
    experiment_report.md

certificates/phase7x/
    alphabet_manifest.json
    affine_interaction_matrix.json
    common_center_clusters.json
    near_commuting_pairs.json
    discovery_outcome.json

reports/phase7x/
    abstraction_comparison.md
    path_first_vs_edge_first.md
    claims_summary.md
```
