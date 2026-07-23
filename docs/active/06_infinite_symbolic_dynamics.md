# Phase 7.3C: Infinite Symbolic Dynamics & 2-Adic Fractal Geometry

## 1. Finite Full-Return Language Theorem

**Theorem**: Every finite macro-symbol word $s = s_1 s_2 \dots s_r \in \{u,v\}^*$ has a non-empty positive guarded return cylinder in $Q_1$ (`CLM-P7X-FINITE-UNIQUENESS-001`).

For depths $r = 1 \dots 12$, the total number of non-empty words in $\{u,v\}^{\le 12}$ is:
$$\sum_{r=1}^{12} 2^r = 2^{13} - 2 = 8,190 \text{ non-empty words}$$
The admissible finite switching language is the full shift on 2 symbols, with topological entropy $h_{\text{top}} = \ln 2 \approx 0.693147$.

---

## 2. 2-Adic Cantor Coding & Topological Conjugacy

In the $k$-coordinate, the inverse branches of the based switching maps are:
$$g_u(k) = \frac{16k - 3}{27}, \qquad g_v(k) = \frac{512k - 75}{729}$$

Their 2-adic contraction ratios are $|16/27|_2 = 2^{-4}$ and $|512/729|_2 = 2^{-9}$.
The guard conditions $k \equiv 7 \pmod{16}$ and $k \equiv 61 \pmod{512}$ (where $61 \equiv 13 \pmod{16}$) are disjoint.

The coding map $\pi: \{u,v\}^\mathbb{N} \to G_\infty$ given by $\pi(\omega) = \bigcap_{r=1}^\infty G_{\omega_1 \dots \omega_r} = \{\alpha_\omega\}$ is a homeomorphism satisfying the conjugacy relation:
$$T \circ \pi = \pi \circ \sigma$$
where $T: G_\infty \to G_\infty$ is the piecewise quotient map and $\sigma$ is the left shift.

### 2-Adic Haar Measure Exponential Decay
- Quotient $k$-space (normalized $\mu_k(\mathbb{Z}_2) = 1$): $\mu_k(G_r) = (33/512)^r$.
- Original $n$-space ($Q_1 \subset \mathbb{Z}_2$, $\mu_n(Q_1) = 2^{-5}$): $\mu_n(G_r) = 2^{-5} (33/512)^r$.
- Limit Set Haar Measure: $\mu(G_\infty) = \lim_{r \to \infty} (33/512)^r = 0$.

### 2-Adic Hausdorff Dimension
The similarity dimension $d = \dim_H(G_\infty)$ satisfies:
$$2^{-4d} + 2^{-9d} = 1 \implies d \approx 0.1625357554$$

---

## 3. Structural Realizability Gap: Finite vs. Infinite

While $L_{\text{finite}} = \{u,v\}^*$, the set of **positive infinite realizations** $L_{\text{positive},\omega} \subseteq \{u,v\}^\mathbb{N}$ is severely constrained:

1. **No Ultimately Periodic Realizations**: By the Ultimately Periodic Exclusion Theorem (Phase 7.3C), $L_{\text{positive},\omega} \cap \{ \text{ultimately periodic words } p w^\omega \} = \emptyset$. All rational fixed points $k^*_w = \frac{\eta_w}{2^{A_w} - a_w} < 0$ and their pullbacks $g_p(k^*_w) < 0$ are strictly negative rational 2-adic integers.
2. **Non-$\omega$-Regularity**: Since every non-empty $\omega$-regular language contains an ultimately periodic word, $L_{\text{positive},\omega}$ is **not $\omega$-regular** (unless empty).

---

## 4. Lift-Digit Expansion & Eventual-Zero Realizability Criterion

For an infinite macro stream $\omega = \omega_1 \omega_2 \dots \in \{u,v\}^\mathbb{N}$, child guard inclusion $G_{r_j} \subset G_{r_{j-1}}$ yields:
$$r_j = r_{j-1} + \lambda_j 2^{A_{j-1}}$$
where lift digit $0 \le \lambda_j < 2^{A_j - A_{j-1}}$. In binary, $\lambda_j$ occupies the non-overlapping bit range $[A_{j-1}, A_j - 1]$.

The unique 2-adic point is $\alpha_\omega = \sum_{j=1}^\infty \lambda_j 2^{A_{j-1}}$.

### Positive Realization Theorem
$$\alpha_\omega \in \mathbb{Z}_{\ge 0} \iff (\lambda_j)_{j=1}^\infty \text{ is eventually zero} \iff (r_j) \text{ is eventually constant}$$

Because $G_u = 7 \pmod{16}$ and $G_v = 61 \pmod{512}$ are disjoint, the zero-lift subsystem is a **partial deterministic graph**:
$$D(k) = \begin{cases} \frac{27k+3}{16}, & k \equiv 7 \pmod{16} \\ \frac{729k+75}{512}, & k \equiv 61 \pmod{512} \\ \text{undefined}, & \text{otherwise} \end{cases}$$

An ordinary non-negative integer $u/v$ trajectory exists if and only if $D^j(k)$ is defined for all $j \ge 0$ for some $k \ge 0$. The existence of such an infinite zero-lift ray remains **OPEN (`AperiodicUnresolved`)** and serves as the primary target for Phase 7.3D.
