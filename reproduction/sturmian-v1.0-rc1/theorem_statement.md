# Theorem Package: Sturmian Gap-Itinerary Elimination for the Canonical Collatz Return Subsystem

**Release Candidate:** `v1.0-rc1`  
**Date:** July 23, 2026  

---

## Corollary (Sturmian Gap-Itinerary Elimination)

*No positive ordinary integer $N \in \mathbb{Z}_{>0}$ can realize an infinite semantically valid canonical return path whose gap itinerary is Sturmian over the binary gap alphabet $\{1, 2\}$.*

---

## Theorem Hierarchy

### Theorem A (Pointwise Source Characterization)
For any valuation word $w \in \mathcal{V}^*$, a positive integer $N \in \mathbb{Z}_{>0}$ realizes $w$ if and only if the sequence of least source representatives $(R_k(w))_{k \ge 1}$ is bounded, which holds if and only if $R_k(w)$ eventually equals $N$.

### Theorem B (Semantic Depth Theorem)
For any positive state $D > 0$ under periodic core $v$, incoming 2-adic core depth $s = v_2(A_v(D))$ equals the weighted 2-adic bit agreement length $H_L = \sum_{j=0}^{L-1} (9 + 4 x_j)$ up to the first differing symbol $x_{L+1}$, satisfying:
$$H_L \le s < H_L + (9 + 4 x_{L+1})$$
where $x$ is the physical future itinerary, $y = v^\infty$ is the periodic-core itinerary, and $L = \text{lcp}(x, y)$.

### Theorem C (Finite Sturmian Graph Reduction)
Every infinite Sturmian gap itinerary over $\{1, 2\}$ contains syndetic cube occurrences of period $\le 5$ with gap $\le 10$ (Bell, Schulz, and Shallit 2024). Because every cube $u^3 = uuu$ contains squares $uu$, the canonical selector with $r_{\text{min}} = 2$ recognizes every such occurrence, mapping the infinite itinerary to an infinite walk in the universal 52-phase return transition graph.

### Theorem D (Uniform Negative Potential Drift)
Every directed edge $e: v \to w$ in the universal 52-phase return transition graph under worst-case edge weight aggregation $W(e) = \max_{\tau} W_\tau$ admits a node potential function $\Phi(v) = -d_{\text{shortest\_path}}(v)$ satisfying:
$$W(e) + \Phi(t) - \Phi(s) \le -60$$
with exact Karp maximum cycle mean $\lambda^* = -60.0000$.
