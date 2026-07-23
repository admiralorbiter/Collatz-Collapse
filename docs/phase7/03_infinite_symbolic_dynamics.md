# Phase 7.3C: Infinite Symbolic Dynamics & 2-Adic Fractal Geometry

## 1. Finite Full-Return Language Theorem

**Theorem**: Every finite macro-symbol word $s = s_1 s_2 \dots s_r \in \{u,v\}^*$ has a non-empty positive guarded return cylinder in $Q_1$.

*Proof Outline*:
1. Every macro-symbol begins with $u$: $u = u$, $v = u w_2$ where $w_2 = [1,2,1,2,2]$.
2. Expand $s$ into its underlying valuation word $W = \text{expand}(s)$ and append an additional $u$: $W' = W u$.
3. The exact valuation word $W'$ has a non-empty positive integer cylinder $C(W') = \{ n_0 + 2^M m \mid m \ge 0 \}$.
4. Since $W'$ begins with $u$, $n_0 \in Q_1$.
5. At every macro boundary $s_i$, the suffix of $W'$ begins with $u$, guaranteeing state membership in $Q_1$.
6. After $s_r$, the appended $u$ guarantees that the target state lands back in $Q_1$.

Thus:
$$L_{\text{finite}} = \{u,v\}^*$$
The admissible finite switching language is the full shift on 2 symbols, with topological entropy $h_{\text{macro}} = \log 2$.

---

## 2. Structural Realizability Gap: Finite vs. Infinite

While $L_{\text{finite}} = \{u,v\}^*$, the set of **positive infinite realizations** $L_{\text{positive},\omega} \subseteq \{u,v\}^\omega$ is severely constrained:

1. **No Ultimately Periodic Realizations**: By the Ultimately Periodic Exclusion Theorem (Phase 7.3A), $L_{\text{positive},\omega} \cap \{ \text{ultimately periodic words} \} = \emptyset$.
2. **Non-$\omega$-Regularity**: Since every non-empty $\omega$-regular language contains an ultimately periodic word, $L_{\text{positive},\omega}$ is **not $\omega$-regular** (unless empty).
3. **Automata Interpretation**: Finite Büchi automata over-approximate 2-adic switching sequences, but cannot recognize $L_{\text{positive},\omega}$ exactly. Any SCC lasso found by an automaton is a 2-adic obstruction or abstraction artifact.

---

## 3. 2-Adic Iterated Function System (IFS) Geometry

In the $k$-coordinate, the inverse branches of the based switching maps are:
$$g_u(k) = \frac{16k - 3}{27}, \qquad g_v(k) = \frac{512k - 75}{729}$$

Their 2-adic contraction ratios are:
$$|16/27|_2 = 2^{-4}, \qquad |512/729|_2 = 2^{-9}$$

The guard conditions $k \equiv 7 \pmod{16}$ and $k \equiv 61 \pmod{512}$ (where $61 \equiv 13 \pmod{16}$) are disjoint.

### 2-Adic Haar Measure Zero
Let $X \subset \mathbb{Z}_2$ be the set of 2-adic states supporting infinite $u/v$ switching. $X = g_u(X) \sqcup g_v(X)$.
By 2-adic measure scaling:
$$\mu(X) = (2^{-4} + 2^{-9})\mu(X) = \frac{512}{33} \mu(X) \implies \mu(X) = 0$$

### 2-Adic Hausdorff Dimension
The similarity dimension $s$ of $X$ satisfies:
$$2^{-4s} + 2^{-9s} = 1 \implies s \approx 0.1625357554$$

### Three Canonical Quantitative Metrics
1. **Macro-symbol entropy**: $h_{\text{macro}} = \log 2$
2. **Entropy per odd Collatz step**: $e^{-3h} + e^{-6h} = 1 \implies h = \frac{\ln \phi}{3} \approx 0.1604$
3. **2-Adic Hausdorff dimension**: $s \approx 0.1625357554$

---

## 4. Residue-Lifting Transducer & Eventual-Zero Analysis

For any infinite macro stream $s = s_0 s_1 s_2 \dots \in \{u,v\}^\omega$, there is a unique 2-adic source $k_\infty(s) \in \mathbb{Z}_2$.

The **Residue-Lifting Transducer** processes $s$ digit by digit and emits the binary expansion of $k_\infty(s)$:
$$k_\infty(s) = \sum_{j=0}^\infty b_j 2^j, \quad b_j \in \{0,1\}$$

### Positive Realization Criterion
A 2-adic integer is a positive integer $k \in \mathbb{N}_0$ **if and only if** its binary expansion is eventually zero:
$$\exists N, \forall j \ge N: b_j = 0$$

The positivity question is thus converted into:
$$\text{Can the output of the residue-lifting transducer become eventually zero?}$$

---

## 5. Source Height Growth $M_r$

Define the minimum canonical source height for an $r$-block return path:
$$M_r = \min \{ k \in \mathbb{N}_0 \mid k \text{ supports a valid guarded return path of length } r \}$$

Proving $M_r \to \infty$ as $r \to \infty$ provides an explicit finite-depth certificate showing no positive integer can support an infinite switching path.
