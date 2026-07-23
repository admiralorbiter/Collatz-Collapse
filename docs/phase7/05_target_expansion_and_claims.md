# Phase 7.3E: Target Expansion Discipline & Claims Registry

## 1. Target Expansion Discipline

Before expanding research beyond the minimal noncommuting core $(u,v)$, Phase 7.3 must complete full classification of Target A:

```text
Target A (Minimal Core):
  u = [1,1,2]
  v = [1,1,2,1,2,2]
```

Expansion to broader targets occurs strictly in stages:
1. **Target A**: Complete verification of exact $k$-machine, quotient simulation, 2-adic IFS geometry, transducer analysis, and ranking proof systems on $(u,v)$ at $Q_1$.
2. **Target B**: Higher-depth macro-switching pairs (e.g., 3-word switching systems at $Q_1$).
3. **Target C**: Full noncommuting branching network across all residue classes $Q_0, Q_1, Q_2, Q_3$.

---

## 2. Claims Registry Integration (`claims/claims.toml`)

Phase 7.3 adds the following claim definitions to `claims/claims.toml`:

```toml
[claims.q1_k_machine_exact]
id = "CLAIM-7.3-01"
title = "Exact Q1 Integer Register Machine Semantics"
status = "PROPOSED"
phase = "7.3A"
proof_schema = "q1_quotient_machine_v1"

[claims.no_ultimately_periodic_positive_path]
id = "CLAIM-7.3-02"
title = "Exclusion of Positive Ultimately Periodic Switching Paths"
status = "PROPOSED"
phase = "7.3A"
proof_schema = "ultimately_periodic_exclusion_v1"

[claims.finite_full_shift_return]
id = "CLAIM-7.3-03"
title = "Finite Full-Return Language Theorem"
status = "PROPOSED"
phase = "7.3C"
proof_schema = "finite_full_shift_return_v1"

[claims.switching_limit_set_haar_zero]
id = "CLAIM-7.3-04"
title = "Zero 2-Adic Haar Measure of Switching Limit Set"
status = "PROPOSED"
phase = "7.3C"
proof_schema = "switching_limit_set_measure_v1"

[claims.switching_limit_set_hausdorff_dimension]
id = "CLAIM-7.3-05"
title = "2-Adic Hausdorff Dimension s ≈ 0.1625"
status = "PROPOSED"
phase = "7.3C"
proof_schema = "switching_limit_set_dimension_v1"
```
