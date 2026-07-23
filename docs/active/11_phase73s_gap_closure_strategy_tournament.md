# Phase 7.3S — Gap-Closure Strategy Tournament & Shortcut Audit

**Status**: **ACTIVE SPECIFICATION**  
**Preceding Phase**: Phase 7.3D-R2A (`VERIFIED_LOCAL_ZERO_LIFT_TRANSITION` & `SOUND_ACCELERATED_UNRANKED`)  
**Completed Sub-Phases**:
- `Phase 7.3S.0`: Active specifications and evidence report models (`[COMPLETE]`).
- `Phase 7.3S.1`: Corpus & Theorem-Backed Generators (`[COMPLETE / FROZEN]`).
  - Badges: `VERIFIED_CORPUS_SCHEMA`, `GHOST_ATLAS_COORDINATE_AUDIT_PASSED`, `VERIFIED_PERIODIC_FIXED_POINT_GENERATOR`, `VERIFIED_BOUNDED_EXTREMAL_SEARCH_INFRASTRUCTURE`, `VERIFIED_EXTREMAL_TABLE_H36_J2`.
- `Phase 7.3S.1B`: Zero-Tail & Large-Gap Stress Audit (`[COMPLETE]`).
  - Badges: `VERIFIED_ZERO_TAIL_INVARIANT_AUDIT`, `VERIFIED_SINGLE_GAP_NO_CHEAP_PRECISION`, `FIXED_PERIOD_GHOST_ZERO_TAIL_BOUND_PROVED`, `NO_CHEAP_PRECISION_OBSERVED_THROUGH_J32`.
**Next Queued Task**:
- `Phase 7.3S.2`: Eventual-Zero Automata Quotient Minimizer (`[NOT STARTED / NEXT]`).

---

## 1. Executive Summary & Problem Diagnosis

### 1.1 The Fundamental Obstruction Exposed in Phase 7.3D-R2A
Phase 7.3D-R2A isolated the structural reason why finite-depth searches, scalar ranking functions, and acyclic abstractions fail to settle global divergence:

$$\text{Arbitrarily long finite positive behavior can shadow a negative 2-adic orbit } z_w^* < 0.$$

For every finite periodic gap word $w \in \{u,v\}^*$, the composite accelerated affine map:
$$F_w(z) = \frac{a_w z + b_w}{2^{B_w}}$$
possesses a unique fixed point in $\mathbb{Z}_2$:
$$z_w^* = \frac{b_w}{2^{B_w} - a_w} < 0$$

Because $z_w^*$ is a negative rational, it does not exist as a positive integer trajectory. However, positive integers $z_r \equiv z_w^* \pmod{2^{B_w(r+1)}}$ approximate $z_w^*$ to arbitrary 2-adic precision, executing at least $r+1$ consecutive repetitions of $w$ before escaping into an undefined residue region.

This structural phenomenon explains why:
1. **Finite-depth searches** never settle global divergence.
2. **Finite acyclic abstractions** produce false cycle loops.
3. **Periodic patterns** persist for arbitrarily long finite steps.
4. **Forward magnitude** is an invalid ranking metric (positive integers grow while shadowing negative attractors).
5. **An actual infinite positive realization** may still be impossible.

### 1.2 Alignment with Theoretical Computer Science (TCS)
Recent TCS treatments of one-variable loop termination establish a matching structural division:
- **Cyclic traces**: The manageable component (eliminated in our framework by `CLM-P7X-PERIODIC-DIVERGENCE-001`).
- **Self-avoiding infinite traces**: The hard generalized-Collatz reachability component.

---

## 2. Phase 7.3S.1 Frozen Mathematical Audit & Infrastructure

### 2.1 Branch Constants & Orientation Identity Test
For return gap $j \ge 0$, the exact normalized affine return map on $\mathbb{Z}_2$ is:
$$F_j(z) = \frac{Q_j z + \beta_j}{M_j}$$
where $Q_j = 3^{6+3j} = 729 \cdot 27^j$, $M_j = 2^{9+4j}$, and the constant term $\beta_j$ is given by the exact formula:
$$\beta_j = M_j D_j - Q_j C_j$$

Every return branch satisfies the **mandatory orientation identity**:
$$Q_j C_j + \beta_j = M_j D_j$$

#### Frozen Branch Parameter Table
- **$j=0$ ($v$)**: $M_0 = 512, Q_0 = 729, C_0 = 342, D_0 = 487, \beta_0 = 26$.
- **$j=1$ ($vu$)**: $M_1 = 8192, Q_1 = 19683, C_1 = 7392, D_1 = 17761, \beta_1 = 1376$.
- **$j=2$ ($vuu$)**: $M_2 = 131072, Q_2 = 531441, C_2 = 86208, D_2 = 349537, \beta_2 = 47936$.
- **$j=3$ ($vuuu$)**: $M_3 = 2097152, Q_3 = 14348907, C_3 = 1764032, D_3 = 12069670, \beta_3 = 1466816$.

### 2.2 Composition Law & Struct Separation
To prevent confusing source residues $\rho_w$ with affine intercepts $\beta_w$, the data structures are permanently separated:

```rust
pub struct CompositeAffineMap {
    pub multiplier: BigUint,   // Q_w = 3^{B_w_odd}
    pub denominator: BigUint,  // M_w = 2^{B_w}
    pub intercept: BigInt,     // \beta_w > 0
}

pub struct CanonicalGuardedWord {
    pub source_residue: BigUint, // \rho_w
    pub endpoint: BigUint,       // D_w
    pub affine: CompositeAffineMap,
    pub gap_sequence: Vec<u64>,
    pub accelerated_depth: usize,
}
```

