---

# Source Document: `README.md`

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


---

# Source Document: `00_research_direction_overview.md`

# Phase 7X Research Direction Overview
## Affine Interaction and Ultrametric Fuel

## 1. Why this direction exists

Milestones 7.1 and 7.2 exposed a repeated structural failure:

1. A coarse residue graph appears to contain a recurrent cycle.
2. Destination-aware refinement reveals that the edge depends on quotient bits discarded by the source state.
3. A composite valuation word remains arithmetically valid.
4. The composite word has a correct Phase 6D finite-fuel certificate.
5. The original recurrent graph claim does not survive exact path-cylinder validation.

This pattern suggests that the main obstacle is not merely insufficient implementation precision. It is that each macrostep consumes a predictable number of 2-adic bits, while static residue states forget the exact information later divisions expose.

The new direction studies that information loss directly.

## 2. Primary research question

> Can multi-word Collatz behavior be represented by a finite or finitely parameterized switching system whose state records precision debt, affine interaction, and 2-adic cancellation depth, rather than relying primarily on ever-deeper raw residue partitions?

## 3. Secondary research questions

1. Can exact destination precision be expressed as a theorem of information consumption?
2. Can the interaction of two macrosteps be summarized by one affine commutator constant?
3. Can exact-word source cylinders be recovered as cancellation conditions in a fixed-point coordinate?
4. Do families of macrosteps sharing one rational fixed point admit arbitrary-switching finite-fuel theorems?
5. Do pairs with large \(v_2(\Delta)\) identify the genuinely difficult low-bit branching targets?
6. Can a cancellation automaton be smaller and more stable than a destination-refined residue graph?
7. Can path cylinders be generated and verified before graph construction, eliminating spurious SCCs by design?

## 4. The proposed mathematical objects

For a nonempty valuation word \(p\), define:

\[
F_p(n)=\frac{a_pn+c_p}{b_p},
\qquad
a_p=3^{k_p},
\qquad
b_p=2^{A_p},
\]

\[
d_p=b_p-a_p,
\qquad
H_p(n)=d_pn-c_p.
\]

The rational fixed point is:

\[
x_p^*=\frac{c_p}{d_p}.
\]

For two words \(p,q\), define the affine interaction constant:

\[
\Delta_{p,q}=d_pc_q-d_qc_p.
\]

Define the interaction depth:

\[
\kappa_{p,q}=
\begin{cases}
v_2(\Delta_{p,q}), & \Delta_{p,q}\ne0,\\
\infty, & \Delta_{p,q}=0.
\end{cases}
\]

Define destination precision debt for a source state of exponent \(M\), a macrostep of total valuation \(A\), and requested target exponent \(q_t\):

\[
D_{\mathrm{prec}}=A+q_t-M.
\]

The required extra bits are:

\[
h_{\mathrm{add}}=\max(0,D_{\mathrm{prec}}).
\]

## 5. Established versus proposed results

### Established algebraic identities

The following are exact identities and should be formalized generically:

1. Destination precision:
   \[
   M\ge A+q_t
   \]
   is necessary and sufficient for a full source cylinder \(R\bmod2^M\) to have one deterministic image modulo \(2^{q_t}\).

2. Same-form eigenidentity:
   \[
   b_pH_p(F_p(n))=a_pH_p(n).
   \]

3. Cross-form identity:
   \[
   b_qH_p(F_q(n))
   =
   a_qH_p(n)+\Delta_{p,q}.
   \]

4. Affine commutator identity:
   \[
   b_pb_q\big(F_q(F_p(n))-F_p(F_q(n))\big)
   =
   \Delta_{p,q}.
   \]

5. Common-center criterion:
   \[
   \Delta_{p,q}=0
   \]
   if and only if \(p\) and \(q\) have the same rational fixed point.

### Proposed research directions

These are not yet theorems of the project:

- finite-state ultrametric cancellation abstraction;
- arbitrary-switching termination for common-center families;
- near-commuting pair selection by large \(v_2(\Delta)\);
- symbolic transducer states replacing deep residue partitions;
- finite classification of all recurrent components under a bounded word library.

## 6. Potentially important accidental insight

