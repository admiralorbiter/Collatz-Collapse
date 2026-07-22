# Phase 7.1 Final Review Package: Semantic Graph Discovery & Target Classification

**Project:** Collatz Research Workbench (`Collatz-Collapse`)  
**Phase:** Phase 7 Size-Change Termination & Language Invariants  
**Milestone:** Milestone 7.1 (Semantic Graph Discovery & Target Classification)  
**Status:** **VERIFIED BOUNDED SEMANTIC-REFINEMENT RESULT**  
**SCT Status:** **NOT APPLICABLE**  
**Recurrent SCC Status:** **NOT ESTABLISHED**  
**Original Target Status:** **INVALIDATED**  
**Composite Phase 6D Certificate:** **VERIFIED**

---

## 1. Official Verified Outcome Statement

> **Milestone 7.1 Verified Bounded Semantic-Refinement Result:**  
> *The exact $[1,1,2]$ source cylinder $7 \bmod 32$ was partitioned into eight disjoint subcylinders modulo 256, producing universally valid target images modulo 16. This refinement invalidated the proposed static two-state abstraction but did not contain enough source precision to classify membership in the proposed $43 \bmod 64$ successor state; doing so requires source precision modulo 1024. The reverse proposed transition is independently invalid because $F_{[1,2,2]}(43) = 37 \equiv 5 \bmod 32 \neq 7 \bmod 32$. Separately, the composite word $W=[1,1,2,1,2,2]$ has a verified Phase 6D finite-fuel certificate based on $L_W(n) = 217n + 881$. No sound recurrent SCC or Phase 7 SCT target was established.*

---

## 2. Mathematical Summary of Validated Component Claims

### A. Independent Phase 6D Composite Word Certificate (`composite_W_finite_fuel.json`)
- **Word:** $W = [1,1,2,1,2,2]$ ($k=6, A=9, c_W=881$).
- **Affine Map:** $F_W(n) = \frac{729n + 881}{512}$.
- **Recomputed Fixed Point:** $x_W^* = -\frac{881}{217}$ ($217n + 881 = 0$).
- **Normalized Positive Linear Form:** $L_W(n) = 217n + 881$.
- **Transformation Multiplier:** $L_W(F_W(n)) = \mathbf{\frac{729}{512}} L_W(n)$.
- **Valuation Drop:** $v_2(L_W(F_W(n))) = v_2(L_W(n)) - 9$.
- **Source Cylinders:**
  - Broad terminal-at-least ($M=9$): $n \equiv 423 \bmod 512 \implies F_W(423) = 604$ (EVEN!). Fuel $= 0$.
  - Exact-word cylinder ($M=10$): $n \equiv 935 \bmod 1024 \implies F_W(935) = 1333$ (ODD!). $1333 \equiv 21 \bmod 32$. Fuel $= 1$.
- **Fuel Formula:** $N_W(n) = \max\left(0, \left\lfloor \frac{v_2(217n + 881) - 1}{9} \right\rfloor\right)$.
- **Exact One-Lap Return Behavior:**  
  *The exact one-lap return starting at $n=1959 \bmod 16384$ moves from $167 \bmod 256$ to $231 \bmod 256$, so it does not produce a self-loop on the original mod-256 subcell. Moreover, $2791 \not\equiv 935 \bmod 1024$, so the exact $W$-source cylinder is not re-entered, and the same composite word cannot immediately repeat. The finite-fuel calculation confirms this: $v_2(217 \times 1959 + 881) = 15 \implies N_W(1959) = \lfloor (15 - 1)/9 \rfloor = 1$, supporting exactly one complete $W$-lap.*

### B. Modulo 256 Partition Set (`partition_7_mod_32.json`)
- Source $7 \bmod 32$ ($M=5$) partitioned into 8 disjoint cells modulo 256 ($M=8$).
- Soundly determines target residues modulo 16:
  - $7 \bmod 256 \to 13 \bmod 16$
  - $39 \bmod 256 \to 3 \bmod 16$
  - $71 \bmod 256 \to 9 \bmod 16$
  - $103 \bmod 256 \to 15 \bmod 16$
  - $135 \bmod 256 \to 5 \bmod 16$
  - $167 \bmod 256 \to 11 \bmod 16$
  - $199 \bmod 256 \to 1 \bmod 16$
  - $231 \bmod 256 \to 7 \bmod 16$

---

## 3. Key Methodological Accomplishments for Phase 7.2

1. **Destination-Modulus Precision Rule ($M_{\text{source}} \ge A + q$):**  
   Targeting $Q_2 = 43 \bmod 64$ ($q=6$) with $w_1 = [1,1,2]$ ($A=4$) requires source precision $M_{\text{source}} \ge 4 + 6 = 10$ ($1024$). Modulo 256 determines targets only modulo 16.
2. **Canonical Image of $F_2(43)$:**  
   $F_2(43) = 37 \equiv \mathbf{5} \bmod 32 \neq 7 \bmod 32$.
3. **Exact Composite Return Cylinder ($n \equiv 1959 \bmod 16384$):**  
   One-lap return requires $M=14$ ($16384$). $F_W(1959) = 2791 \equiv 7 \bmod 32$, but $2791 \equiv 231 \bmod 256$, which lands in a different mod 256 subcell than $1959 \equiv 167 \bmod 256$. This demonstrates finite repetition fuel while ruling out static residue recurrence.
4. **Exact Alphabet Symbol $[1,1,1,2]$:**  
   $15 \bmod 32$ is a broad `TerminalAtLeast` guard ($F(15) = 40$ EVEN). The exact-word guard is $n \equiv 47 \bmod 64 \implies F(47) = 121$ (ODD!).

---

## 4. Claims Registry Status (`claims/claims.toml`)

- **`CLM-SCT-M71-SEMANTIC-REFINEMENT-001` (Verified Finite Theorem):** The exact source cylinder $7 \bmod 32$ partitions into eight disjoint subcylinders modulo 256, each having a universally determined $[1,1,2]$ image modulo 16. This refinement is insufficient to determine membership in the proposed $43 \bmod 64$ successor state and invalidates the original static two-state abstraction.
- **`CLM-SCT-M71-COMPOSITE-W-001` (Domain-Scoped Certificate):** Composite word $W=[1,1,2,1,2,2]$ has finite repetition fuel on positive integers, certified by normalized linear form $L_W(n) = 217n + 881$.
- **`CLM-SCT-M71-COUNTEREXAMPLE-103` (Verified Finite Theorem):** Cross-feature weak relation $v_2(L_1(F_1(n))) \le v_2(L_2(n))$ is refuted on $7 \bmod 32$; $n=103$ is proved minimal (checked 7, 39, 71).
