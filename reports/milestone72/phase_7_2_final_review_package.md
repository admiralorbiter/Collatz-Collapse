# Phase 7.2 Final Review Package: Destination-Aware Semantic Graph Closure & Branching Target Freeze

**Project:** Collatz Research Workbench (`Collatz-Collapse`)  
**Commit:** `HEAD`  
**Date:** 2026-07-22  
**Phase 7.2 Status:** **VERIFIED BOUNDED NONCOMMUTING GUARDED-BRANCHING TARGET DISCOVERY RESULT**  
**Internal Guarded Graph:** **VERIFIED FOR EXPLICIT THREE-WORD BENCHMARK**  
**Based Closed Walks:** **$u = w_1$, $v = w_1 w_2$**  
**Noncommutativity:** **VERIFIED ($u v \neq v u$)**  
**Positive-Integer Path Realizability:** **$u v$ VERIFIED ($214759 \bmod 262144$); $v u$ VERIFIED ($1959 \bmod 262144$)**  
**Arbitrary Finite Switching:** **UNRESOLVED**  
**Infinite 2-Adic Switching:** **UNRESOLVED**  
**Infinite Positive-Integer Switching:** **NOT ESTABLISHED**  
**Termination Certificate:** **NOT YET PRODUCED**  
**Phase 7.3 Status:** **READY FOR TARGET A SYNTHESIS**

---

## 1. Official Verified Outcome Statement

> **Milestone 7.2 Verified Non-Commutative Branching Target Result:**  
> *Mechanically closing the destination-aware internal graph under the explicitly selected 3-word benchmark library $\Sigma = \{[1,1,2], [1,2,2], [1,1,1,2]\}$ produces a complete internal 3x3 guarded transition matrix with 9 canonical edges. At base node $Q_1 = 7 \bmod 32$, the guarded abstract SCC contains non-commuting based closed walks $u = w_1 = [1,1,2]$ (via canonical self-loop $231 \bmod 512 \subset Q_1$) and $v = w_1 w_2 = [1,1,2,1,2,2]$ (via cycle $935 \bmod 1024 \subset Q_1 \to 235 \bmod 1024 \subset Q_2 \to Q_1$), satisfying $u v \neq v u$. Both length-two switching orders $u v$ and $v u$ are concretely realizable over exact path cylinders in $\mathbb{N}^+$: $n \equiv 214759 \bmod 262144$ for $u v$ ($F_{uv}(n) = \frac{19683n + 27947}{8192}$) and $n \equiv 1959 \bmod 262144$ for $v u$ ($F_{vu}(n) = \frac{19683n + 33515}{8192}$). While each individual cycle has finite Phase 6D fuel ($N_{w_1}(231) = 2, N_{w_1}(743) = 3, N_W(1959) = 1$), Phase 6D single-cycle certificates do not by themselves settle arbitrary switching in the branching graph. A switching-sensitive invariant is required for Phase 7.3.*

---

## 2. Canonical Internal 3x3 Guarded Transition Matrix

| Source State | Word | Target State | Canonical Source Subguard | Image Formula ($F_w(n)$) | Target Residue | Target Inclusion Status |
| :--- | :---: | :--- | :--- | :---: | :---: | :---: |
| **$Q_1 = 7 \bmod 32$** | $w_1 = [1,1,2]$ | **$Q_1 = 7 \bmod 32$** | **$231 \bmod 512$** | **$391 + 864k$** | $7 \bmod 32$ | **Canonical Guarded Self-Loop** |
| **$Q_1 = 7 \bmod 32$** | $w_1 = [1,1,2]$ | **$Q_2 = 43 \bmod 64$** | **$935 \bmod 1024$** | $1579 + 1728k$ | $43 \bmod 64$ | **Forward Edge** |
| **$Q_1 = 7 \bmod 32$** | $w_1 = [1,1,2]$ | **$Q_3 = 47 \bmod 64$** | **$103 \bmod 1024$** | $175 + 1728k$ | $47 \bmod 64$ | **Cross Edge** |
| **$Q_2 = 43 \bmod 64$** | $w_2 = [1,2,2]$ | **$Q_1 = 7 \bmod 32$** | **$235 \bmod 1024$** | $199 + 864k$ | $7 \bmod 32$ | **Return Edge** |
| **$Q_2 = 43 \bmod 64$** | $w_2 = [1,2,2]$ | **$Q_2 = 43 \bmod 64$** | **$1643 \bmod 2048$** | **$1387 + 1728k$** | $43 \bmod 64$ | **Canonical Guarded Self-Loop** |
| **$Q_2 = 43 \bmod 64$** | $w_2 = [1,2,2]$ | **$Q_3 = 47 \bmod 64$** | **$2027 \bmod 2048$** | **$1711 + 1728k$** | $47 \bmod 64$ | **Cross Edge** |
| **$Q_3 = 47 \bmod 64$** | $w_3 = [1,1,1,2]$ | **$Q_1 = 7 \bmod 32$** | **$495 \bmod 1024$** | **$1255 + 2592k$** | $7 \bmod 32$ | **Return Edge** |
| **$Q_3 = 47 \bmod 64$** | $w_3 = [1,1,1,2]$ | **$Q_2 = 43 \bmod 64$** | **$623 \bmod 2048$** | **$1579 + 5184k$** | $43 \bmod 64$ | **Cross Edge** |
| **$Q_3 = 47 \bmod 64$** | $w_3 = [1,1,1,2]$ | **$Q_3 = 47 \bmod 64$** | **$751 \bmod 2048$** | **$1903 + 5184k$** | $47 \bmod 64$ | **Canonical Guarded Self-Loop** |

