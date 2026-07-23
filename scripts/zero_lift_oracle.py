#!/usr/bin/env python3
"""
Python Differential Oracle for Phase 7.3D-R2A:
Reachability-Corrected Zero-Tail Semantics & Exact Arbitrary-j Successor Solver
"""

import sys

def compute_mu_j(c_j, m_j):
    target = (1 - (c_j % 11)) % 11
    m_mod = m_j % 11
    for mu in range(11):
        if (m_mod * mu) % 11 == target:
            return mu
    raise ValueError("mu_j not found")

def get_branch_normal_form(j):
    k_exp = 9 + 4 * j
    m_j = 1 << k_exp
    q_j = 3 ** (6 + 3 * j)
    
    inv_729 = pow(729, -1, m_j)
    inv_pow27 = pow(27**j, -1, m_j)
    
    term1 = (81 * (1 << (1 + 4 * j))) % m_j
    term2 = (term1 * inv_pow27) % m_j
    diff = (term2 - 231) % m_j
    c_j = (diff * inv_729) % m_j
    
    pow_27 = 27**j
    pow_2 = 1 << (1 + 4 * j)
    val_c = 231 + 729 * c_j
    base_u = val_c // pow_2
    u_next = base_u * pow_27
    d_j = (u_next - 81) // 256
    
    mu_j = compute_mu_j(c_j, m_j)
    C_j = (c_j - 1 + m_j * mu_j) // 11
    D_j = (d_j - 1 + q_j * mu_j) // 11
    
    return {
        "j": j,
        "c_j": c_j,
        "m_j": m_j,
        "d_j": d_j,
        "q_j": q_j,
        "mu_j": mu_j,
        "C_j": C_j,
        "D_j": D_j,
    }

def find_exact_successor(y_s):
    t = 1 + 11 * y_s
    val = 231 + 729 * t
    delta = (val & -val).bit_length() - 1
    if delta < 1 or (delta - 1) % 4 != 0:
        return None
    j = (delta - 1) // 4
    b = get_branch_normal_form(j)
    if y_s % b["m_j"] == b["C_j"]:
        return j
    return None

def evaluate_concrete_orbit(j_0, n_0, max_steps=10):
    b0 = get_branch_normal_form(j_0)
    current_z = b0["C_j"] + b0["m_j"] * n_0
    gaps = []
    
    for _ in range(max_steps):
        j = find_exact_successor(current_z)
        if j is not None:
            gaps.append(j)
            bj = get_branch_normal_form(j)
            e = (current_z - bj["C_j"]) // bj["m_j"]
            current_z = bj["D_j"] + bj["q_j"] * e
        else:
            break
            
    return gaps

def main():
    print("--- Phase 7.3D-R2A Reachability-Corrected Oracle ---")
    
    # Test user's counterexample: n_0 = 251271 => 3 consecutive j=0 steps
    n_0 = 251271
    gaps = evaluate_concrete_orbit(0, n_0, 10)
    print(f"n_0 = {n_0} concrete orbit gap sequence: {gaps}")
    assert gaps == [0, 0, 0], f"Expected [0, 0, 0], got {gaps}"
    
    # Test canonical child state D_0 = 487 (zero-lift suffix length = 0)
    b0 = get_branch_normal_form(0)
    d0 = b0["D_j"]
    succ_d0 = find_exact_successor(d0)
    print(f"Canonical child D_0 = {d0}: exact successor = {succ_d0}")
    assert succ_d0 is None, f"Expected None for D_0, got {succ_d0}"
    
    print("ALL PHASE 7.3D-R2A ORACLE ASSERTIONS PASSED (0 DIFF).")

if __name__ == '__main__':
    main()
