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

### CLM-P7X-FINITE-UNIQUENESS-001

**Category:** Verified Finite Theorem

**Statement:**

For every finite valuation word $s \in \{u,v\}^*$, there exists exactly one unique quotient guard cylinder $k \equiv r_s \pmod{2^{A(s)}}$. Every finite word is positively realizable on integer representatives.

---

### CLM-P7X-PERIODIC-DIVERGENCE-001

**Category:** Verified Finite Theorem

**Statement:**

For any fixed non-empty word $s \in \{u,v\}^+$, the composite quotient map $T_s(k) = \frac{a_s k + \eta_s}{2^{A_s}}$ has a strictly negative rational fixed point $k^*_s = \frac{\eta_s}{2^{A_s} - a_s} < 0$. The guard for $s^m$ is the $2^{m A_s}$-adic truncation of $k^*_s$, so $r_{s^m} \to \infty$ as $m \to \infty$. No positive integer can realize an ultimately periodic infinite switching tail $s^\omega$.

---

### CLM-P7X-CANTOR-GUARD-TREE-001

**Category:** Verified Bounded Classification

**Statement:**

The $Q_1$ quotient return guards form a full disjoint nested binary tree in $\mathbb{Z}_2$. Its infinite limit set $G_\infty$ is a measure-zero Cantor set with Haar measure $\mu(G_r) = (33/512)^r \to 0$ and 2-adic Hausdorff dimension $d \approx 0.1625357554$.

---

### CLM-P7X-COMMONCENTER-001

**Category:** Theorem Candidate until formalized

**Statement:**

A finite family with pairwise \(\Delta=0\) admits a common fixed-point form whose 2-adic valuation drops by \(A_p\) under every valid switch.

**Gate:**

Do not mark verified until the zero case, positive-integer fixed point, and domain semantics are fully handled.

## 3. Phase 7.3 Sub-Phase Review Gates

### Gate 7.3A — Generic Affine Identity Kernel
Pass if:
- Same-form, cross-form, and affine commutator identities hold symbolically;
- $\Delta_{u,v} = -5568, b_u b_v = 8192, v_2(\Delta_{u,v}) = 6$ recomputed independently;
- Broad and exact resonance recovery matches direct modular inversion;
- Identity verifiers PASS across Rust, Python oracle, and Lean 4 formal proofs.

### Gate 7.3B — Sound Register Machine Abstraction
Pass if:
- Single-coordinate $L_u(n) = 11n + 19$ state machine $(x, \text{res})$ has precise concretizations;
- Cancellation gate $x=6, U \equiv 1 \pmod{16}$ universally validated;
- Feature CEGAR adds $L_v(n)$ or extra $U$-bits only upon concrete counterexample.

### Gate 7.3C — Symbolic Return Language & Entropy Classification
Pass if:
- All words $s \in \{u,v\}^{\le 12}$ enumerated with exact path cylinders;
- All block-boundary $Q_1$-return conditions verified;
- Spectral radius, topological entropy, and language growth reported.

### Gate 7.3D — Termination Proof System Evaluation (Target A)
Pass if Target A ($u/v$ core) is classified under one of the 4 competing proof architectures:
- `TERMINATED_PATH_COMPLETE_RANKING` (`path_complete_ranking_v1`)
- `TERMINATED_DISJUNCTIVE_INVARIANT` (`disjunctive_transition_invariant_v1`)
- `TERMINATED_LEXICOGRAPHIC`
- `TERMINATED_MULTIPHASE`
- `TERMINATED_SCT`
- `SOUND_UNRANKED`

### Gate 7.3E — Staged Target Expansion
Pass if:
- Target A ($u/v$ core) fully classified before Target B ($Q_2$ loop) expansion;
- Target B fully classified before Target C (full 3-state SCC) expansion;
- Word library expansion delayed until Phase 7.4.


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
