# Collatz Research Workbench — Master Synthesis

**Snapshot:** 2026-07-23  
**Purpose:** Single-file compilation of the project knowledge base.

> This document synthesizes project-reported theorems and external literature. Artifact-level verification remains the responsibility of the frozen proof package and independent reviewers.


---

# 00. Executive Overview

## 1. Current scientific position

The project has developed a formally specified induced Collatz system in which finite return words determine exact affine maps, exact source residue cylinders, and semantically checked paths. The work has progressed through four major layers:

1. **Exact finite return calculus.** Finite gap words define affine return maps and exact source cylinders.
2. **Annealed renewal geometry.** Under Haar measure, the return process has an exact geometric gap law and renewal structure; finite experiments through depth eight agree closely with these predictions.
3. **Quenched pointwise calculus.** A fixed ordinary source is characterized by eventual stabilization of compatible least source representatives. Periodic cores, core switching, and precision accounting are exact.
4. **Aperiodic class elimination.** A complete finite graph and negative potential reportedly eliminate all semantically valid Sturmian gap itineraries over `{1,2}`.

The strongest responsible summary is:

> The project has crossed the distributional-to-pointwise barrier for one complete, minimally complex aperiodic class inside a rigorously defined Collatz return subsystem.

## 2. What has been proved in the project package

According to the frozen package, the project proves:

- exact canonical return formulas and semantic guard cylinders;
- a noncommuting guarded branching core;
- a 2-adic source spine and shell oracle;
- exact Haar renewal identities for the ambient return process;
- projective source compatibility and the ordinary-integer stabilization criterion;
- rational periodic cores and exact error transport;
- non-realizability of positive periodic and eventually periodic return itineraries;
- an exact core interaction determinant and four-case switching law;
- a semantic bridge from future periodic agreement to 2-adic depth;
- a complete finite Sturmian transition graph for gap alphabet `{1,2}`;
- a worst-case edge-weight certificate and uniform negative potential;
- the resulting Sturmian gap-itinerary elimination theorem.

## 3. What remains open

The full Collatz conjecture would require at least two further bridges.

### 3.1 Subsystem coverage

A complete reduction must establish precisely that every hypothetical positive Collatz counterexample yields an infinite sustaining path in the canonical subsystem. If this implication is conditional or partial, the current theorem remains a subsystem theorem.

### 3.2 Exhaustion of arbitrary aperiodic paths

A surviving path could have:

- a gap alphabet other than `{1,2}`;
- unbounded gaps;
- linear but non-Sturmian complexity;
- substitutive, automatic, or S-adic structure;
- sparse defects in long periodic regions;
- high symbolic complexity and no finite structured description.

The long-term program must prove that every sustaining path is either:

- **structured enough** for a finite graph and negative-potential certificate; or
- **unstructured enough** for deterministic conditional flattening or another pointwise anti-concentration theorem.

## 4. Novelty in one paragraph

Most individual ingredients—2-adic Collatz conjugacy, affine cycle formulas, rational periodic points, Fine–Wilf periodic overlap, Sturmian return words, path-set graphs, and Lyapunov potentials—have substantial prior literature. The likely novelty lies in their combination inside the specific canonical return subsystem: the weighted semantic cylinders, the core-interaction determinant as a switch-depth invariant, the semantic-depth theorem, the normalized precision ledger, and the complete Sturmian `{1,2}` negative-potential elimination certificate.

## 5. Readiness assessment

### Publication readiness

The work appears close to a focused computer-assisted theorem paper, subject to independent artifact review and careful comparison with recent Collatz/Sturmian preprints.

### Full Collatz readiness

The work is not close enough to justify a percentage estimate. It has established a credible method and one nontrivial class theorem, but the remaining path space is vastly larger.

### Best current action

Freeze and externally audit the Sturmian theorem package while extracting the abstract theorem behind it:

> A symbolic class with syndetic bounded-period powers, a complete finite selector graph, edge weights dominating physical precision change, and a uniformly negative graph potential is not realizable by one positive ordinary source.


---

# 01. Research History and Evolution

## 1. Initial computational objective

The project began as a computationally exact exploration of Collatz return patterns. The early emphasis was on:

- representing accelerated odd-to-odd behavior by gap words;
- calculating exact source residue classes;
- pruning impossible branches;
- generating independently verifiable certificates;
- using Rust for exhaustive search and Lean for formal identities.

The original hope was that increasing depth and stronger sieves might reveal a direct contradiction or a finite obstruction.

## 2. Discovery of a noncommuting guarded core

A major early milestone was a verified branching state with based closed walks such as

\[
u=[1,1,2], \qquad v=[1,1,2,1,2,2],
\]

for which both concatenations had valid positive-integer cylinders but the affine actions did not commute. This established that the subsystem was not merely a collection of independent scalar cycles: it contained genuine noncommuting symbolic dynamics.

This shifted the research question from isolated cycles to the dynamics of a guarded affine semigroup.

## 3. Source geometry: spine and shells

The source classes were found to lie near a single 2-adic spine

\[
C_\infty=-\frac{320}{2673}\in\mathbb Z_2,
\]

with exact valuations

\[
v_2(C_k-C_\infty)=1+4k.
\]

Each guard occupies an eight-bit shell around the spine. This produced a fast shell oracle and clarified that the branch family is highly organized rather than randomly scattered through residue space.

## 4. Haar renewal and the ensemble phase

Under normalized Haar measure on a valid source cylinder, the quotient coordinate is Haar and the affine successor is Haar because the multiplier is odd. The conditional gap law became

\[
\Pr(J=j)=\frac{15}{16^{j+1}},
\]

with one-return measure `1/480` and formal renewal measure

\[
\mu(E_r)=480^{-r}.
\]

This motivated exhaustive depth experiments.

### U7

The project found 25 double-zero witnesses, close to the Haar-scale expectation, and no triple witness.

### U8

The preregistered depth-eight run found:

- first-return and double-return counts close to Haar predictions;
- the first certified triple-zero witness;
- no fourth return, which was expected to be too rare for the search depth to be informative.

The key methodological lesson was that ensemble agreement does not settle any one infinite path.

## 5. The pointwise turn

The project then asked the decisive question:

> What property distinguishes a symbolic path generated by one fixed ordinary integer from a generic 2-adic or Haar path?

The answer was the projective source representative. An infinite path defines compatible residues modulo increasing powers of two. One ordinary nonnegative integer realizes the path exactly when the least representatives eventually stabilize—or equivalently, when the newly exposed lift blocks are eventually zero.

This reframed the target from probability to source-height divergence.

## 6. Periodic cores

The first triple-zero witness was found to agree for 29 low bits with the rational 2-adic fixed point

\[
\xi_0=-\frac{26}{217}.
\]

The exact valuation explained why three repetitions succeeded and the fourth failed:

\[
\left\lfloor\frac{29}{9}\right\rfloor=3.
\]

