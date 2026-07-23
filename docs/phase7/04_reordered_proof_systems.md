# Phase 7.3D: Reordered Proof Systems & Certificate Schemas

## 1. Reordered Proof Hierarchy

The proof system architectures are reordered by analytical power for guarded, non-monotonic switching systems:

1. **Direct Source-Height / Cylinder-Floor Bounds** (`source_height_bound_v1`)
   - Direct lower bounds on source integer height $M_r$ per path length $r$.
2. **Monotonicity Constraints**
   - Strictly generalizes Size-Change Termination (SCT) by incorporating numerical order relations on source/target state coordinates.
3. **Lexicographic & Multiphase Rankings**
   - Leverages multi-coordinate tuples $(v_2(L_u(n)), v_3(11k+3), \text{control\_phase})$.
4. **Disjunctive Transition Invariants** (`disjunctive_transition_invariant_v1`)
   - Finite union of well-founded relations covering the transitive closure of switching transitions.
5. **Path-Complete Graph Lyapunov Rankings** (`path_complete_ranking_v1`)
   - Multiple Lyapunov functions attached to nodes/edges of the verified guard transition graph.
6. **Classical SCT Projection**
   - Size-Change graphs checking multiset ordering.

---

## 2. Complete Registry of Phase 7.3 Certificate Schemas

All verification certificates are serialized as JSON and checked by `collatz-cert`:

| Schema Name | Sub-Phase | Description |
| :--- | :--- | :--- |
| `q1_quotient_machine_v1` | 7.3A | Exact $k$-register machine semantics ($k \equiv 7 \pmod{16}$ and $k \equiv 61 \pmod{512}$) |
| `based_return_transition_v1` | 7.3B | Verified return transition with $U \equiv 81 \pmod{256}$ guard check |
| `finite_full_shift_return_v1` | 7.3C | Certificate proving $L_{\text{finite}} = \{u,v\}^*$ via cylinder construction |
| `ultimately_periodic_exclusion_v1` | 7.3A | Proof certificate for non-existence of positive ultimately periodic paths |
| `residue_lifting_transducer_v1` | 7.3C | Residue-lifting transducer specification and binary output stream check |
| `source_height_bound_v1` | 7.3C/D | Divergence certificate $M_r \ge f(r)$ ruling out positive realizations |
| `switching_limit_set_measure_v1` | 7.3C | Certificate verifying 2-adic Haar measure $\mu(X) = 0$ |
| `switching_limit_set_dimension_v1` | 7.3C | Certificate verifying 2-adic Hausdorff dimension $s \approx 0.1625357554$ |
| `abstraction_simulation_v1` | 7.3B | Simulation theorem certificate mapping concrete $k$-machine to abstract quotient |

---

## 3. Verifier Gate Criteria (Gate 7.3D)

To pass Gate 7.3D:
1. `collatz-cert` must parse and validate all 9 schemas.
2. The verifier must reject certificates attempting to pass $U \equiv 1 \pmod{16}$ as a proof of based $v$-return.
3. Every abstract graph must be accompanied by an `abstraction_simulation_v1` certificate.
