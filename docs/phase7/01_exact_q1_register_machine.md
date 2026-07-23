# Phase 7.3A: Exact $Q_1$ Register Machine & Reference Semantics

## 1. Parameterization of $Q_1$

All states in $Q_1 = 7 + 32\mathbb{N}_0$ are represented by an exact non-negative integer coordinate $k$:
$$n = 7 + 32k, \quad k \in \mathbb{N}_0$$

This parameterization has no hidden congruence invariant.

---

## 2. Derivation of $u$-return and $v$-return Guards

### $u$-return Derivation
Applying macro $u = [1,1,2]$ to $n = 7 + 32k$:
$$F_u(n) = \frac{27n + 19}{16} = \frac{27(7 + 32k) + 19}{16} = \frac{189 + 864k + 19}{16} = \frac{864k + 208}{16} = 54k + 13$$

For $F_u(n)$ to land back in $Q_1 = 7 + 32k'$:
$$54k + 13 \equiv 7 \pmod{32} \iff 54k \equiv -6 \equiv 26 \pmod{32} \iff 27k \equiv 13 \pmod{16}$$
Multiplying by $27^{-1} \equiv 3 \pmod{16}$:
$$k \equiv 13 \cdot 3 = 39 \equiv 7 \pmod{16}$$

Writing $k = 7 + 16t$:
$$k' = \frac{54(7 + 16t) + 13 - 7}{32} = \frac{378 + 864t + 6}{32} = \frac{864t + 384}{32} = 27t + 12 = \frac{27(k - 7)/16 \cdot 16 + 192}{16} = \frac{27k + 3}{16}$$

Thus, the exact $u$-return transition is:
$$\text{Guard: } k \equiv 7 \pmod{16}, \qquad k' = \frac{27k + 3}{16}$$

---

### $v$-return Derivation
Applying macro $v = [1,1,2,1,2,2]$ to $n = 7 + 32k$:
$$F_v(n) = \frac{729n + 881}{512} = \frac{729(7 + 32k) + 881}{512} = \frac{5103 + 23328k + 881}{512} = \frac{23328k + 5984}{512}$$

For $F_v(n)$ to land back in $Q_1 = 7 + 32k'$:
$$\frac{23328k + 5984}{512} \equiv 7 \pmod{32} \iff 23328k + 5984 \equiv 3584 \pmod{16384} \iff 23328k \equiv -2400 \equiv 13984 \pmod{16384}$$
Dividing by $\gcd(23328, 16384) = 32$:
$$729k \equiv 437 \pmod{512}$$
Multiplying by $729^{-1} \pmod{512}$:
$$729 \equiv 217 \pmod{512}, \quad 217^{-1} \equiv 33 \pmod{512} \implies k \equiv 437 \cdot 33 = 14421 \equiv 61 \pmod{512}$$

Writing $k = 61 + 512t$:
$$k' = \frac{F_v(7 + 32(61 + 512t)) - 7}{32} = 729t + 87 = \frac{729(k - 61) + 44544}{512} = \frac{729k + 75}{512}$$

Thus, the exact $v$-return transition is:
$$\text{Guard: } k \equiv 61 \pmod{512}, \qquad k' = \frac{729k + 75}{512}$$

---

## 3. Reference Register Machine Semantics

The $Q_1$ core is governed by the deterministic partial register machine:

```text
U-return:
    guard k ≡ 7 mod 16
    k := (27k + 3) / 16

V-return:
    guard k ≡ 61 mod 512
    k := (729k + 75) / 512
```

### Relationship to $L_u(n) = 11n + 19$
For $n = 7 + 32k$:
$$L_u(n) = 11(7 + 32k) + 19 = 77 + 352k + 19 = 352k + 96 = 32(11k + 3)$$
Let $x = v_2(L_u(n))$:
$$x = 5 + v_2(11k + 3)$$

- $x \ge 5$ corresponds to membership in $Q_1$.
- $x = 6 \iff v_2(11k + 3) = 1 \iff 11k + 3 \equiv 2 \pmod 4 \iff 11k \equiv 7 \pmod 4 \iff k \equiv 1 \pmod 4$.
- Based $v$-return ($k \equiv 61 \pmod{512}$) implies $x = 6$ and $U \equiv 81 \pmod{256}$, which is strictly stronger than $U \equiv 1 \pmod{16}$ (which only guarantees exact $v$-execution, not return to $Q_1$).

---

## 4. Ultimately Periodic Exclusion Theorem

**Theorem**: No positive integer $n \in \mathbb{N}^+$ realizes an ultimately periodic infinite switching path $\alpha \beta^\omega$ with non-empty block $\beta \in \{u,v\}^+$.

*Proof*:
Any composite macrostep $\beta$ corresponds to an affine map $F_\beta(n) = \frac{a_\beta n + c_\beta}{b_\beta}$.
Since both $u$ ($a_u/b_u = 27/16 > 1$) and $v$ ($a_v/b_v = 729/512 > 1$) are real-expanding, any non-empty composition $\beta$ satisfies:
$$a_\beta > b_\beta > 0 \quad \text{and} \quad c_\beta > 0$$

The unique 2-adic rational fixed point of $F_\beta$ is:
$$n_\beta^* = \frac{c_\beta}{b_\beta - a_\beta} < 0$$

Since $c_\beta > 0$ and $b_\beta - a_\beta < 0$, $n_\beta^*$ is strictly negative.

Pulling $n_\beta^*$ back through any finite prefix $\alpha$ via inverse affine steps $F_{p}^{-1}(y) = \frac{b_p y - c_p}{a_p}$ preserves negativity because $y < 0 \implies b_p y - c_p < 0$. Thus, any state supporting a periodic tail must be a strictly negative 2-adic rational, so no positive integer can lie on an ultimately periodic path. $\blacksquare$
