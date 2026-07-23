#!/usr/bin/env python3
"""
Full Independent Reconstruction & Certificate Verifier:
Sturmian Gap-Itinerary Elimination over {1, 2}
Computes full 64-character SHA-256 digests and verifies Karp's Max Cycle Mean.
"""

import json
import hashlib

def gcd(a, b):
    while b:
        a, b = b, a % b
    return a

def step_cost_1_2(symbol):
    return 9 + 4 * symbol

def weighted_bit_precision(word, m):
    period = len(word)
    total = 0
    for j in range(m):
        sym = word[j % period]
        total += step_cost_1_2(sym)
    return total

def is_balanced(slice_words, gap_b):
    length = len(slice_words)
    for k in range(1, length + 1):
        min_b = float('inf')
        max_b = 0
        for i in range(length - k + 1):
            window = slice_words[i:i+k]
            cnt = window.count(gap_b)
            if cnt < min_b: min_b = cnt
            if cnt > max_b: max_b = cnt
        if max_b - min_b > 1:
            return False
    return True

def generate_length_32_templates(gap_a, gap_b):
    length = 32
    templates = []
    for num_b in range(length + 1):
        word = []
        for i in range(length):
            val = gap_b if ((i + 1) * num_b) // length > (i * num_b) // length else gap_a
            word.append(val)
        if is_balanced(word, gap_b):
            templates.append(word)
    return templates

def generate_14_necklaces(gap_a, gap_b):
    return [
        [gap_a], [gap_b],
        [gap_a, gap_b],
        [gap_a, gap_a, gap_b], [gap_a, gap_b, gap_b],
        [gap_a, gap_a, gap_a, gap_b], [gap_a, gap_a, gap_b, gap_b], [gap_a, gap_b, gap_b, gap_b],
        [gap_a, gap_a, gap_a, gap_a, gap_b], [gap_a, gap_a, gap_a, gap_b, gap_b],
        [gap_a, gap_a, gap_b, gap_a, gap_b], [gap_a, gap_a, gap_b, gap_b, gap_b],
        [gap_a, gap_b, gap_a, gap_b, gap_b], [gap_a, gap_b, gap_b, gap_b, gap_b]
    ]

def compute_lcp(word, template):
    min_len = min(len(word), len(template))
    l = 0
    while l < min_len and word[l] == template[l]:
        l += 1
    return l

def compute_d_v(word):
    E = len(word)
    B = sum(word)
    return (3**E) - (2**B)