For word extension by gap $j$:
1. Solve $R \equiv Q_w^{-1} (C_j - D_w) \pmod{M_j}$.
2. Extended canonical source residue: $\rho_{wj} = \rho_w + M_w R$.
3. Composite affine intercept: $\beta_{wj} = Q_j \beta_w + M_w \beta_j$.
4. Extended destination endpoint: $D_{wj} = \frac{Q_{wj} \rho_{wj} + \beta_{wj}}{M_{wj}}$.

Every `CanonicalGuardedWord` asserts structural invariants:
$$0 \le \rho_w < M_w, \qquad M_w = 2^{B_w}, \qquad Q_w \rho_w + \beta_w = M_w D_w.$$

---

## 3. Verified Bounded Experimental Results ($H=36, J=2$)

### 3.1 Exact-Precision Minima $E_{H,J}(b) = \min \{\rho_s : B_s = b\}$

| Precision $b$ | $E_{H,J}(b)$ (Canonical Source $\rho_w$) | $\beta_w$ Affine Intercept | Winning Gap Sequence |
| :---: | :---: | :---: | :---: |
| **9** | **342** | 26 | `["j=0"]` |
| **13** | **7392** | 1376 | `["j=1"]` |
| **17** | **86208** | 47936 | `["j=2"]` |
| **18** | **200534** | 32266 | `["j=0", "j=0"]` |
| **22** | **672598** | 1216270 | `["j=0", "j=1"]` |
| **26** | **5768022** | 38360698 | `["j=0", "j=2"]` |
| **27** | **128651094** | 30337658 | `["j=0", "j=0", "j=0"]` |
| **30** | **350179520** | 1123879360 | `["j=2", "j=1"]` |
| **31** | **732495072** | 995585888 | `["j=1", "j=0", "j=0"]` |
| **34** | **9539899584** | 31758223168 | `["j=2", "j=2"]` |
| **35** | **6227033312** | 29707779872 | `["j=1", "j=0", "j=1"]` |
| **36** | **23750971222** | 25605813610 | `["j=0", "j=0", "j=0", "j=0"]` |

### 3.2 Suffix-Minimum Threshold Function $M_{H,J}(B) = \min_{B \le b \le H} E_{H,J}(b)$

| Target $B$ | Actual $B_s$ | $M_{H,J}(B)$ (Source $\rho_w$) | Original Register $k$-Coordinate | Winning Gap Sequence | $\alpha(B) = \frac{\log_2 \rho_w}{B}$ | $\alpha_{\text{witness}}(B_s) = \frac{\log_2 \rho_w}{B_s}$ | Reciprocal $\frac{B}{\log_2 \rho_w}$ |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **9** | 9 | **342** | 175,165 | `["j=0"]` | **0.9353** | **0.9353** | 1.0692 |
| **13** | 13 | **7392** | 3,784,765 | `["j=1"]` | **0.9886** | **0.9886** | 1.0115 |
| **17** | 17 | **86208** | 44,138,557 | `["j=2"]` | **0.9644** | **0.9644** | 1.0369 |
| **18** | 18 | **200534** | 102,673,469 | `["j=0", "j=0"]` | **0.9785** | **0.9785** | 1.0219 |
| **22** | 22 | **672598** | 344,370,237 | `["j=0", "j=1"]` | **0.8800** | **0.8800** | 1.1364 |
| **26** | 26 | **5768022** | 2,953,227,325 | `["j=0", "j=2"]` | **0.8638** | **0.8638** | 1.1576 |
| **27** | 27 | **128651094** | 65,869,360,189 | `["j=0", "j=0", "j=0"]` | **0.9977** | **0.9977** | 1.0023 |
| **30** | 30 | **350179520** | 179,291,914,301 | `["j=2", "j=1"]` | **0.9461** | **0.9461** | 1.0570 |
| **31** | 31 | **732495072** | 375,037,476,925 | `["j=1", "j=0", "j=0"]` | **0.9499** | **0.9499** | 1.0527 |
| **34** | 35 | **6227033312** | 3,188,241,055,805 | `["j=1", "j=0", "j=1"]` | **0.9569** | **0.9296** | 1.0450 |
| **35** | 35 | **6227033312** | 3,188,241,055,805 | `["j=1", "j=0", "j=1"]` | **0.9296** | **0.9296** | 1.0757 |
| **36** | 36 | **23750971222** | 12,160,497,265,725 | `["j=0", "j=0", "j=0", "j=0"]` | **0.9574** | **0.9574** | 1.0445 |

---

## 4. Staged Sub-Phase Roadmap & Next Execution Step

```text
Phase 7.3S.0: Formal Mathematical Specifications & Equivalence Models (PASSED)
Phase 7.3S.1: Corpus & Theorem-Backed Generators (PASSED / FROZEN)
Phase 7.3S.2: Automata Transducer Probe & Bounded Equivalence Minimization (NEXT)
Phase 7.3S.3: Deterministic Rational Rigidity Orbit Probe
Phase 7.3S.4: Invariant Falsification on Enriched Corpus
Phase 7.3S.5: Evidence Report Synthesis & Human Strategy Selection
```

### Next Immediate Task: Phase 7.3S.2
1. Create `crates/collatz-cegar/src/eventual_zero_automata.rs`:
   - Implement `VanDerPutTransducer` computing 2-adic coefficients.
   - Implement `EventualZeroQuotientMinimizer` evaluating bounded equivalence $x \sim_{m,h} y$.
2. Create Python differential oracle `scripts/automata_small_modulus_oracle.py`.
3. Register unit/integration tests in `tests/phase73s2_math_audit.rs`.
