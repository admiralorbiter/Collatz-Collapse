# 02. Canonical Return System and Core Calculus

## 1. Canonical branch parameters

For gap symbol `j >= 0`, the project uses

\[
M_j=2^{9+4j},\qquad B_j=9+4j,\qquad Q_j=3^{6+3j}.
\]

Associated source and endpoint constants `C_j`, `D_j`, and affine constant `beta_j` define the branch map

\[
F_j(D)=\frac{Q_jD+\beta_j}{M_j}.
\]

The frozen convention is:

```text
CANONICAL_RETURN_CONVENTION_V1
F_v(D) = (Q_v * D + beta_v) / M_v
M_v = 2^B_v
Q_v = 3^E_v
d_v = Q_v - M_v
xi_v = -beta_v / d_v
A_v(D) = d_v * D + beta_v
Gamma(v,w) = d_v * beta_w - d_w * beta_v
F_v(D) - xi_v = (Q_v / M_v) * (D - xi_v)
F_w(F_v(D)) - F_v(F_w(D)) = -Gamma(v,w) / (M_v * M_w)
```

The zero-block fingerprint is

\[
F_0(342)=487.
\]

## 2. Word composition

A finite word `v=(h_1,...,h_k)` determines a composed affine map

\[
F_v(D)=\frac{Q_vD+\beta_v}{M_v},
\]

where `M_v` is a power of two and `Q_v` is odd. The source-cylinder recursion gives a unique residue class for each semantically valid prefix.

The distinction between two levels is essential:

- **Affine validity:** the rational affine formula is defined.
- **Semantic validity:** every intermediate guard and branch choice is valid.

All pointwise theorems require semantic validity, not merely a composed formula.

## 3. Rational periodic cores

The unique 2-adic fixed point of `F_v` is

\[
\xi_v=-\frac{\beta_v}{Q_v-M_v}.
\]

Because `Q_v > M_v > 0` and `beta_v > 0`,

\[
\xi_v<0
\]

as an ordinary rational number, although `xi_v` is a valid 2-adic integer because its denominator is odd.

The error transports exactly:

\[
F_v(D)-\xi_v=\frac{Q_v}{M_v}(D-\xi_v),
\]

and therefore

\[
v_2(F_v^r(D)-\xi_v)=v_2(D-\xi_v)-rB_v.
\]

Each repetition consumes exactly `B_v` bits of agreement.

## 4. Integer primitive form

Define

\[
A_v(D)=(Q_v-M_v)D+\beta_v.
\]

Since `Q_v-M_v` is odd,

\[
v_2(D-\xi_v)=v_2(A_v(D)).
\]

This avoids rational arithmetic in certificates.

## 5. Periodic and eventually periodic elimination

If an ordinary positive state followed `v^infinity`, it would have to lie in all nested repetition cylinders and therefore equal `xi_v`. But `xi_v < 0`, contradiction.

The same applies after a finite prefix: a positive state cannot enter the negative rational core of an eventually periodic tail.

Thus, according to the project theorem package:

- no positive purely periodic return itinerary is realizable;
- no positive eventually periodic return itinerary is realizable.

## 6. Core interaction determinant

For two blocks `v,w`, define

\[
\Gamma_{v,w}=d_v\beta_w-d_w\beta_v,
\qquad d_v=Q_v-M_v.
\]

Then

\[
\xi_v-\xi_w=\frac{\Gamma_{v,w}}{d_vd_w},
\]

so

\[
v_2(\xi_v-\xi_w)=v_2(\Gamma_{v,w}).
\]

The affine commutator is

\[
F_w(F_v(D))-F_v(F_w(D))
=-\frac{\Gamma_{v,w}}{M_vM_w}.
\]

Therefore:

\[
\Gamma_{v,w}=0
\iff
\xi_v=\xi_w
\iff
F_v\circ F_w=F_w\circ F_v
\]

at the affine-formula level.

## 7. Exact switch identity

The integer core errors satisfy

\[
d_vA_w(D)=d_wA_v(D)+\Gamma_{v,w}.
\]

Let

\[
s=v_2(A_v(D)),\qquad \kappa=v_2(\Gamma_{v,w}).
\]

Then:

- **Same core:** `Gamma=0`, so the depth is unchanged.
- **Inherited:** `s < kappa`, so `v_2(A_w(D))=s`.
- **Reset:** `s > kappa`, so `v_2(A_w(D))=kappa`.
- **Resonant:** `s=kappa`, so normalized odd terms cancel and the new depth is `kappa+g`, with `g>=1` or infinity under exact cancellation.

For positive ordinary states, exact landing on a negative core is impossible.

## 8. Semantic-depth bridge

Arithmetic proximity is meaningful only when tied to the actual future itinerary. The project reports a theorem of the form

\[
H_L\le v_2(A_v(D))<H_L+B_{\text{first mismatch}},
\]

where `L` is the number of future symbols agreeing with `v^infinity` and `H_L` is the weighted precision of that common prefix.

For two periodic phase sequences `x=v^infinity`, `y=w^infinity`, the analogous theorem bounds

\[
v_2(\Gamma_{v,w})
\]

between the weighted common-prefix depth and the next branch increment.

This is the key bridge between symbolic combinatorics and 2-adic arithmetic.
