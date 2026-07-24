# Architecture and Crate Organization

## 1. Executive Overview

The **Collatz Research Workbench** is organized as a modular Rust workspace and a Lean 4 formal verification package:

```
collatz-workspace/
├── crates/
│   ├── collatz-affine/     # Affine modular core, backward pullbacks, 2-adic cylinder compiler
│   ├── collatz-cert/       # Machine-checkable JSON certificate engine & verifier
│   ├── collatz-sieve/      # Multi-scale 2-adic sieve & modular solver
│   └── collatz-cegar/      # Abstract domain, CEGAR refinement, and graph contraction
├── lean/
│   └── PhaseI1CounterexampleCapture.lean # Formal Lean 4 proof file (25 non-sorry theorems)
└── docs/
    └── active/             # Active documentation ledger and roadmap
```

---

## 2. Core Modules & Data Structures (`canonical_math`)

### 2.1 `canonical_math/cocycle.rs`
- **`ExactWordCylinder`**: Contains `residue` and `modulus` ($2^{B+1}$) for exact valuation words.
- **`DestinationPullbackCylinder`**: Destination pullback cylinder $\sigma_{w, r_t} \pmod{2^{B+q}}$.
- **`CompiledSemanticReturn`**: Combined semantic return cylinder with first-return flag $R(w) \subseteq R(u)$.
- **`FirstReturnSymbol`**: Certified first-return symbol containing valuation word $w$, gap $j$, total exponent $B(w)$, $\alpha_w$, live shift $\eta_w$, and refined source cylinder $R(w)$.
- **`DyadicExponent`**: BigUint-precision dyadic exponent representation.
- **`DyadicWeight`**: Exact exponent representation $2^{-\text{exponent}}$ preventing floating-point rounding.
- **`PrefixLiftDigit`**: Lift-digit jump $d_m = (r_{m+1} - r_m) / 2^{H_m}$.
- **`PrefixRepresentativeStep`**: Captures step-by-step representative progression $(r_m, H_m, d_m)$.
- **`ItineraryPrefixCylinder`**: Lazy iterator computing finite nested cylinder approximations $\phi_{w_0} \circ \dots \circ \phi_{w_{m-1}}(Q_1)$ with lift-digit steps.
- **`CensusManifest`**: Machine-readable JSON manifest exporting exhaustive census data and rejection genealogy records ($R_{2 \leftarrow 0}$ and $R_{2 \leftarrow 1}$).
