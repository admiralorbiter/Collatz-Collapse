# Phase 7.3S Authoritative Proof Audit Packet & Counterexample Replay

**Schema Version**: `4.0.0`  
**Git Commit**: `fd16e0e5b89b61a4fe6247df5ba9567749c1b82a`  
**Table Hash (`j=0..32`)**: `b6c8fe576f2ce75e`  
**Primary Verification Module**: [crates/collatz-cegar/src/precision_aware_cylinder.rs](file:///c:/Users/admir/Github/Collatz-Collapse/crates/collatz-cegar/src/precision_aware_cylinder.rs)  
**Backward Probe Module**: [crates/collatz-cegar/src/backward_fixed_point_probe.rs](file:///c:/Users/admir/Github/Collatz-Collapse/crates/collatz-cegar/src/backward_fixed_point_probe.rs)  
**Independent Python Oracle**: [scripts/phase73s1c_proof_packet_generator.py](file:///c:/Users/admir/Github/Collatz-Collapse/scripts/phase73s1c_proof_packet_generator.py)  
**Lean 4 Theorem Files**: [lean/EventualZeroEndpointReduction.lean](file:///c:/Users/admir/Github/Collatz-Collapse/lean/EventualZeroEndpointReduction.lean), [lean/EventuallyPeriodicGhostSourceDensity.lean](file:///c:/Users/admir/Github/Collatz-Collapse/lean/EventuallyPeriodicGhostSourceDensity.lean).

---

## Status Badges Audit Summary

| Status Badge | Verdict | Reason & Scope |
| :--- | :---: | :--- |
| `DISCREPANCY_2_8_0_RESOLVED` | 🟢 **FROZEN** | Corrected $B(2,8,0)=67$. Proven that prior $B=84$ belonged to `(2,2,8,0)`. |
| `CORRECTED_DEPTH3_J8_TABLE` | 🟢 **FROZEN** | True 3-gap max: Word `(0, 3, 1)` with $Z=11, B=43, \ell=32, Z/B=0.2558$. |
| `EVENTUALLY_PERIODIC_PREIMAGE_FORMULA_VERIFIED` | 🟢 **FROZEN** | Exact preimage $x = -P/R$ ($R = q Q_u$ odd) verified in $\mathbb{Z}_2$. |
| `PLAIN_MODULAR_CEGAR_UNSOUNDNESS_DEMONSTRATED` | 🟢 **FROZEN** | Proven that plain $D \bmod 2^m$ collapses quotient $n = (D - C_j)/M_j$. |
| `EVENTUAL_ZERO_ENDPOINT_REDUCTION` | 🟡 **PROVISIONAL** | Equation verified as $D_{u j} = D_j + Q_j \frac{D_u - C_j}{M_j} = F_j(D_u)$ on genuine prefix $u=(0,0,7)$. |
| `EVENTUALLY_PERIODIC_GUARD_ADMISSIBILITY` | 🟡 **PROVISIONAL** | Conditional on guarded admissibility of sequence $u w^\omega$. |
| `UNIFORM_BOUND_K_3_2_7` | 🟡 **INCOMPLETE** | Maximum observed $Z=17$ for $u=[5,2,3], w=[6,7]$. Analytical $r_0(u,w)$ thresholds pending. |
| `EXACT_PRECISION_AWARE_CYLINDER_CALCULUS` | 🟢 **ACTIVE** | Implemented exact $\text{Post}_j$ and $\text{Pre}_j$ cylinder transformers. |
| `BACKWARD_ZERO_LIFT_FIXED_POINT_PROBE_J32` | 🟢 **ACTIVE** | Implemented coinductive backward fixed-point operator $\Phi_J(X)$. |
| `REACHABLE_ENDPOINT_INTERSECTION_ANALYSIS` | 🟢 **ACTIVE** | Evaluating intersection $I \cap \mathcal{E}_\infty$. |

---

## Section 0: Discrepancy Reconciliation

### 0.1 Resolution of Word $(2, 8, 0)$
- **Calculation**: $B(2, 8, 0) = (9 + 4 \cdot 2) + (9 + 4 \cdot 8) + (9 + 4 \cdot 0) = 17 + 41 + 9 = \mathbf{67}$ bits.
- **Root Cause**: The prior script mislabeled word `(2, 2, 8, 0)` ($B = 17 + 17 + 41 + 9 = 84, \ell = 74, Z = 10$) as `(2, 8, 0)` due to loop index nesting.
- **Corrected $(2, 8, 0)$ Vectors**:
  - $M_{(2, 8, 0)} = 147,573,952,589,676,412,928 = 2^{67}$
  - $\rho_{(2, 8, 0)} = 130,960,401,311,064,871,104$
  - $\ell = 67, B = 67, Z = \mathbf{0}$
  - $\Lambda = [86208, 792011552335, 454]$

### 0.2 TRUE Exhaustive Depth-Three Rankings ($J \le 8$, 729 words)
- **Top Absolute $Z$ Winner**: Word `(0, 3, 1)` with $Z = 11, B = 43, \ell = 32, Z/B = 0.2558$.
- **Top Normalized $Z/B$ Winner**: Word `(0, 3, 1)` with $Z/B = 0.2558, Z = 11, B = 43, \ell = 32$.
- **Top 5 Absolute $Z$**:
  1. `(0, 3, 1)`: $Z=11, B=43, \ell=32, Z/B=0.2558$
  2. `(1, 5, 0)`: $Z=6, B=51, \ell=45, Z/B=0.1176$
  3. `(3, 2, 3)`: $Z=6, B=59, \ell=53, Z/B=0.1017$
  4. `(7, 1, 0)`: $Z=6, B=59, \ell=53, Z/B=0.1017$
  5. `(1, 4, 7)`: $Z=6, B=75, \ell=69, Z/B=0.0800$

---

## Section 1: Exact Cylinder Calculus ($\text{Post}_j$ and $\text{Pre}_j$)

### 1.1 Exact Forward Cylinder Transformer $\text{Post}_j$
$$\text{Post}_j([r]_p) = [D_j + Q_j t]_{p - B_j} \qquad \text{where } t = \frac{r - C_j}{2^{B_j}}$$
Acyclic in $p$ because $p \to p - B_j < p$.

### 1.2 Exact Backward Cylinder Transformer $\text{Pre}_j$
$$\text{Pre}_j([s]_m) = \left[ C_j + M_j \cdot \left( Q_j^{-1} (s - D_j) \bmod 2^m \right) \right]_{m + B_j}$$
- **Round-Trip Theorem**: $\text{Post}_j(\text{Pre}_j([s]_m)) = [s]_m$ at exact target precision $m$.
- **Guard Inclusion**: $\text{Pre}_j([s]_m) \subseteq [C_j]_{B_j}$.

---

## Section 2: Coinductive Greatest Fixed Point $\mathcal{E}_\infty = \nu X. \Phi_J(X)$

Define backward fixed-point operator:
$$\Phi_J(X) = \bigcup_{j=0}^J \text{Pre}_j(X) = \bigcup_{j=0}^J \{ D \in \mathbb{Z}_2 : D \equiv C_j \pmod{M_j}, F_j(D) \in X \}$$
$$\mathcal{E}_\infty = \nu X. \Phi_J(X)$$
Target: Prove $I \cap \mathcal{E}_\infty = \emptyset$ where $I = \{ D_u : u \text{ finite canonical prefix} \}$.

---

## Section 3: Mutation Testing Suite Results (100% Passed)

| Mutation ID | Description | Result | Safety Status |
| :--- | :--- | :---: | :---: |
| **Mutation 1** | Corrupt $C_j$ by $+M_j$ | **REJECTED** | 🟢 **SAFE** |
| **Mutation 2** | Swap $c_j$ and $C_j$ | **REJECTED** | 🟢 **SAFE** |
| **Mutation 3** | Flip sign of $\beta_j$ | **REJECTED** | 🟢 **SAFE** |
| **Mutation 4** | Coherent Shift ($C_j' = C_j + M_j, D_j' = D_j + Q_j$) | **REJECTED** | 🟢 **SAFE** |

**Conclusion**: 100% of adversarial mutations are caught and rejected.
