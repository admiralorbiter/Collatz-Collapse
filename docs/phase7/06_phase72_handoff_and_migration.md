# Phase 7.2 Handoff and Migration to Phase 7X

## 1. Preserve these Phase 7.2 results

The following should remain frozen as verified local results:

1. Destination-aware refinement uses:
   \[
   h_{\mathrm{add}}
   =
   \max(0,A+q_t-M_{\mathrm{curr}}).
   \]

2. The \(w_1=[1,1,2]\) source cylinder \(7\bmod32\) can be refined to modulus \(1024\) to classify images modulo \(64\).

3. The subcell:
   \[
   935\bmod1024
   \]
   maps under \(w_1\) to:
   \[
   1579\equiv43\bmod64.
   \]

4. The \(w_2=[1,2,2]\) source cylinder \(43\bmod64\) can be refined into 16 cells modulo \(1024\).

5. The subcell:
   \[
   235\bmod1024
   \]
   maps under \(w_2\) to:
   \[
   199\equiv7\bmod32.
   \]

6. The one-loop guarded path cylinder is:
   \[
   1959\bmod16384.
   \]

7. The composite word:
   \[
   W=[1,1,2,1,2,2]
   \]
   retains its valid Phase 6D finite-fuel certificate.

These are valuable regression fixtures for Phase 7X.

## 2. Reclassify the graph result carefully

The coarse control cycle:

\[
7\bmod32
\to
43\bmod64
\to
7\bmod32
\]

is supported only on refined path subcylinders.

It should be represented as a guarded path schema, not as evidence that every member of either coarse state follows the cycle.

Recommended terminology:

- coarse control return;
- guarded one-loop path cylinder;
- guarded two-loop path cylinder;
- exact composite word cylinder;
- refined arithmetic source cell.

Avoid calling the coarse states a deterministic SCC unless every outgoing branch is included and graph completeness is proved relative to the selected library.

## 3. Convert Phase 7.2 artifacts into regression tests

### Destination precision regression

Reject any implementation that classifies:

\[
43\bmod64\xrightarrow{w_2}7\bmod32
\]

without refining to modulus \(1024\).

### Path-cylinder regression

Verify:

\[
1959\bmod16384
\]

for one guarded loop.

### Fuel regression

Verify:

\[
N_W(1959)=1.
\]

### Exact-word versus guarded-path regression

Keep distinct:

\[
935\bmod1024
\]

for the exact word \(W\), and:

\[
1959\bmod16384
\]

for the guarded coarse return.

## 4. New benchmark interpretation

The pair \(w_1,w_2\) becomes the first affine interaction benchmark.

Its interaction data are:

\[
\Delta_{1,2}=-348,
\qquad
v_2(\Delta_{1,2})=2.
\]

The broad source of \(w_2\) appears as a resonance condition in \(H_1\), and the broad source of \(w_1\) appears as a resonance condition in \(H_2\).

This makes the benchmark useful even though it does not produce genuine noncommuting branching.

## 5. Updated roadmap position

Recommended roadmap:

```text
Phase 7.1:
    Bounded semantic-refinement result

Phase 7.2:
    Destination-aware refinement and guarded periodic path validation

Phase 7X:
    Affine interaction, resonance cylinders, and ultrametric switching

Phase 7.3:
    Expanded symbolic word library and path-first recurrent-language search

Phase 7.4:
    Ranking synthesis over sound branching components
```

## 6. Phase 7.3 entry criteria

Do not expand the word library until Phase 7X establishes:

- generic interaction identities;
- cross-form cylinder recovery;
- an interaction-spectrum report;
- one abstraction comparison;
- path-first certificate generation;
- explicit negative-outcome schemas.

This prevents the larger search from reproducing the same false SCCs at greater scale.