The broad and exact source cylinders of one macrostep can be represented as cancellation conditions in the fixed-point form of another macrostep.

For nonempty \(p,q\), because \(d_p\) and \(c_p\) are odd:

\[
2^{A_q}\mid a_qH_p(n)+\Delta_{p,q}
\]

is equivalent to the prescribed \(q\)-macrostep being integral, and

\[
2^{A_q+1}\mid a_qH_p(n)+\Delta_{p,q}
\]

is equivalent to the resulting prescribed quotient being odd, hence to the exact valuation word \(q\).

This reframes exact valuation cylinders as ultrametric resonance conditions.

## 7. Why this may be more powerful than raw SCT

SCT needs sound transition relations over well-founded features. Earlier attempts guessed cross-feature inequalities and then rejected them with counterexamples.

The cross-form identity provides the exact transition law first. The valuation relation is then derived from:

\[
v_2\!\left(a_qH_p(n)+\Delta_{p,q}\right),
\]

rather than guessed.

This may produce:

- exact piecewise transition relations;
- a small cancellation control state;
- bounded-reset or multiphase rankings;
- or a principled proof that the chosen feature family is insufficient.

## 8. Valid negative outcomes

Phase 7X is successful if it produces any of the following:

1. A verified common-center arbitrary-switching theorem.
2. A verified cancellation automaton smaller than the residue graph.
3. A proof that cancellation states still require unbounded precision.
4. A ranked list of near-commuting target pairs.
5. A verified path-first graph construction.
6. A counterexample showing that the proposed ultrametric feature set is incomplete.
7. A bounded result showing all surviving components collapse to Phase 6D.

The milestone must not require a branching SCT certificate to exist.


---

# Source Document: `01_affine_interaction_theory.md`

# Affine Interaction Theory for Collatz Macrosteps

## 1. Macrostep data

Let \(p=(a_1,\ldots,a_k)\) be a nonempty valuation word. Define:

\[
K_p=k,
\qquad
A_p=\sum_{i=1}^k a_i,
\]

\[
a_p=3^{K_p},
\qquad
b_p=2^{A_p}.
\]

The affine constant \(c_p\) is generated by:

\[
c_0=0,
\qquad
c_{i+1}=3c_i+2^{A_i}.
\]

The prescribed macrostep is:

\[
F_p(n)=\frac{a_pn+c_p}{b_p}.
\]

Define:

\[
d_p=b_p-a_p,
\qquad
H_p(n)=d_pn-c_p.
\]

Because \(a_p\) is odd and \(b_p\) is even:

\[
d_p\text{ is odd}.
\]

For every nonempty valuation word, \(c_p\) is also odd.

The rational fixed point is:

\[
x_p^*=\frac{c_p}{d_p}.
\]

The sign-normalized form used for presentation may be \(\pm H_p\), but all interaction calculations should use the raw \(H_p=d_pn-c_p\).

---

## 2. Destination Precision Theorem

Let a source cylinder be:

\[
n=R+2^M u.
\]

Assume the prescribed macrostep \(F_p\) is integral on that cylinder. Then:

\[
F_p(R+2^Mu)
=
F_p(R)+a_p2^{M-A_p}u.
\]

Since \(a_p\) is odd, the image is independent of \(u\) modulo \(2^q\) if and only if:

\[
M-A_p\ge q.
\]

Therefore:

\[
\boxed{M\ge A_p+q}
\]

is necessary and sufficient for deterministic target classification modulo \(2^q\).

### Incremental refinement

Given current exponent \(M_{\mathrm{curr}}\):

\[
h_{\mathrm{add}}
=
\max(0,A_p+q-M_{\mathrm{curr}}).
\]

The source splits into:

\[
2^{h_{\mathrm{add}}}
\]

subcells.

### Precision debt

Define:

\[
D_{\mathrm{prec}}(p,q,M)
=
A_p+q-M.
\]

Interpretation:

- \(D_{\mathrm{prec}}\le0\): no additional residue information is needed;
- \(D_{\mathrm{prec}}>0\): exactly \(D_{\mathrm{prec}}\) quotient bits remain unresolved.

This is an information-consumption law, not merely a heuristic refinement rule.

