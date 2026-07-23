# Phase H.2: Canonical Core Selector, Semantic Bridge & Fine-Wilf Precision Ledger

## 1. Executive Summary & Purpose

Phase H.2 establishes the canonical core selection mechanism (**Phase H.2A**), the repaired pathwise 2-adic precision ledger with Fine-Wilf overlap bounds (**Phase H.2B**), and the semantic core-distance bridge connecting 2-adic core proximity to symbolic longest common prefix length (**Phase H.2C**).

---

## 2. Status Badges & Registry

- `CANONICAL_RETURN_CONVENTION_V1_FROZEN`
- `CANONICAL_RETURN_DEFINITION_FINGERPRINT_PASSED`
- `PHASE_H0C_ARITHMETIC_CORE_INTERACTION_COMPLETE`
- `PHASE_H1_MINIMAL_POINTWISE_REDUCTION_COMPLETE`
- `H2A_SELECTOR_AXIOMS_FROZEN`
- `PHASE_H2A_SELECTOR_IMPLEMENTATION_TESTS_PASS`
- `PHASE_H2B_PRECISION_LEDGER_FORMULAS_REPAIRED`
- `CONDITIONAL_RESONANCE_BUDGET_IDENTITY_PROVED`
- `PHASE_H2B_PHASE_VERSUS_PRIMITIVE_ORBIT_FIXTURE_PASSED`
- `PERIODIC_CORE_CONGRUENCE_PREFIX_EQUIVALENCE_PROVED`
- `WEIGHTED_LONGEST_COMMON_PREFIX_INTERVAL_PROVED`
- `FINE_WILF_WEIGHTED_CORE_SEPARATION_BOUND_PROVED`
- `GENUINE_INCOMPATIBLE_PRIMITIVE_CORE_SWITCH_FIXTURE_PASSED`
- `PHASE_H2_COMPLETE`

---

## 3. Phase H.2A: Canonical Semantic Core Selector

### 3.1 Verified Selector Implementation
The `CanonicalCoreSelector` maps an observed valuation history $h = (h_1, \dots, h_n)$ to `StructuredCore(PrimitiveCoreSelection)` or `NoStructuredCore`.

### 3.2 The 7 Verified Selector Axioms
1. **Symbolic-first:** Selected strictly from the observed gap itinerary.
2. **Prefix-measurable:** Depends only on observed history $h_1, \dots, h_n$.
3. **Primitive normalization:** Stores canonical primitive root $v_0$ (lexicographically minimal cyclic shift).
4. **Phase awareness:** Tracks `primitive_orbit_id`, `phase_offset`, and `phase_core`.
5. **Deterministic tie-breaking:** Shorter period first, then higher repetition count.
6. **Null output:** Returns `NoStructuredCore` when no structured core is present.
7. **Extension stability reporting:** Emits `PersistedCore`, `AdvancedPhase`, `ExtendedWindow`, `SwitchedCore`, `LostStructure`.

---

## 4. Phase H.2B: Repaired Precision Ledger & Fine-Wilf Bounds

### 4.1 Four-Case Core Switch Valuation Law
For incoming depth $s_i$ and $r_i$ shadowed cycles, the pre-switch depth is $t_i = s_i - r_i B_{v_i}$.
The exact non-resonant switch depth is:
$$s_{i+1} = \min(t_i, \kappa_i) \quad \text{for } t_i \neq \kappa_i \qquad (\text{NO } +1)$$
- **Case 1 (Inherited):** $t_i < \kappa_i \implies s_{i+1} = t_i$.
- **Case 2 (Reset):** $t_i > \kappa_i \implies s_{i+1} = \kappa_i$ (reset loss $\Delta_i = t_i - \kappa_i$).
- **Case 3 (Resonant):** $t_i = \kappa_i \implies s_{i+1} = t_i + g_i$ ($g_i \ge 1$).

### 4.2 Conditional Resonance Budget Identity (`CONDITIONAL_RESONANCE_BUDGET_IDENTITY`)
The exact finite telescoping ledger equation over $N$ core switches is:
$$s_{N+1} = s_1 - \sum_{i=1}^N r_i B_{v_i} - \sum_{i \in \text{Reset}} (t_i - \kappa_i) + \sum_{i \in \text{Resonant}} g_i$$
Maintaining non-negative 2-adic precision $s_{N+1} \ge 0$ requires cumulative resonance gains to satisfy:
$$\sum_{i \in \text{Resonant}} g_i \ge \sum_{i=1}^N r_i B_{v_i} + \sum_{i \in \text{Reset}} (t_i - \kappa_i) - s_1$$

---

## 5. Phase H.2C: Semantic Core Distance Bridge (`SemanticCoreDistanceBridge`)

### 5.1 The Double-Sided LCP Interval Bound
Let $L = \text{lcp}(v^\infty, w^\infty)$ be the longest common valuation prefix length, and $H_L = \sum_{j=0}^{L-1} b_j$ be the weighted cumulative precision.

**Theorem (`WEIGHTED_LONGEST_COMMON_PREFIX_INTERVAL_PROVED`):**
$$H_L \le \kappa(v,w) = v_2(\Gamma_{v,w}) < H_{L+1}$$

### 5.2 Fine–Wilf Weighted Separation Bound
For distinct primitive periodic cores $v, w$, Fine-Wilf bounds common prefix length to $L \le T - 1 = |v| + |w| - \gcd(|v|, |w|) - 1$.
Therefore:
$$\kappa(v,w) < H_T \le B_{\text{max}} (|v| + |w| - \gcd(|v|, |w|) - 1)$$

### 5.3 Positive State Non-Exact Core Theorem
**Theorem (`positive_ordinary_state_never_exact_negative_core`):**
For any positive integer $D > 0$ and return core $v$ with fixed point $\xi_v < 0$, $A_v(D) = d_v D + \beta_v > 0$ strictly. Exact core landing ($A_v(D) = 0$) is **impossible** for positive ordinary states.

### 5.4 Genuine 3-Primitive-Orbit Test Fixture (`GENUINE_INCOMPATIBLE_PRIMITIVE_CORE_SWITCH_FIXTURE_PASSED`)
Tested across 3 genuinely incompatible primitive orbits:
- Orbit 1: $v_1 = [1, 2]$ (Primitive root $[1, 2]$)
- Orbit 2: $v_2 = [1, 1, 1, 1, 1, 1]$ (Primitive root $[1]$)
- Orbit 3: $v_3 = [1, 1, 2, 1, 1, 2]$ (Primitive root $[1, 1, 2]$)
All 3 have distinct orbit IDs, no pair related by rotation, valid return guards, and execute inherited, reset, and resonant core switches.
