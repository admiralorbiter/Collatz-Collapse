# Proof Dependency DAG: Sturmian Gap-Itinerary Elimination

```
Bell, Schulz, and Shallit (2024), "Consecutive Power Occurrences in Sturmian Words"
    ↓
Candidate cube coverage (52-phase return universe)
    ↓
Canonical selector completeness (r_min = 2)
    ↓
52-phase transition graph completeness
    ↓
State-core semantic depth bridge (H_L <= s < H_L + B_{x_{L+1}})
    ↓
Period-extension invariance (STURMIAN_NORMALIZED_WEIGHT_PERIOD_EXTENSION_INVARIANCE_PROVED)
    ↓
Worst-case edge domination (W(e) = max_\tau W_\tau)
    ↓
Potential inequality (W(e) + \Phi(t) - \Phi(s) <= -60)
    ↓
Negative precision drift (s_N <= s_0 + C - 60 N -> -\infty)
    ↓
H.1 ordinary-integer contradiction (s_N = v_2(A_v(D_N)) >= 0)
```
