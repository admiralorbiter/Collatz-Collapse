# 08. Research Paths Forward

## 1. Recommended order of work

The project should proceed on three parallel tracks, with different risk levels.

### Track A — Validate and publish the current theorem

This is the highest-confidence and highest-immediate-value path.

### Track B — Generalize the structured-side theorem

Extend negative-potential elimination to broader symbolic classes.

### Track C — Develop the unstructured/high-complexity side

Pursue deterministic flattening and cross-adic incompatibility.

## 2. Track A: theorem validation

### A1. External combinatorics review

Ask a Sturmian/combinatorics-on-words expert to audit:

- use of the cube-gap theorem;
- length-32 template completeness;
- selector graph completeness;
- treatment of rotations and primitive words.

### A2. Collatz and 2-adic review

Ask a Collatz specialist to audit:

- the canonical subsystem definition;
- semantic source-cylinder correctness;
- H.1 ordinary-source reduction;
- distinction from standard valuation and parity coding.

### A3. Formal-methods review

Ask a formal verification expert to audit:

- Lean axiom output;
- binding of generated graph data to theorem statements;
- reproducibility and mutation testing;
- trusted computing base.

## 3. Track B1: abstract syndetic-power theorem

Formulate a reusable theorem:

> If a symbolic path class has syndetic bounded-period powers, a complete finite semantic selector graph, safe worst-case precision weights, and a uniformly negative potential, then no path in the class is realizable by one positive ordinary source.

This would make the Sturmian theorem a corollary rather than an isolated graph computation.

## 4. Track B2: general two-gap Sturmian embeddings

The current theorem covers `{1,2}`. Investigate ordered embeddings `(a,b)`.

### Questions

- Does increasing a gap always increase precision cost enough to make the graph more negative?
- Can edge weights be ordered monotonically in `a,b`?
- Does the potential `Phi(v)=-(Q_v-M_v)` work symbolically?
- Is `{1,2}` a worst case for a broad region?

A monotonicity theorem would be far more valuable than checking many embeddings individually.

## 5. Track B3: primitive substitutions

Durand’s finite-derived-sequence theorem suggests a finite graph for each primitive substitutive sequence.

### Initial targets

- Fibonacci;
- Thue–Morse;
- period doubling;
- Tribonacci.

### Pipeline

1. prove the derived graph complete;
2. attach selector and phase state;
3. prove exact or worst-case edge weights;
4. compute a candidate potential;
5. certify every edge and cycle;
6. issue sequence-specific elimination or identify a surviving obstruction.

## 6. Track B4: broader low-complexity families

Potential classes:

- linearly recurrent words;
- automatic sequences;
- S-adic systems;
- Arnoux–Rauzy and episturmian words;
- bounded numbers of return words;
- periodic words with sparse defects.

The key research question is not factor complexity alone. It is whether the class admits a finite or controlled semantic graph with safe precision weights.

## 7. Track C1: deterministic flattening

The long-term high-complexity target is a theorem of the form:

\[
\text{not close to any low-description-complexity family}
\Longrightarrow
\text{uniform conditional Fourier or }L^2\text{ contraction}.
\]

Promising tools include:

- projective transfer operators;
- Fourier decay modulo powers of two;
- additive energy and sum-product ideas;
- Dobrushin contraction;
- nonstationary cocycle singular values;
- Tao-style renewal transport;
- entropy growth under noncommuting affine branches.

The proof must be prefix-uniform or otherwise pointwise, not merely averaged over all words.

## 8. Track C2: cross-adic elimination

For an exponent or return code, track simultaneously:

- least 2-adic source representative;
- least 3-adic endpoint representative;
- real affine drift.

A genuine positive orbit must satisfy all three. Seek a theorem that no infinite path can maintain:

- bounded or subexponential 2-adic source height;
- compatible 3-adic endpoint height;
- noncontracting real behavior.

Recent exponent-code work provides a useful comparison and necessary rate conditions.

## 9. Track C3: path-set dimensions

When structured families are finite-state, use p-adic path-set theory to compute:

- topological entropy;
- Hausdorff dimension;
- closure under arithmetic maps;
- intersections among source and endpoint constraints.

Dimension-zero or entropy-deficit results will not by themselves exclude every integer, but may connect structured class elimination with the Haar/annealed program.

## 10. Research priority matrix

| Direction | Near-term feasibility | Full-Collatz relevance | Publication value |
|---|---:|---:|---:|
| Current Sturmian theorem paper | Very high | Moderate | Very high |
| Abstract syndetic-power theorem | High | High | High |
| Other `{a,b}` Sturmian embeddings | Medium-high | Moderate | High |
| Concrete substitutions | Medium-high | Moderate | High |
| Full low-complexity hierarchy | Medium | High | High |
| Conditional flattening | Low-medium | Very high | Very high |
| Cross-adic elimination | Medium | Very high | High |
| Larger witness searches | High | Low alone | Moderate |

## 11. Decision criteria

A new phase should begin only if it targets one of:

- a general theorem;
- a complete class elimination;
- a counterexample to a proposed bridge;
- an explicit reduction of the remaining Collatz obstruction.

Avoid phases whose main deliverable is only a larger finite search or a denser atlas.