For a general return block `v`, the project introduced

\[
\xi_v=-\frac{\beta_v}{Q_v-M_v}
\]

and proved exact error transport

\[
v_2(F_v^r(D)-\xi_v)=v_2(D-\xi_v)-rB_v.
\]

Since every nonempty core is a negative rational in the ordinary real order, no positive state can realize a purely periodic or eventually periodic return itinerary.

## 7. Core switching

The interaction determinant

\[
\Gamma_{v,w}=(Q_v-M_v)\beta_w-(Q_w-M_w)\beta_v
\]

was recognized as the exact measure of:

- core separation;
- affine noncommutativity;
- precision surviving a core switch.

The project proved inherited, reset, resonant, and exact-core switch cases and built a telescoping precision ledger.

## 8. Semantic depth and symbolic overlap

A load-bearing bridge linked arithmetic depth to actual future symbolic agreement. The valuation relative to a periodic core is controlled by the weighted length of the common future prefix. For periodic cores, this connected the determinant valuation to weighted longest common prefix length; Fine–Wilf then bounded incompatible periodic overlap.

This unified:

\[
\text{symbolic overlap}
\leftrightarrow
\text{2-adic proximity}
\leftrightarrow
\text{precision survival}.
\]

## 9. Sturmian elimination

The external theorem of Bell, Schulz, and Shallit guarantees syndetic bounded-period cubes in every Sturmian word. The project used this to construct a finite 52-phase selector graph for gap alphabet `{1,2}`.

After addressing graph completeness, right-censoring, period-extension invariance, worst-case edge aggregation, and ordered embeddings, the project obtained a potential certificate with final slack 60. Summation forces the shadow precision negative, contradicting positive ordinary realization.

This was the first complete aperiodic class elimination produced by the framework.

## 10. Retrospective lessons

### Productive decisions

- insisting on exact integer arithmetic;
- maintaining theorem-status badges;
- separating semantic validity from affine algebra;
- preregistering finite experiments;
- treating contradictory results as reasons to revise the map;
- searching for the pointwise obstruction rather than extending averages indefinitely.

### Important corrected errors

The project caught and repaired several serious issues:

- fixed-point sign;
- 27-bit versus exact 29-bit agreement;
- numerator/denominator convention for the affine map;
- commutator denominator;
- a false `+1` in nonresonant switching;
- Fine–Wilf off-by-one;
- primitive necklace counts;
- right-censored gain underestimation;
- overstatement of graph completeness and class scope.

The final theorem is more credible because these corrections were documented rather than hidden.


---

# 02. Canonical Return System and Core Calculus

## 1. Canonical branch parameters

For gap symbol `j >= 0`, the project uses

\[
M_j=2^{9+4j},\qquad B_j=9+4j,\qquad Q_j=3^{6+3j}.
\]

Associated source and endpoint constants `C_j`, `D_j`, and affine constant `beta_j` define the branch map

\[
F_j(D)=\frac{Q_jD+\beta_j}{M_j}.
\]

The frozen convention is:

```text
CANONICAL_RETURN_CONVENTION_V1
F_v(D) = (Q_v * D + beta_v) / M_v
M_v = 2^B_v
Q_v = 3^E_v
d_v = Q_v - M_v
xi_v = -beta_v / d_v
A_v(D) = d_v * D + beta_v
Gamma(v,w) = d_v * beta_w - d_w * beta_v
F_v(D) - xi_v = (Q_v / M_v) * (D - xi_v)
F_w(F_v(D)) - F_v(F_w(D)) = -Gamma(v,w) / (M_v * M_w)
```

The zero-block fingerprint is

\[
F_0(342)=487.
\]

## 2. Word composition

A finite word `v=(h_1,...,h_k)` determines a composed affine map

\[
F_v(D)=\frac{Q_vD+\beta_v}{M_v},
\]

where `M_v` is a power of two and `Q_v` is odd. The source-cylinder recursion gives a unique residue class for each semantically valid prefix.

The distinction between two levels is essential:

- **Affine validity:** the rational affine formula is defined.
- **Semantic validity:** every intermediate guard and branch choice is valid.

All pointwise theorems require semantic validity, not merely a composed formula.

## 3. Rational periodic cores

The unique 2-adic fixed point of `F_v` is

\[
\xi_v=-\frac{\beta_v}{Q_v-M_v}.
\]

Because `Q_v > M_v > 0` and `beta_v > 0`,

\[
\xi_v<0
\]

as an ordinary rational number, although `xi_v` is a valid 2-adic integer because its denominator is odd.

The error transports exactly:

\[
F_v(D)-\xi_v=\frac{Q_v}{M_v}(D-\xi_v),
\]

and therefore

\[
v_2(F_v^r(D)-\xi_v)=v_2(D-\xi_v)-rB_v.
\]

Each repetition consumes exactly `B_v` bits of agreement.

## 4. Integer primitive form

Define

\[
A_v(D)=(Q_v-M_v)D+\beta_v.
\]

Since `Q_v-M_v` is odd,

\[
v_2(D-\xi_v)=v_2(A_v(D)).
\]

This avoids rational arithmetic in certificates.

## 5. Periodic and eventually periodic elimination

If an ordinary positive state followed `v^infinity`, it would have to lie in all nested repetition cylinders and therefore equal `xi_v`. But `xi_v < 0`, contradiction.

The same applies after a finite prefix: a positive state cannot enter the negative rational core of an eventually periodic tail.

Thus, according to the project theorem package:

- no positive purely periodic return itinerary is realizable;
- no positive eventually periodic return itinerary is realizable.

## 6. Core interaction determinant

For two blocks `v,w`, define

\[
\Gamma_{v,w}=d_v\beta_w-d_w\beta_v,
\qquad d_v=Q_v-M_v.
\]

Then

\[
\xi_v-\xi_w=\frac{\Gamma_{v,w}}{d_vd_w},
\]

so

\[
v_2(\xi_v-\xi_w)=v_2(\Gamma_{v,w}).
\]

The affine commutator is

\[
F_w(F_v(D))-F_v(F_w(D))
=-\frac{\Gamma_{v,w}}{M_vM_w}.
\]

Therefore:

\[
\Gamma_{v,w}=0
\iff
\xi_v=\xi_w
\iff
F_v\circ F_w=F_w\circ F_v
\]

at the affine-formula level.

## 7. Exact switch identity

The integer core errors satisfy

\[
d_vA_w(D)=d_wA_v(D)+\Gamma_{v,w}.
\]

Let

\[
s=v_2(A_v(D)),\qquad \kappa=v_2(\Gamma_{v,w}).
\]

Then:

- **Same core:** `Gamma=0`, so the depth is unchanged.
- **Inherited:** `s < kappa`, so `v_2(A_w(D))=s`.
- **Reset:** `s > kappa`, so `v_2(A_w(D))=kappa`.
- **Resonant:** `s=kappa`, so normalized odd terms cancel and the new depth is `kappa+g`, with `g>=1` or infinity under exact cancellation.