---

## 3. Same-Form Eigenidentity

Substituting \(F_p\) into \(H_p\):

\[
H_p(F_p(n))
=
d_p\frac{a_pn+c_p}{b_p}-c_p.
\]

Multiplying by \(b_p\):

\[
b_pH_p(F_p(n))
=
d_pa_pn+d_pc_p-b_pc_p.
\]

Since \(d_p=b_p-a_p\):

\[
d_pc_p-b_pc_p=-a_pc_p.
\]

Thus:

\[
\boxed{
b_pH_p(F_p(n))
=
a_pH_p(n)
}.
\]

Whenever the macrostep is valid and \(H_p(n)\ne0\):

\[
v_2(H_p(F_p(n)))
=
v_2(H_p(n))-A_p.
\]

This is the Phase 6D finite-fuel mechanism.

---

## 4. Affine Interaction Constant

For two words \(p,q\), define:

\[
\boxed{
\Delta_{p,q}=d_pc_q-d_qc_p
}.
\]

Properties:

\[
\Delta_{q,p}=-\Delta_{p,q}.
\]

\[
\Delta_{p,p}=0.
\]

### Fixed-point interpretation

Because \(x_p^*=c_p/d_p\):

\[
\Delta_{p,q}=0
\]

if and only if:

\[
\frac{c_p}{d_p}
=
\frac{c_q}{d_q}.
\]

Thus \(\Delta=0\) means the two macrosteps share one rational fixed point.

---

## 5. Affine Commutator Identity

Using:

\[
F_q(F_p(n))
=
\frac{a_qa_pn+a_qc_p+b_pc_q}{b_pb_q},
\]

and:

\[
F_p(F_q(n))
=
\frac{a_pa_qn+a_pc_q+b_qc_p}{b_pb_q},
\]

their difference is:

\[
b_pb_q
\left(
F_q(F_p(n))-F_p(F_q(n))
\right)
=
a_qc_p+b_pc_q-a_pc_q-b_qc_p.
\]

Rearranging:

\[
\boxed{
b_pb_q
\left(
F_q(F_p(n))-F_p(F_q(n))
\right)
=
\Delta_{p,q}
}.
\]

The order defect is independent of \(n\).

### Interpretation

- \(\Delta=0\): the affine maps commute;
- small \(v_2(\Delta)\): order becomes visible at shallow 2-adic precision;
- large \(v_2(\Delta)\): the maps nearly commute in low bits;
- \(\Delta\ne0\): different composition orders have distinct affine constants.

---

## 6. Cross-Linear-Form Identity

Evaluate \(H_p\) after \(F_q\):

\[
H_p(F_q(n))
=
d_p\frac{a_qn+c_q}{b_q}-c_p.
\]

Multiplying by \(b_q\):

\[
b_qH_p(F_q(n))
=
d_pa_qn+d_pc_q-b_qc_p.
\]

Add and subtract \(a_qc_p\):

\[
b_qH_p(F_q(n))
=
a_q(d_pn-c_p)
+
d_pc_q-(b_q-a_q)c_p.
\]

Since \(d_q=b_q-a_q\):

\[
\boxed{
b_qH_p(F_q(n))
=
a_qH_p(n)+\Delta_{p,q}
}.
\]

This is the basic switching identity.

---

## 7. Broad and Exact Cylinders as Resonance Conditions

Because \(d_p\) is odd, it is invertible modulo every power of two.

Rewrite the cross identity as:

\[
a_qH_p(n)+\Delta_{p,q}
=
d_p(a_qn+c_q)-b_qc_p.
\]

Modulo \(b_q=2^{A_q}\), the term \(b_qc_p\) vanishes. Therefore:

\[
2^{A_q}
\mid
a_qH_p(n)+\Delta_{p,q}
\]

if and only if:

\[
2^{A_q}
\mid
a_qn+c_q.
\]

Hence:

\[
\boxed{
q\text{ broad source cylinder}
\iff
v_2(a_qH_p(n)+\Delta_{p,q})\ge A_q
}.
\]

Now suppose the broad condition holds, so \(F_q(n)\) is integral. Since \(d_p\) and \(c_p\) are odd:

