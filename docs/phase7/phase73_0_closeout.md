# Phase 7.3-0 Closeout Report: Semantic Normalization & Trust Verification

## 1. Environment & Complete Reproducibility Metadata

- **Audit Date**: 2026-07-22T19:22:14-05:00
- **Repository**: `admiralorbiter/Collatz-Collapse`
- **Git Commit SHA**: `15b4347caa846ebf69ceaf6406fc4aa7d6eed9b8`
- **Working Tree Status**: Active Phase 7.3-0 verification patch (61 modified source files, 15 untracked artifacts/docs)
- **Rust Toolchain**: `rustc 1.91.0 (f8297e351 2025-10-28)`
  ```text
  rustc 1.91.0 (f8297e351 2025-10-28)
  binary: rustc
  commit-hash: f8297e351a40c1439a467bbbb6879088047f50b3
  commit-date: 2025-10-28
  host: x86_64-pc-windows-msvc
  release: 1.91.0
  LLVM version: 21.1.2
  ```
- **Cargo Version**: `cargo 1.91.0 (ea2d97820 2025-10-10)`
- **Python Version**: `Python 3.12.0`
- **Cargo.lock SHA-256**: `F4BB3C125E4DD55DD49DC1374043CDE49F2B2BE61DF909D2F31E1A299B22E50F`
- **Toolchain Pinning**: `rust-toolchain.toml` not present in repository root; environment anchored by local `rustc 1.91.0`.
- **Exact Verification Commands Executed**:
  ```powershell
  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace
  python scripts/independent_composition_oracle.py
  ```
- **Workspace Test Counts**:
  - Pre-Phase 7.3-0 Total Tests: 128
  - Post-Phase 7.3-0 Total Tests: 142 passed; 0 failed; 0 ignored.
  - Certificate Fixtures Status: Verified & 100% backward compatible.

---

## 2. Canonical Arithmetic Result Table & Sign Convention

Under left-to-right execution `apply_sequence([s_1, s_2])` ($F_{s_2} \circ F_{s_1}$):

| Sequence | Execution Meaning | Composite $a = 3^k$ | Composite $b = 2^A$ | Composite $c$ | Exact Valuation-Word Cylinder | Complete Guarded $Q_1$-Path Cylinder |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `[u, v]` | $u$ first, then $v$ | 19683 | 8192 | 27947 | $1767 \pmod{16384}$ | **$214759 \pmod{262144}$** |
| `[v, u]` | $v$ first, then $u$ | 19683 | 8192 | 33515 | $1959 \pmod{16384}$ | **$1959 \pmod{262144}$** |

### Commutator Constant & Sign Convention
$$C_{[v,u]} - C_{[u,v]} = 33515 - 27947 = 5568$$

- **Interaction Constant**: $\Delta_{u,v} = d_u c_v - d_v c_u = (-11)(881) - (-217)(19) = -5568$.
- **Explicit Subtraction Order**: $C_{[v,u]} - C_{[u,v]} = -\Delta_{u,v} = 5568$.
- The positive difference $5568$ corresponds directly to evaluating $F_u \circ F_v - F_v \circ F_u$ on the numerator constant, exactly matching $-\Delta_{u,v}$.

---

## 3. Explicit Universal Checkpoint Evidence & Image-Cylinder Formulas