For positive ordinary states, exact landing on a negative core is impossible.

## 8. Semantic-depth bridge

Arithmetic proximity is meaningful only when tied to the actual future itinerary. The project reports a theorem of the form

\[
H_L\le v_2(A_v(D))<H_L+B_{\text{first mismatch}},
\]

where `L` is the number of future symbols agreeing with `v^infinity` and `H_L` is the weighted precision of that common prefix.

For two periodic phase sequences `x=v^infinity`, `y=w^infinity`, the analogous theorem bounds

\[
v_2(\Gamma_{v,w})
\]

between the weighted common-prefix depth and the next branch increment.

This is the key bridge between symbolic combinatorics and 2-adic arithmetic.


---

# 03. Annealed Renewal and Experimental Program

## 1. Ambient Haar renewal law

For a first gap `j`, the branch precision is

\[
B_j=9+4j.
\]

The Haar mass of its source cylinder is `2^{-B_j}`. Summing over all gaps gives

\[
\sum_{j\ge0}2^{-(9+4j)}=\frac1{480}.
\]

Conditioned on a return, the gap distribution is

\[
p_j=480\,2^{-B_j}=\frac{15}{16^{j+1}}.
\]

The fixed-fiber affine successor has odd slope, so it preserves Haar measure. This yields an exact ambient renewal law:

\[
\Pr(J_1=j_1,\dots,J_r=j_r\mid E_r)
=\prod_{i=1}^r\frac{15}{16^{j_i+1}},
\]

and

\[
\mu(E_r)=480^{-r}.
\]

This is an **ambient Haar theorem**, not automatically a theorem about deterministic endpoint ensembles or individual ordinary orbits.

## 2. Exact code-length identity

Because

\[
p_h=480\,2^{-B_h},
\]

we have

\[
B_h=\log_2(480)-\log_2 p_h.
\]

For a block `v=(h_1,...,h_k)`,

\[
B_v=k\log_2(480)-\log_2\prod_{i=1}^kp_{h_i}.
\]

This is an exact renewal code-length identity. It becomes a compression theorem only after proving decodability, bounded multiplicity, and net bit saving.

## 3. Depth-seven experiment

The project reports approximately 5.38 million cumulative words through depth seven.

Observed results included:

- first-return count close to Haar expectation;
- 25 double-return witnesses, again close to Haar scale;
- no triple return, consistent with the small expected value;
- all initial subtrees represented;
- no proof of deterministic independence or convergence to Haar.

## 4. Depth-eight preregistered experiment

At exact depth eight, the search covered `9^8 = 43,046,721` words. The preregistered results included:

- first-return count inside the frozen prediction range;
- double-return count inside the frozen prediction range;
- pooled gap frequencies close to the geometric Haar law;
- one first triple-zero witness;
- no fourth return, which had expectation far below one.

The correct interpretation is:

> finite-depth preregistered support for Haar-scale renewal behavior.

It is not a proof that deterministic endpoint measures converge to Haar.

## 5. First triple witness

The first triple-zero witness was

```text
[8, 7, 6, 3, 5, 0, 5, 1]
```

with three successful zero-gap successors and a failed fourth guard.

The endpoint agrees with

\[
\xi_0=-\frac{26}{217}
\]

for exactly 29 low bits:

\[
v_2(D_u-\xi_0)=29.
\]

Because `B_0=9`, this gives exactly

\[
\left\lfloor\frac{29}{9}\right\rfloor=3
\]

repetitions, with two residual bits.

This observation triggered the periodic-core program.

## 6. Annealed versus quenched

### Annealed question

Does the endpoint ensemble at depth `d` become equidistributed modulo fixed powers of two?

A representative target is

\[
\left|\mu_d([a]_m)-2^{-m}\right|\le C_m\rho_m^d.
\]

### Quenched question

Can one bound every conditional distribution after every reachable prefix, except for a classified structured family?

This is much stronger and is the relevant pointwise direction.

The Sturmian theorem is important because it is a quenched result: it eliminates every path in one infinite symbolic class, not merely almost every member.

## 7. Relation to Tao and Kontorovich–Sinai

Kontorovich and Sinai developed a statistical structure theorem for accelerated Collatz data and random-walk-like residues. Tao proved that almost all Collatz orbits attain almost bounded values in logarithmic density using approximate transport and high-frequency characteristic-function estimates on a 3-adic cyclic group.

These works support the renewal and flattening side of the project but also illustrate the central limitation: ensemble control does not automatically control one exceptional deterministic orbit.

## 8. Future role of the empirical program

The finite searches should now be used to:

- discover candidate structured obstructions;
- preregister and test transfer-operator predictions;
- identify worst states for formal analysis;
- test proposed graph abstractions;
- search for counterexamples to intermediate conjectures.

They should not be used as a substitute for the pointwise theorem chain.


---

# 04. Pointwise Bridge and Projective Sources

## 1. Infinite paths define inverse systems

Let an infinite semantically valid gap path have source precision levels

\[
H_0<H_1<H_2<\cdots,\qquad H_n\to\infty.
\]

Each prefix determines a source residue

\[
r_n\pmod{2^{H_n}}.
\]

Semantic extension implies compatibility:

\[
r_{n+1}\equiv r_n\pmod{2^{H_n}}.
\]

Thus the path determines a point of the inverse limit

\[
\varprojlim\mathbb Z/2^{H_n}\mathbb Z\cong\mathbb Z_2.
\]

## 2. Least source representatives

Let

\[
0\le R_n<2^{H_n}
\]

be the least nonnegative representative of `r_n`.

Compatibility implies a unique lift block `lambda_{n+1}` with

\[
R_{n+1}=R_n+\lambda_{n+1}2^{H_n},
\]

where

\[
0\le\lambda_{n+1}<2^{H_{n+1}-H_n}.
\]

Therefore `R_n` is nondecreasing.

## 3. Ordinary-integer characterization

A compatible inverse-limit point is represented by one ordinary nonnegative integer `N` exactly when the least representatives eventually stabilize:

\[
\exists K\ \forall n\ge K:\ R_n=N.
\]

Equivalent forms are:

\[
\sup_nR_n<\infty,
\]

and

\[
\lambda_n=0\quad\text{for all sufficiently large }n.
\]

This gives the central H.1 equivalence:

\[
\boxed{
\text{one fixed ordinary source}
\iff
\text{bounded least representatives}
\iff
\text{eventual stabilization}
\iff
\text{eventual zero lift blocks}.
}
\]

## 4. Why this is the key pointwise reduction

A generic 2-adic path always has compatible residues. The difficult condition is not local consistency; it is that the infinite 2-adic source has only finitely many nonzero binary digits and is therefore an ordinary nonnegative integer.

This is the exact distinction between:

- a locally valid 2-adic or “ghost” trajectory;
- a path generated by one positive ordinary source.

