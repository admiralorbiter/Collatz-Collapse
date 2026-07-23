# Phase 7.3D-R Closeout & Audit Report — Dyadic Branch Transition System & Accelerated Source-Lift Engine

**Date**: July 22, 2026  
**Status**: **`VERIFIED_GENERIC_BRANCH_AND_EDGE_IDENTITIES` ($\forall j \ge 0$), `VERIFIED_BOUNDED_ACCELERATED_ANALYSIS` ($j \le 8$), & `SOUND_ACCELERATED_UNRANKED` (GLOBAL DYNAMICS)**  
**Crates Modified/Created**: `collatz-cegar`, `collatz-cert`, `collatz-affine`  
**Test Matrix**: 100% Rust workspace test suite, Python differential oracle (`scripts/accelerated_invariant_oracle.py`, **0 diff**), 10-corruption mutation matrix (**10/10 REJECTED**)

---

## 1. Executive Summary

Phase 7.3D-R achieves a complete mathematical formalization and computational engine for the **Dyadic Branch Transition System** $\mathcal{G}_{\text{acc}}$ over nonoverlapping blocks $B_j = v u^j$ (with precision $A(B_j) = 9 + 4j$).

It proves that $\mathcal{G}_{\text{acc}}$ is the **complete directed graph on $\mathbb{N}_0$**, deriving the exact complete edge normal form:
$$n = R_{j,j'} + M_{j'} h \implies n' = S_{j,j'} + Q_j h$$
and proving that positive realizability of infinite $D$-trajectories is equivalent to eventual-zero output of the accelerated source-lift digit sequence $(\Lambda_m)_{m=1}^\infty$.

---

## 2. Core Mathematical Theorems & Exact Reference Packet

### Theorem 1 (Generic Branch Normal Form)
For all $j \ge 0$, moduli $M_j = 2^{9+4j}$ and multipliers $Q_j = 3^{6+3j}$, the exact branch parameters are defined by:
$$c_j \equiv 729^{-1} \left(81 \cdot 2^{1+4j} \cdot 27^{-j} - 231\right) \pmod{M_j}$$
$$d_j = \frac{27^j (231 + 729 c_j) / 2^{1+4j} - 81}{256}$$
$$\mu_j \equiv (1 - c_j) M_j^{-1} \pmod{11}, \qquad 0 \le \mu_j < 11$$
$$C_j = \frac{c_j - 1 + M_j \mu_j}{11}, \qquad D_j = \frac{d_j - 1 + Q_j \mu_j}{11}$$

The normalized $z$-coordinate branch mapping is:
$$z = C_j + M_j n \implies z_{\text{next}} = D_j + Q_j n$$

#### Proof of Equivalence & Disjointness
Membership $t \equiv c_j \pmod{M_j} \iff z \in C_j + M_j \mathbb{Z}_2$ is equivalent to BOTH $v_2(231+729t) = 1+4j$ AND $\frac{231+729t}{2^{1+4j}} \cdot 27^j \equiv 81 \pmod{256}$. Since $1+4j \neq 1+4j'$ for $j \neq j'$, the branch cylinders $C_j + M_j \mathbb{Z}_2$ are pairwise disjoint (`CLM-P7X-DISJOINT-BRANCH-DOMAINS-001`).

