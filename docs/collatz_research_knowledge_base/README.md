# Collatz Research Workbench — Knowledge Base

**Snapshot date:** 2026-07-23  
**Scope:** Canonical return subsystem, 2-adic renewal geometry, periodic-core interaction calculus, pointwise source reduction, and Sturmian gap-itinerary elimination.

This directory consolidates the mathematical development, research history, literature connections, theorem hierarchy, limitations, verification architecture, publication options, and next research paths of the current project.

## Important status note

The mathematical statements attributed to the project are based on the frozen theorem package and verification summaries reported by the project as of the snapshot date. The knowledge base distinguishes:

- **Classical or external theorem:** established in the cited literature.
- **Project theorem:** claimed proved in the frozen Rust/Lean/certificate package.
- **Computer-assisted theorem:** depends on a finite generated graph or certificate plus formal or independent verification.
- **Preregistered empirical result:** finite computational evidence with a frozen prediction.
- **Conjecture or open bridge:** not yet proved.

External review should inspect the actual artifacts rather than relying solely on this synthesis.

## Navigation

1. [Executive overview](00_executive_overview.md)
2. [Research history and evolution](01_research_history_and_evolution.md)
3. [Canonical return system and core calculus](02_canonical_return_system_and_core_calculus.md)
4. [Annealed renewal and experimental program](03_annealed_renewal_and_empirical_program.md)
5. [Pointwise bridge and projective sources](04_pointwise_bridge_and_projective_sources.md)
6. [Sturmian elimination theorem](05_sturmian_elimination_theorem.md)
7. [Novelty and literature map](06_novelty_and_literature_map.md)
8. [Implications, limitations, and claim registry](07_implications_limitations_and_claim_registry.md)
9. [Research paths forward](08_research_paths_forward.md)
10. [Publication strategy](09_publication_strategy.md)
11. [Verification, reproduction, and external review](10_verification_reproduction_and_external_review.md)
12. [Glossary and notation](11_glossary_and_notation.md)
13. [Bibliography](12_bibliography.md)
14. [Master synthesis](MASTER_SYNTHESIS.md)

## Current headline result

> **Sturmian Gap-Itinerary Elimination for the Canonical Collatz Return Subsystem.**  
> No positive ordinary integer can realize an infinite semantically valid canonical return path whose gap itinerary is Sturmian over the gap alphabet `{1,2}`.

This is a narrowly scoped subsystem theorem. It does **not** prove the Collatz conjecture, eliminate all Sturmian parity vectors, or cover arbitrary gap alphabets.