## 5. Source-height target

The natural global target becomes:

\[
R_n(w)\to\infty
\]

for every infinite sustaining path `w`, or at least

\[
\sup_nR_n(w)=\infty.
\]

Either conclusion rules out realization by one ordinary nonnegative integer.

## 6. Relation to recent exponent-code work

A recent 2026 exponent-code preprint independently defines least 2-adic start representatives for accelerated Collatz codes and proves that a code generated by one fixed positive integer has vanishing normalized source-residue growth. It also introduces a combined 2-adic, 3-adic, and real diagnostic.

This is close in spirit to the H.1 reduction but is formulated for standard exponent codes rather than the project’s canonical guarded return subsystem.

## 7. Subsystem coverage caveat

The H.1 theorem is internal to the canonical path model. A full Collatz reduction additionally requires:

\[
\boxed{
\text{hypothetical Collatz counterexample}
\Longrightarrow
\text{infinite semantically valid canonical sustaining path}.
}
\]

This implication must be stated and proved separately. It should never be hidden inside the abstract inverse-limit theorem.

## 8. Precision ledger

When a path shadows a periodic core, repetitions consume bits. Core switches may preserve, reset, or increase the remaining depth. For selected core segments the finite ledger has the form

\[
s_{N+1}
=s_1
-\sum_i r_iB_{v_i}
-\sum_{i\in\mathrm{Reset}}(t_i-\kappa_i)
+\sum_{i\in\mathrm{Resonant}}g_i.
\]

Only resonances can replenish precision. The Sturmian proof packages this accounting into a finite graph with normalized transition weights.


---

# 05. Sturmian Gap-Itinerary Elimination Theorem

## 1. The theorem

> **Sturmian Gap-Itinerary Elimination for the Canonical Collatz Return Subsystem.**  
> No positive ordinary integer `N > 0` can realize an infinite semantically valid canonical return path whose gap itinerary is Sturmian over the binary gap alphabet `{1,2}`.

This is a project-reported computer-assisted theorem. Its scope is the frozen canonical return subsystem and the exact selector, graph, and certificate package.

## 2. What “Sturmian” means

A binary infinite word is Sturmian if it is aperiodic and has minimal possible factor complexity:

\[
p(n)=n+1
\]

for every positive integer `n`.

Sturmian words are highly structured but never eventually periodic. Every factor has exactly two return words.

## 3. External combinatorial input

Bell, Schulz, and Shallit proved in 2024 that in every Sturmian word:

- the distance between consecutive cube-ending positions is at most 10;
- cubes of period at most 5 suffice;
- the bound 10 is optimal.

This supplies syndetic bounded-period powers.

Over a binary alphabet, primitive words of periods 1 through 5 form:

- 14 primitive necklaces up to cyclic rotation;
- 52 phase words when every rotation is retained.

## 4. Selector and graph reduction

The canonical semantic selector is:

- symbolic-first;
- prefix-measurable;
- primitive-normalized;
- phase-aware;
- deterministic;
- allowed to return no core;
- equipped with extension-state reporting.

The project reports that every infinite Sturmian selector execution over gap alphabet `{1,2}` maps to an infinite walk in a frozen 52-phase graph.

The graph includes both ordered embeddings:

- binary `0 -> 1`, `1 -> 2`;
- binary `0 -> 2`, `1 -> 1`.

## 5. Semantic depth

For state `D` and target core `v`, the depth

\[
s=v_2(A_v(D))
\]

is tied to the number of future gap symbols agreeing with `v^infinity`. This turns future periodic continuation into an exact or bounded arithmetic quantity.

## 6. Normalized transition weights

A raw resonance gain can be arbitrarily extended by further repetitions of the target core. The project normalizes this by charging the corresponding target repetition cost. Period-extension invariance ensures that continuing by full target periods does not change the normalized transition value.

For each graph edge, the certificate stores the worst-case value

\[
W(e)=\max_\tau W_\tau
\]

across all local templates producing that transition.

The project reports a theorem that every physical precision change is dominated by the graph weight:

\[
\Delta s_{\mathrm{physical}}\le W(e).
\]

## 7. Negative potential

For the frozen graph, a node potential `Phi` reportedly satisfies

\[
W(e)+\Phi(t(e))-\Phi(s(e))\le -60
\]

for every directed edge.

Summing along a graph walk of length `N` yields

\[
\sum_{i=1}^NW(e_i)
\le
-60N+\Phi(s_0)-\Phi(s_N).
\]

Because the graph is finite, `Phi` is bounded. Combining graph domination and boundary corrections gives

\[
s_N\le s_0+C-60N,
\]

which becomes negative for large `N`.

But `s_N` is a 2-adic valuation of a nonzero integer and therefore cannot be negative. For positive states, exact landing on a negative core is also impossible.

This contradiction proves the theorem.

## 8. Frozen certificate data

Project-reported final data:

- 52 phase nodes;
- 14 primitive necklaces;
- 2,482 directed worst-case edges;
- minimum integer slack: 60;
- exact or independently checked maximum cycle mean reported as `-60`;
- two ordered-embedding graph artifacts;
- full SHA-256 digests:
  - `(1,2)`: `e93f8fc30780fa06b45e2e6c5fec08e9997dff54b0175e71eca9ab7a01a8e50c`
  - `(2,1)`: `3a22e0d48807b09601d6134407003d30ce04712d72ad841ab5512b1ab2ab6a24`

## 9. What the theorem does not say

It does not eliminate:

- all Sturmian parity vectors;
- all odd-to-odd valuation words;
- all two-gap alphabets `{a,b}`;
- all bounded-gap paths;
- all linearly recurrent words;
- all primitive substitutions;
- all possible Collatz counterexamples.

The distinction among parity words, standard accelerated exponent codes, and canonical return-gap words must remain explicit.

## 10. Why the result matters

This is the first project theorem that eliminates a complete, genuinely aperiodic symbolic class through a pointwise contradiction. It demonstrates that the project’s structured-side program is viable:

\[
\text{finite symbolic representation}
+\text{worst-case precision weights}
+\text{negative potential}
\Rightarrow
\text{ordinary unrealizability}.
\]


---

# 06. Novelty and Literature Map

## 1. Novelty summary

The project’s likely novelty is not one isolated formula. It is a new synthesis of:

- an exact guarded Collatz return subsystem;
- weighted 2-adic source cylinders;
- rational periodic-core shadowing;
- an affine interaction determinant controlling switch depth;
- a pointwise source stabilization criterion;
- semantic-depth equivalences;
- finite symbolic graph reduction;
- worst-case precision weights and negative potentials;
- formal and independently regenerated certificates.

A targeted literature search did not reveal the exact Sturmian `{1,2}` return-gap elimination theorem. This supports a claim of **potential novelty**, not a definitive priority claim.

## 2. Classical 2-adic Collatz dynamics

### Bernstein and Lagarias

