# 11. Glossary and Notation

## Canonical symbols

| Symbol | Meaning |
|---|---|
| `h,j` | Return-gap symbols |
| `B_h=9+4h` | Binary precision consumed by gap `h` |
| `M_h=2^{B_h}` | Power-of-two denominator of a branch |
| `Q_h=3^{6+3h}` | Odd branch multiplier numerator |
| `C_h` | Source residue for branch `h` |
| `D_h` | Canonical successor constant for branch `h` |
| `beta_h` | Affine constant in the branch map |
| `v,w` | Finite return blocks |
| `F_v(D)` | Composed affine return map `(Q_v D + beta_v)/M_v` |
| `d_v=Q_v-M_v` | Positive odd expansion difference |
| `xi_v=-beta_v/d_v` | Rational 2-adic periodic core |
| `A_v(D)=d_vD+beta_v` | Integer primitive measuring distance to `xi_v` |
| `Gamma(v,w)` | Core interaction determinant |
| `kappa` | `v_2(Gamma(v,w))`, core separation depth |
| `s` | Incoming core depth `v_2(A_v(D))` |
| `t` | Depth after consuming a selected repeated segment |
| `g` | Extra resonant gain after normalized cancellation |
| `H_n` | Total source precision after `n` path symbols |
| `r_n` | Source residue modulo `2^{H_n}` |
| `R_n` | Least nonnegative representative of `r_n` |
| `lambda_n` | Newly exposed lift block between precision levels |
| `p_x(n)` | Factor complexity of word `x` |
| `LCP` | Longest common prefix |
| `W(e)` | Worst-case normalized precision change on graph edge `e` |
| `Phi` | Graph potential / Lyapunov function |

## Status vocabulary

### Project theorem

Claimed proved in the frozen project proof package.

### External theorem

Imported from the published or preprint literature.

### Computer-assisted theorem

The proof includes a finite exhaustive graph or certificate checked by software.

### Finite-range exhaustive result

Every object in a bounded finite search was checked; no infinite theorem follows automatically.

### Preregistered empirical result

A finite prediction was frozen before computation and then tested.

### Conjecture

A mathematically specific statement not yet proved.

### Refuted

A previous claim contradicted by proof or counterexample and removed from the active theorem ledger.

## Word models that must not be conflated

### Parity word

The binary parity sequence of the standard Collatz map.

### Accelerated exponent/valuation word

The sequence `v_2(3n+1)` along odd-to-odd accelerated steps.

### Canonical return-gap word

The symbol sequence in the project’s guarded return subsystem.

A paper must state the coding relationship among these objects before transferring a theorem from one model to another.
