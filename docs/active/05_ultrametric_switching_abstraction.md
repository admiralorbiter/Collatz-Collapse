# Ultrametric Switching Abstraction
## A Proposed Replacement for Raw Residue Refinement Alone

## 1. Design goal

Construct a sound abstract transition system for a bounded macrostep library that records:

- the active control state;
- valuation of selected fixed-point forms;
- whether the state is below, above, or on an interaction threshold;
- normalized odd residues only on cancellation surfaces;
- destination precision debt;
- exact path-cylinder provenance.

The abstraction must never use a history tag as a substitute for missing arithmetic information.

## 2. Reference-form coordinates

Choose a finite feature library:

\[
\mathcal H=\{H_{p_1},\ldots,H_{p_m}\}.
\]

For each feature and concrete integer \(n\), record:

\[
x_i=v_2(H_{p_i}(n)),
\]

unless \(H_{p_i}(n)=0\), which must be a separate state.

For a transition labeled \(q\), each feature has threshold:

\[
\kappa_{i,q}=v_2(\Delta_{p_i,q}).
\]

The transition behavior depends on the comparison:

\[
x_i<\kappa_{i,q},
\qquad
x_i=\kappa_{i,q},
\qquad
x_i>\kappa_{i,q}.
\]

## 3. Abstract feature state

A first implementation may use:

```rust
enum ValuationRegion {
    Zero,
    Below { exact_v2: u32 },
    Resonant {
        kappa: u32,
        odd_residue: u64,
        residue_bits: u32,
    },
    Above { lower_bound: u32 },
}
```

A product state:

```rust
struct UltrametricState {
    control_state: u32,
    reference_features: Vec<FeatureRegion>,
    precision_debt: u32,
    concrete_guard: Option<CanonicalCylinder>,
}
```

The concrete cylinder remains available as a proof witness, but feature coordinates drive classification and merging.

## 4. Cancellation residue

On the resonance layer:

\[
H_p(n)=2^\kappa U,
\qquad U\text{ odd}.
\]

For a transition \(q\), define:

\[
\delta=\Delta_{p,q}/2^\kappa.
\]

The additional cancellation depth is:

\[
\gamma=v_2(a_qU+\delta).
\]

Only enough bits of \(U\) to determine the required \(\gamma\) should be retained.

If the destination requires broad \(q\)-integrality:

\[
\gamma\ge A_q-\kappa.
\]

For exact \(q\)-forcing:

\[
\gamma\ge A_q+1-\kappa.
\]

This creates a demand-driven normalized-residue refinement.

## 5. Proposed transition categories

Every feature transition should be classified as one of:

1. **Deterministic descent**
   \[
   x' = x-A_q.
   \]

2. **Threshold reset**
   \[
   x' = \kappa-A_q.
   \]

3. **Cancellation-controlled**
   \[
   x' = \kappa+\gamma-A_q.
   \]

4. **Infeasible**
   The divisibility required for a valid macrostep cannot hold.

5. **Zero-form exceptional**
   The concrete state lies at the rational fixed point of the reference form.

These are exact semantic categories, not guessed SCT relations.

## 6. Sound merging rule

Two concrete states may be merged only if they agree on:

- automaton control meaning;
- all feature regions relevant to outgoing transitions;
- cancellation residues at the bit depth demanded by those transitions;
- path-scope and positivity requirements.

States must not be merged merely because they share one coarse residue or one previous word.

## 7. Subsumption

A state \(S_1\) subsumes \(S_2\) only if:

\[
\gamma(S_2)\subseteq\gamma(S_1),
\]

and every outgoing abstract transition of \(S_1\) is sound for all concrete states in \(S_2\).

Suggested subsumption checks:

- cylinder inclusion;
- valuation interval inclusion;
- matching cancellation residue;
- equal control semantics;
- no loss of exact-word forcing.

## 8. Path-first semantics

Before inserting a closed walk into an SCC report:

1. Compose the full affine path.
2. Compute its broad and exact cylinders.
3. Intersect with every intermediate guard.
4. Prove the intersection is nonempty.
5. Verify every intermediate image.
6. Record the final canonical state.

A graph SCC is a summary of verified path semantics, not the source of path truth.

## 9. Interaction graph versus trajectory graph

Maintain two separate graphs.

### Affine interaction graph

Vertices are macrostep words. Edge \(p\to q\) stores:

- \(\Delta_{p,q}\);
- \(\kappa_{p,q}\);
- common-center status;
- broad/exact resonance congruences.

This graph is independent of reachability.

### Semantic trajectory graph

Vertices are concrete abstract states. Edges represent verified macrostep transitions.

This separation prevents algebraic similarity from being mistaken for trajectory composability.

## 10. Near-commuting target heuristic

Rank word pairs by:

\[
\kappa_{p,q}=v_2(\Delta_{p,q}).
\]

Interpretation:

- \(\kappa=\infty\): common center;
- large finite \(\kappa\): low-bit near commutation and potentially deep cancellation;
- small \(\kappa\): shallow separation.

Use \(\kappa\) only for search prioritization. Verification always uses the exact \(\Delta\).

## 11. Candidate ranking systems

Try proof systems in this order:

1. Same-form finite fuel.
2. Common-center arbitrary-switching fuel.
3. Direct lexicographic decrease on valuation features.
4. Cancellation-depth countdown.
5. Multiphase ranking.
6. SCT closure over universally proved relations.
7. Richer monotonicity constraints.

Do not encode bounded resets as ordinary weak SCT edges.

## 12. Failure classifications

A candidate SCC may fail because:

- an edge is destination-imprecise;
- the path cylinder is empty;
- the cycle is only 2-adically realizable;
- the cycle collapses to one composite word;
- two cycles commute or share one center;
- cancellation requires unbounded residue depth;
- no well-founded relation is found.

Each failure should produce a distinct artifact.