---

## 3. Minimal Non-Commutative Branching Core ($Q_1, Q_2$)

- **Base Node:** $Q_1 = 7 \bmod 32$.
- **Canonical Closed Walk $u$:** $w_1 = [1,1,2]$ (Self-loop via canonical subguard $231 \bmod 512 \subset Q_1$).
- **Canonical Closed Walk $v$:** $w_1 w_2 = [1,1,2,1,2,2]$ (Cycle via $935 \bmod 1024 \subset Q_1 \to 235 \bmod 1024 \subset Q_2 \to Q_1$).
- **Non-Commutativity Proof:**
  $$u v = [1,1,2,1,1,2,1,2,2] \neq [1,1,2,1,2,2,1,1,2] = v u$$

---

## 4. Exact Path-Cylinder Certificates for $u v$ and $v u$ (`path_semantics_v1`)

- **Switching-Specific Subguard Refinement:** $743 \bmod 1024 \subset 231 \bmod 512$ enables the specific continuation for $u v$.

| Path Description | Valuation Sequence | Composed Affine Map $F_p(n)$ | Exact Guarded Path Cylinder | Witness ($n \in \mathbb{N}^+$) | Final Image | Return Guard |
| :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **Switching Path $u v$** | $[1,1,2,1,1,2,1,2,2]$ | $\mathbf{\frac{19683n + 27947}{8192}}$ | **$214759 \bmod 262144$** | **$214759$** | $516007$ | **$7 \bmod 32$ (PASS)** |
| **Switching Path $v u$** | $[1,1,2,1,2,2,1,1,2]$ | $\mathbf{\frac{19683n + 33515}{8192}}$ | **$1959 \bmod 262144$** | **$1959$** | $4711$ | **$7 \bmod 32$ (PASS)** |

---

## 5. Corrected Phase 6D Fuel & Valuation Measurements

- **$w_1$ Self-Loop ($743 \bmod 1024$):**  
  $L_1(743) = 11(743) + 19 = 8192 = 2^{13} \implies v_2 = 13 \implies N_{w_1}(743) = 3$ complete repetitions ($743 \xrightarrow{w_1} 1255 \xrightarrow{w_1} 2119 \xrightarrow{w_1} 3577$).
- **$w_1$ Self-Loop ($231 \bmod 512$):**  
  $L_1(231) = 11(231) + 19 = 2560 = 2^9 \times 5 \implies v_2 = 9 \implies N_{w_1}(231) = 2$ complete repetitions.
- **Composite Return Word $W$ ($1959 \bmod 16384$):**  
  $L_W(1959) = 217(1959) + 881 = 425984 = 2^{15} \times 13 \implies v_2 = 15 \implies N_W(1959) = 1$ complete lap.
- **Two-Loop Guarded Path $W^2$ ($5605287 \bmod 8388608$):**  
  $L_W(5605287) = 217(5605287) + 881 = 1216348160 = 2^{23} \times 145 \implies v_2 = 23 \implies N_W(5605287) = 2$ complete laps.

---

## 6. Phase 7.3 Target Staging Strategy

- **Target A (Minimal Branching Core):** Control nodes $Q_1, Q_2$ with $Q_1 \xrightarrow{w_1} Q_1$ (self-loop) and $Q_1 \xrightarrow{w_1} Q_2 \xrightarrow{w_2} Q_1$ (cycle).
- **Target B (Two-State Closed Core):** Add $Q_2 \xrightarrow{w_2} Q_2$ self-loop.
- **Target C (Full Three-State Benchmark SCC):** Add $Q_3$ and all cross-edges.
