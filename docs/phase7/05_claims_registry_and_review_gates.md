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