\[
H_p(F_q(n))
=
d_pF_q(n)-c_p
\]

is even if and only if \(F_q(n)\) is odd.

Using the cross identity:

\[
\boxed{
q\text{ exact source cylinder}
\iff
v_2(a_qH_p(n)+\Delta_{p,q})\ge A_q+1
}.
\]

This is the **cross-form exactness theorem**.

### Research significance

Exact valuation words can be detected as cancellation depth in any other primitive fixed-point coordinate \(H_p\).

This creates a bridge between:

- residue-cylinder semantics;
- fixed-point linear forms;
- ultrametric feature transitions;
- and symbolic automata.

---

## 8. Ultrametric Transition Law

Let:

\[
x=v_2(H_p(n)),
\qquad
\kappa=v_2(\Delta_{p,q}),
\]

with \(\Delta_{p,q}\ne0\).

Since \(a_q\) is odd:

\[
v_2(a_qH_p(n))=x.
\]

The strong triangle law gives:

### Case 1: \(x<\kappa\)

\[
v_2(a_qH_p(n)+\Delta)=x.
\]

### Case 2: \(x>\kappa\)

\[
v_2(a_qH_p(n)+\Delta)=\kappa.
\]

### Case 3: \(x=\kappa\)

Write:

\[
H_p(n)=2^\kappa U,
\qquad
\Delta=2^\kappa\delta,
\]

with \(U,\delta\) odd. Then:

\[
v_2(a_qH_p(n)+\Delta)
=
\kappa+v_2(a_qU+\delta).
\]

Only the equality layer permits extra cancellation.

---

## 9. Resonance Gate

Suppose:

\[
\kappa<A_q.
\]

For the \(q\)-macrostep to be broadly integral, the cross-form exactness theorem requires:

\[
v_2(a_qH_p(n)+\Delta)\ge A_q.
\]

If \(x<\kappa\) or \(x>\kappa\), then:

\[
v_2(a_qH_p(n)+\Delta)
=
\min(x,\kappa)
\le\kappa<A_q,
\]

which is impossible.

Therefore every valid broad \(q\)-source must satisfy:

\[
\boxed{x=\kappa}.
\]

Moreover:

\[
a_qU+\delta\equiv0
\pmod{2^{A_q-\kappa}}.
\]

For the exact \(q\)-word:

\[
a_qU+\delta\equiv0
\pmod{2^{A_q+1-\kappa}}.
\]

This is a compact resonance guard.

---

## 10. Benchmark example

For:

\[
w_1=[1,1,2],
\]

\[
a_1=27,\quad b_1=16,\quad c_1=19,\quad d_1=-11,
\]

and:

\[
w_2=[1,2,2],
\]

\[
a_2=27,\quad b_2=32,\quad c_2=23,\quad d_2=5.
\]

Then:

\[
\Delta_{1,2}=(-11)(23)-(5)(19)=-348,
\]

\[
\kappa_{1,2}=v_2(348)=2.
\]

The cross identity is:

\[
32H_1(F_2(n))
=
27H_1(n)-348.
\]

Because:

\[
\kappa=2<A_2=5,
\]

every broad \(w_2\)-source lies on:

\[
v_2(H_1(n))=2.
\]

Writing:

\[
H_1(n)=4U,
\qquad
-348=4(-87),
\]

the broad resonance condition is:

\[
27U-87\equiv0\pmod8.
\]

This simplifies to:

\[
U\equiv5\pmod8.
\]

In \(n\)-coordinates this recovers:

\[
n\equiv11\pmod{32},
\]

the broad source cylinder of \(w_2\).

The exact-word condition uses one more bit and recovers:

\[
n\equiv43\pmod{64}.
\]

Similarly, the reverse cross identity recovers the broad and exact cylinders for \(w_1\).

---

## 11. Common-center arbitrary-switching theorem candidate

Suppose a finite family \(\mathcal F\) satisfies:

\[
\Delta_{p,q}=0
\qquad
\forall p,q\in\mathcal F.
\]

Then all maps share a rational fixed point \(C/D\) in reduced form.

Define:

\[
H(n)=Dn-C.
\]