Machine-produced proof artifacts exported to `artifacts/phase73_0/`:
- [artifacts/phase73_0/guarded_path_uv.json](file:///c:/Users/admir/Github/Collatz-Collapse/artifacts/phase73_0/guarded_path_uv.json)
- [artifacts/phase73_0/guarded_path_vu.json](file:///c:/Users/admir/Github/Collatz-Collapse/artifacts/phase73_0/guarded_path_vu.json)

For each step along the closed path, universal verification proves exact valuation execution, target base state membership, and precision sufficiency across the **entire parameter space $t \in \mathbb{Z}_{\ge 0}$**.

### A. Sequence `[u, v]` ($n = 214759 + 262144 t = 214759 + 2^{18} t$)

1. **Source Base State**: $n = 214759 + 262144 t \equiv 214759 \equiv 7 \pmod{32} \implies n \in Q_1$ universally for all $t$.
2. **Step 1 ($u = [1,1,2]$)**:
   - **Induced Image Formula**:
     $$F_u(n) = \frac{27(214759 + 2^{18} t) + 19}{16} = \frac{5798530 + 7077888 t}{16} = 362407 + 442368 t$$
   - **Exactness Check**:
     - Numerator $5798530 + 7077888 t = 16 \times (362407 + 442368 t)$ is divisible by $2^4 = 16$.
     - $\frac{5798530}{16} = 362407 \equiv 1 \pmod 2$ (odd quotient $\implies$ total 2-adic valuation $A_u = 4$ exactly).
   - **Target Membership Check**:
     $$442368 t = 32 \times (13824 t) \equiv 0 \pmod{32}, \quad 362407 = 32 \times (11325) + 7 \equiv 7 \pmod{32}$$
     $$\implies F_u(n) \equiv 7 \pmod{32} \implies F_u(n) \in Q_1 \quad \text{universally for all } t.$$
3. **Step 2 ($v = [1,1,2,1,2,2]$)**:
   - **Induced Image Formula**:
     $$F_v(F_u(n)) = \frac{729(362407 + 442368 t) + 881}{512} = \frac{264195584 + 322486272 t}{512} = 516007 + 629856 t$$
   - **Exactness Check**:
     - Numerator $264195584 + 322486272 t = 512 \times (516007 + 629856 t)$ is divisible by $2^9 = 512$.
     - $\frac{264195584}{512} = 516007 \equiv 1 \pmod 2$ (odd quotient $\implies$ total 2-adic valuation $A_v = 9$ exactly).
   - **Target Membership Check**:
     $$629856 t = 32 \times (19683 t) \equiv 0 \pmod{32}, \quad 516007 = 32 \times (16125) + 7 \equiv 7 \pmod{32}$$
     $$\implies F_v(F_u(n)) \equiv 7 \pmod{32} \implies F_v(F_u(n)) \in Q_1 \quad \text{universally for all } t.$$

### B. Sequence `[v, u]` ($n = 1959 + 262144 t = 1959 + 2^{18} t$)

1. **Source Base State**: $n = 1959 + 262144 t \equiv 1959 \equiv 7 \pmod{32} \implies n \in Q_1$ universally for all $t$.
2. **Step 1 ($v = [1,1,2,1,2,2]$)**:
   - **Induced Image Formula**:
     $$F_v(n) = \frac{729(1959 + 2^{18} t) + 881}{512} = \frac{1428992 + 191102976 t}{512} = 2791 + 373248 t$$
   - **Exactness Check**:
     - Numerator $1428992 / 512 = 2791$ (odd $\implies A_v = 9$ exactly).
   - **Target Membership Check**:
     $$373248 t = 32 \times (11664 t) \equiv 0 \pmod{32}, \quad 2791 = 32 \times (87) + 7 \equiv 7 \pmod{32}$$
     $$\implies F_v(n) \in Q_1 \quad \text{universally for all } t.$$
3. **Step 2 ($u = [1,1,2]$)**:
   - **Induced Image Formula**:
     $$F_u(F_v(n)) = \frac{27(2791 + 373248 t) + 19}{16} = \frac{75376 + 10077696 t}{16} = 4711 + 629856 t$$
   - **Exactness Check**:
     - Numerator $75376 / 16 = 4711$ (odd $\implies A_u = 4$ exactly).
   - **Target Membership Check**:
     $$629856 t = 32 \times (19683 t) \equiv 0 \pmod{32}, \quad 4711 = 32 \times (147) + 7 \equiv 7 \pmod{32}$$
     $$\implies F_u(F_v(n)) \in Q_1 \quad \text{universally for all } t.$$

---

## 4. Single-Step $u$ vs. $v$ Exact-Word vs. Based-Return Classifications

Phase 7.3A requires complete, symmetric quotient register machine specifications for both alphabet symbols $u = [1,1,2]$ and $v = [1,1,2,1,2,2]$.

### A. Single-Step $v = [1,1,2,1,2,2]$ Classification
- **Exact $v$-execution**:
  $$\text{ExactWord}(v): n \equiv 935 \pmod{1024} \quad (k \equiv 29 \pmod{32}, U \equiv 1 \pmod{16})$$
- **Based $v$-return to $Q_1$**:
  $$\text{BasedReturn}(v, Q_1): n \equiv 1959 \pmod{16384} \quad (k \equiv 61 \pmod{512}, U \equiv 81 \pmod{256})$$
- **Strict Containment**: $\text{BasedReturn}(v, Q_1) \subsetneq \text{ExactWord}(v)$.
- **Negative Witness**:
  $$n = 935 \in \text{ExactWord}(v), \quad F_v(935) = \frac{729(935) + 881}{512} = \frac{682496}{512} = 1333$$
  $$1333 = 32 \times (41) + 21 \equiv 21 \pmod{32} \neq 7 \pmod{32} \implies 935 \notin \text{BasedReturn}(v, Q_1).$$

### B. Single-Step $u = [1,1,2]$ Classification
- **Exact $u$-execution**:
  $$\text{ExactWord}(u): n \equiv 7 \pmod{32} \quad (\text{coincides with entire } Q_1 \text{ base state})$$
- **Based $u$-return to $Q_1$**:
  Writing $n = 7 + 32k \implies F_u(n) = \frac{27(7 + 32k) + 19}{16} = 13 + 54k$.
  For $F_u(n) \in Q_1$, we require:
  $$13 + 54k \equiv 7 \pmod{32} \implies 54k \equiv 26 \pmod{32} \implies 11k \equiv 13 \pmod{16} \implies k \equiv 7 \pmod{16}.$$
  Substituting $k = 7 + 16k'$ into $n = 7 + 32k$:
  $$n = 7 + 32(7 + 16k') = 231 + 512 k' \implies \text{BasedReturn}(u, Q_1): n \equiv 231 \pmod{512}.$$
- **Strict Containment**: $\text{BasedReturn}(u, Q_1) \subsetneq \text{ExactWord}(u)$ (mod 512 vs. mod 32).
- **Negative Witness**:
  $$n = 7 \in \text{ExactWord}(u), \quad F_u(7) = \frac{27(7) + 19}{16} = 13 \equiv 13 \pmod{32} \neq 7 \pmod{32} \implies 7 \notin \text{BasedReturn}(u, Q_1).$$

### C. Frozen Quotient Register Transitions for Phase 7.3A
With $n = 7 + 32k$, the closed quotient register transitions $k \mapsto k'$ are:
- **Symbol $u$**:
  $$k \equiv 7 \pmod{16}, \quad k' = \frac{27k + 3}{16}$$
- **Symbol $v$**:
  $$k \equiv 61 \pmod{512}, \quad k' = \frac{729k + 75}{512}$$

---

## 5. Negative Regression Results

All 4 negative test cases pass strictly:

1. **Case A (Exact word is not a guarded return)**:
   - $n = 1767 \in \text{ExactWord}([u,v])$ (mod 16384).
   - $1767 \xrightarrow{u} 2983 \in Q_1$, but $2983 \xrightarrow{v} 4249 \equiv 25 \pmod{32} \notin Q_1$.
   - Outcome: `ExactWord([u,v])` PASS; `GuardedPath([u,v], Q1)` REJECT.

2. **Case B (Representative success is not universal proof)**:
   - Both $1959$ and $18343 = 1959 + 16384$ belong to $\text{ExactWord}([v,u])$ (mod 16384).
   - $1959 \xrightarrow{v,u} 4711 \in Q_1$ (PASS).
   - $18343 \xrightarrow{v,u} 44077 \equiv 13 \pmod{32} \notin Q_1$ (REJECT).
   - Outcome: $1959 \in \text{GuardedPath}([v,u] \text{ mod } 262144)$; $18343 \notin \text{GuardedPath}([v,u] \text{ mod } 262144)$.

3. **Case C (Direction mutation)**:
   - Swapping $[u,v]$ to $[v,u]$ alters constant ($27947 \to 33515$), exact cylinder ($1767 \to 1959 \pmod{16384}$), and guarded path cylinder ($214759 \to 1959 \pmod{262144}$).

4. **Case D (Ambiguous schema rejection)**:
   - `collatz-cert` verifier rejects bare fields like `{"path": "uv"}` or `{"composition": "u o v"}` and requires explicit `left_to_right_v1` schema with structured `steps` arrays.

---

## 6. Rust/Python Differential Oracle Output

- **Rust Export**: [artifacts/phase73_0/rust_semantic_results.json](file:///c:/Users/admir/Github/Collatz-Collapse/artifacts/phase73_0/rust_semantic_results.json)
- **Python Export**: [artifacts/phase73_0/python_semantic_results.json](file:///c:/Users/admir/Github/Collatz-Collapse/artifacts/phase73_0/python_semantic_results.json)
- **Canonical Diff**: **EMPTY (100% Identical)**

---

## 7. Property-Test Summary

The `proptest` suite in `crates/collatz-affine/tests/proptest_composition.rs` was executed with 5,000 cases per property:

- **Pairs Checked**: 5,000 random valuation-word pairs ($w_1, w_2$).
- **Triples Checked**: 5,000 random valuation-word triples ($w_1, w_2, w_3$).
- **Cylinder Bounds Checked**: 5,000 random residue/modulus pairs.
- **Failures**: 0 failures.
- **Properties Verified**:
  - `apply([p,q], n) == F_q(F_p(n))`
  - `flatten([p,q]) == p ++ q`
  - `F_[p,q,r] == F_r o F_q o F_p`
  - Associativity: `combine([p,q,r]) == combine([combine([p,q]), r]) == combine([p, combine([q,r])])`
  - Empty sequence identity: $F_{[]} = \text{id}$.
  - Cylinder normalization ($0 \le r < m$, power of two modulus, canonical equivalence).

---

## 8. Full Workspace Regression Results & Dependency Boundary

### Build & Verification Commands
- `cargo fmt --all -- --check`: **PASS**
- `cargo clippy --workspace --all-targets -- -D warnings`: **PASS** (0 warnings)
- `cargo test --workspace`: **PASS** (142/142 tests passed)

### Dependency-Boundary Evidence
Running `cargo tree -p collatz-cert` confirms strict architecture isolation:
```text
collatz-cert v0.1.0
├── collatz-affine v0.1.0
│   └── collatz-core v0.1.0
├── collatz-core v0.1.0
├── num-bigint v0.4.8
├── num-rational v0.4.2
├── num-traits v0.2.19
├── serde v1.0.229
├── serde_json v1.0.151
├── sha2 v0.10.9
└── thiserror v1.0.69
```
- Zero dependency on `collatz-cegar`, search crates, SMT solvers, or floating-point code.

---

## 9. Documentation Migration Audit

A repository-wide search (`rg -n '"uv"|"vu"|uv_path|vu_path|F_uv|F_vu|exact path cylinder|1337'`) produced:

- **1337 Erroneous Value Replaced**: Replaced all occurrences of erroneous 1337 transcription with correct exact value $1333 \equiv 21 \pmod{32}$.
- **Migrated to Structured Sequence Notation**: All Phase 7.3 documentation now explicitly distinguishes:
  - Exact concatenated valuation-word cylinder: $1959 \pmod{16384}$
  - Complete guarded $Q_1$-return cylinder: $1959 \pmod{262144}$
- **Historical Code Qualified**: `GraphClosureEngine::verify_uv_realizability` doc comment in `crates/collatz-cegar/src/graph_closure_m72.rs` updated from "exact path cylinder" to "complete guarded path cylinder".

---

## 10. Certificate Round-Trip & Mutation Matrix

Artifact exported: [artifacts/phase73_0/mutation_test_report.json](file:///c:/Users/admir/Github/Collatz-Collapse/artifacts/phase73_0/mutation_test_report.json)

- Valid fixtures accepted: 100%
- Mutated fixtures rejected: 9/9 (100%)
- Unexpected accepts: 0
- Unexpected rejects: 0

---

## Move-Forward Gate Checklist

- [x] Correct $F_v(935) = 1333 \equiv 21 \pmod{32}$ verified in Rust test suite and documentation.
- [x] Single-step $u$-return classification ($231 \pmod{512}$) and quotient transition $k' = \frac{27k+3}{16}$ added and tested.
- [x] Rust and Python independently agree on all affine and cylinder results (canonical diff empty).
- [x] Exact-word and guarded-path objects separated at type and schema level.
- [x] Negative representative tests behave correctly ($1767$ fails final $Q_1$ return, $18343$ fails $Q_1$ return, $7$ fails $u$-return).
- [x] Explicit universal image-cylinder formulas ($362407 + 442368 t$ and $516007 + 629856 t$) documented.
- [x] Complete reproducibility metadata (Git SHA, rustc -Vv, Cargo.lock SHA-256) recorded.
- [x] Workspace build, clippy, fmt, and tests pass cleanly (142/142).

Phase 7.3-0 is **FROZEN AND CLOSED**. Ready to proceed to Phase 7.3A.
