# 10. Verification, Reproduction, and External Review

## 1. Verification philosophy

The project should distinguish four layers:

1. **Theorem proof:** symbolic mathematical reasoning.
2. **Finite certificate generation:** graph, weights, potential, hashes.
3. **Certificate verification:** checking every finite inequality and structural invariant.
4. **Independent reconstruction:** regenerating the certificate from definitions using separate code.

Passing one layer does not imply the others.

## 2. Frozen theorem package

A recommended release tree is:

```text
sturmian-return-elimination-v1.0/
├── theorem_statement.md
├── proof_dependency_dag.md
├── notation_and_definitions.md
├── literature_scope.md
├── lean/
├── rust/
├── certificates/
│   ├── embedding_1_2.json
│   ├── embedding_2_1.json
│   └── manifest.json
├── verify_frozen_certificate.py
├── regenerate_and_verify_certificate.py
├── mutation_tests/
├── REPRODUCE.md
└── LIMITATIONS.md
```

## 3. Manifest requirements

The manifest should include:

- schema version;
- canonical convention version and hash;
- selector version and hash;
- generator commit;
- Rust, Lean, and Python versions;
- dependency locks;
- node and edge counts;
- ordered embedding;
- minimum slack;
- exact maximum cycle mean;
- full SHA-256 of every artifact;
- external theorem citation;
- trusted computing base.

## 4. Lean audit

Archive actual output for:

```lean
#print axioms sturmian_gap_alphabet_1_2_eliminated
#print axioms sturmian_selector_path_to_graph_completeness
#print axioms sturmian_graph_edge_weight_domination
#print axioms normalized_transition_weight_period_extension_invariant
```

Review for:

- `sorry` or placeholders;
- hidden axioms;
- mismatch between theorem names and statements;
- assumptions imported from external theorems;
- exact graph-data binding;
- map-convention consistency.

## 5. Independent certificate verification

A minimal verifier should check only:

- artifact hashes;
- node and edge schema;
- worst-case edge aggregation;
- every potential inequality;
- minimum slack;
- exact cycle-mean result or an independent upper bound.

It should not trust the graph generator.

## 6. Independent reconstruction

A stronger script should independently implement:

- primitive necklace generation;
- phase expansion;
- balanced-template enumeration;
- selector behavior;
- transition extraction;
- semantic depth bounds;
- normalized weights;
- worst-case aggregation;
- graph serialization and hashing;
- maximum cycle mean.

Document all shared code. Independence is strongest when only the mathematical specification is shared.

## 7. Mutation tests

Required mutations include:

- swap `M` and `Q`;
- reverse the fixed-point sign;
- use the wrong commutator denominator;
- replace maximum edge aggregation with minimum;
- remove one graph edge;
- add `+1` to nonresonant switch depth;
- alter one edge weight by one;
- alter one potential value;
- change selector tie-breaking;
- collapse phase rotations;
- truncate a hash;
- restore provisional slack 72 instead of final 60;
- undercount right-censored continuation.

Every mutation should be rejected by at least one independent verifier.

## 8. Clean-room reproduction

Provide a one-command deterministic build in a clean environment:

```text
./reproduce_all.sh
```

The command should:

1. build Rust;
2. run all tests;
3. build Lean with no `sorry`;
4. regenerate certificates;
5. verify full hashes;
6. run independent Python checks;
7. produce a final signed report.

No network access should be required after dependencies are pinned.

## 9. External review packet

### Combinatorics reviewer

Focus:

- balanced templates;
- Sturmian cube theorem use;
- graph completeness;
- rotations, primitive periods, and selector locality.

### Collatz/2-adic reviewer

Focus:

- canonical return semantics;
- source cylinders;
- H.1 pointwise reduction;
- core calculations;
- scope relative to standard accelerated Collatz.

### Formal-methods reviewer

Focus:

- trusted computing base;
- generated-data binding;
- independent reconstruction;
- theorem assumptions and certificate soundness.

## 10. Falsification checklist

Invite reviewers to find:

- an actual Sturmian selector transition missing from the graph;
- an edge whose stored weight is too negative;
- a right-censored continuation that violates period-extension invariance;
- a positive state reaching an exact negative core;
- a source path that evades H.1 stabilization logic;
- a mismatch between the documented and implemented return map;
- an overlap with prior literature that invalidates the novelty statement.

A theorem package becomes more credible when it makes falsification easy.
