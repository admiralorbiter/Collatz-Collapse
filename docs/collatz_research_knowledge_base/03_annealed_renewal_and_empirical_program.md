# 03. Annealed Renewal and Experimental Program

## 1. Ambient Haar renewal law

For a first gap `j`, the branch precision is

\[
B_j=9+4j.
\]

The Haar mass of its source cylinder is `2^{-B_j}`. Summing over all gaps gives

\[
\sum_{j\ge0}2^{-(9+4j)}=\frac1{480}.
\]

Conditioned on a return, the gap distribution is

\[
p_j=480\,2^{-B_j}=\frac{15}{16^{j+1}}.
\]

The fixed-fiber affine successor has odd slope, so it preserves Haar measure. This yields an exact ambient renewal law:

\[
\Pr(J_1=j_1,\dots,J_r=j_r\mid E_r)
=\prod_{i=1}^r\frac{15}{16^{j_i+1}},
\]

and

\[
\mu(E_r)=480^{-r}.
\]

This is an **ambient Haar theorem**, not automatically a theorem about deterministic endpoint ensembles or individual ordinary orbits.

## 2. Exact code-length identity

Because

\[
p_h=480\,2^{-B_h},
\]

we have

\[
B_h=\log_2(480)-\log_2 p_h.
\]

For a block `v=(h_1,...,h_k)`,

\[
B_v=k\log_2(480)-\log_2\prod_{i=1}^kp_{h_i}.
\]

This is an exact renewal code-length identity. It becomes a compression theorem only after proving decodability, bounded multiplicity, and net bit saving.

## 3. Depth-seven experiment

The project reports approximately 5.38 million cumulative words through depth seven.

Observed results included:

- first-return count close to Haar expectation;
- 25 double-return witnesses, again close to Haar scale;
- no triple return, consistent with the small expected value;
- all initial subtrees represented;
- no proof of deterministic independence or convergence to Haar.

## 4. Depth-eight preregistered experiment

At exact depth eight, the search covered `9^8 = 43,046,721` words. The preregistered results included:

- first-return count inside the frozen prediction range;
- double-return count inside the frozen prediction range;
- pooled gap frequencies close to the geometric Haar law;
- one first triple-zero witness;
- no fourth return, which had expectation far below one.

The correct interpretation is:

> finite-depth preregistered support for Haar-scale renewal behavior.

It is not a proof that deterministic endpoint measures converge to Haar.

## 5. First triple witness

The first triple-zero witness was

```text
[8, 7, 6, 3, 5, 0, 5, 1]
```

with three successful zero-gap successors and a failed fourth guard.

The endpoint agrees with

\[
\xi_0=-\frac{26}{217}
\]

for exactly 29 low bits:

\[
v_2(D_u-\xi_0)=29.
\]

Because `B_0=9`, this gives exactly

\[
\left\lfloor\frac{29}{9}\right\rfloor=3
\]

repetitions, with two residual bits.

This observation triggered the periodic-core program.

## 6. Annealed versus quenched

### Annealed question

Does the endpoint ensemble at depth `d` become equidistributed modulo fixed powers of two?

A representative target is

\[
\left|\mu_d([a]_m)-2^{-m}\right|\le C_m\rho_m^d.
\]

### Quenched question

Can one bound every conditional distribution after every reachable prefix, except for a classified structured family?

This is much stronger and is the relevant pointwise direction.

The Sturmian theorem is important because it is a quenched result: it eliminates every path in one infinite symbolic class, not merely almost every member.

## 7. Relation to Tao and Kontorovich–Sinai

Kontorovich and Sinai developed a statistical structure theorem for accelerated Collatz data and random-walk-like residues. Tao proved that almost all Collatz orbits attain almost bounded values in logarithmic density using approximate transport and high-frequency characteristic-function estimates on a 3-adic cyclic group.

These works support the renewal and flattening side of the project but also illustrate the central limitation: ensemble control does not automatically control one exceptional deterministic orbit.

## 8. Future role of the empirical program

The finite searches should now be used to:

- discover candidate structured obstructions;
- preregister and test transfer-operator predictions;
- identify worst states for formal analysis;
- test proposed graph abstractions;
- search for counterexamples to intermediate conjectures.

They should not be used as a substitute for the pointwise theorem chain.
