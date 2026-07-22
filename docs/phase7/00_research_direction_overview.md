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
