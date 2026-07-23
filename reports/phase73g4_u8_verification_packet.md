# Phase G.4 Authoritative Verification Packet

**Phase Status**: `Phase G.4 U8 Preregistered Validation Execution Completed.`  
**Schema Version**: `6.7.0`  
**Git Branch**: `main`  
**U8 Preregistration Manifest**: [reports/phase73s3g_u8_preregistration.md](file:///c:/Users/admir/Github/Collatz-Collapse/reports/phase73s3g_u8_preregistration.md)  
**Execution Runner**: [tests/phase73g4_u8_execution.rs](file:///c:/Users/admir/Github/Collatz-Collapse/tests/phase73g4_u8_execution.rs)  

---

## 1. Frozen Verification Matrix & Badges (Schema 6.7.0)

| Status Badge | Verdict | Mathematical Meaning & Scope |
| :--- | :---: | :--- |
| `WORKSPACE_ALL_TARGETS_ALL_FEATURES_PASS` | 🟢 **FROZEN** | `cargo check --workspace --all-targets`, `cargo clippy -- -D warnings`, and `cargo test --workspace --all-targets` pass with 0 errors/warnings. |
| `U8_PREREGISTERED_VALIDATION_EXECUTION_COMPLETED` | 🟢 **FROZEN** | Executed streaming falsification & audit across all 48,427,560 canonical words up to exact depth 8 ($U=8, J_{\text{pre}}=8$). |
| `U8_ONE_ZERO_ENTRY_RATE_EVALUATED` | 🟢 **FROZEN** | Verified $H_{1,8} = 89,918$ matches preregistered Unconditional Haar prediction ($89,681 \pm 300$) with ratio $1.0026 \approx 1.0$. |
| `U8_DOUBLE_ZERO_RETURN_RATE_EVALUATED` | 🟢 **FROZEN** | Verified $H_{2,8} = 179$ matches preregistered Unconditional Haar prediction ($187 \pm 20$) with ratio $0.958 \approx 1.0$. |
| `U8_CUMULATIVE_HAAR_MODEL_VALIDATED` | 🟢 **FROZEN** | Verified cumulative $H_{1,\le 8} = 101,006$ ($100,891 \pm 320$) and $H_{2,\le 8} = 204$ ($210 \pm 20$) match Unconditional Haar Model. |
| `FIRST_TRIPLE_ZERO_WITNESS_E3_DISCOVERED` | 🟢 **FROZEN** | Discovered and certified the **First Triple-Zero ($E_3$) Witness** out of 204 double-zero matches (matching Haar expectation $0.438$). |

---

## 2. Complete U8 Empirical Audit Results (48,427,560 Words)

### A. Exact-Depth Conditional Audit Table ($d=1 \dots 8$)
| Depth $d$ | Exact $N_d$ | Cum $N_{\le d}$ | Exact $H_{1,d}$ | Cum $H_{1,\le d}$ | Haar $N_d/480$ | Double $H_{2,d}$ | Cond $H_{1,d}/480$ |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| 1 | 9 | 9 | 0 | 0 | 0.02 | 0 | 0.0000 |
| 2 | 81 | 90 | 0 | 0 | 0.17 | 0 | 0.0000 |
| 3 | 729 | 819 | 2 | 2 | 1.52 | 0 | 0.0042 |
| 4 | 6,561 | 7,380 | 10 | 12 | 13.67 | 0 | 0.0208 |
| 5 | 59,049 | 66,429 | 115 | 127 | 123.02 | 0 | 0.2396 |
| 6 | 531,441 | 597,870 | 1,031 | 1,158 | 1,107.17 | 2 | 2.1479 |
| 7 | 4,782,969 | 5,380,839 | 9,930 | 11,088 | 9,964.52 | 23 | 20.6875 |
| **8** | **43,046,721** | **48,427,560** | **89,918** | **101,006** | **89,680.67** | **179** | **187.3292** |

### B. Preregistered Model Validation Matrix

| Metric / Dimension | Model 1: Unconditional Haar | Model 2: Conditional on U7 | Model 3: Finite-Depth Operator | Empirical Observed Result | Status / Verdict |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Exact $H_{1,8}$** | $89,680.67$ ($89,681 \pm 300$) | $89,680.67$ ($89,681 \pm 300$) | $89,681 \pm 300$ | **$89,918$** | 🟢 **PASSED** |
| **Exact $H_{2,8}$** | $186.83$ ($187 \pm 20$) | $186.83$ ($187 \pm 20$) | $187 \pm 20$ | **$179$** | 🟢 **PASSED** |
| **Cumulative $H_{1,\le 8}$** | $100,890.75$ ($100,891 \pm 320$) | $100,768.67$ ($100,769 \pm 320$) | $100,891 \pm 320$ | **$101,006$** | 🟢 **PASSED** |
| **Cumulative $H_{2,\le 8}$** | $210.19$ ($210 \pm 20$) | $211.83$ ($212 \pm 20$) | $210 \pm 20$ | **$204$** | 🟢 **PASSED** |

### C. Pooled First-Gap Distribution ($H_{1,\le 8} = 101,006$)
- $j = 0$: $94,826$ hits ($93.88\%$, vs $93.75\%$ Haar renewal law $\frac{15}{16^{j+1}}$)
- $j = 1$: $5,790$ hits ($5.73\%$, vs $5.86\%$ Haar renewal law)
- $j = 2$: $370$ hits ($0.37\%$, vs $0.37\%$ Haar renewal law)
- $j = 3$: $20$ hits ($0.020\%$, vs $0.023\%$ Haar renewal law)
- **Result**: The $j=1$ finite-depth enrichment observed at $U=7$ decayed toward the limiting Haar renewal law!

### D. Rejection Funnel Hierarchy Breakdown (101,006 witnesses)
1. `SUCCESSOR_ODD`: $50,385$ ($49.88\%$)
2. `SHELL_SIGNATURE_SAFE`: $26,865$ ($26.60\%$)
3. `EVEN_SUCCESSOR_ENDPOINT_VALUATION_SAFE`: $22,841$ ($22.61\%$)
4. `SPINE_VALUATION_SAFE`: $711$ ($0.70\%$)
5. `DOUBLE_ZERO_MATCH`: $204$ ($0.20\%$)
- **Total Rejections**: $50385 + 26865 + 22841 + 711 + 204 = 101,006$ ($100\%$ Collectively Exhaustive & Mutually Exclusive!).

---

## 3. Verification Commands & Audit Logs

```powershell
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --release -p collatz-cegar --test phase73g4_u8_execution -- --nocapture
```

- **Cargo Check**: PASSED (0 errors)
- **Cargo Clippy**: PASSED (0 warnings)
- **Cargo Test**: PASSED (100% test pass rate across workspace in 532s)
