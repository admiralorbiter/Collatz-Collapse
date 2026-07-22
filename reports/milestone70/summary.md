# Milestone 7.0 Expert Audit Final Package & Milestone 7.1 Architecture Roadmap

> **Current Project Status:** **Phase 7 Milestone 7.0 Infrastructure Prototype Complete — Target Failure Audited**  
> *The Milestone 7.0 target graph failed its mathematical audit. Milestone 7.1 is designated an **Abstraction-Discovery Phase** to synthesize the smallest quotient-sensitive state representation producing a genuinely branching, positive-integer-relevant SCC.*

---

## 1. Final Package Corrections Summary

### A. Non-Commuting Cycles Requirement ($uv \neq vu$)
- **Rule:** Simple 2-edge cycles $Q_1 \xrightarrow{u} Q_2 \xrightarrow{v} Q_1$ reduce to Phase 6D composite word $W = uv$.
- **Revised Milestone 7.1 Target Gate:** The target SCC must contain at least two repeatable return cycles with label words $u, v$ satisfying **$uv \neq vu$** (proving non-commutativity and non-equivalence to any single periodic word $W$).

### B. Quotient-Sensitive Refinement & Canonicalization
- **State Representation:** $Q = (q, r \bmod 2^m, t \bmod 2^h)$ where $n = r + 2^m t$.
- **Equivalence:** $(r \bmod 2^m, t \bmod 2^h) \iff n \bmod 2^{m+h}$.
- **Canonicalization Rule:** Use quotient tuple internally for transition synthesis, but flatten to $n \bmod 2^{m+h}$ for equality, hashing, and verification.

### C. Witness-Scoped Target Fields & Invalidated Target Artifact
- **Target File:** [`certificates/invalidated/milestone70_invalidated_target.json`](file:///c:/Users/admir/Github/Collatz-Collapse/certificates/invalidated/milestone70_invalidated_target.json)
- **Witness Fields:** `"witness_source": "7"`, `"witness_target_image": "13"`, `"witness_target_residue": "13"`, `"witness_target_modulus": 16`.
- **Universal Field:** `"universal_target_membership": false`.

### D. Claims Registry Registry Separation (`claims/claims.toml`)
- `CLM-SCT-GRAPH-ALGEBRA-001` (Category: Verified Finite Theorem, Status: **Verified**, Limitation: Proves graph closure algebra only).
- `CLM-SCT-M70-TARGET-001` (Category: Domain-Scoped Certificate, Status: **Invalidated**, Reason: Target state membership failed).

---

## 2. Four Independent Validity Layers

Every Phase 7 result must independently report all 4 validity layers:

| Validity Layer | Milestone 7.0 Audit Status | Description |
| :--- | :---: | :--- |
| **1. Arithmetic Validity** | **VALID** | Exact words, affine constants, starting cylinders, and fixed points ($L_2(n) = 5n - 23$). |
| **2. Abstract Semantic Validity** | **INVALIDATED** | Target state membership failed ($13 \not\equiv 43 \pmod{64}$, $37 \not\equiv 7 \pmod{32}$). |
| **3. Termination-Algebra Validity** | **VALID** | Transitive closure and idempotent graph algebra (6 graphs generated). |
| **4. Positive-Integer Scope Validity** | **UNRESOLVED** | Static low-modulus residue states do not preserve target quotient bits. |

---

## 3. Mandatory Semantic Gate & Milestone 7.1 Execution Roadmap

No candidate edge enters a proof graph until passing the **Mandatory Semantic Gate**:
$$\forall n \in \text{Guard}(Q_s), \qquad F_w(n) \in \text{Guard}(Q_t)$$

### Milestone 7.1 Action Sequence:
1. **Symbolic Cylinder Image Derivation (`CylinderImage`):** $F_w(n) = \frac{3^k r + c}{2^A} + 3^k 2^{m-A} t$.
2. **Reachable Graph Construction:** Build graph from mechanically verified cylinder images allowing sound finite target unions.
3. **Non-Commuting Cycle Detection:** Find repeatable return cycles $u, v$ satisfying $uv \neq vu$.
4. **Feature Relation Counterexample Search:** Search pairs of features and edge transitions, storing smallest counterexamples for refuted relations.
5. **SCT Verification:** Run SCT strictly over verified edge relations.
