#!/usr/bin/env python3
"""
Phase 7.3S.1D Oracle: Auxiliary Diagnostics Layer (2-adic, 3-adic, Real Drift)
Evaluates real drift ratio delta_real(w), additive drift chi(w), and 3-adic endpoint bit length ell_3(eta_w).
"""

import math

def compute_perfect_beta_j(j):
    beta = 26
    for k in range(1, j + 1):
        beta = 27 * beta + 674 * (16 ** (k - 1))
    return beta

def compute_authoritative_branch(j):
    b = 9 + 4 * j
    m = 1 << b
    q = 729 * (27 ** j)
    inv_27_j = pow(27 ** j, -1, m)
    inv_729 = pow(729, -1, m)
    term1 = 81 * (1 << (1 + 4 * j)) * inv_27_j
    num = (term1 - 231) % m
    c_j = (inv_729 * num) % m
    inv_m_11 = pow(m, -1, 11)
    mu_j = ((1 - c_j) * inv_m_11) % 11
    C_j = (c_j - 1 + m * mu_j) // 11
    beta_j = compute_perfect_beta_j(j)
    D_j = (q * C_j + beta_j) // m
    d_j = 11 * D_j + 1 - q * mu_j
    return m, q, c_j, d_j, mu_j, C_j, D_j, beta_j

branches = {j: compute_authoritative_branch(j) for j in range(33)}

def extend_guarded_word(parent, j):
    m_w, q_w, c_w, d_w, mu_w, C_w, D_w, beta_w = parent
    m_j, q_j, c_j, d_j, mu_j, C_j, D_j, beta_j = branches[j]
    inv_q = pow(q_w % m_j, -1, m_j)
    target_diff = (C_j - D_w) % m_j
    r = (inv_q * target_diff) % m_j
    rho_next = C_w + m_w * r
    q_next = q_w * q_j
    m_next = m_w * m_j
    beta_next = q_j * beta_w + m_w * beta_j
    D_next = (q_next * rho_next + beta_next) // m_next
    return (m_next, q_next, c_j, d_j, mu_j, rho_next, D_next, beta_next), r

def evaluate_diagnostics(gap_word):
    w_curr = branches[gap_word[0]]
    for g in gap_word[1:]:
        w_curr, _ = extend_guarded_word(w_curr, g)
        
    m_w = w_curr[0]
    q_w = w_curr[1]
    rho = w_curr[5]
    D_w = w_curr[6]
    beta_w = w_curr[7]
    
    b_bits = m_w.bit_length() - 1
    ell = rho.bit_length()
    z = b_bits - ell
    
    # Corrected Real Drift: delta_real = (6r + 3*sum(j_i)) * log2(3) / (9r + 4*sum(j_i))
    r_len = len(gap_word)
    sum_j = sum(gap_word)
    log2_q = (6 * r_len + 3 * sum_j) * math.log2(3)
    log2_m = 9 * r_len + 4 * sum_j
    delta_real = log2_q / log2_m
    chi_additive = log2_q - log2_m
    
    # 3-adic Endpoint Residue: eta_w = (beta_w * M_w^{-1}) mod Q_w
    inv_m_q = pow(m_w % q_w, -1, q_w)
    eta_w = (beta_w * inv_m_q) % q_w
    ell_3 = math.ceil(math.log(eta_w + 1, 3)) if eta_w > 0 else 0
    k_3 = 6 * r_len + 3 * sum_j
    z_3 = k_3 - ell_3
    
    return {
        "word": gap_word,
        "B": b_bits,
        "ell_2": ell,
        "Z_2": z,
        "Z_2/B": z / b_bits,
        "delta_real": delta_real,
        "chi_additive": chi_additive,
        "K_3": k_3,
        "ell_3": ell_3,
        "Z_3": z_3
    }

def main():
    print("=== Phase 7.3S.1D Auxiliary Diagnostics Layer (Schema 3.0.0) ===")
    benchmark_words = [
        [18],
        [6, 2],
        [0, 3, 1],
        [2, 8, 0],
        [0, 0, 7, 0],
        [19, 5, 30, 20, 0]
    ]
    
    hdr_w, hdr_b, hdr_z2, hdr_r2, hdr_dr, hdr_chi, hdr_l3 = "Word", "B", "Z_2", "Z_2/B", "delta_real", "chi_add", "ell_3"
    print(f"{hdr_w:<20} | {hdr_b:<4} | {hdr_z2:<4} | {hdr_r2:<7} | {hdr_dr:<10} | {hdr_chi:<8} | {hdr_l3:<5}")
    print("-" * 75)
    
    for w in benchmark_words:
        d = evaluate_diagnostics(w)
        print(f"{str(w):<20} | {d['B']:<4} | {d['Z_2']:<4} | {d['Z_2/B']:.4f}  | {d['delta_real']:.6f}   | {d['chi_additive']:<+8.2f} | {d['ell_3']:<5}")

if __name__ == "__main__":
    main()
