#!/usr/bin/env python3
"""
Python Differential Oracle for Phase 7.3D-R:
Dyadic Branch Transition System, Complete Edge Normal Form, and Accelerated Source-Lift
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
    assert (u_next - 81) % 256 == 0
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

def compute_complete_edge(j, j_next):
    b1 = get_branch_normal_form(j)
    b2 = get_branch_normal_form(j_next)
    
    m_next = b2["m_j"]
    q_inv = pow(b1["q_j"], -1, m_next)
    diff = (b2["C_j"] - b1["D_j"]) % m_next
    R_j_jnext = (q_inv * diff) % m_next
    
    num_s = b1["D_j"] + b1["q_j"] * R_j_jnext - b2["C_j"]
    assert num_s % m_next == 0
    S_j_jnext = num_s // m_next
    
    # Theorem: S_{j,j'} >= 0 for all j, j' => h_min = 0
    assert S_j_jnext >= 0
    min_h = 0
    
    return {
        "source_gap": j,
        "target_gap": j_next,
        "R_j_jnext": R_j_jnext,
        "M_jnext": m_next,
        "S_j_jnext": S_j_jnext,
        "Q_j": b1["q_j"],
        "min_h": min_h,
    }

def main():
    print("--- Phase 7.3D-R Accelerated Invariant Differential Oracle ---")
    
    # Verify exact reference edge values
    e_0_0 = compute_complete_edge(0, 0)
    assert e_0_0["R_j_jnext"] == 391 and e_0_0["S_j_jnext"] == 557

    e_0_1 = compute_complete_edge(0, 1)
    assert e_0_1["R_j_jnext"] == 1313 and e_0_1["S_j_jnext"] == 116

    e_1_0 = compute_complete_edge(1, 0)
    assert e_1_0["R_j_jnext"] == 327 and e_1_0["S_j_jnext"] == 12605

    e_1_1 = compute_complete_edge(1, 1)
    assert e_1_1["R_j_jnext"] == 2485 and e_1_1["S_j_jnext"] == 5972

    e_2_3 = compute_complete_edge(2, 3)
    assert e_2_3["R_j_jnext"] == 1201743 and e_2_3["S_j_jnext"] == 304534

    print("Reference edge assertions verified (R_{2,3}=1201743, S_{2,3}=304534, h_min=0).")

    # Check 81 complete edges for j, j_next in 0..8
    total_edges = 0
    for j in range(9):
        for j_next in range(9):
            edge = compute_complete_edge(j, j_next)
            total_edges += 1
            assert edge["min_h"] == 0

    print(f"Verified {total_edges} complete edges across j=0..8 with 100% property conformance.")
    print("ALL PHASE 7.3D-R ORACLE ASSERTIONS PASSED (0 DIFF).")

if __name__ == '__main__':
    main()