For every \(p\in\mathcal F\):

\[
\boxed{
b_pH(F_p(n))
=
a_pH(n)
}.
\]

Therefore, along any valid switching sequence \(p_1,\ldots,p_t\):

\[
v_2(H(n_t))
=
v_2(H(n_0))
-
\sum_{j=1}^t A_{p_j}.
\]

Unless \(H(n_0)=0\), the sequence can realize only finitely many complete macrosteps from the family.

This would be a genuine infinite-language generalization of Phase 6D.

### Required caveats

A final theorem must handle:

- the zero case \(H(n)=0\);
- positivity of the rational fixed point;
- exact path validity;
- whether the fixed point is a positive integer;
- source-domain semantics for each switch.

---

## 12. Open theoretical questions

1. Does the cancellation state admit a finite quotient for bounded word libraries?
2. Can high-\(\kappa\) pairs produce unbounded cancellation depth?
3. Can a multiphase ranking be derived from descending cancellation depth?
4. Does common-center arbitrary switching produce useful nontrivial Collatz families?
5. Can path cylinders be reconstructed entirely from cross-form congruences?
6. Can the affine interaction matrix identify all Phase 6D collapses before graph construction?


---

# Source Document: `02_ultrametric_switching_abstraction.md`

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


---

# Source Document: `03_phase7x_implementation_and_experiments.md`

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


---

# Source Document: `04_certificate_schemas_and_verification.md`

# Phase 7X Certificate Schemas and Verification

## 1. Design principles

Every proof object must distinguish:

- arithmetic identity;
- domain semantics;
- path validity;
- graph classification;
- termination algebra;
- bounded search scope.

No boolean field such as `"verified": true` is trusted without recomputation.

## 2. `destination_precision_v1`

Purpose: prove the exact source precision needed to determine a target modulus.

```json
{
  "schema_version": "destination_precision_v1",
  "word": [1, 1, 2],
  "odd_steps_k": 3,
  "total_valuation_a": 4,
  "source_residue": "7",
  "current_source_exponent": 5,
  "target_exponent": 6,
  "required_source_exponent": 10,
  "additional_bits": 5
}
```

Verifier recomputes:

\[
M_{\mathrm{required}}=\max(M_{\mathrm{current}},A+q).
\]

It must also verify necessity by showing that if \(M-A<q\), two quotient values produce distinct target residues.

## 3. `affine_commutator_v1`

```json
{
  "schema_version": "affine_commutator_v1",
  "left_word": [1, 1, 2],
  "right_word": [1, 2, 2],
  "left": {
    "k": 3,
    "A": 4,
    "a": "27",
    "b": "16",
    "c": "19",
    "d": "-11"
  },
  "right": {
    "k": 3,
    "A": 5,
    "a": "27",
    "b": "32",
    "c": "23",
    "d": "5"
  },
  "delta": "-348",
  "delta_v2": 2,
  "common_fixed_point": false
}
```

Verifier recomputes all fields from the words.

## 4. `cross_linear_form_transition_v1`

```json
{
  "schema_version": "cross_linear_form_transition_v1",
  "reference_word": [1, 1, 2],
  "transition_word": [1, 2, 2],
  "identity": {
    "left_multiplier": "32",
    "transition_multiplier": "27",
    "delta": "-348"
  },
  "broad_required_v2": 5,
  "exact_required_v2": 6
}
```

The displayed identity string is informational. Structured integers are authoritative.

Verifier checks:

\[
b_qH_p(F_q(n))=a_qH_p(n)+\Delta.
\]

## 5. `resonance_cylinder_v1`

```json
{
  "schema_version": "resonance_cylinder_v1",
  "reference_word": [1, 1, 2],
  "transition_word": [1, 2, 2],
  "interaction_v2_kappa": 2,
  "semantics": "exact_word",
  "normalized_odd_residue": "5",
  "normalized_modulus_exponent": 4,
  "recovered_source_residue": "43",
  "recovered_source_exponent": 6
}
```

Verifier must:

1. recompute \(\Delta\) and \(\kappa\);
2. solve the normalized congruence;
3. map back to \(n\)-coordinates;
4. compare to direct exact-cylinder modular inversion.

