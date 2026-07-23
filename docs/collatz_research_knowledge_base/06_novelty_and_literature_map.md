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
