---

# Source Document: `README.md`

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


---

# Source Document: `00_research_direction_overview.md`

# Phase 7.3 Research Direction Overview
## Affine Interaction, Ultrametric Fuel, and Symbolic Return Dynamics

## 1. Why this direction exists

Phase 7.2 completed with a major discovery: destination-aware refinement successfully produced a **sound noncommuting guarded-branching target** at state $Q_1$ with based closed walks:
- $u = [1,1,2]$
- $v = w_1 w_2 = [1,1,2,1,2,2]$

Both $uv$ and $vu$ possess exact positive-integer path cylinders, confirming genuine noncommuting branching ($uv \neq vu$).

However, raw residue states do not yet provide a compact switching invariant. Each macrostep consumes 2-adic bits, while static residue partitions forget information later divisions expose.

Phase 7.3 merges the ultrametric affine interaction machinery to serve as the mathematical, symbolic, and verification engine for this verified $(u, v)$ branching target.

## 2. Primary research question

> Can the switching behavior of the verified $(u, v)$ branching core be represented by a compact, switching-sensitive ultrametric invariant (or affine register machine) that tracks 2-adic cancellation depth and symbolic return dynamics, rather than relying on ever-deeper raw residue partitions?

## 3. Secondary research questions

1. **Symbolic Return Dynamics**: Can the switching language $s \in \{u,v\}^*$ be modeled as an induced return language to $Q_1$?
2. **Finite Return Language**: Does every word $s \in \{u,v\}^*$ have a nonempty positive-integer cylinder returning to $Q_1$ at block boundaries?
3. **Graph Lyapunov Rankings**: Can a state-indexed or path-complete graph Lyapunov function (`path_complete_ranking_v1`) certify stability over accepted words?
4. **Disjunctive Transition Invariants**: Can a finite union of well-founded relations (`disjunctive_transition_invariant_v1`) cover the transitive closure of the switching relation?
5. **Feature CEGAR**: Is a single reference coordinate $L_u(n) = 11n + 19$ sufficient, or does feature CEGAR demand adding $L_v(n) = 217n + 881$?
6. **Language Entropy**: What are the growth rate, spectral radius, and topological entropy of the admissible $u/v$ switching language?

## 4. Mathematical Objects & Benchmark Parameters

### Benchmark Switching Pair $(u, v)$ at $Q_1$
- $u = [1,1,2]$: $a_u = 27, b_u = 16, c_u = 19, d_u = -11$. Linear form $L_u(n) = 11n + 19$.
- $v = [1,1,2,1,2,2]$: $a_v = 729, b_v = 512, c_v = 881, d_v = -217$. Linear form $L_v(n) = 217n + 881$.

### Commutator Data
$$\Delta_{u,v} = d_u c_v - d_v c_u = (-11)(881) - (-217)(19) = -5568 = -2^6 \cdot 87, \qquad \kappa_{u,v} = v_2(\Delta_{u,v}) = 6$$

### Affine Commutator Identity
$$b_u b_v \big(F_{uv}(n) - F_{vu}(n)\big) = 8192 \left(-\frac{5568}{8192}\right) = -5568$$

### Concrete Switching Coordinate & Cancellation Gate
Let $x = v_2(L_u(n))$:
- **$u$-transition**: $x \mapsto x - 4$.
- **$v$-transition**: Supported on resonance layer $x = 6$, $L_u(n) = 2^6 U$ ($U$ odd, $U \equiv 1 \pmod{16}$):
  $$x' = v_2(729 U + 87) - 3$$

---

## 5. Established versus Proposed Results

### Established Algebraic Identities (Lean 4 Formalization Target)
1. **Destination Precision**: $M \ge A + q_t$ is necessary and sufficient for full source cylinder determinism modulo $2^{q_t}$.
2. **Same-Form Eigenidentity**: $b_p H_p(F_p(n)) = a_p H_p(n)$.
3. **Cross-Form Identity**: $b_q H_p(F_q(n)) = a_q H_p(n) + \Delta_{p,q}$.
4. **Affine Commutator Identity**: $b_p b_q (F_{q,p}(n) - F_{p,q}(n)) = \Delta_{p,q}$.
5. **Common-Center Criterion**: $\Delta_{p,q} = 0 \iff x_p^* = x_q^*$.

### Competing Proof Architectures (Phase 7.3D)
1. **Path-Complete Graph Lyapunov Rankings** (`path_complete_ranking_v1`)
2. **Disjunctive Transition Invariants** (`disjunctive_transition_invariant_v1`)
3. **Lexicographic & Multiphase Rankings**
4. **Size-Change Termination (SCT)**

---

## 6. Phase 7.3 Sub-Phase Roadmap

```text
Phase 7.3A: Generic Affine Interaction & Symbolic Theorem Kernel
Phase 7.3B: Minimal Single-Coordinate Ultrametric Register Machine (L_u(n) = 11n+19)
Phase 7.3C: Symbolic Return-Language & Entropy Probe (s ∈ {u,v}^≤12)
Phase 7.3D: Four Competing Proof Architectures on Target A (u/v core)
Phase 7.3E: Target Expansion (Minimal Core -> Target B -> Target C)
```


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

---

# Source Document: `03_phase7x_implementation_and_experiments.md`

# Phase 7.3 Implementation and Experiment Plan

## 1. Milestone Title

**Phase 7.3 — Affine Interaction, Ultrametric Register Machine, and Symbolic Return Invariants**

## 2. Primary Objective

Build and verify a compact, switching-sensitive ultrametric invariant for the verified Phase 7.2 noncommuting branching core at state $Q_1$:
- $u = [1,1,2]$
- $v = w_1 w_2 = [1,1,2,1,2,2]$

---

# Source Document: `04_certificate_schemas_and_verification.md`

# Phase 7.3 Certificate Schemas and Verification

Schemas include `destination_precision_v1`, `affine_commutator_v1`, `cross_linear_form_transition_v1`, `resonance_cylinder_v1`, `based_switching_core_v1`, `finite_switching_language_v1`, `path_complete_ranking_v1`, `disjunctive_transition_invariant_v1`, and `language_growth_report_v1`.

---

# Source Document: `05_claims_registry_and_review_gates.md`

# Phase 7.3 Claims Registry and Review Gates

Includes candidate claims CLM-P7X-PRECISION-001 through CLM-P7X-RESONANCE-001, and sub-phase review gates Gate 7.3A through Gate 7.3E.

---

# Source Document: `06_phase72_handoff_and_migration.md`

# Phase 7.2 Handoff and Milestone Completion Summary

Phase 7.2 established a sound noncommuting branching core ($u, v$) at $Q_1$. Phase 7.3 integrates Phase 7X as its mathematical and verification engine.