## 6. `common_fixed_point_family_v1`

```json
{
  "schema_version": "common_fixed_point_family_v1",
  "family_id": "CF-001",
  "words": [],
  "common_fixed_point": {
    "numerator": "C",
    "denominator": "D"
  },
  "common_form": {
    "alpha": "D",
    "beta": "-C"
  },
  "zero_case": {
    "positive_integer_fixed_point": false
  }
}
```

Verifier checks pairwise \(\Delta=0\) and every family identity.

## 7. `path_cylinder_v1`

```json
{
  "schema_version": "path_cylinder_v1",
  "path_words": [
    [1, 1, 2],
    [1, 2, 2]
  ],
  "composite": {
    "k": 6,
    "A": 9,
    "c": "881"
  },
  "source_semantics": "exact_guarded_path",
  "source_residue": "1959",
  "source_exponent": 14,
  "intermediate_guards": [],
  "target_guard": {
    "residue": "7",
    "exponent": 5
  }
}
```

Verifier must recompute every intermediate image universally, not only test the representative.

## 8. `ultrametric_cancellation_v1`

```json
{
  "schema_version": "ultrametric_cancellation_v1",
  "reference_word": [],
  "transition_word": [],
  "kappa": 0,
  "source_region": "resonant",
  "normalized_source_residue": "0",
  "normalized_source_bits": 0,
  "additional_cancellation_v2": 0,
  "target_feature_v2": 0
}
```

This schema is accepted only when the verifier proves the complete quantified transition.

## 9. `phase7x_discovery_outcome_v1`

Always emitted.

```json
{
  "schema_version": "phase7x_discovery_outcome_v1",
  "scope": {
    "max_word_length": 0,
    "max_step_valuation": 0,
    "max_path_length": 0,
    "max_residue_exponent": 0
  },
  "outcome": "SOUND_UNRANKED",
  "artifacts": []
}
```

Allowed outcomes:

- `COMMON_CENTER_FAMILY_FOUND`
- `NEAR_COMMUTING_TARGETS_FOUND`
- `CANCELLATION_AUTOMATON_TERMINATED`
- `PHASE6D_COLLAPSE`
- `PATH_INCOMPATIBLE`
- `SOUND_UNRANKED`
- `NO_RECURRENT_COMPONENT`
- `REFINEMENT_LIMIT`

## 10. Independent validity layers

Every final report must show:

```text
Layer 1: Macrostep arithmetic
Layer 2: Interaction identities
Layer 3: Cylinder and path semantics
Layer 4: Abstract-state soundness
Layer 5: Recurrent-language classification
Layer 6: Termination algebra
Layer 7: Claim-scope validity
```

Each layer returns:

- `VALID`
- `INVALID`
- `UNRESOLVED`
- `NOT_APPLICABLE`

## 11. Lean trust boundary

Lean should prove generic identities and selected quantified instances.

Rust and Python should remain responsible for:

- artifact parsing;
- bounded enumeration;
- graph construction;
- SCC extraction;
- certificate routing.

The final theorem must disclose exactly which parts were checked by Lean and which were checked by independent executable verifiers.


---

# Source Document: `05_claims_registry_and_review_gates.md`

# Phase 7X Claims Registry and Review Gates

## 1. Claim categories

Use only these categories:

- `Verified Algebraic Identity`
- `Verified Finite Theorem`
- `Verified Bounded Classification`
- `Domain-Scoped Certificate`
- `Experimental Observation`
- `Conjectural Pattern`
- `Open Question`
- `Refuted Candidate`

## 2. Initial claims

### CLM-P7X-PRECISION-001

**Category:** Verified Algebraic Identity

**Statement:**

For a valid macrostep with total valuation \(A\), a complete source cylinder modulo \(2^M\) has a deterministic image modulo \(2^q\) if and only if:

\[
M\ge A+q.
\]

**Required proof:**

Generic algebra plus a necessity witness using two quotient values.

---

### CLM-P7X-CROSSFORM-001

**Category:** Verified Algebraic Identity

**Statement:**

For all nonempty macrosteps \(p,q\):

