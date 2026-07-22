# Phase 3 Depth 20 Flagship Experimental Results (Corrected 2-Adic Metrics)

## Executive Summary
This report presents the flagship experimental results at valuation depth 20 using the updated 2-adic Patricia Trie verifier engine and corrected dual valuation semantics.

## Experimental Parameters
- **Valuation Depth ($k$):** 20
- **Active Sieves:** `DescentSieve`, `MinimalCounterexampleSieve`, `Mod9PreimageSieve`, `PathMergingSieve`
- **Total Certificates Generated:** 24,805,616
- **Execution Time:** 301.02 seconds (~5 minutes)
- **Throughput:** ~82,406 certificates / second

## The Four Verified 2-Adic Metrics (Depth 20)

| Metric | Rational Value | Floating-Point | Interpretation |
| :--- | :--- | :--- | :--- |
| **1. Exact-Cylinder Lower Bound ($\mu_{\text{exact}}$)** | $\frac{1,524,876,280,571}{2,199,023,255,552}$ | **0.693433 (69.34%)** | Provable lower bound from disjoint exact 2-adic cylinders |
| **2. Broad-Certificate Union Measure ($\mu_{\text{union}}$)** | $\frac{992,442,370,557}{1,099,511,627,776}$ | **0.902621 (90.26%)** | True canonical 2-adic set measure of covered odd integers |
| **3. Raw Overlap-Weighted Mass ($\text{Mass}_{\text{broad}}$)** | $\frac{1,524,876,280,571}{1,099,511,627,776}$ | **1.386867 (138.69%)** | Raw sum of broad certificate measures (quantifies overlap) |
| **4. Unresolved 2-Adic Measure ($\mu_{\text{unresolved}}$)** | $\frac{107,069,257,219}{1,099,511,627,776}$ | **0.097379 (9.74%)** | Exact 2-adic measure remaining uncovered at depth 20 |

## Comparison: Depth 10 vs. Depth 18 vs. Depth 20

| Depth ($k$) | Certificates | Exact Lower Bound ($\mu_{\text{exact}}$) | Broad Union Measure ($\mu_{\text{union}}$) | Overlap Mass | Unresolved Measure |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Depth 10** | 2,080 | 65.65% | 85.74% | 1.3130 | 14.26% |
| **Depth 18** | 3,229,802 | 68.97% | 89.81% | 1.3793 | 10.19% |
| **Depth 20** | 24,805,616 | **69.34%** | **90.26%** | **1.3869** | **9.74%** |

## Key Insights
1. **90%+ Certified Coverage:** Over **90.26%** of all 2-adic odd integers are strictly certified to descend within 20 odd steps.
2. **Unresolved Residual Frontier Reduced to Sub-10%:** The unresolved 2-adic measure drops to **9.74%**, isolating the exact target set for Phase 4 adversarial search.
3. **Canonical Boundary Integrity:** All canonical union measures remain strictly $\le 1.0$ under the LSB Patricia Trie verifier engine.