Bernstein and Lagarias proved that the standard 2-adic Collatz map is topologically and metrically conjugate to the binary shift. This makes parity itineraries exact 2-adic coordinates and establishes strong mixing in the 2-adic extension.

**Relation to this project:** foundational background for treating symbolic paths as 2-adic points. The canonical return system is an induced, guarded, weighted subsystem rather than the original parity shift.

## 3. Affine cycle formulas

### Böhm and Sontacchi

Böhm and Sontacchi developed effective rational formulas for candidate cycles of prescribed patterns and lengths.

### Ghost-cycle literature

A 2026 preprint studies unique 2-adic solutions of prescribed Collatz cycle equations, calls them ghost cycles, and proves an obstruction to Presburger-semilinear descriptions of the integrality condition.

**Relation to this project:** rational cores are not novel in isolation. The project’s contribution is the error-transport and switching calculus and the use of ordinary-source stabilization to eliminate whole symbolic families.

## 4. Statistical and almost-all approaches

### Kontorovich and Sinai

Their statistical structure theorem describes residue classes and random-walk-like behavior in generalized Collatz dynamics.

### Tao

Tao proved that almost all Collatz orbits attain almost bounded values in logarithmic density. The proof uses approximate transport, renewal structure, and high-frequency characteristic-function estimates on a 3-adic cyclic group.

**Relation to this project:** the Haar renewal and future flattening programs are close in spirit. The project’s Sturmian theorem is narrower but pointwise rather than almost-all.

## 5. Symbolic coding and parity vectors

### Stérin and related parity-vector work

Finite parity vectors determine exact residue classes and affine behavior. Recent work gives sharp finite counts and density-zero results for bounded-length paradoxical patterns.

### López and Stoll

They study Sturmian words used as parity vectors under the 2-adic Collatz conjugacy and analyze real and 3-adic series associated with those itineraries.

**Critical distinction:** the project’s symbols are canonical return gaps. A paper must include a formal translation table among:

- binary parity vectors;
- odd-to-odd valuation/exponent words;
- canonical return-gap words.

## 6. Combinatorics on words

### Fine–Wilf

If a finite word has periods `p` and `q` for at least `p+q-gcd(p,q)` symbols, it also has period `gcd(p,q)`. The project uses this to cap incompatible periodic overlap.

### Morse–Hedlund

An aperiodic finite-alphabet word has factor complexity at least `n+1`; equality characterizes Sturmian words in the binary case.

### Sturmian return words

Every factor of a Sturmian word has exactly two return words. This strongly constrains recurrent symbolic structure.

### Bell–Schulz–Shallit

Every Sturmian word has cubes of bounded period occurring with uniformly bounded gaps. This is the finite-combinatorial input for the 52-phase graph.

### Durand

A sequence is primitive substitutive exactly when it has finitely many derived sequences. This suggests finite recursive graphs for primitive substitutions.

## 7. Automata and p-adic path sets

### Anashin

A 1-Lipschitz map on `Z_p` is finite-state exactly under a criterion on its van der Put coefficients and p-kernel.

### Abram and Lagarias

Finite graph presentations define p-adic path-set fractals with computable Hausdorff dimensions and strong closure properties.

**Relation to this project:** these frameworks may quantify or classify structured exceptional sets. They do not by themselves establish ordinary-integer unrealizability.

## 8. Graph potentials and Lyapunov ideas

The inequality

\[
W(e)+\Phi(t)-\Phi(s)<0
\]

is a finite-state Lyapunov or subaction certificate. Neighboring fields include:

- path-complete graph Lyapunov functions;
- switched-system stability;
- ergodic optimization;
- maximum cycle mean and difference constraints.

This language may help abstract the Sturmian proof and automate future graph refinements.

## 9. Recent 2026 work requiring close comparison

### Human–LLM Collatz structural preprint

A rapidly evolving 2026 preprint develops multiple reductions, including a Sturmian obstruction and carry-contamination route, but explicitly leaves an orbitwise anti-concentration or weak-mixing condition open.

**Research need:** compare its valuation-word model with the canonical return-gap model. Determine whether the symbolic classes coincide, overlap, or are genuinely distinct.

### Exponent-code 2–3–infinity diagnostic

A July 2026 preprint studies finite exponent codes through real drift, least 2-adic start representatives, and 3-adic endpoint representatives. It proves necessary vanishing normalized residue rates for codes generated by one fixed positive integer.

**Relation:** close conceptual support for the projective-source and future cross-adic program.

### Parity vectors and paradoxical sequences

A May 2026 note proves sharp finite parity-vector density and analytic counts for paradoxical finite sequences, while making no claim toward the full conjecture.

**Relation:** useful finite-level comparison, but not a pointwise infinite-path result.

## 10. Novelty matrix

| Component | Prior-art proximity | Likely novelty |
|---|---:|---:|
| Rational 2-adic fixed point for a prescribed block | High | Low alone |
| Exact affine error transport | Moderate | Moderate in subsystem |
| Core determinant = distance = commutator obstruction | Moderate | Moderate to high |
| Projective source stabilization | Moderate | Moderate in guarded subsystem |
| Weighted semantic-depth theorem | Low direct overlap found | High potential |
| Complete 52-phase Sturmian return-gap graph | Low direct overlap found | High potential |
| Worst-case negative-potential class elimination | Low direct overlap found | High potential |
| Rust–Lean–Python certificate pipeline | Methods overlap exists | Distinctive integration |

## 11. Responsible novelty language

Use:

- “appears to be new”;
- “we found no prior statement of this exact theorem”;
- “to our knowledge, subject to further literature review”;
- “for the canonical return subsystem defined here.”

Avoid:

- “first ever”;
- “solves Sturmian Collatz trajectories”;
- “major advance toward Collatz” without the subsystem qualifier.


---

# 07. Implications, Limitations, and Claim Registry

## 1. Main implications

### 1.1 The pointwise framework works on a nontrivial class

The project no longer merely proposes a structured-versus-mixing dichotomy. It reportedly carries one structured family all the way to contradiction.

### 1.2 Minimal aperiodic complexity is not sufficient for survival

Sturmian words are the least complex aperiodic binary words. Their elimination shows that escaping eventual periodicity is not enough; even highly organized aperiodicity can lose precision too quickly.

### 1.3 Finite graph potentials can certify ordinary unrealizability

The proof architecture can be abstracted beyond Sturmian words:

1. obtain syndetic or otherwise controlled structured windows;
2. build a complete finite selector graph;
3. attach safe worst-case precision weights;
4. find a uniformly negative potential;
5. invoke H.1.

### 1.4 Rational cores are coordinates, not candidate positive cycles

The negative rational cores organize finite shadowing and switch behavior but are excluded as ordinary positive endpoints. They are best viewed as local centers for structured 2-adic behavior.

## 2. Limitations

### 2.1 Subsystem limitation

