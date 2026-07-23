# Phase 7.3 Research Direction Overview
## Affine Interaction, Ultrametric Fuel, and Symbolic Return Dynamics

---

## 1. Why this direction exists

Phase 7.2 completed with a major discovery: destination-aware refinement successfully produced a **sound noncommuting guarded-branching target** at state $Q_1 = 7 + 32\mathbb{N}_0$ with based closed walks:
- $u = [1,1,2]$
- $v = w_1 w_2 = [1,1,2,1,2,2]$

Both $uv$ and $vu$ possess exact positive-integer path cylinders ($1767 \pmod{16384}$ and $1959 \pmod{16384}$ respectively under left-to-right composition), confirming genuine noncommuting branching ($uv \neq vu$).

However, raw residue states do not provide a compact switching invariant. Each macrostep consumes 2-adic bits, while static residue partitions forget information exposed by subsequent divisions.

Phase 7.3 replaces heuristic residue refinement with an **exact integer reference register machine** ($k$-machine) for $Q_1$, constructs proved quotient abstractions, and analyzes the infinite symbolic dynamics of $u/v$ switching.

---

## 2. Primary Research Questions

1. **Exact Reference Semantics**: What is the minimal exact integer register machine governing based returns to $Q_1$?
2. **Quotient Simulation**: Is the ultrametric single-coordinate abstraction a sound simulation quotient of the exact $k$-machine?
3. **Finite vs. Infinite Language**: Is the finite return language $L_{\text{finite}} = \{u,v\}^*$, and what structural constraints govern the infinite positive realization language $L_{\text{positive},\omega}$?
4. **Ultimately Periodic Exclusion**: Can any positive integer realize an ultimately periodic infinite $u/v$ switching sequence?
5. **2-Adic IFS Fractal Geometry**: What are the 2-adic Haar measure and Hausdorff dimension of the set of states supporting infinite $u/v$ switching?
6. **Residue-Lifting Transducer**: Can the positive integer realization problem be reduced to an eventual-zero output property on a binary residue-lifting transducer?
7. **3-Adic Branch History Signal**: Does the output coordinate retain a strong 3-adic branch history signal suitable for lexicographic/multiphase termination rankings?

---

## 3. Mathematical Objects & Benchmark Parameters

### Benchmark Switching Pair $(u, v)$ at $Q_1$
- $u = [1,1,2]$: $a_u = 27, b_u = 16, c_u = 19, d_u = -11$. Linear form $L_u(n) = 11n + 19$.
- $v = [1,1,2,1,2,2]$: $a_v = 729, b_v = 512, c_v = 881, d_v = -217$. Linear form $L_v(n) = 217n + 881$.

### Commutator Data
$$\Delta_{u,v} = d_u c_v - d_v c_u = (-11)(881) - (-217)(19) = -5568 = -2^6 \cdot 87, \qquad \kappa_{u,v} = v_2(\Delta_{u,v}) = 6$$

### Affine Commutator Identity
$$b_u b_v \big(F_{uv}(n) - F_{vu}(n)\big) = 8192 \left(-\frac{5568}{8192}\right) = -5568$$

---

## 4. Exact Integer Reference Register Machine ($k$-Machine)

Every state in $Q_1$ is written as $n = 7 + 32k$ ($k \ge 0$). Direct substitution into $F_u(n)$ and $F_v(n)$ yields the exact, deterministic partial register machine:

$$\text{U-return: guard } k \equiv 7 \pmod{16}, \quad k' = \frac{27k + 3}{16}$$
$$\text{V-return: guard } k \equiv 61 \pmod{512}, \quad k' = \frac{729k + 75}{512}$$

### Key Distinction: Valuation Word vs. Based Return
- Executing valuation word $v$ requires $x = v_2(L_u(n)) = 6$ and $U \equiv 1 \pmod{16}$.
- Returning to $Q_1$ requires the stronger guard: $x = 6$ and $U \equiv 81 \pmod{256} \iff k \equiv 61 \pmod{512}$.
- All abstractions are verified against this exact $k$-machine reference semantics.

---

## 5. Infinite Dynamics & Structural Theorems

1. **Finite Full-Return Language Theorem**:
   $$\forall s \in \{u,v\}^*, \quad s \text{ has a nonempty positive guarded return cylinder in } Q_1. \implies L_{\text{finite}} = \{u,v\}^*$$

2. **No Ultimately Periodic Positive Switching Path**:
   No positive integer realizes an ultimately periodic infinite $u/v$ path ($\alpha \beta^\omega$). Every composite macrostep $\beta$ is real-expanding ($a_\beta > b_\beta > 0, c_\beta > 0$), forcing its unique fixed point $n_\beta^* < 0$. Consequently, $L_{\text{positive},\omega}$ is either empty or non-$\omega$-regular.

3. **2-Adic Fractal Geometry**:
   - 2-adic Haar measure: $\mu(X) = 0$
   - 2-adic Hausdorff dimension: $s \approx 0.1625357554$, unique real solution to $2^{-4s} + 2^{-9s} = 1$.

4. **Residue-Lifting Transducer**:
   Transforms the positive realization problem from 2-adic path existence to asking whether the binary output of a residue-lifting transducer is eventually zero.

---

## 6. Revised Sub-Phase Roadmap

```text
Phase 7.3-0: Semantic Normalization & Composition Conventions
Phase 7.3A:  Algebra & Exact Reference Semantics (k-machine, n/k/L_u equivalence, periodic exclusion)
Phase 7.3B:  Proved Quotient Abstraction & 3-Adic Signal (x-partitions, simulation theorem, U ≡ 81 mod 256)
Phase 7.3C:  Infinite Symbolic Dynamics & Transducer (finite full shift, transducer eventual zero, Haar 0, s ≈ 0.1625, M_r bounds)
Phase 7.3D:  Reordered Proof System Hierarchy (source height -> monotonicity -> lexicographic/multiphase -> disjunctive -> path-complete -> SCT)
Phase 7.3E:  Target Expansion Discipline (Target A -> Target B -> Target C)
```
