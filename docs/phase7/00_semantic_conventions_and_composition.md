# Phase 7.3-0: Semantic Conventions & Sequence Composition Specification

## 1. Sequence Composition Order Standard

All macro sequence computations in the Collatz Research Workbench adhere strictly to **left-to-right evaluation order**:

$$\text{Execution sequence } S = [s_1, s_2, \dots, s_r] \implies F_S(n) = \left(F_{s_r} \circ \dots \circ F_{s_2} \circ F_{s_1}\right)(n)$$

- In Rust: `ExecutionSequence::new(vec![s1, s2]).apply_left_to_right(n)` applies $s_1$ first, then $s_2$.
- In code API: `u.then(v)` specifies that macro $u$ is executed first, followed by macro $v$.
- Bare ambiguous strings (e.g. `"uv"`) without explicit directional context are prohibited in schema definitions and formal log outputs.

---

## 2. Four Separate Mathematical Concepts & Types

The framework enforces strict structural separation between four distinct mathematical concepts:

### 1. `ExactWordCylinder` (Crate: `collatz-affine`)
Represents the exact congruence class of initial states $n$ that force the exact execution of a valuation word $w$ (i.e. every division in $w$ matches exact odd/even parity steps).
- *Does not enforce or imply that the target state lands back in a specific base set.*

### 2. `StateMembership` (Crate: `collatz-cegar`)
Represents set membership in a specific residue class (e.g., $Q_1 = 7 \pmod{32}$).

### 3. `BasedReturnCylinder` (Crate: `collatz-cegar`)
Represents the exact congruence class of states in a base class $Q_i$ that execute macro-word $w$ and return to base class $Q_i$.

### 4. `GuardedPathCylinder` (Crate: `collatz-cegar`)
Represents the complete, path-first guarded return cylinder for a sequence of macrosteps $[s_1, s_2, \dots, s_r]$.
- Constructed by composing the path, intersecting every intermediate guard ($n_0 \in Q_{i_0}, n_1 \in Q_{i_1}, \dots, n_r \in Q_{i_r}$), verifying intermediate positivity, and asserting final state membership.

---

## 3. Headline Benchmark Cylinders on $Q_1 = 7 \pmod{32}$

Given $u = [1,1,2]$ and $v = [1,1,2,1,2,2]$:

| Execution Sequence | Affine Map $F_{[s]}(n) = \frac{A n + C}{B}$ | Exact Valuation-Word Cylinder | Complete Guarded $Q_1$-Return Cylinder |
| :--- | :--- | :--- | :--- |
| `[u, v]` | $\frac{19683 n + 27947}{8192}$ | $1767 \pmod{16384}$ | **$214759 \pmod{262144}$** |
| `[v, u]` | $\frac{19683 n + 33515}{8192}$ | $1959 \pmod{16384}$ | **$1959 \pmod{262144}$** |

### Affine Commutator Difference
$$C_{[v,u]} - C_{[u,v]} = 33515 - 27947 = 5568 = -\Delta_{u,v}$$

### Essential Cylinder Distinctions & Counterexamples
1. **$1767 \pmod{16384}$**: Executes exact $u$ then $v$, but $1767 \xrightarrow{u} 2983 \equiv 7 \pmod{32}$ and $2983 \xrightarrow{v} 4249 \equiv 25 \pmod{32} \neq 7 \pmod{32}$. It is an exact word cylinder, **not** a complete $Q_1 \to Q_1 \to Q_1$ guarded path cylinder.
2. **$18343 \pmod{16384}$**: Note $18343 = 1959 + 16384 \in \text{ExactWord}([v,u])$. While $1959$ completes the $Q_1 \xrightarrow{v} Q_1 \xrightarrow{u} Q_1$ path, $18343$ lands in $44077 \equiv 13 \pmod{32} \neq 7 \pmod{32}$. Therefore, representative success on $1959$ does not imply universal cylinder return for $1959 \pmod{16384}$. The complete guarded return cylinder requires modulus $262144$.

---

## 4. Schema-Level Representation (`left_to_right_v1`)

All proof objects containing sequence compositions must specify:
```json
{
  "execution_semantics": "left_to_right_v1",
  "steps": [
    { "symbol": "u", "valuation_word": [1, 1, 2] },
    { "symbol": "v", "valuation_word": [1, 1, 2, 1, 2, 2] }
  ],
  "flattened_valuation_word": [1, 1, 2, 1, 1, 2, 1, 2, 2]
}
```

The independent verifier (`collatz-cert`) must recompute the flattened valuation word, composite affine form, intermediate guards, and cylinder claims directly from `steps` and reject unannotated or bare string paths.
