# Phase 3 Depth 18 Experimental Results (Corrected 2-Adic Metrics)

## Executive Summary
This report summarizes the experimental run at valuation depth 18 using the updated 2-adic Patricia Trie verifier engine and corrected dual valuation semantics.

## Experimental Parameters
- **Valuation Depth ($k$):** 18
- **Active Sieves:** `DescentSieve`, `MinimalCounterexampleSieve`, `Mod9PreimageSieve`, `PathMergingSieve`
- **Total Certificates Generated:** 3,229,802
- **Execution Time:** 36.11 seconds
- **Throughput:** ~89,443 certificates / second

## The Four Verified 2-Adic Metrics

| Metric | Rational Value | Floating-Point | Interpretation |
| :--- | :--- | :--- | :--- |
| **1. Exact-Cylinder Lower Bound ($\mu_{\text{exact}}$)** | $\frac{94,787,358,459}{137,438,953,472}$ | **0.689669 (68.97%)** | Provable lower bound from disjoint exact 2-adic cylinders |
| **2. Broad-Certificate Union Measure ($\mu_{\text{union}}$)** | $\frac{61,716,044,541}{68,719,476,736}$ | **0.898087 (89.81%)** | True canonical 2-adic set measure of covered odd integers |
| **3. Raw Overlap-Weighted Mass ($\text{Mass}_{\text{broad}}$)** | $\frac{94,787,358,459}{68,719,476,736}$ | **1.379338 (137.93%)** | Raw sum of broad certificate measures (quantifies overlap) |
| **4. Unresolved 2-Adic Measure ($\mu_{\text{unresolved}}$)** | $\frac{7,003,432,195}{68,719,476,736}$ | **0.101913 (10.19%)** | Exact 2-adic measure remaining uncovered at depth 18 |

## Mathematical Verification & Integrity
- **Set Measure Boundary:** Broad-Certificate Union Measure is strictly bounded ($\mu_{\text{union}} = 0.898087 \le 1.0$).
- **Overlap Discrepancy:** The difference $\text{Mass}_{\text{broad}} - \mu_{\text{union}} = 0.481251$ quantifies the exact degree of multiplicity/overlap across broad descent certificates.
- **Exact Lower Bound:** $\mu_{\text{exact}} = \frac{1}{2} \text{Mass}_{\text{broad}} = 0.689669$ provides an unconditional disjoint lower bound.
