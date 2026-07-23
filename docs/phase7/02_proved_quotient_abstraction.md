# Phase 7.3B: Proved Quotient Abstraction & 3-Adic Branch History Signal

## 1. Simulation Theorem Requirement

Every abstract domain (e.g., ultrametric $x$-partition, residue abstraction, interval abstraction) must be proved to be a **sound simulation quotient** of the exact reference $k$-machine:

$$\text{Concrete Transition: } k \xrightarrow{\sigma} k' \implies \text{Abstract Transition: } \alpha(k) \xrightarrow{\sigma} \alpha(k')$$

No abstract graph or transition relation is admissible unless this simulation theorem holds.

---

## 2. Refined Ultrametric $x$-Partition

Let $x = v_2(L_u(n)) = 5 + v_2(11k + 3)$. The 3-region split $(x<6, x=6, x>6)$ is refined into 6 abstract states to avoid conflating membership, execution, and closed returns:

| Abstract Region | Concrete Condition on $k$ | Semantics |
| :--- | :--- | :--- |
| $x < 5$ | $n \notin Q_1$ | Outside $Q_1$ |
| $x = 5$ | $k \equiv 1 \pmod 2$ | Inside $Q_1$, no returns enabled |
| $x = 6$ | $k \equiv 1 \pmod 4$ | Resonance layer ($v$-execution if $U \equiv 1 \pmod{16}$; based $v$-return iff $U \equiv 81 \pmod{256} \iff k \equiv 61 \pmod{512}$) |
| $x = 7$ | $k \equiv 5 \pmod 8$ | Inside $Q_1$, no returns enabled |
| $x = 8$ | $k \equiv 13 \pmod{16}$ | Inside $Q_1$, no returns enabled |
| $x \ge 9$ | $k \equiv 7 \pmod{16}$ | Enables based $u$-return |

---

## 3. Mandatory Verifier Enforcement Rule

The certificate verifier (`collatz-cert`) will explicitly reject any proof claiming that $U \equiv 1 \pmod{16}$ guarantees a based return to $Q_1$.

- $U \equiv 1 \pmod{16}$ guarantees exact valuation word execution of $v = [1,1,2,1,2,2]$.
- $U \equiv 81 \pmod{256} \iff k \equiv 61 \pmod{512}$ is **required** to guarantee return to $Q_1$.

---

## 4. The 3-Adic Branch History Signal

Evaluating the output $k'$ of the exact register machine exposes an unexpected branch-history marker in base 3.

### $u$-return Branch History
Writing $k = 7 + 16t$:
$$k' = 12 + 27t \implies k' \equiv 12 \pmod{27}$$

### $v$-return Branch History
Writing $k = 61 + 512t$:
$$k' = 87 + 729t \implies k' \equiv 87 \pmod{729}$$

### Behavior in $y = 11k + 3$ Coordinate
In the quotient coordinate $y = 11k + 3$:
- A $u$-return multiplies $y$ by $27/16$ (plus offset), increasing $v_3(y)$ by 3.
- A $v$-return resets $v_3(y')$ to 1 ($v_3(87 \cdot 11 + 3) = v_3(960) = 1$).

### Multi-Coordinate Feature Tuple
This secondary 3-adic signal is combined into a rich state tuple for lexicographic and multiphase termination rankings:
$$\left( v_2(L_u(n)), \; v_3(11k+3), \; \text{control\_phase} \right)$$