The theorem applies only to paths in the canonical subsystem. Full relevance depends on a precise coverage theorem for hypothetical counterexamples.

### 2.2 Alphabet limitation

The class theorem currently covers gap alphabet `{1,2}`. It does not cover arbitrary two-symbol embeddings or gaps including zero.

### 2.3 Symbolic-model limitation

Return gaps are not the same object as parity digits or standard accelerated valuations. Any claim of overlap with other Sturmian Collatz work requires a formal coding map.

### 2.4 Complexity limitation

Sturmian words are one low-complexity class. Linear-complexity, substitutive, automatic, S-adic, sparse-defect, and high-complexity words remain.

### 2.5 External validation limitation

Multiple implementations by one research effort reduce software risk but do not replace independent peer review.

## 3. Claim status registry

### Project-reported symbolic/formal theorems

- canonical return convention and fingerprint;
- exact source-cylinder recursion;
- source spine and shell identities;
- periodic-core fixed-point theorem;
- exact error transport;
- periodic and eventually periodic elimination;
- projective source stabilization equivalence;
- core interaction and four-case switch law;
- semantic-depth theorem;
- weighted common-prefix interval;
- period-extension invariance;
- graph path completeness;
- edge domination;
- Sturmian `{1,2}` elimination.

### Ambient Haar theorems

- conditional gap law `15/16^(j+1)`;
- fixed-fiber Haar preservation;
- ambient renewal measure `480^{-r}`.

### Preregistered empirical evidence

- U7 and U8 endpoint counts;
- pooled gap frequencies;
- first E3 witness;
- finite character-decay observations.

### Open conjectures or bridges

- full subsystem coverage of all hypothetical counterexamples;
- arbitrary low-complexity class elimination;
- general two-gap Sturmian embedding theorem;
- deterministic high-complexity flattening;
- cross-adic elimination of all remaining structured paths;
- a complete structured-versus-mixing dichotomy.

## 4. Common overclaims to avoid

Do not state:

- endpoint ensembles converge to Haar, unless separately proved;
- the Sturmian theorem applies to all parity vectors;
- all low-complexity aperiodic paths are eliminated;
- finite graph tests prove a universal class theorem without graph completeness;
- independent internal scripts constitute external validation;
- the result makes the full Collatz conjecture “nearly solved.”

## 5. What would materially strengthen the full program

The following would be major upgrades:

1. a complete counterexample-to-subsystem reduction;
2. a general syndetic-power elimination theorem;
3. a symbolic proof that the potential `Phi=-d` works for broad gap embeddings;
4. elimination of primitive substitutive families through finite derived graphs;
5. a quantitative complexity-to-flattening theorem;
6. a cross-adic theorem coupling 2-adic source growth, 3-adic endpoint growth, and real drift.


---

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


---

# 09. Publication Strategy

## 1. Recommended publication split

The project is too broad for one first paper. A focused series is more credible.

## Paper 1 — Flagship theorem

### Working title

**Sturmian Gap-Itinerary Elimination in a Canonical Collatz Return System**

### Central result

No positive ordinary integer realizes an infinite semantically valid canonical return path with Sturmian gap itinerary over `{1,2}`.

### Suggested structure

1. Introduction and precise scope.
2. Canonical return subsystem.
3. Exact source cylinders and semantic paths.
4. Pointwise source characterization.
5. Rational cores and interaction calculus.
6. State-core semantic-depth theorem.
7. Sturmian bounded-period cube input.
8. Complete finite selector graph.
9. Normalized edge domination.
10. Negative potential and contradiction.
11. Computer-assisted verification.
12. Limitations and open problems.

### What to omit

- most U7/U8 data;
- the full phase history;
- broad speculation about solving Collatz;
- unrelated certificate engines.

## Paper 2 — Renewal geometry and experiments

### Working title

**A 2-Adic Renewal Structure in a Canonical Collatz Return System**

### Content

- source spine and shell structure;
- exact Haar return law;
- fiberwise Haar preservation;
- U7/U8 preregistered experiments;
- E2/E3 witness distribution;
- periodic-core discovery;
- annealed versus quenched limitations.

### Literature context

- Kontorovich–Sinai;
- Tao;
- parity-vector density results;
- p-adic induced maps.

## Paper 3 — Verified computational methodology

### Working title

**Proof-Producing Experimental Mathematics for a Collatz Return System**

### Content

- Rust exact arithmetic;
- Lean theorem hierarchy;
- Python independent regeneration;
- graph and certificate hashes;
- mutation testing;
- preregistration;
- theorem-status ledger;
- lessons from corrected errors.

This could be a software paper, formal-methods paper, or companion artifact.

## Paper 4 — General structured-family theorem

After further work, a stronger paper could present:

- the abstract syndetic-power negative-potential theorem;
- general gap embeddings;
- primitive substitutions;
- several class-elimination corollaries.

## 2. Novelty section for Paper 1

A careful novelty statement should say:

- rational 2-adic points for prescribed patterns are classical;
- Sturmian parity-vector studies already exist;
- bounded-gap cubes in Sturmian words are external prior work;
- the new contribution is the canonical return-gap model, semantic-depth calculus, complete graph reduction, edge domination, and negative-potential class elimination.

## 3. Suggested abstract language

> We define a canonical guarded return subsystem for the accelerated Collatz map and characterize ordinary positive realizability through stabilization of compatible 2-adic source representatives. Each finite return block determines a rational 2-adic core and an exact interaction invariant controlling precision under symbolic switching. We prove that semantic continuation depth is equivalent to weighted 2-adic core depth. Using a theorem of Bell, Schulz, and Shallit on syndetic bounded-period cubes in Sturmian words, we reduce all Sturmian gap itineraries over `{1,2}` to a finite 52-state phase graph. Exact worst-case edge weights admit a uniformly negative potential, forcing the available source precision to become negative along every infinite graph path. Consequently, no positive ordinary integer realizes an infinite semantically valid canonical return path with such a Sturmian gap itinerary. The finite graph and potential are independently regenerated and verified by Rust, Lean, and Python artifacts.

## 4. Reviewer concerns to anticipate

- Is the subsystem natural or engineered to fit the theorem?
- Does semantic path validity exactly match ordinary Collatz behavior?
- Is graph completeness proved or sampled?
- Is the external Sturmian theorem imported correctly?
- Are edge weights true worst-case upper bounds?
- Does the theorem cover parity words or only return gaps?
- Is the finite data bound into Lean, or only checked beside it?
- How independent are the verification implementations?

Address these questions proactively.

## 5. Authorship and presentation

The paper should identify clearly:

- which proofs are conceptual;
- which steps are computer-assisted;
- which external theorem is assumed;
- which finite artifacts are authoritative;
- how the human–AI workflow contributed.

Avoid presenting AI involvement as mathematical authority. Authority comes from proofs, certificates, reproducibility, and review.

## 6. Release sequence