\[
b_qH_p(F_q(n))
=
a_qH_p(n)+\Delta_{p,q}.
\]

---

### CLM-P7X-COMMUTATOR-001

**Category:** Verified Algebraic Identity

**Statement:**

\[
b_pb_q(F_q(F_p(n))-F_p(F_q(n)))
=
\Delta_{p,q}.
\]

---

### CLM-P7X-EXACTNESS-001

**Category:** Verified Finite Theorem

**Statement:**

For any nonempty reference word \(p\), the broad and exact source cylinders of \(q\) are respectively characterized by:

\[
v_2(a_qH_p(n)+\Delta_{p,q})\ge A_q
\]

and:

\[
v_2(a_qH_p(n)+\Delta_{p,q})\ge A_q+1.
\]

**Required caveat:**

The statement uses the prescribed affine macrostep and the parity of its quotient.

---

### CLM-P7X-RESONANCE-001

**Category:** Verified Finite Theorem

**Statement:**

If:

\[
\kappa=v_2(\Delta_{p,q})<A_q,
\]

then every broad \(q\)-source satisfies:

\[
v_2(H_p(n))=\kappa.
\]

The remaining source condition is one normalized odd-residue congruence.

---

### CLM-P7X-COMMONCENTER-001

**Category:** Theorem Candidate until formalized

**Statement:**

A finite family with pairwise \(\Delta=0\) admits a common fixed-point form whose 2-adic valuation drops by \(A_p\) under every valid switch.

**Gate:**

Do not mark verified until the zero case, positive-integer fixed point, and domain semantics are fully handled.

## 3. Review gates

### Gate A — Algebra

Pass if:

- all macrostep data are recomputed;
- identities hold symbolically;
- sign conventions are consistent;
- no formula strings are trusted.

### Gate B — Cylinder semantics

Pass if:

- broad and exact cylinders agree with direct inversion;
- parity forcing is explicit;
- zero cases are separated;
- all congruences are universal.

### Gate C — Path semantics

Pass if:

- complete path maps are recomputed;
- exact path cylinders are nonempty;
- intermediate guards are universal;
- final canonical states are correct.

### Gate D — Abstraction

Pass if:

- every abstract state has a concrete meaning;
- merges preserve outgoing semantics;
- cancellation residues are sufficient;
- precision debt is not hidden in history strings.

### Gate E — Recurrence

Pass if:

- SCCs are computed from the actual verified graph;
- every cycle has a path certificate;
- abstract branching and finite path compatibility are separated;
- cycle rank is computed correctly.

### Gate F — Termination

Pass if one exact proof system succeeds:

- Phase 6D finite fuel;
- common-center fuel;
- lexicographic ranking;
- multiphase ranking;
- SCT idempotent criterion;
- another explicitly defined well-founded relation.

### Gate G — Scope

Pass if the claim states:

- alphabet bounds;
- path bounds;
- precision bounds;
- control-state bounds;
- unresolved branches;
- what was not proved.

## 4. Breakthrough language policy

Use the word **breakthrough** only for one of:

1. a new theorem materially extending the project’s proved scope;
2. a verified abstraction that eliminates a previously unbounded source of spuriousness;
3. a new invariant family proving termination of a genuinely multi-word language;
4. a result with clear novelty after literature review.

Use **innovation candidate** for:

- a useful algebraic reframing;
- a new certificate architecture;
- a promising empirical pattern;
- a compression of known residue semantics.

## 5. Negative-result templates

### No common-center families

> Within the frozen library, all pairwise \(\Delta=0\) clusters were equivalent to trivial powers, rotations, or duplicate segmentations.

### Cancellation abstraction did not stabilize

> The resonance abstraction remained sound but required unbounded normalized residue precision within the tested component.

### Near-commuting pairs collapsed

> Every high-\(v_2(\Delta)\) candidate either failed path compatibility or reduced to a Phase 6D composite word.

### Sound but unranked

> A path-realizable recurrent component survived all semantic checks, but no verified ranking relation was found within the declared feature grammar.

## 6. Final milestone decision table

