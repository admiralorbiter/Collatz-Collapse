# 05. Sturmian Gap-Itinerary Elimination Theorem

## 1. The theorem

> **Sturmian Gap-Itinerary Elimination for the Canonical Collatz Return Subsystem.**  
> No positive ordinary integer `N > 0` can realize an infinite semantically valid canonical return path whose gap itinerary is Sturmian over the binary gap alphabet `{1,2}`.

This is a project-reported computer-assisted theorem. Its scope is the frozen canonical return subsystem and the exact selector, graph, and certificate package.

## 2. What “Sturmian” means

A binary infinite word is Sturmian if it is aperiodic and has minimal possible factor complexity:

\[
p(n)=n+1
\]

for every positive integer `n`.

Sturmian words are highly structured but never eventually periodic. Every factor has exactly two return words.

## 3. External combinatorial input

Bell, Schulz, and Shallit proved in 2024 that in every Sturmian word:

- the distance between consecutive cube-ending positions is at most 10;
- cubes of period at most 5 suffice;
- the bound 10 is optimal.

This supplies syndetic bounded-period powers.

Over a binary alphabet, primitive words of periods 1 through 5 form:

- 14 primitive necklaces up to cyclic rotation;
- 52 phase words when every rotation is retained.

## 4. Selector and graph reduction

The canonical semantic selector is:

- symbolic-first;
- prefix-measurable;
- primitive-normalized;
- phase-aware;
- deterministic;
- allowed to return no core;
- equipped with extension-state reporting.

The project reports that every infinite Sturmian selector execution over gap alphabet `{1,2}` maps to an infinite walk in a frozen 52-phase graph.

The graph includes both ordered embeddings:

- binary `0 -> 1`, `1 -> 2`;
- binary `0 -> 2`, `1 -> 1`.

## 5. Semantic depth

For state `D` and target core `v`, the depth

\[
s=v_2(A_v(D))
\]

is tied to the number of future gap symbols agreeing with `v^infinity`. This turns future periodic continuation into an exact or bounded arithmetic quantity.

## 6. Normalized transition weights

A raw resonance gain can be arbitrarily extended by further repetitions of the target core. The project normalizes this by charging the corresponding target repetition cost. Period-extension invariance ensures that continuing by full target periods does not change the normalized transition value.

For each graph edge, the certificate stores the worst-case value

\[
W(e)=\max_\tau W_\tau
\]

across all local templates producing that transition.

The project reports a theorem that every physical precision change is dominated by the graph weight:

\[
\Delta s_{\mathrm{physical}}\le W(e).
\]

## 7. Negative potential

For the frozen graph, a node potential `Phi` reportedly satisfies

\[
W(e)+\Phi(t(e))-\Phi(s(e))\le -60
\]

for every directed edge.

Summing along a graph walk of length `N` yields

\[
\sum_{i=1}^NW(e_i)
\le
-60N+\Phi(s_0)-\Phi(s_N).
\]

Because the graph is finite, `Phi` is bounded. Combining graph domination and boundary corrections gives

\[
s_N\le s_0+C-60N,
\]

which becomes negative for large `N`.

But `s_N` is a 2-adic valuation of a nonzero integer and therefore cannot be negative. For positive states, exact landing on a negative core is also impossible.

This contradiction proves the theorem.

## 8. Frozen certificate data

Project-reported final data:

- 52 phase nodes;
- 14 primitive necklaces;
- 2,482 directed worst-case edges;
- minimum integer slack: 60;
- exact or independently checked maximum cycle mean reported as `-60`;
- two ordered-embedding graph artifacts;
- full SHA-256 digests:
  - `(1,2)`: `e93f8fc30780fa06b45e2e6c5fec08e9997dff54b0175e71eca9ab7a01a8e50c`
  - `(2,1)`: `3a22e0d48807b09601d6134407003d30ce04712d72ad841ab5512b1ab2ab6a24`

## 9. What the theorem does not say

It does not eliminate:

- all Sturmian parity vectors;
- all odd-to-odd valuation words;
- all two-gap alphabets `{a,b}`;
- all bounded-gap paths;
- all linearly recurrent words;
- all primitive substitutions;
- all possible Collatz counterexamples.

The distinction among parity words, standard accelerated exponent codes, and canonical return-gap words must remain explicit.

## 10. Why the result matters

This is the first project theorem that eliminates a complete, genuinely aperiodic symbolic class through a pointwise contradiction. It demonstrates that the project’s structured-side program is viable:

\[
\text{finite symbolic representation}
+\text{worst-case precision weights}
+\text{negative potential}
\Rightarrow
\text{ordinary unrealizability}.
\]