1. Internal release candidate.
2. Clean-room reproduction.
3. Private expert review.
4. Revised technical report.
5. Public preprint and artifact repository.
6. Journal submission.

## 7. Potential venues

Depending on final emphasis:

- number theory/dynamical systems journal;
- experimental mathematics journal;
- symbolic dynamics or combinatorics-on-words venue;
- formalized mathematics or certified-computation venue;
- a combined theorem paper plus archival software release.

Venue choice should follow external expert feedback on perceived mathematical centrality.


---

# 10. Verification, Reproduction, and External Review

## 1. Verification philosophy

The project should distinguish four layers:

1. **Theorem proof:** symbolic mathematical reasoning.
2. **Finite certificate generation:** graph, weights, potential, hashes.
3. **Certificate verification:** checking every finite inequality and structural invariant.
4. **Independent reconstruction:** regenerating the certificate from definitions using separate code.

Passing one layer does not imply the others.

## 2. Frozen theorem package

A recommended release tree is:

```text
sturmian-return-elimination-v1.0/
├── theorem_statement.md
├── proof_dependency_dag.md
├── notation_and_definitions.md
├── literature_scope.md
├── lean/
├── rust/
├── certificates/
│   ├── embedding_1_2.json
│   ├── embedding_2_1.json
│   └── manifest.json
├── verify_frozen_certificate.py
├── regenerate_and_verify_certificate.py
├── mutation_tests/
├── REPRODUCE.md
└── LIMITATIONS.md
```

## 3. Manifest requirements

The manifest should include:

- schema version;
- canonical convention version and hash;
- selector version and hash;
- generator commit;
- Rust, Lean, and Python versions;
- dependency locks;
- node and edge counts;
- ordered embedding;
- minimum slack;
- exact maximum cycle mean;
- full SHA-256 of every artifact;
- external theorem citation;
- trusted computing base.

## 4. Lean audit

Archive actual output for:

```lean
#print axioms sturmian_gap_alphabet_1_2_eliminated
#print axioms sturmian_selector_path_to_graph_completeness
#print axioms sturmian_graph_edge_weight_domination
#print axioms normalized_transition_weight_period_extension_invariant
```

Review for:

- `sorry` or placeholders;
- hidden axioms;
- mismatch between theorem names and statements;
- assumptions imported from external theorems;
- exact graph-data binding;
- map-convention consistency.

## 5. Independent certificate verification

A minimal verifier should check only:

- artifact hashes;
- node and edge schema;
- worst-case edge aggregation;
- every potential inequality;
- minimum slack;
- exact cycle-mean result or an independent upper bound.

It should not trust the graph generator.

## 6. Independent reconstruction

A stronger script should independently implement:

- primitive necklace generation;
- phase expansion;
- balanced-template enumeration;
- selector behavior;
- transition extraction;
- semantic depth bounds;
- normalized weights;
- worst-case aggregation;
- graph serialization and hashing;
- maximum cycle mean.

Document all shared code. Independence is strongest when only the mathematical specification is shared.

## 7. Mutation tests

Required mutations include:

- swap `M` and `Q`;
- reverse the fixed-point sign;
- use the wrong commutator denominator;
- replace maximum edge aggregation with minimum;
- remove one graph edge;
- add `+1` to nonresonant switch depth;
- alter one edge weight by one;
- alter one potential value;
- change selector tie-breaking;
- collapse phase rotations;
- truncate a hash;
- restore provisional slack 72 instead of final 60;
- undercount right-censored continuation.

Every mutation should be rejected by at least one independent verifier.

## 8. Clean-room reproduction

Provide a one-command deterministic build in a clean environment:

```text
./reproduce_all.sh
```

The command should:

1. build Rust;
2. run all tests;
3. build Lean with no `sorry`;
4. regenerate certificates;
5. verify full hashes;
6. run independent Python checks;
7. produce a final signed report.

No network access should be required after dependencies are pinned.

## 9. External review packet

### Combinatorics reviewer

Focus:

- balanced templates;
- Sturmian cube theorem use;
- graph completeness;
- rotations, primitive periods, and selector locality.

### Collatz/2-adic reviewer

Focus:

- canonical return semantics;
- source cylinders;
- H.1 pointwise reduction;
- core calculations;
- scope relative to standard accelerated Collatz.

### Formal-methods reviewer

Focus:

- trusted computing base;
- generated-data binding;
- independent reconstruction;
- theorem assumptions and certificate soundness.

## 10. Falsification checklist

Invite reviewers to find:

- an actual Sturmian selector transition missing from the graph;
- an edge whose stored weight is too negative;
- a right-censored continuation that violates period-extension invariance;
- a positive state reaching an exact negative core;
- a source path that evades H.1 stabilization logic;
- a mismatch between the documented and implemented return map;
- an overlap with prior literature that invalidates the novelty statement.

A theorem package becomes more credible when it makes falsification easy.


---

# 11. Glossary and Notation

## Canonical symbols

| Symbol | Meaning |
|---|---|
| `h,j` | Return-gap symbols |
| `B_h=9+4h` | Binary precision consumed by gap `h` |
| `M_h=2^{B_h}` | Power-of-two denominator of a branch |
| `Q_h=3^{6+3h}` | Odd branch multiplier numerator |
| `C_h` | Source residue for branch `h` |
| `D_h` | Canonical successor constant for branch `h` |
| `beta_h` | Affine constant in the branch map |
| `v,w` | Finite return blocks |
| `F_v(D)` | Composed affine return map `(Q_v D + beta_v)/M_v` |
| `d_v=Q_v-M_v` | Positive odd expansion difference |
| `xi_v=-beta_v/d_v` | Rational 2-adic periodic core |
| `A_v(D)=d_vD+beta_v` | Integer primitive measuring distance to `xi_v` |
| `Gamma(v,w)` | Core interaction determinant |
| `kappa` | `v_2(Gamma(v,w))`, core separation depth |
| `s` | Incoming core depth `v_2(A_v(D))` |
| `t` | Depth after consuming a selected repeated segment |
| `g` | Extra resonant gain after normalized cancellation |
| `H_n` | Total source precision after `n` path symbols |
| `r_n` | Source residue modulo `2^{H_n}` |
| `R_n` | Least nonnegative representative of `r_n` |
| `lambda_n` | Newly exposed lift block between precision levels |
| `p_x(n)` | Factor complexity of word `x` |
| `LCP` | Longest common prefix |
| `W(e)` | Worst-case normalized precision change on graph edge `e` |
| `Phi` | Graph potential / Lyapunov function |

## Status vocabulary

### Project theorem

Claimed proved in the frozen project proof package.

### External theorem

Imported from the published or preprint literature.

### Computer-assisted theorem

The proof includes a finite exhaustive graph or certificate checked by software.

### Finite-range exhaustive result

Every object in a bounded finite search was checked; no infinite theorem follows automatically.

### Preregistered empirical result

A finite prediction was frozen before computation and then tested.

### Conjecture

