# Phase 7.3S.3G Preregistered U8 Prediction Manifest

**Status**: `FROZEN PREREGISTRATION MANIFEST (Pre-Execution)`  
**Target Space**: $U=8, J_{\text{pre}}=8$ ($9$-symbol alphabet $\{0 \dots 8\}$)  
**Total Canonical Words at Exact Depth 8**: $N_8 = 9^8 = 43,046,721$  
**Cumulative Words through Depth 8**: $N_{\le 8} = \sum_{r=1}^8 9^r = 48,427,560$  

---

## 1. Dual Quantitative Cumulative & Exact-Depth Predictions

| Metric / Metric Dimension | Dual Prediction Model | Preregistered Prediction | Tolerance Band (`preregistered_absolute_tolerance`) |
| :--- | :--- | :---: | :---: |
| **Exact Depth 8 One-Zero Hits ($H_{1,8}$)** | Unconditional Haar Renewal | **$89,680.67$** | **$89,681 \pm 300$** |
| **Exact Depth 8 Double-Zero Matches ($H_{2,8}$)** | Unconditional Haar Renewal | **$186.83$** | **$187 \pm 20$** |
| **Cumulative One-Zero Hits ($H_{1,\le 8}$)** | Unconditional Haar Model | **$100,890.75$** | **$100,891 \pm 320$** |
| **Cumulative One-Zero Hits ($H_{1,\le 8}$)** | Conditional on Observed U7 | **$100,768.67$** | **$100,769 \pm 320$** |
| **Cumulative Double-Zero Matches ($H_{2,\le 8}$)** | Unconditional Haar Model | **$210.19$** | **$210 \pm 20$** |
| **Cumulative Double-Zero Matches ($H_{2,\le 8}$)** | Conditional on Observed U7 | **$211.83$** | **$212 \pm 20$** |

---

## 2. Preregistered Exact-Depth 8 Gap-Pair Returns ($H_{2,8} \approx 186.83$)

| Exact-Depth Gap Pair $(j, k)$ | Unconditional Product Haar Calibration | Predicted Hits at Exact Depth 8 | Tolerance Band |
| :---: | :---: | :---: | :---: |
| $(j=0, k=0)$ | $\frac{225}{256} = 87.89\%$ | **$164.21$** | **$164 \pm 18$** |
| $(j=1, k=0)$ | $\frac{15}{256} = 5.49\%$ | **$10.26$** | **$10 \pm 5$** |
| $(j=0, k=1)$ | $\frac{15}{256} = 5.49\%$ | **$10.26$** | **$10 \pm 5$** |
| $(j=1, k=1)$ | $\frac{1}{256} = 0.34\%$ | **$0.64$** | **$1 \pm 1$** |
| `OTHER_GAP_PAIRS` ($j \ge 2 \lor k \ge 2$) | $\frac{256 - 256}{256}$ tail | **$1.46$** | **$1 \pm 2$** |
| **Total Exact-Depth 8 Matches** | **$100.0\%$** | **$186.83$** | **$187 \pm 20$** |

---

## 3. Preregistration Verification Protocol

```powershell
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test -p collatz-cegar --test phase73s3g_soundness_gate -- --nocapture
```