def regenerate_and_verify(gap_a=1, gap_b=2):
    necklaces = generate_14_necklaces(gap_a, gap_b)
    nodes = []
    for neck in necklaces:
        period = len(neck)
        for k in range(period):
            rotated = neck[k:] + neck[:k]
            d_v = compute_d_v(rotated)
            nodes.append({
                'id': len(nodes),
                'neck': neck,
                'word': rotated,
                'period': period,
                'd_v': d_v
            })
    
    templates = generate_length_32_templates(gap_a, gap_b)
    max_edges = {}

    for i, s_node in enumerate(nodes):
        for j, t_node in enumerate(nodes):
            if s_node['neck'] != t_node['neck']: # distinct primitive orbits
                p_v = s_node['period']
                p_w = t_node['period']
                g_sym = gcd(p_v, p_w)
                max_incompat = p_v + p_w - g_sym - 1

                b_v = weighted_bit_precision(s_node['word'], p_v)
                b_w = weighted_bit_precision(t_node['word'], p_w)

                source_consumed = 2 * b_v

                for template in templates:
                    raw_l = compute_lcp(t_node['word'], template)
                    l = min(raw_l, max_incompat)
                    r_target = max(2, l // p_w)
                    target_cost = r_target * b_w
                    bounded_gain = weighted_bit_precision(t_node['word'], l)
                    net_weight = bounded_gain - (source_consumed + target_cost)

                    pair = (i, j)
                    if pair not in max_edges or net_weight > max_edges[pair]:
                        max_edges[pair] = net_weight

    edges = [(s, t, w) for (s, t), w in max_edges.items()]
    N = len(nodes)

    # Bellman-Ford Potential Search on c(e) = -W(e) - 60
    aug_edges = [(s, t, -w - 60) for s, t, w in edges]
    for i in range(N):
        aug_edges.append((N, i, 0))
    
    dist_aug = [10**9] * (N + 1)
    dist_aug[N] = 0

    for _ in range(N + 1):
        updated = False
        for s, t, cost in aug_edges:
            if dist_aug[s] != 10**9 and dist_aug[s] + cost < dist_aug[t]:
                dist_aug[t] = dist_aug[s] + cost
                updated = True
        if not updated:
            break

    potential = {i: -dist_aug[i] for i in range(N)}

    min_slack = float('inf')
    valid = True
    for s, t, w in edges:
        phi_s = potential[s]
        phi_t = potential[t]
        slack = -w - phi_t + phi_s
        if slack < min_slack:
            min_slack = slack
        if w + phi_t - phi_s > -60:
            valid = False

    # Karp's Maximum Cycle Mean Algorithm
    dp = [[float('-inf')] * N for _ in range(N + 1)]
    dp[0][0] = 0.0

    for k in range(N):
        for s, t, w in edges:
            if dp[k][s] != float('-inf'):
                if dp[k][s] + w > dp[k+1][t]:
                    dp[k+1][t] = dp[k][s] + w

    max_cycle_mean = float('-inf')
    for u in range(N):
        if dp[N][u] != float('-inf'):
            min_ratio = float('inf')
            for k in range(N):
                if dp[k][u] != float('-inf'):
                    ratio = (dp[N][u] - dp[k][u]) / (N - k)
                    if ratio < min_ratio:
                        min_ratio = ratio
            if min_ratio > max_cycle_mean:
                max_cycle_mean = min_ratio

    # Export Certificate Package JSON
    cert_data = {
        'schema_version': '1.0.0',
        'gap_embedding': [gap_a, gap_b],
        'node_count': N,
        'edge_count': len(edges),
        'min_slack_epsilon_star': min_slack,
        'max_cycle_mean_lambda_star': max_cycle_mean,
        'edges': edges,
        'potentials': potential
    }
    
    serialized = json.dumps(cert_data, sort_keys=True)
    full_hash = hashlib.sha256(serialized.encode('utf-8')).hexdigest()
    short_hash = full_hash[:16]

    return {
        'cert_data': cert_data,
        'full_sha256': full_hash,
        'short_sha256': short_hash,
        'file_size_bytes': len(serialized.encode('utf-8'))
    }

if __name__ == '__main__':
    r12 = regenerate_and_verify(1, 2)
    print("=== Sturmian Gap (1, 2) Full Reconstruction Audit ===")
    print(f"Nodes: {r12['cert_data']['node_count']}, Edges: {r12['cert_data']['edge_count']}")
    print(f"Certificate Valid (<= -60): {r12['cert_data']['min_slack_epsilon_star'] >= 60}")
    print(f"Minimum Integer Slack epsilon*: {r12['cert_data']['min_slack_epsilon_star']}")
    print(f"Karp's Maximum Cycle Mean lambda*: {r12['cert_data']['max_cycle_mean_lambda_star']:.4f}")
    print(f"File Size Bytes: {r12['file_size_bytes']}")
    print(f"Full SHA-256 Digest:  {r12['full_sha256']}")
    print(f"Short 16-Hex Prefix:  {r12['short_sha256']}")

    r21 = regenerate_and_verify(2, 1)
    print("\n=== Sturmian Gap (2, 1) Full Reconstruction Audit ===")
    print(f"Nodes: {r21['cert_data']['node_count']}, Edges: {r21['cert_data']['edge_count']}")
    print(f"Certificate Valid (<= -60): {r21['cert_data']['min_slack_epsilon_star'] >= 60}")
    print(f"Minimum Integer Slack epsilon*: {r21['cert_data']['min_slack_epsilon_star']}")
    print(f"Karp's Maximum Cycle Mean lambda*: {r21['cert_data']['max_cycle_mean_lambda_star']:.4f}")
    print(f"File Size Bytes: {r21['file_size_bytes']}")
    print(f"Full SHA-256 Digest:  {r21['full_sha256']}")
    print(f"Short 16-Hex Prefix:  {r21['short_sha256']}")
