# Milestone 7.0 Certificate Mutation Test Summary

| Mutation Test Scenario | Rust Verifier (`collatz-verify`) | Python Oracle (`reference_oracle.py`) | Lean 4 Export Status |
| :--- | :---: | :---: | :---: |
| **Remove strict self-edge** | `INVALID` | `INVALID` | Not generated |
| **Demote strict arrow to weak** | `INVALID` | `INVALID` | Inequality failure |
| **Demote weak arrow to absent** | `INVALID` | `INVALID` | Unresolved relation |
| **Alter $L_1(n)$ coefficient** | `INVALID` | `INVALID` | Mismatch error |
| **Alter macrostep valuation word** | `INVALID` | `INVALID` | Constant mismatch |
| **Alter SCC state ID** | `INVALID` | `INVALID` | Not generated |
| **Delete closure graph** | `INVALID` | `INVALID` | Not generated |
| **Corrupt canonical hash** | `INVALID` | `INVALID` | Digest mismatch |
| **Reorder graphs only** | `VALID` | `VALID` | Unchanged |
| **Hit resource ceiling limit** | `UNRESOLVED` | `UNRESOLVED` | Not generated |

All 10 mutation scenarios behave exactly as specified by the Phase 7 security and verification model.