| Arithmetic | Paths | Recurrent component | Ranking | Result |
|---|---|---|---|---|
| Valid | Invalid | — | — | Path-incompatible |
| Valid | Valid | None | — | No recurrent component |
| Valid | Valid | Periodic | Phase 6D | Phase 6D collapse |
| Valid | Valid | Common center | Common form | Arbitrary-switching fuel |
| Valid | Valid | Branching | Ranking found | Phase 7 theorem |
| Valid | Valid | Branching | None | Sound unranked |
| Unresolved | — | — | — | Refinement limit |


---

# Source Document: `06_phase72_handoff_and_migration.md`

# Phase 7.2 Handoff and Migration to Phase 7X

## 1. Preserve these Phase 7.2 results

The following should remain frozen as verified local results:

1. Destination-aware refinement uses:
   \[
   h_{\mathrm{add}}
   =
   \max(0,A+q_t-M_{\mathrm{curr}}).
   \]

2. The \(w_1=[1,1,2]\) source cylinder \(7\bmod32\) can be refined to modulus \(1024\) to classify images modulo \(64\).

3. The subcell:
   \[
   935\bmod1024
   \]
   maps under \(w_1\) to:
   \[
   1579\equiv43\bmod64.
   \]

4. The \(w_2=[1,2,2]\) source cylinder \(43\bmod64\) can be refined into 16 cells modulo \(1024\).

5. The subcell:
   \[
   235\bmod1024
   \]
   maps under \(w_2\) to:
   \[
   199\equiv7\bmod32.
   \]

6. The one-loop guarded path cylinder is:
   \[
   1959\bmod16384.
   \]

7. The composite word:
   \[
   W=[1,1,2,1,2,2]
   \]
   retains its valid Phase 6D finite-fuel certificate.

These are valuable regression fixtures for Phase 7X.

## 2. Reclassify the graph result carefully

The coarse control cycle:

\[
7\bmod32
\to
43\bmod64
\to
7\bmod32
\]

is supported only on refined path subcylinders.

It should be represented as a guarded path schema, not as evidence that every member of either coarse state follows the cycle.

Recommended terminology:

- coarse control return;
- guarded one-loop path cylinder;
- guarded two-loop path cylinder;
- exact composite word cylinder;
- refined arithmetic source cell.

Avoid calling the coarse states a deterministic SCC unless every outgoing branch is included and graph completeness is proved relative to the selected library.

## 3. Convert Phase 7.2 artifacts into regression tests

### Destination precision regression

Reject any implementation that classifies:

\[
43\bmod64\xrightarrow{w_2}7\bmod32
\]

without refining to modulus \(1024\).

### Path-cylinder regression

Verify:

\[
1959\bmod16384
\]

for one guarded loop.

### Fuel regression

Verify:

\[
N_W(1959)=1.
\]

### Exact-word versus guarded-path regression

Keep distinct:

\[
935\bmod1024
\]

for the exact word \(W\), and:

\[
1959\bmod16384
\]

for the guarded coarse return.

## 4. New benchmark interpretation

The pair \(w_1,w_2\) becomes the first affine interaction benchmark.

Its interaction data are:

\[
\Delta_{1,2}=-348,
\qquad
v_2(\Delta_{1,2})=2.
\]

The broad source of \(w_2\) appears as a resonance condition in \(H_1\), and the broad source of \(w_1\) appears as a resonance condition in \(H_2\).

This makes the benchmark useful even though it does not produce genuine noncommuting branching.

## 5. Updated roadmap position

Recommended roadmap:

```text
Phase 7.1:
    Bounded semantic-refinement result

Phase 7.2:
    Destination-aware refinement and guarded periodic path validation

Phase 7X:
    Affine interaction, resonance cylinders, and ultrametric switching

Phase 7.3:
    Expanded symbolic word library and path-first recurrent-language search

Phase 7.4:
    Ranking synthesis over sound branching components
```

## 6. Phase 7.3 entry criteria

Do not expand the word library until Phase 7X establishes:

- generic interaction identities;
- cross-form cylinder recovery;
- an interaction-spectrum report;
- one abstraction comparison;
- path-first certificate generation;
- explicit negative-outcome schemas.

This prevents the larger search from reproducing the same false SCCs at greater scale.
