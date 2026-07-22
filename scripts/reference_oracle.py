#!/usr/bin/env python3
"""
Independent Python Reference Oracle for Phase 6D (Collatz Research Workbench)

Verifies affine constants c_k, 2-adic fixed points x* = c_k / (2^A - 3^k),
canonical linear forms L_w(n) = alpha * n + beta, return congruence gate (2^A - 3^k)r_0 - c_w == 0 mod 2^m,
exact rational fixed-point replay, one-lap witness, countdown metric calculations, and 5-way classification.
"""

import sys
import json
from math import gcd

def compute_affine_prefix(vals):
    k = len(vals)
    c_k = 0
    A_curr = 0
    for v in vals:
        c_k = 3 * c_k + (1 << A_curr)
        A_curr += v
    return k, A_curr, c_k

def canonicalize_primitive(vals):
    n = len(vals)
    for length in range(1, n // 2 + 1):
        if n % length == 0:
            unit = vals[:length]
            if unit * (n // length) == vals:
                return unit, True
    return vals, False

def verify_rational_fixed_point_replay(c_k, A, k, vals):
    diff_d = (1 << A) - (3**k)
    if diff_d < 0:
        raw_q, raw_p = -c_k, -diff_d
    else:
        raw_q, raw_p = c_k, diff_d

    g = gcd(raw_q, raw_p)
    q, p = raw_q // g, raw_p // g
    if p < 0:
        p, q = -p, -q

    initial_q, initial_p = q, p

    for idx, expected_v in enumerate(vals):
        y = 3 * q + p
        if y == 0:
            return False, idx, expected_v, "Infinite"
        actual_v = (abs(y) & -abs(y)).bit_length() - 1
        if actual_v != expected_v:
            return False, idx, expected_v, actual_v
        next_q_num = y >> expected_v
        next_p = p
        next_g = gcd(next_q_num, next_p)
        q, p = next_q_num // next_g, next_p // next_g

    if q * initial_p != initial_q * p:
        return False, len(vals), 0, 0

    return True, None, None, None

def solve_one_lap_witness(vals, start_r, m):
    k, A, c_k = compute_affine_prefix(vals)
    target_exp = max(m, A) + A
    mod_search = 1 << target_exp
    for n in range(start_r, start_r + mod_search * 50, 1 << m):
        if n <= 0:
            continue
        curr = n
        valid = True
        for v in vals:
            if curr < 1:
                valid = False
                break
            num = 3 * curr + 1
            tz = (num & -num).bit_length() - 1
            if tz != v:
                valid = False
                break
            curr = num >> v
        if valid and curr >= 1:
            return n
    return None

def synthesize_phase_6d(raw_vals, start_r, m):
    # Step 1: One-lap witness check on ORIGINAL candidate (includes positivity guards n_i >= 1)
    witness = solve_one_lap_witness(raw_vals, start_r, m)
    if witness is None:
        return {"classification": "InfeasibleAbstractCycle", "reason": "No positive 1-lap witness found"}

    k_orig, A_orig, c_k_orig = compute_affine_prefix(raw_vals)
    two_a_orig = 1 << A_orig
    three_k_orig = 3**k_orig
    diff_d_orig = two_a_orig - three_k_orig

    # Step 2: Return state congruence gate on ORIGINAL candidate
    return_diff = diff_d_orig * start_r - c_k_orig
    mod_m = 1 << m
    if (return_diff % mod_m) != 0:
        return {"classification": "NonReturningWord", "start_r": start_r, "reason": "Return congruence fails"}

    # Step 3: Exact rational fixed point replay gate on ORIGINAL candidate
    ok, mismatch_step, exp_v, act_v = verify_rational_fixed_point_replay(c_k_orig, A_orig, k_orig, raw_vals)
    if not ok:
        return {
            "classification": "FixedPointWordMismatch",
            "fixed_point": f"{c_k_orig}/{diff_d_orig}",
            "first_mismatch_step": mismatch_step,
            "expected_valuation": exp_v,
            "actual_valuation": act_v
        }

    # Step 4: ONLY NOW Apply Primitive & Rotational Canonicalization (after candidate validation!)
    prim_vals, is_repeated = canonicalize_primitive(raw_vals)
    k, A, c_k = compute_affine_prefix(prim_vals)
    two_a = 1 << A
    three_k = 3**k
    diff_d = two_a - three_k

    if prim_vals == [2] and start_r % 4 == 1:
        return {
            "classification": "TrivialPositiveCycle",
            "start_n": 1,
            "is_primitive_canonical": is_repeated
        }

    if diff_d < 0:
        alpha = -diff_d
        beta = c_k
        fp_num = -c_k
        fp_den = alpha
    else:
        alpha = diff_d
        beta = -c_k
        fp_num = c_k
        fp_den = alpha

    if diff_d > 0 and (c_k % diff_d == 0) and (c_k // diff_d > 0):
        return {
            "classification": "PositiveCycleCandidate",
            "starting_n": c_k // diff_d,
            "valuation_word": prim_vals
        }

    return {
        "classification": "FiniteFuelMacrocycle",
        "valuation_word": prim_vals,
        "k": k,
        "A": A,
        "c_k": c_k,
        "alpha": alpha,
        "beta": beta,
        "fixed_point": f"{fp_num}/{fp_den}",
        "one_lap_witness": witness,
        "countdown_offset": m,
        "valuation_drop": A
    }

def main():
    print("=== Phase 6D Independent Python Reference Oracle ===")
    
    # Regression test affine constants for (2), (2,2), (2,2,2)
    k1, A1, c1 = compute_affine_prefix([2])
    print(f"affine([2]) => k={k1}, A={A1}, c={c1}")
    assert (k1, A1, c1) == (1, 2, 1)

    k2, A2, c2 = compute_affine_prefix([2, 2])
    print(f"affine([2,2]) => k={k2}, A={A2}, c={c2}")
    assert (k2, A2, c2) == (2, 4, 7)

    k3, A3, c3 = compute_affine_prefix([2, 2, 2])
    print(f"affine([2,2,2]) => k={k3}, A={A3}, c={c3}")
    assert (k3, A3, c3) == (3, 6, 37)

    # Test w = (1)
    res_w1 = synthesize_phase_6d([1], 3, 2)
    print(f"w=(1), r=3 mod 4 => {res_w1['classification']} (alpha={res_w1.get('alpha')}, beta={res_w1.get('beta')})")
    assert res_w1['classification'] == "FiniteFuelMacrocycle"
    assert res_w1['alpha'] == 1 and res_w1['beta'] == 1

    # Test w = (1, 1, 2)
    res_w112 = synthesize_phase_6d([1, 1, 2], 7, 4)
    print(f"w=(1,1,2), r=7 mod 16 => {res_w112['classification']} (alpha={res_w112.get('alpha')}, beta={res_w112.get('beta')})")
    assert res_w112['classification'] == "FiniteFuelMacrocycle"
    assert res_w112['alpha'] == 11 and res_w112['beta'] == 19

    # Test w = (2)
    res_w2 = synthesize_phase_6d([2], 1, 2)
    print(f"w=(2), r=1 mod 4 => {res_w2['classification']}")
    assert res_w2['classification'] == "TrivialPositiveCycle"

    # Test w = (5, 5)
    res_infeasible = synthesize_phase_6d([5, 5], 7, 4)
    print(f"w=(5,5), r=7 mod 16 => {res_infeasible['classification']}")
    assert res_infeasible['classification'] == "InfeasibleAbstractCycle"

    print("\n--- Bounded Corpus Search over Alphabet {1,2,3} (k=1..3, m=4) ---")
    summary = {}
    expanding_count = 0
    contracting_count = 0

    import itertools
    for k in range(1, 4):
        for vals in itertools.product((1, 2, 3), repeat=k):
            a_sum = sum(vals)
            three_k = 3**k
            two_a = 1 << a_sum
            if three_k > two_a:
                expanding_count += 8
            else:
                contracting_count += 8

            for r in range(1, 16, 2):
                res = synthesize_phase_6d(list(vals), r, 4)
                cls = res['classification']
                summary[cls] = summary.get(cls, 0) + 1
    
    print(f"Expanding Candidates (3^k > 2^A): {expanding_count}")
    print(f"Contracting Candidates (2^A > 3^k): {contracting_count}")
    print(json.dumps(summary, indent=2))
    print("Python Reference Oracle: ALL CHECKS PASSED SUCCESSFULLY.")

if __name__ == '__main__':
    main()
