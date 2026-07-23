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