### Theorem 2 (Complete Directed Graph & Nonnegative Edge Offset $S_{j,j'} \ge 0$)
The accelerated gap graph $\mathcal{G}_{\text{acc}}$ is the **complete directed graph on $\mathbb{N}_0$**. Every edge $j \to j'$ exists.
For parameter $n$ in branch $j$, entry into branch $j'$ requires $z_{\text{next}} \equiv C_{j'} \pmod{M_{j'}}$, yielding:
$$n \equiv R_{j,j'} \equiv Q_j^{-1} (C_{j'} - D_j) \pmod{M_{j'}}$$
$$S_{j,j'} = \frac{D_j + Q_j R_{j,j'} - C_{j'}}{M_{j'}} \ge 0 \implies h_{\text{min}}(j, j') = 0$$
The canonical parameter $n = R_{j,j'}$ maps directly to nonnegative target $n' = S_{j,j'}$.

#### Verified Reference Orientation Edge Values
- $0 \to 0$: $R_{0,0} = 391, S_{0,0} = 557, h_{\text{min}} = 0$.
- $0 \to 1$: $R_{0,1} = 1313, S_{0,1} = 116, h_{\text{min}} = 0$.
- $1 \to 0$: $R_{1,0} = 327, S_{1,0} = 12605, h_{\text{min}} = 0$.
- $1 \to 1$: $R_{1,1} = 2485, S_{1,1} = 5972, h_{\text{min}} = 0$.
- $2 \to 3$: $R_{2,3} = 1,201,743, S_{2,3} = 304,534, h_{\text{min}} = 0$.

---

## 3. Composite Affine Map & Complete Child-State Update

### Theorem 3 (Shift Identity & Complete Child Update)
For accelerated prefix $s$ with accumulated state $(B_s, A_s^{\text{odd}}, \rho_s, y_s)$, the composite affine map is $T_s(z) = \frac{2^{B_s} A_s^{\text{odd}} z + E_s}{2^{B_s}}$ with endpoint $y_s = T_s(\rho_s)$.
Shifting the source by $2^{B_s} \Lambda$ shifts the endpoint by $A_s^{\text{odd}} \Lambda$:
$$T_s(\rho_s + 2^{B_s} \Lambda) = y_s + A_s^{\text{odd}} \Lambda$$
The block lift digit is:
$$\Lambda_j(s) \equiv (C_j - y_s) (A_s^{\text{odd}})^{-1} \pmod{M_j}$$
Defining shifted parameter $q = \frac{y_s + A_s^{\text{odd}} \Lambda_j(s) - C_j}{M_j} \ge 0$, the complete child-state update is:
$$\begin{pmatrix} \rho_{sj} \\ B_{sj} \\ A_{sj}^{\text{odd}} \\ y_{sj} \end{pmatrix} = \begin{pmatrix} \rho_s + 2^{B_s} \Lambda_j(s) \\ B_s + 9 + 4j \\ Q_j A_s^{\text{odd}} \\ D_j + Q_j q \end{pmatrix}$$

---

## 4. 3-Way Agreement & Unaccelerated Guard Alignment

1. **Empty Prefix Agreement**: $\Lambda_j(\epsilon) = C_j$ for all $j$.
2. **Two-Step Sequence Agreement**: $\Lambda_{j'}(j) = R_{j,j'}$ for all $j, j'$.
3. **Quotient Guard Alignment**: For any gap sequence $j_1, \dots, j_r$ with $B_r = \sum (9+4j_i)$, quotient state $k \equiv 61 + 512 \rho_r \pmod{2^{B_r + 9}}$ agrees 100% with the unaccelerated guard engine across test sequences `[0]`, `[1]`, `[2]`, `[0,0]`, `[0,1]`, `[1,0]`, `[1,2]`, `[2,3]`.

---

## 5. Accelerated Realizability & Survivor Haar Measure

- **Realizability Theorem**: For stream $j = (j_1, j_2, \dots)$:
  $$z \in \mathbb{Z}_{\ge 0} \iff k = 61 + 512 z \in \mathbb{Z}_{\ge 0} \iff \text{block lift digits } \Lambda_m = 0 \text{ eventually} \iff (\rho_m) \text{ eventually constant}$$
- **Survivor Haar Measure**: The 2-adic geometric density of $v$-event states in $z$-space admitting another accelerated step is:
  $$\mu_z \left(\bigcup_{j=0}^\infty (C_j + M_j \mathbb{Z}_2)\right) = \sum_{j=0}^\infty 2^{-(9+4j)} = \frac{1/512}{1 - 1/16} = \frac{1}{480}$$
  In quotient $k$-space ($k = 61 + 512z$), the absolute 2-adic density is $2^{-9} \cdot \frac{1}{480} = \frac{1}{245,760}$.

---

## 6. Verification Status & Roadmap Statement

- **Closed & Verified Infrastructure**: Dyadic branch engine, complete edge composition, accelerated coding, and accelerated lift semantics.
- **Global Dynamics Status**: `SOUND_ACCELERATED_UNRANKED` (The global invariant-search problem remains open; Target A termination is not declared until a well-founded invariant or ranking is established over $\mathcal{G}_{\text{acc}}$).
- **Workspace Tests**: `cargo test --workspace` (**ALL PASSED**).
- **Clippy**: `cargo clippy --workspace --all-targets -- -D warnings` (**0 warnings**).
- **Python Oracle**: `scripts/accelerated_invariant_oracle.py` (**0 diff**).
