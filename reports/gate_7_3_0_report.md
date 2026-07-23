# Gate 7.3-0 Verification Report: Semantic Normalization

**Date:** July 2026  
**Status:** PASSED  
**Gate:** 7.3-0 (Semantic Normalization & Composition Conventions)

---

## Executive Summary

Phase 7.3-0 has established strict semantic normalization across the Collatz Research Workbench. It eliminates all sequence composition notation ambiguity, enforces path-first guarded return cylinder semantics, separates exact-word and guarded-path concepts across crate trust boundaries, and passes all differential oracle and property-based test suites.

---

## Verified Completion Criteria (9/9 Steps Completed)

### 1. Composition Direction Standard (Step 7.3-0.1)
- Frozen standard: `apply_sequence([u, v])` applies $u$ first, then $v$ (left-to-right evaluation order).
- Formalized in [docs/phase7/00_semantic_conventions_and_composition.md](file:///c:/Users/admir/Github/Collatz-Collapse/docs/phase7/00_semantic_conventions_and_composition.md).

### 2. Trust Boundary & Struct Types (Step 7.3-0.2)
- **`collatz-core`**: Pure arithmetic kernel (`ValuationWord`, concrete steps).
- **`collatz-affine`**: `CanonicalCylinder`, `ExactWordCylinder`, `ExecutionSequence<T>`.
- **`collatz-cegar`**: `StateMembership`, `BasedReturnCylinder`, `GuardCheckpoint`, `GuardedPathCylinder`.
- **`collatz-cert`**: `left_to_right_v1` proof-object schemas (`GuardedPathCertificateJson`).

### 3. Directional Combinator (Step 7.3-0.3)
- Implemented `ExecutionSequence<T>` with `apply_left_to_right` and `u.then(v)`. Prohibited `Mul` implementation for affine composition.

### 4. Path-First Cylinder Derivation (Step 7.3-0.4)
- Universal determinism condition $M \ge A + q_{\text{target}}$ enforced in `GuardedPathCylinder::compute`.

### 5. Schema Normalization (Step 7.3-0.5)
- Registered `left_to_right_v1` schema format with explicit `steps` and `flattened_valuation_word`.

### 6. Headline & Negative Regression Tests (Step 7.3-0.6)
Verified headline benchmark results on $Q_1 = 7 \pmod{32}$ for $u = [1,1,2]$ and $v = [1,1,2,1,2,2]$:

| Execution Sequence | Affine Form $F_{[s]}(n)$ | Exact Valuation-Word Cylinder | Complete Guarded $Q_1$-Return Cylinder |
| :--- | :--- | :--- | :--- |
| `[u, v]` | $\frac{19683 n + 27947}{8192}$ | $1767 \pmod{16384}$ | **$214759 \pmod{262144}$** |
| `[v, u]` | $\frac{19683 n + 33515}{8192}$ | $1959 \pmod{16384}$ | **$1959 \pmod{262144}$** |

- **Commutator Identity Match**: $C_{[v,u]} - C_{[u,v]} = 33515 - 27947 = 5568 = -\Delta_{u,v}$.
- **Negative Regressions Verified**:
  - $1767 \in \text{ExactWord}([u,v])$, but lands in $4249 \equiv 25 \pmod{32} \neq 7$ (fails $Q_1$ return).
  - $18343 = 1959 + 16384 \in \text{ExactWord}([v,u])$, but lands in $44077 \equiv 13 \pmod{32} \neq 7$ (fails $Q_1$ return).
- **Strict Inclusion Verified**: `BasedReturnCylinder(v, Q1) ⊊ ExactWordCylinder(v)`.

### 7. Independent Differential Python Oracle (Step 7.3-0.7)
- [scripts/independent_composition_oracle.py](file:///c:/Users/admir/Github/Collatz-Collapse/scripts/independent_composition_oracle.py) executed independently and passed with 100% agreement on all constants and cylinders.

### 8. Property-Based Testing (Step 7.3-0.8)
- `proptest` suite in `crates/collatz-affine/tests/proptest_composition.rs` passed 256 random composition tests verifying left-to-right evaluation, flattening, and associativity.

### 9. Verification & Audit (Step 7.3-0.9)
- All Phase 7 documentation and claims updated and verified.

---

## Conclusion

Gate 7.3-0 is **FULLY PASSED**. The codebase is now mathematically normalized and ready for Phase 7.3A (Algebra & Exact Reference Semantics).