A mathematically specific statement not yet proved.

### Refuted

A previous claim contradicted by proof or counterexample and removed from the active theorem ledger.

## Word models that must not be conflated

### Parity word

The binary parity sequence of the standard Collatz map.

### Accelerated exponent/valuation word

The sequence `v_2(3n+1)` along odd-to-odd accelerated steps.

### Canonical return-gap word

The symbol sequence in the project’s guarded return subsystem.

A paper must state the coding relationship among these objects before transferring a theorem from one model to another.


---

# 12. Selected Bibliography and Literature Guide

The entries below emphasize primary sources most directly connected to the project. Recent 2026 preprints are preliminary and should be treated accordingly.

## Collatz and 2-adic dynamics

1. **Bernstein, D. J.; Lagarias, J. C.** “The 3x+1 Conjugacy Map.” *Canadian Journal of Mathematics* 48 (1996), 1154–1169. DOI: `10.4153/CJM-1996-060-x`.  
   Primary source for the 2-adic conjugacy between the Collatz map and the shift.

2. **Böhm, C.; Sontacchi, G.** “On the Existence of Cycles of Given Length in Integer Sequences Like ...” *Atti Accad. Naz. Lincei Rend.* 64 (1978), 260–264.  
   Early affine/rational cycle-pattern framework.

3. **Lagarias, J. C.** “The 3x+1 Problem: An Annotated Bibliography (1963–1999).” arXiv:`math/0309224`.  
   Broad historical bibliography.

4. **Kontorovich, A.; Sinai, Ya. G.** “Structure Theorem for (d,g,h)-Maps.” arXiv:`math/0201102`.  
   Statistical structure and random-walk viewpoint for generalized Collatz maps.

5. **Tao, T.** “Almost All Orbits of the Collatz Map Attain Almost Bounded Values.” *Forum of Mathematics, Pi* 10 (2022), e12; arXiv:`1909.03562` (latest revision noted July 2026).  
   Strongest broad analytic almost-all result; approximate transport and high-frequency 3-adic analysis.

## Symbolic and finite-pattern Collatz work

6. **Stérin, T.** “On the First Occurrence of Parity Vectors and the Structure of the Collatz Graph.” arXiv:`1907.00775`.  
   Exact finite parity-vector/residue structure.

7. **López, J.; Stoll, P.** “The 3x+1 Periodicity Conjecture in R.” arXiv:`2101.12747`.  
   Studies Sturmian words as parity vectors and associated real/2-adic/3-adic behavior.

8. **Niu, T.** “Parity Vectors and Paradoxical Sequences in the Accelerated Collatz Map.” arXiv:`2605.13886` (2026 preprint).  
   Sharp finite parity-vector density and finite paradoxical-pattern counts.

9. **Kramer, O.** “Adaptive Search in Collatz Exponent-Code Space via 2-adic and 3-adic Constraints.” arXiv:`2607.10041` (2026 preprint).  
   2–3–infinity diagnostic and necessary residue-rate conditions for fixed positive sources.

10. **Dhiman, M.; Pandey, R.** “2-Adic Obstructions to Presburger-Definable Characterizations of Collatz Cycles.” arXiv:`2601.12772` (2026 preprint).  
    Ghost cycles and non-semilinearity of the integrality predicate.

11. **Chang, E. Y.** “Exploring Collatz Dynamics with Human–LLM Collaboration.” arXiv:`2603.11066` (rapidly revised 2026 preprint).  
    Broad structural reduction program, including a Sturmian/carry route and an explicitly unresolved orbitwise gap.

## Combinatorics on words

12. **Fine, N. J.; Wilf, H. S.** “Uniqueness Theorems for Periodic Functions.” *Proceedings of the American Mathematical Society* 16 (1965), 109–114. DOI: `10.2307/2034009`.  
    Classical periodic-overlap theorem.

13. **Rankin, S. A.** “Fine–Wilf Graphs and the Generalized Fine–Wilf Theorem.” arXiv:`0906.1780`.  
    Modern graph formulation and generalized context.

14. **Morse, M.; Hedlund, G. A.** Classical papers on symbolic dynamics and factor complexity.  
    Source of the periodicity/complexity criterion underlying Sturmian minimality.

15. **Matomäki, K.; Saari, K.** “A New Geometric Approach to Sturmian Words.” arXiv:`1201.4468`.  
    Includes a proof that every factor of a Sturmian word has exactly two return words.

16. **Bell, J.; Schulz, C.; Shallit, J.** “Consecutive Power Occurrences in Sturmian Words.” *Comptes Rendus Mathématique* 362 (2024); arXiv:`2402.09597`.  
    Proves cube-ending gaps at most 10 and period-at-most-5 sufficiency.

17. **Durand, F.** “A Characterization of Substitutive Sequences Using Return Words.” arXiv:`0807.3322`.  
    Primitive substitutive sequences are characterized by finiteness of derived sequences.

18. **Balková, Ľ.; Pelantová, E.; Steiner, W.** “Sequences with Constant Number of Return Words.” arXiv:`math/0608603`.  
    Return-word complexity beyond Sturmian words.

## p-adic automata and path sets

19. **Anashin, V.** “Automata Finiteness Criterion in Terms of van der Put Series of Automata Functions.” arXiv:`1112.5089`.  
    Finite-state criterion for 1-Lipschitz p-adic functions.

20. **Abram, W.; Lagarias, J. C.** “Path Sets and Their Symbolic Dynamics.” arXiv:`1207.5004`.  
    One-sided graph path sets and symbolic structure.

21. **Abram, W.; Lagarias, J. C.** “p-Adic Path Set Fractals and Arithmetic.” arXiv:`1210.2478`.  
    Automaton-defined p-adic sets, arithmetic closure, and computable dimensions.

## Graph potentials and switching systems

22. **Ahmadi, A. A.; Jungers, R.; Parrilo, P. A.; Roozbehani, M.** “Joint Spectral Radius and Path-Complete Graph Lyapunov Functions.” arXiv:`1111.3427`.  
    Graph-indexed Lyapunov certificates for arbitrary switching.

23. **Angeli, D.; Philippe, M.; Athanasopoulos, N.; Jungers, R.** “Path-Complete Graphs and Common Lyapunov Functions.” arXiv:`1612.03983`.  
    Relations among graph Lyapunov criteria.

24. **Ninite, L.; Jungers, R. M.** “Iterative Graph Lifting for Automatic Design of Path-Complete Stability Certificates.” arXiv:`2607.00637` (2026 preprint).  
    Recent graph-refinement ideas potentially relevant to automated selector-state refinement.

## How to use this bibliography

- Paper 1 should cite items 1, 2, 7, 12, 15, 16, and relevant graph-potential literature.
- Paper 2 should emphasize items 4, 5, 6, and recent parity/exponent-code work.
- Future substitution work should begin with item 17.
- Automata/dimension extensions should begin with items 19–21.
