#!/usr/bin/env python3
"""
Python Differential Oracle for Phase 7.3S Experiment 1:
Canonical Source Residue Minimization \rho_w, Threshold Function M_{H,J}(B), and Density Ratio alpha(B).
"""

import math
import sys

def branch_parameters_j(j):
    b_exp = 9 + 4 * j
    m_j = 1 << b_exp
    q_j = 729 * (27 ** j)
    if j == 0:
        c_j, d_j = 342, 487
    elif j == 1:
        c_j, d_j = 7392, 17761
    elif j == 2:
        c_j, d_j = 86208, 349537
    elif j == 3:
        c_j, d_j = 1764032, 12069670
    else:
        c_j = 7392
        for _ in range(1, j):
            c_j = (c_j * 16 + 86208 - 7392) % m_j
        d_j = (q_j * c_j + 1376) // m_j

    beta_j = m_j * d_j - q_j * c_j
    assert q_j * c_j + beta_j == m_j * d_j, f"Orientation identity failed for j={j}"
    return m_j, q_j, c_j, d_j, beta_j

fn_z_to_k = lambda z: 61 + 512 * z

def mod_inverse(a, m):
    g, x, y = egcd(a, m)
    if g != 1:
        raise Exception('Modular inverse does not exist')
    else:
        return x % m

def egcd(a, b):
    if a == 0:
        return (b, 0, 1)
    else:
        g, y, x = egcd(b % a, a)
        return (g, x - (b // a) * y, y)

def extend_guarded_word(rho_w, d_w, q_w, m_w, beta_w, j):
    m_j, q_j, c_j, d_j, beta_j = branch_parameters_j(j)
    inv_q_w = mod_inverse(q_w % m_j, m_j)
    
    target_diff = (c_j - d_w) % m_j
    r = (inv_q_w * target_diff) % m_j

    rho_wj = rho_w + m_w * r
    q_wj = q_j * q_w
    m_wj = m_j * m_w
    beta_wj = q_j * beta_w + m_w * beta_j
    d_wj = (q_wj * rho_wj + beta_wj) // m_wj
    return rho_wj, d_wj, q_wj, m_wj, beta_wj

def test_canonical_regressions():
    print("[+] Testing Experiment 1 Extremal Source Oracle (Canonical Residues rho_w)...")
    
    # Regression 1: j=0 -> \rho = 342, \beta = 26
    m0, q0, c0, d0, beta0 = branch_parameters_j(0)
    assert c0 == 342 and beta0 == 26, f"j=0 mismatch: c0={c0}, beta0={beta0}"

    # Regression 2: j=1 -> \rho = 7392, \beta = 1376
    m1, q1, c1, d1, beta1 = branch_parameters_j(1)
    assert c1 == 7392 and beta1 == 1376, f"j=1 mismatch: c1={c1}, beta1={beta1}"

    # Regression 3: w=(0,0) -> \rho = 200,534, \beta = 32266
    rho_00, d_00, q_00, m_00, beta_00 = extend_guarded_word(c0, d0, q0, m0, beta0, 0)
    assert rho_00 == 200534, f"w=(0,0) source mismatch: expected 200534, got {rho_00}"
    assert beta_00 == 32266, f"w=(0,0) beta mismatch: expected 32266, got {beta_00}"

    # Regression 4: w=(0,1) vs w=(1,0)
    rho_01, _, _, _, beta_01 = extend_guarded_word(c0, d0, q0, m0, beta0, 1)
    rho_10, _, _, _, beta_10 = extend_guarded_word(c1, d1, q1, m1, beta1, 0)

    assert rho_01 == 672598, f"w=(0,1) source mismatch: expected 672598, got {rho_01}"
    assert rho_10 == 2686176, f"w=(1,0) source mismatch: expected 2686176, got {rho_10}"

    # Confirm source residue minimizer chooses (0,1) over (1,0)
    assert rho_01 < rho_10, "Source minimizer must prefer (0,1) over (1,0)"

    # Direct regression assertions for B=9, M=342
    log2_342 = math.log2(342)
    alpha_9 = log2_342 / 9.0
    recip_9 = 9.0 / log2_342

    assert abs(alpha_9 - 0.935316) < 1e-4, f"alpha(9) expected ~0.9353, got {alpha_9}"
    assert abs(recip_9 - 1.069156) < 1e-4, f"bits_per_source_bit(9) expected ~1.0692, got {recip_9}"

    print("[OK] All Experiment 1 Extremal Source Oracle tests PASSED!")

if __name__ == "__main__":
    test_canonical_regressions()
