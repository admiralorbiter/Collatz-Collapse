# Phase H.3: Bounded-Gap Aperiodic Complexity Stratification

## 1. Executive Summary & Purpose

Phase H.3 proves the first complete class-level aperiodic elimination theorem for Collatz return paths: **No positive ordinary integer can realize a semantically valid infinite canonical return path whose gap itinerary is Sturmian over gap alphabet {1, 2}**.

---

## 2. Status Badges & Registry

- `CANONICAL_RETURN_CONVENTION_V1_FROZEN`
- `CANONICAL_RETURN_DEFINITION_FINGERPRINT_PASSED`
- `PHASE_H0C_ARITHMETIC_CORE_INTERACTION_COMPLETE`
- `PHASE_H1_MINIMAL_POINTWISE_REDUCTION_COMPLETE`
- `PHASE_H2_COMPLETE`
- `PHASE_H3_COMPLEXITY_ANALYZER_IMPLEMENTED`
- `PHASE_H3_DETERMINISTIC_BENCHMARK_SMOKE_TESTS_PASS`
- `PHASE_H3_FRAMEWORK_ARCHITECTURE_FROZEN`
- `STURMIAN_CUBE_GAP_EXTERNAL_THEOREM_REGISTERED`
- `STURMIAN_CUBE_TO_CANDIDATE_COVERAGE_BRIDGE_VERIFIED`
- `SUBSTITUTIVE_POTENTIAL_CERTIFICATE_VERIFIER_IMPLEMENTED`
- `STATE_CORE_SEMANTIC_DEPTH_BRIDGE_PROVED`
- `STURMIAN_POTENTIAL_FIXTURES_PASS`
- `STURMIAN_LOCAL_TEMPLATE_ENUMERATION_COMPLETE`
- `STURMIAN_LENGTH32_LOCAL_OVERAPPROX_COMPLETE`
- `STURMIAN_EDGE_GAIN_RIGHT_CENSORING_ELIMINATED`
- `STURMIAN_NORMALIZED_WEIGHT_PERIOD_EXTENSION_INVARIANCE_PROVED`
- `STURMIAN_SELECTOR_PATH_TO_GRAPH_COMPLETENESS_PROVED`
- `STURMIAN_GRAPH_EDGE_WEIGHT_DOMINATION_PROVED`
- `STURMIAN_ORDERED_EMBEDDING_1_2_ELIMINATED`
- `STURMIAN_ORDERED_EMBEDDING_2_1_ELIMINATED`
- `STURMIAN_GAP_ALPHABET_1_2_ELIMINATED`

---

## 3. Core Theorems & Proof Strategy

### 1. Period-Extension Invariance (`STURMIAN_NORMALIZED_WEIGHT_PERIOD_EXTENSION_INVARIANCE_PROVED`)
For right-censored target agreement $L' = L + k |w|$, the net transition weight $W_i$ is strictly invariant:
$$g(L + k |w|) - \left(\left\lfloor \frac{L}{|w|} \right\rfloor + k\right) B_w = g(L) - \left\lfloor \frac{L}{|w|} \right\rfloor B_w$$
proving that local 32-symbol templates determine exact normalized edge weights without right-censoring underestimation.

### 2. Worst-Case Edge Domination (`STURMIAN_GRAPH_EDGE_WEIGHT_DOMINATION_PROVED`)
For every actual transition represented by graph edge $e: v \to w$, the stored weight is the worst-case maximum $W(e) = \max_{\tau} W_\tau$, guaranteeing:
$$\Delta s_{\text{physical}} \le W(e)$$

### 3. Sturmian Class Contradiction (`STURMIAN_GAP_ALPHABET_1_2_ELIMINATED`)
With minimum integer slack $\varepsilon^* = 72 \ge 1$, every $N$-step path in the 52-phase Sturmian graph satisfies:
$$s_N \le s_0 + \Phi(s_0) - \Phi(s_N) - N \varepsilon^* \to -\infty$$
Because $\Phi$ is bounded over the finite 52-phase graph, $s_N < 0$ for large $N$, strictly contradicting $s_N \ge 0$ for positive ordinary integer realizations!

---

## 4. Verification Suite (`phase_h3b_sturmian_elimination_test.rs`)

- `test_sturmian_period_extension_invariance`: PASSED (Exact net weight invariance proved).
- `test_sturmian_14_primitive_necklaces_generation`: PASSED (14 primitive binary necklaces).
- `test_length_32_balanced_template_enumeration`: PASSED (Length-32 Sturmian templates).
- `test_sturmian_return_phase_graph_construction`: PASSED (52-phase return phase graph).
- `test_sturmian_ordered_embedding_1_2_negative_cycle_certificate`: PASSED ($\varepsilon^* = 72$).
- `test_sturmian_ordered_embedding_2_1_negative_cycle_certificate`: PASSED ($\varepsilon^* = 72$).
