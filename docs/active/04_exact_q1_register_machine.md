# Phase 7.3B-1: Exact $Q_1$ Register Machine & Reference Semantics

## 1. Parameterization of $Q_1$

All states in $Q_1 = \{ n \in \mathbb{N}^+ \mid n \equiv 7 \pmod{32} \}$ are represented by an exact non-negative integer coordinate $k$:
$$n = 32k + 7, \quad k \in \mathbb{Z}_{\ge 0}$$

Constants:
- `pub const Q1_RESIDUE: u64 = 7;`
- `pub const Q1_EXPONENT: u32 = 5;` (modulus $2^5 = 32$)

---

## 2. Generic Quotient-Return Theorem ($\eta_p$)

For a base cylinder $B = r \pmod{2^q}$ ($r = 7, q = 5$), write $n = 2^q k + r$.
A macrostep $p$ with affine form $F_p(n) = \frac{a_p n + c_p}{2^{A_p}}$ returns to the same base state when:
$$F_p(n) = 2^q k' + r \iff 2^{A_p} k' = a_p k + \eta_p$$

where:
$$\eta_p = \frac{7 a_p + c_p - 7 \cdot 2^{A_p}}{32}$$

For $Q_1$ ($r = 7, q = 5$):
- $\eta_u = \frac{7(27) + 19 - 7(16)}{32} = 3$.
- $\eta_v = \frac{7(729) + 881 - 7(512)}{32} = 75$.

The based-return guard is uniquely determined by:
$$k \equiv -\eta_p a_p^{-1} \pmod{2^{A_p}}$$

---

## 3. Derivation of $u$-return and $v$-return Guards

### $u$-return Derivation
Applying macro $u = [1,1,2]$ to $n = 32k + 7$:
$$F_u(n) = \frac{27(32k + 7) + 19}{16} = 54k + 13$$

- Guard: $k \equiv 7 \pmod{16} \iff n \equiv 231 \pmod{512}$.
- Register transition: $k' = \frac{27k + 3}{16}$ (for $k = 16m + 7 \implies k' = 27m + 12$).

### $v$-return Derivation
Applying macro $v = [1,1,2,1,2,2]$ to $n = 32k + 7$:
$$F_v(n) = \frac{729(32k + 7) + 881}{512} = \frac{729k + 187}{16}$$

- Guard: $k \equiv 61 \pmod{512} \iff n \equiv 1959 \pmod{16384}$.
- Register transition: $k' = \frac{729k + 75}{512}$ (for $k = 512m + 61 \implies k' = 729m + 87$).

---

## 4. Reference Register Machine Semantics

```text
U-return:
    guard k ≡ 7 mod 16
    k := (27k + 3) / 16

V-return:
    guard k ≡ 61 mod 512
    k := (729k + 75) / 512
```

### Three-Way Semantic Outcomes over $0 \le k < 512$
- $u$: 512 exact word, 32 based returns ($k \equiv 7 \pmod{16}$), 480 exact-but-leaving, 0 not exact.
- $v$: 16 exact word ($k \equiv 29 \pmod{32}$), 1 based return ($k = 61$), 15 exact-but-leaving, 496 not exact.

---

## 5. Topological Structure of Guard Tree: Measure-Zero Cantor Set

The quotient return guards form a **full disjoint nested binary tree** of 2-adic cylinders in $\mathbb{Z}_2$:
- $G_u = 7 + 16\mathbb{Z}_2 \implies \mu(G_u) = 2^{-4} = \frac{1}{16}$.
- $G_v = 61 + 512\mathbb{Z}_2 \implies \mu(G_v) = 2^{-9} = \frac{1}{512}$.
- Disjoint since $61 \equiv 13 \pmod{16} \neq 7 \pmod{16}$.

### Haar Measure and Hausdorff Dimension
- Combined single-step measure: $\mu(G_1) = 2^{-4} + 2^{-9} = \frac{33}{512}$.
- Combined depth-$r$ measure: $\mu(G_r) = \left(\frac{33}{512}\right)^r \xrightarrow{r \to \infty} 0$.
- **Infinite Limit Set**: $\mu(G_\infty) = 0$ (measure-zero Cantor-type subset of $\mathbb{Z}_2$).
- **2-Adic Hausdorff Dimension $d$**: Satisfies $2^{-4d} + 2^{-9d} = 1 \implies d \approx 0.1625357554$.

---

## 6. Theorem: Full Finite Switching Language & Periodic Path Divergence

### Theorem 7.3B.2 (Uniqueness of Finite Quotient Guards)
Every finite word $s \in \{u,v\}^*$ has a **unique** non-empty quotient guard cylinder $k \equiv r_s \pmod{2^{A(s)}}$.
Thus, the finite return language is $\mathcal{L}_{\text{finite}} = \{u, v\}^*$. Every finite word is positively realizable.

### Theorem 7.3B.4 (Periodic Path Divergence)
For any fixed non-empty word $s \in \{u,v\}^+$, the composite quotient map $T_s(k) = \frac{a_s k + \eta_s}{2^{A_s}}$ has a strictly negative rational fixed point $k^*_s = \frac{\eta_s}{2^{A_s} - a_s} < 0$.
The guard for $s^m$ is the $2^{m A_s}$-adic truncation of $k^*_s$.
Therefore, the least non-negative guard representative satisfies:
$$r_{s^m} \longrightarrow \infty \quad \text{as } m \to \infty$$
No positive integer can realize an ultimately periodic infinite switching tail $s^\omega$.

---

## 7. Source-Height Monotonicity & Mixed Word Minimization

- Monotonicity: $M_{r+1} \ge M_r$ for all $r$.
- Observed strictly increasing through depth 15 ($7 \to 23 \to 3351 \to \dots \to 314,433,137,620,049,175$).
- Pure $u^r$ minimizes source height through depth 9, but at depth 10, the mixed word $u^4 v u^5$ achieves $k = 204,094,463,255 < \text{guard}(u^{10}) = 299,866,807,575$.

---

## 8. Transition to Phase 7.3B-2: Ultrametric Cancellation Machine

The exact reference semantics are now frozen. Phase 7.3B-2 constructs the 2-adic ultrametric cancellation register machine:
- Coordinate: $L_u(n) = 11n + 19 = 32(11k + 3)$.
- State: $(x, U)$ where $L_u(n) = 2^x U$ ($U$ odd, $x \ge 5$).
- Exact Guard Equivalences:
  - $u$-return $\iff x \ge 9$.
  - $v$-exact $\iff x = 6$ and $U \equiv 1 \pmod{16}$.
  - $v$-return $\iff x = 6$ and $U \equiv 81 \pmod{256}$.
- Transition Laws:
  - $u: (x, U) \mapsto (x - 4, 27U)$.
  - $v: (x = 6, U) \mapsto (x' = \gamma - 3, U' = \frac{729U + 87}{2^\gamma})$ where $\gamma = v_2(729U + 87)$.
