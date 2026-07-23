# Clean-Room Reproduction Instructions

To independently verify the Sturmian Gap-Itinerary Elimination package from scratch:

## 1. Rust Integration Verification Suite
```bash
cargo test -p collatz-affine --test phase_h3b_sturmian_elimination_test
```
*Output: All 6 integration tests PASS.*

## 2. Independent Python Full Reconstruction Verifier
```bash
python reproduction/sturmian-v1.0-rc1/regenerate_and_verify_certificate.py
```
*Output: 52 Nodes, 2482 Edges, Potential Certificate Valid (<= -60): True, Karp Max Cycle Mean lambda* = -60.0000.*

## 3. Standalone Frozen JSON Verifier
```bash
python reproduction/sturmian-v1.0-rc1/verify_frozen_certificate.py
```
*Output: 52 Nodes, 2482 Edges, Potential Certificate Valid (<= -60): True.*

## 4. Adversarial Mutation Audit Suite
```bash
python reproduction/sturmian-v1.0-rc1/mutation_tests.py
```
*Output: All 5 adversarial corruptions REJECTED.*
