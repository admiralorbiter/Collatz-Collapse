# Phase 7X Claims Registry and Review Gates

## 1. Claim categories

Use only these categories:

- `Verified Algebraic Identity`
- `Verified Finite Theorem`
- `Verified Bounded Classification`
- `Domain-Scoped Certificate`
- `Experimental Observation`
- `Conjectural Pattern`
- `Open Question`
- `Refuted Candidate`

---

## 2. Initial & Phase 7.3 Claims

### CLM-P7X-PRECISION-001

**Category:** Verified Algebraic Identity

**Statement:**

For a valid macrostep with total valuation \(A\), a complete source cylinder modulo \(2^M\) has a deterministic image modulo \(2^q\) if and only if \(M \ge A + q\).

---

### CLM-P7X-CROSSFORM-001

**Category:** Verified Algebraic Identity

**Statement:**

For all nonempty macrosteps \(p,q\): \(b_q H_p(F_q(n)) = a_q H_p(n) + \Delta_{p,q}\).

---

### CLM-P7X-COMMUTATOR-001

**Category:** Verified Algebraic Identity

**Statement:**

\(b_p b_q(F_q(F_p(n)) - F_p(F_q(n))) = \Delta_{p,q}\).

---

### CLM-P7X-PERIODIC-DIVERGENCE-001

**Category:** Verified Finite Theorem

**Statement:**

For every primitive period word $w \in \{u,v\}^+$, $\eta_w > 0$ and $a_w = 3^{K_w} > 2^{A_w} = b_w$. The rational 2-adic fixed point $k^*_w = \frac{\eta_w}{2^{A_w} - a_w} < 0$ is strictly negative. For any negative rational $y < 0$, pullback maps $g_u(y) < 0$ and $g_v(y) < 0$ preserve strict negativity. Therefore, no ultimately periodic stream $p w^\omega$ has a non-negative integer realization.

---

### CLM-P7X-LIFT-DIGIT-REALIZABILITY-001

**Category:** Verified Algebraic Identity

**Statement:**

An infinite symbolic stream $\omega \in \{u,v\}^\mathbb{N}$ codes an ordinary non-negative integer $\alpha_\omega \in \mathbb{Z}_{\ge 0}$ if and only if its lift-digit sequence $(\lambda_j)_{j=1}^\infty$ defined by $r_{j+1} = r_j + \lambda_{j+1} 2^{A_j}$ is eventually zero.

---

### CLM-P7X-SOURCE-HEIGHT-EQUIV-001

**Category:** Verified Finite Theorem

**Statement:**

For $M_r = \min_{|s|=r} r_s$, the following equivalence holds via König's Lemma:
$$M_r \to \infty \iff \text{no non-negative integer supports an infinite } u/v \text{ return path}$$

---

### CLM-P7X-OMEGA-REGULAR-COLLAPSE-001

**Category:** Verified Finite Theorem

**Statement:**

If the language of non-negative infinite addresses $\mathcal{L}_{\text{positive},\omega} \subseteq \{u,v\}^\mathbb{N}$ is $\omega$-regular, then it is empty ($\mathcal{L}_{\text{positive},\omega} = \emptyset$).

---

### CLM-P7X-UNBOUNDED-FINITE-ORBITS-001

**Category:** Verified Finite Theorem

**Statement:**

For every $r \ge 1$, there exists $y_r \ge 0$ such that the orbit of $y_r$ under $D(y)$ has length $\ge r$. Consequently, no sound 1-step finite abstraction graph of all positive $D$ trajectories can be acyclic.

---

### CLM-P7X-U-PHASE-COUNTDOWN-001

**Category:** Verified Algebraic Identity

**Statement:**

Under $j$ applications of $u$, $T_u^j(k) + \frac{3}{11} = \left(\frac{27}{16}\right)^j \left(k + \frac{3}{11}\right)$. Valuation transforms as $x \mapsto x - 4$. Thus every pure-$u$ block terminates in $j = \lfloor \frac{x-6}{4} \rfloor$ steps.

---

### CLM-P7X-INDUCED-V-MAP-001

**Category:** Verified Algebraic Identity

**Statement:**

At a $v$-return ($U = 81 + 256t$), the intervening $u$-count is $j = \frac{\delta - 1}{4}$ where $\delta = v_2(231 + 729t)$. The induced unit is $U_{\text{next}} = \frac{231 + 729t}{2^{1+4j}} \cdot 27^j$.

---

## 3. Phase 7.3 Sub-Phase Review Gates

### Gate 7.3A — Generic Affine Identity Kernel (PASSED)
Pass if:
- Same-form, cross-form, and affine commutator identities hold symbolically;
- $\Delta_{u,v} = -5568, b_u b_v = 8192, v_2(\Delta_{u,v}) = 6$ recomputed independently;
- Broad and exact resonance recovery matches direct modular inversion;
- Identity verifiers PASS across Rust, Python oracle, and Lean 4 formal proofs.

### Gate 7.3B — Sound Register Machine Abstraction (PASSED)
Pass if:
- Single-coordinate $L_u(n) = 11n + 19$ state machine $(x, \text{res})$ has precise concretizations;
- Cancellation gate $x=6, U \equiv 1 \pmod{16}$ universally validated;
- Feature CEGAR adds $L_v(n)$ or extra $U$-bits only upon concrete counterexample.

### Gate 7.3C — Symbolic Return Language & Entropy Classification (PASSED)
Pass if:
- All words $s \in \{u,v\}^{\le 12}$ ($8,190$ non-empty words) enumerated with exact path cylinders;
- All block-boundary $Q_1$-return conditions verified;
- Topological entropy $h_{\text{top}} = \ln 2$, Hausdorff dimension $\dim_H(G_\infty) \approx 0.162536$, and dual Haar measure decay reported;
- **Lift-Digit Realizability Theorem** proven and verified;
- Zero-lift subsystem proven to be partial and deterministic ($D(y)$), with positive infinite trajectory existence correctly left **UNRESOLVED** for Phase 7.3D.

### Gate 7.3D — Induced $v$-to-$v$ Map Acceleration & Multiphase Ranking (Target A)
Pass if Target A ($u/v$ core) is classified under one of the proof architectures:
- `TERMINATED_ACCELERATED_LEXICOGRAPHIC` (`accelerated_lexicographic_v1`)
- `TERMINATED_MULTIPHASE_U_V` (`multiphase_u_v_v1`)
- `TERMINATED_DUAL_ADIC` (`dual_adic_invariant_v1`)
- `TERMINATED_PATH_COMPLETE_RANKING` (`path_complete_ranking_v1`)
- `SOUND_ACCELERATED_UNRANKED`
- `REFINEMENT_LIMIT`

---

## 4. Final Milestone Decision Table

| $v$-Event Abstraction | Intervening $u$-Count $j$ | Induced $v$-Map | Dual-Adic Filter | Result |
|---|---|---|---|---|
| $u$-Countdown $x \mapsto x-4$ | $\delta = 1+4j$ | Well-founded Rank | — | `TERMINATED_ACCELERATED_LEXICOGRAPHIC` |
| Multiphase $(x, U)$ | Closed-form $T_u^j$ | Monotone Rank | — | `TERMINATED_MULTIPHASE_U_V` |
| Dual-Adic $q \pmod{27 \cdot 2^m}$ | Dynamic Transition | Dynamic Check | Incompatible | `TERMINATED_DUAL_ADIC` |
| Bounded Refinement | Unbounded $j$ | Unranked SCC | Unresolved | `SOUND_ACCELERATED_UNRANKED` |
