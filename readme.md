# Collatz Research Workbench in Rust

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

An experimental computing platform for investigating the Collatz Conjecture ($3x+1$ problem) through exact arithmetic, symbolic residue-class analysis, automated program verification (CEGAR & SyGuS), and machine-verifiable certificates.

Rather than running brute-force numerical searches to verify isolated large integers, this workbench produces **compact, machine-verifiable mathematical artifacts**: residue-class descent certificates, adversarial valuation prefixes, transition invariants, and SAT/Lean proof bundles.

---

## Key Core Mathematical Identities

### 1. Accelerated Odd-Only Map
For an odd integer $n \in 2\mathbb{N}+1$ with valuation $a = v_2(3n+1)$:
$$S(n) = \frac{3n+1}{2^a}$$

### 2. Exact Affine Transformation
After $k$ odd steps with valuation word $(a_0, a_1, \ldots, a_{k-1})$ and total valuation $A_k = \sum a_i$:
$$n_k = \frac{3^k n_0 + c_k}{2^{A_k}}, \qquad c_0 = 0, \quad c_{i+1} = 3c_i + 2^{A_i}$$

### 3. Closed-Form Modular Inversion Identity
The starting congruence class $n_0 \pmod{2^{A_k}}$ for any valuation word is determined in closed form by:
$$n_0 \equiv -c_k \cdot (3^k)^{-1} \pmod{2^{A_k}}$$

### 4. Exact Integer Descent Threshold $B$
When $2^{A_k} > 3^k$, every integer in the residue class satisfying $n_0 \ge B$ descends ($S^k(n_0) < n_0$):
$$B = \left\lfloor \frac{c_k}{2^{A_k} - 3^k} \right\rfloor + 1$$

---

## Documentation Architecture (`docs/`)

The detailed specifications, mathematical derivations, verification protocols, and architecture guides are organized in the `docs/` directory:

| Document | Description |
| :--- | :--- |
| **[00_overview_and_roadmap.md](file:///c:/Users/admir/Github/Collatz-Collapse/docs/00_overview_and_roadmap.md)** | Mission statement, research philosophy, certificate separation, and 9-phase roadmap. |
| **[01_mathematical_foundation.md](file:///c:/Users/admir/Github/Collatz-Collapse/docs/01_mathematical_foundation.md)** | Collatz maps, affine recurrences, modular inversion, 2-adic topology & $-1/3$ pole. |
| **[02_architecture_and_crates.md](file:///c:/Users/admir/Github/Collatz-Collapse/docs/02_architecture_and_crates.md)** | Progressive 4-crate core rollout, crate dependency graph, and tiered arithmetic (`u128`/`num-bigint`/`rug`). |
| **[03_sieves_and_pruning.md](file:///c:/Users/admir/Github/Collatz-Collapse/docs/03_sieves_and_pruning.md)** | Kinematic vs Minimality sieve taxonomy, `PrefixSieve` trait, and Roaring Bitmaps. |
| **[04_cegar_and_synthesis.md](file:///c:/Users/admir/Github/Collatz-Collapse/docs/04_cegar_and_synthesis.md)** | Relational abstract domains, CEGAR loop with Craig interpolation, and SyGuS ranking functions. |
| **[05_certificates_and_verify.md](file:///c:/Users/admir/Github/Collatz-Collapse/docs/05_certificates_and_verify.md)** | JSON BigInt string schemas, `collatz-verify` engine, LRAT proof logging & Lean 4 bridge. |
| **[06_experimental_suite.md](file:///c:/Users/admir/Github/Collatz-Collapse/docs/06_experimental_suite.md)** | Specifications for Core Experiments 0–7 and Side Experiments A–G (Sieve Ablation, Grammar Inference). |

---

## Initial Workspace Structure (Phase 1: Progressive Core)

```text
collatz-lab/
├── Cargo.toml
├── README.md
├── docs/
│   ├── 00_overview_and_roadmap.md
│   ├── 01_mathematical_foundation.md
│   ├── 02_architecture_and_crates.md
│   ├── 03_sieves_and_pruning.md
│   ├── 04_cegar_and_synthesis.md
│   ├── 05_certificates_and_verify.md
│   └── 06_experimental_suite.md
└── crates/
    ├── collatz-core/          # Exact ordinary/odd steps, u128/BigUint arithmetic
    ├── collatz-affine/        # Affine prefix recurrences & closed-form modular inversion
    ├── collatz-cert/          # JSON certificate schemas & collatz-verify verifier
    └── collatz-cli/           # CLI binary for execution and certificate checks
```

---

## Quickstart & Initial CLI Commands

### 1. Run Baseline Arithmetic Tests (Experiment 0)
```bash
cargo run -p collatz-cli -- test core
```

### 2. Trace an Integer Trajectory
```bash
cargo run -p collatz-cli -- trace 27 --odd-only
```

### 3. Generate Residue Descent Certificates (Experiment 2)
```bash
cargo run -p collatz-cli -- cert generate --max-depth 40 --output certificates/
```

### 4. Verify a Certificate File
```bash
cargo run -p collatz-cli -- cert verify certificates/descent_mod64.json
```
