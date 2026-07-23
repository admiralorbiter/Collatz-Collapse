# Phase 7.3D Closeout Report — Phase Acceleration, Induced $v$-to-$v$ Map, and Normalized $z$-Coordinate Branch Engine

**Date**: July 22, 2026  
**Status**: **COMPLETED FOR ACCELERATED BRANCH ENGINE & FROZEN AT `SOUND_ACCELERATED_UNRANKED` (PHASE 7.3D-R INVARIANT SEARCH OPEN)**  
**Crates Modified/Created**: `collatz-affine`, `collatz-cegar`, `collatz-cert`  
**Test Matrix**: 100% Rust workspace test suite, Python differential oracle (`scripts/accelerated_v_map_oracle.py`, **0 diff**), 10-corruption mutation matrix (**10/10 REJECTED**)

---

## 1. Executive Summary

Phase 7.3D replaces flat 1-step finite graph cycle elimination (which is mathematically impossible due to unbounded finite positive orbits, `CLM-P7X-UNBOUNDED-FINITE-ORBITS-001`) with **$u$-Phase Acceleration** and the **Normalized $z$-Coordinate Dyadic Branch Engine**.

Because $u$ acts on valuation $x$ as $x \mapsto x - 4$, every pure-$u$ block from an arbitrary state terminates in $\ell_u(x) = \max\left(0, \left\lfloor \frac{x-5}{4} \right\rfloor\right)$ steps (`CLM-P7X-U-PHASE-COUNTDOWN-001`). Every infinite $D$-orbit must contain infinitely many $v$-events separated by finite $u$-countdowns ($v u^j v$).

---

## 2. Core Mathematical Theorems

### Theorem 1 (Arbitrary $u$-Phase Countdown & Conditioned $v$-Step Intervening Count)
- **Arbitrary $u$-Phase Length**: $\ell_u(x) = \max\left(0, \left\lfloor \frac{x-5}{4} \right\rfloor\right)$.
- **Conditioned Intervening $u$-Count**: Between two $v$-events, post-$v$ valuation is $x' = 5 + \delta$. Requiring $x' - 4j = 6$ yields $\delta = 1 + 4j \implies j = \frac{\delta - 1}{4}$ when $\delta \equiv 1 \pmod 4$ (`CLM-P7X-U-PHASE-COUNTDOWN-001`).

### Theorem 2 (Positive-Realizability Domain & Normalized $z$-Coordinate)
For $U = 81 + 256t$, quotient state $k = \frac{159 + 512t}{11} \ge 0 \iff t \ge 0, t \equiv 1 \pmod{11}$. Defining $z = \frac{t - 1}{11} \in \mathbb{Z}_{\ge 0} \iff t = 1 + 11z$ maps positive realizability directly onto $\mathbb{Z}_{\ge 0}$ without side-conditions (`CLM-P7X-POSITIVE-REALIZABILITY-DOMAIN-001`).

### Theorem 3 (Exact Normalized $z$-Branch Normal Form)
For $t = c_j + M_j m \implies t' = d_j + Q_j m$, positive realizability $t \equiv 1 \pmod{11}$ requires $m = \mu_j + 11n$ where $\mu_j \equiv (1 - c_j) M_j^{-1} \pmod{11}$.
The exact normalized $z$-branch equation is:
$$z = C_j + M_j n \implies z_{\text{next}} = D_j + Q_j n$$
where $C_j = \frac{c_j - 1 + M_j \mu_j}{11}$ and $D_j = \frac{d_j - 1 + Q_j \mu_j}{11}$.

| $j$ | $c_j$ | $d_j$ | $\mu_j \pmod{11}$ | $C_j$ | Modulus $M_j = 2^{9+4j}$ | $D_j$ | Multiplier $Q_j = 3^{6+3j}$ |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 ($v v$) | 179 | 255 | 7 | **342** | 512 | **487** | 729 |
| 1 ($v u v$) | 7,585 | 18,225 | 9 | **7,392** | 8,192 | **17,761** | 19,683 |
| 2 ($v u^2 v$) | 30,785 | 124,821 | 7 | **86,208** | 131,072 | **349,537** | 531,441 |
| 3 ($v u^3 v$) | 529,985 | 3,626,208 | 9 | **1,764,032** | 2,097,152 | **12,069,670** | 14,348,907 |

---

## 3. Verification & Oracle Results

1. **Python Differential Oracle** (`scripts/accelerated_v_map_oracle.py`):
   - Verified exact $\mu_j, C_j, D_j$ across $j = 0 \dots 3$ and property test identity $z \leftrightarrow t \leftrightarrow m \leftrightarrow z'$.
   - 100% agreement with Rust engine (**0 diff**).

2. **10-Corruption Mutation Matrix** (`accelerated_ranking_mutation_test.rs`):
   - Verified rejection of all invalid/corrupted certificates (**10/10 REJECTED**).

---

## 4. Completed Phase 7.3D Branch-Engine Artifacts

- **Exact $u$-Phase Countdown Engine** (`collatz-affine/src/u_block_accelerator.rs`)
- **Induced $v$-to-$v$ Map & Realizability Domain** (`collatz-cegar/src/induced_v_map.rs`)
- **Normalized $z$-Coordinate Dyadic Branch Engine** (`collatz-cegar/src/induced_v_map.rs`)
- **Independent Verifier & Lean 4 Export** (`collatz-cert/src/accelerated_ranking_verifier.rs`)

The accelerated branch engine is frozen at **`SOUND_ACCELERATED_UNRANKED`**. Target A invariant search continues under **Phase 7.3D-R**.
