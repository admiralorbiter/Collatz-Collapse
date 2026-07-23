# 09. Publication Strategy

## 1. Recommended publication split

The project is too broad for one first paper. A focused series is more credible.

## Paper 1 — Flagship theorem

### Working title

**Sturmian Gap-Itinerary Elimination in a Canonical Collatz Return System**

### Central result

No positive ordinary integer realizes an infinite semantically valid canonical return path with Sturmian gap itinerary over `{1,2}`.

### Suggested structure

1. Introduction and precise scope.
2. Canonical return subsystem.
3. Exact source cylinders and semantic paths.
4. Pointwise source characterization.
5. Rational cores and interaction calculus.
6. State-core semantic-depth theorem.
7. Sturmian bounded-period cube input.
8. Complete finite selector graph.
9. Normalized edge domination.
10. Negative potential and contradiction.
11. Computer-assisted verification.
12. Limitations and open problems.

### What to omit

- most U7/U8 data;
- the full phase history;
- broad speculation about solving Collatz;
- unrelated certificate engines.

## Paper 2 — Renewal geometry and experiments

### Working title

**A 2-Adic Renewal Structure in a Canonical Collatz Return System**

### Content

- source spine and shell structure;
- exact Haar return law;
- fiberwise Haar preservation;
- U7/U8 preregistered experiments;
- E2/E3 witness distribution;
- periodic-core discovery;
- annealed versus quenched limitations.

### Literature context

- Kontorovich–Sinai;
- Tao;
- parity-vector density results;
- p-adic induced maps.

## Paper 3 — Verified computational methodology

### Working title

**Proof-Producing Experimental Mathematics for a Collatz Return System**

### Content

- Rust exact arithmetic;
- Lean theorem hierarchy;
- Python independent regeneration;
- graph and certificate hashes;
- mutation testing;
- preregistration;
- theorem-status ledger;
- lessons from corrected errors.

This could be a software paper, formal-methods paper, or companion artifact.

## Paper 4 — General structured-family theorem

After further work, a stronger paper could present:

- the abstract syndetic-power negative-potential theorem;
- general gap embeddings;
- primitive substitutions;
- several class-elimination corollaries.

## 2. Novelty section for Paper 1

A careful novelty statement should say:

- rational 2-adic points for prescribed patterns are classical;
- Sturmian parity-vector studies already exist;
- bounded-gap cubes in Sturmian words are external prior work;
- the new contribution is the canonical return-gap model, semantic-depth calculus, complete graph reduction, edge domination, and negative-potential class elimination.

## 3. Suggested abstract language

> We define a canonical guarded return subsystem for the accelerated Collatz map and characterize ordinary positive realizability through stabilization of compatible 2-adic source representatives. Each finite return block determines a rational 2-adic core and an exact interaction invariant controlling precision under symbolic switching. We prove that semantic continuation depth is equivalent to weighted 2-adic core depth. Using a theorem of Bell, Schulz, and Shallit on syndetic bounded-period cubes in Sturmian words, we reduce all Sturmian gap itineraries over `{1,2}` to a finite 52-state phase graph. Exact worst-case edge weights admit a uniformly negative potential, forcing the available source precision to become negative along every infinite graph path. Consequently, no positive ordinary integer realizes an infinite semantically valid canonical return path with such a Sturmian gap itinerary. The finite graph and potential are independently regenerated and verified by Rust, Lean, and Python artifacts.

## 4. Reviewer concerns to anticipate

- Is the subsystem natural or engineered to fit the theorem?
- Does semantic path validity exactly match ordinary Collatz behavior?
- Is graph completeness proved or sampled?
- Is the external Sturmian theorem imported correctly?
- Are edge weights true worst-case upper bounds?
- Does the theorem cover parity words or only return gaps?
- Is the finite data bound into Lean, or only checked beside it?
- How independent are the verification implementations?

Address these questions proactively.

## 5. Authorship and presentation

The paper should identify clearly:

- which proofs are conceptual;
- which steps are computer-assisted;
- which external theorem is assumed;
- which finite artifacts are authoritative;
- how the human–AI workflow contributed.

Avoid presenting AI involvement as mathematical authority. Authority comes from proofs, certificates, reproducibility, and review.

## 6. Release sequence

1. Internal release candidate.
2. Clean-room reproduction.
3. Private expert review.
4. Revised technical report.
5. Public preprint and artifact repository.
6. Journal submission.

## 7. Potential venues

Depending on final emphasis:

- number theory/dynamical systems journal;
- experimental mathematics journal;
- symbolic dynamics or combinatorics-on-words venue;
- formalized mathematics or certified-computation venue;
- a combined theorem paper plus archival software release.

Venue choice should follow external expert feedback on perceived mathematical centrality.
