# Phase 7.1 Research Report: Mechanical Semantic Graph Discovery

## Executive Summary & Primary Outcome
**Primary Outcome:** **MILESTONE 7.1 BOUNDED SEMANTIC-REFINEMENT RESULT**

*Milestone 7.1 bounded semantic-refinement result: The source cylinder $7 mod 32$ was soundly partitioned modulo 256 to determine its $[1,1,2]$ images modulo 16. This refinement invalidated the original two-state transition abstraction but was not sufficient to determine membership in the proposed $43 mod 64$ successor state (which requires source $mod 1024$). The composite word $W=[1,1,2,1,2,2]$ independently admits a valid Phase 6D finite-fuel certificate. No sound recurrent SCC or Phase 7 SCT target has yet been established.*

---

## 4 Independent Validity Layers

| Validity Layer | Audit Status | Rationale |
| :--- | :---: | :--- |
| **1. Arithmetic Validity** | **VALID** | Composite affine map, multiplier $729/512$, fixed point $x_W^* = -881/217$, and fuel formula verified. |
| **2. Abstract Semantic Validity** | **PARTIALLY VALID** | All 8 subguards mod 256 $	o$ mod 16 verified; target mod 64 requires source mod 1024. |
| **3. Termination-Algebra Validity** | **VALID (Phase 6D)** | Finite fuel drop $v_2(L_W) - 9$ verified for composite word $W$. |
| **4. Positive-Integer Scope Validity** | **UNVERIFIED SCC** | No sound recurrent SCC established at static mod 256 / mod 1024 precision. |

---

## Key Metrics Across Refinement Depth

| Refinement Depth | States | Edges | Sound Recurrent SCCs | Non-Commuting Branching SCCs | Max Exponent | Outcome |
| :---: | :---: | :---: | :---: | :---: | :---: | :--- |
| **Depth 0** | 2 | 2 | 0 | 0 | $2^6 = 64$ | Proposed 2-State Graph Invalidated |
| **Depth 1** | 8 | 8 | 0 | 0 | $2^8 = 256$ | Mod 256 Partition to Target Mod 16 |
| **Depth 2** | 32 | 32 | 0 | 0 | $2^{10} = 1024$ | Mod 1024 Partition to Target Mod 64 (No Recurrent SCC) |
