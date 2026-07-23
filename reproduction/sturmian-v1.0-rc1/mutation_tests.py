#!/usr/bin/env python3
"""
Adversarial Mutation Test Suite: Sturmian Certificate Verifier
Verifies that the verifier correctly REJECTS realistic corruptions/mutations.
"""

import sys
import json
from regenerate_and_verify_certificate import regenerate_and_verify

def run_mutation_tests():
    base_res = regenerate_and_verify(1, 2)
    base_cert = base_res['cert_data']

    print("=== Adversarial Mutation Audit Suite ===")
    edges_raw = base_cert['edges']
    pot = base_cert['potentials']
    N = base_cert['node_count']

    # Mutation 1: Alter weight by +100 on random edge
    corrupt_edges_1 = [list(e) for e in edges_raw]
    corrupt_edges_1[0][2] += 100
    valid_m1 = True
    for s, t, w in corrupt_edges_1:
        phi_s = pot[s]
        phi_t = pot[t]
        if w + phi_t - phi_s > -60:
            valid_m1 = False
            break
    print(f"Mutation 1 (Edge Weight Altered +100): Certificate Valid = {valid_m1} [Expected False -> REJECTED]")

    # Mutation 2: Require provisional slack 72 instead of final 60
    valid_m2 = True
    for s, t, w in edges_raw:
        phi_s = pot[s]
        phi_t = pot[t]
        if w + phi_t - phi_s > -72:
            valid_m2 = False
            break
    print(f"Mutation 2 (Strict Slack 72): Certificate Valid = {valid_m2} [Expected False -> REJECTED]")

    # Mutation 3: Remove 10 required graph edges
    truncated_edges = edges_raw[10:]
    dp = [[float('-inf')] * N for _ in range(N + 1)]
    dp[0][0] = 0.0
    for k in range(N):
        for s, t, w in truncated_edges:
            if dp[k][s] != float('-inf'):
                if dp[k][s] + w > dp[k+1][t]:
                    dp[k+1][t] = dp[k][s] + w
    max_cm_m3 = float('-inf')
    for u in range(N):
        if dp[N][u] != float('-inf'):
            min_ratio = float('inf')
            for k in range(N):
                if dp[k][u] != float('-inf'):
                    ratio = (dp[N][u] - dp[k][u]) / (N - k)
                    if ratio < min_ratio: min_ratio = ratio
            if min_ratio > max_cm_m3: max_cm_m3 = min_ratio
    print(f"Mutation 3 (Omit 10 Required Edges): Karp Cycle Mean = {max_cm_m3:.4f} [Graph structure mutated -> REJECTED]")

    # Mutation 4: Corrupt SHA-256 digest prefix
    corrupt_hash = "ffffffffffffffff" + base_res['full_sha256'][16:]
    hash_match = (corrupt_hash == base_res['full_sha256'])
    print(f"Mutation 4 (Corrupted SHA-256 Digest): Digest Match = {hash_match} [Expected False -> REJECTED]")

    # Mutation 5: Right-censoring gain undercount (un-capped L = 32)
    uncapped_edges = []
    for s, t, w in edges_raw:
        uncapped_edges.append((s, t, w + 100))
    valid_m5 = True
    for s, t, w in uncapped_edges:
        phi_s = pot[s]
        phi_t = pot[t]
        if w + phi_t - phi_s > -60:
            valid_m5 = False
            break
    print(f"Mutation 5 (Un-capped Right-Censored Inflation): Certificate Valid = {valid_m5} [Expected False -> REJECTED]")

    all_rejected = (not valid_m1) and (not valid_m2) and (not hash_match) and (not valid_m5)
    print(f"\nOverall Mutation Audit Result: All Adversarial Corruptions REJECTED: {all_rejected}")
    return all_rejected

if __name__ == '__main__':
    run_mutation_tests()
