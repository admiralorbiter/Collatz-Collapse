#!/usr/bin/env python3
"""
Phase 7.3S.2D Collision & Invariant Explorer (Schema 5.0.0)
1. Mines endpoint collisions D_u == D_v across U <= 4 canonical words (7,380 words).
2. Explores state signatures (D_u mod 2^m, e_u mod 2^m) separating reachable endpoints from E_2^32.
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

def main():
    print("=== Phase 7.3S.2D Endpoint Collision & Invariant Explorer ===")
    
    # Generate words through U=4, J_pre=8
    words_u4 = []
    for j1 in range(9):
        w1 = branches[j1]
        words_u4.append(([j1], w1))
        for j2 in range(9):
            w2, _ = extend_guarded_word(w1, j2)
            words_u4.append(([j1, j2], w2))
            for j3 in range(9):
                w3, _ = extend_guarded_word(w2, j3)
                words_u4.append(([j1, j2, j3], w3))
                for j4 in range(9):
                    w4, _ = extend_guarded_word(w3, j4)
                    words_u4.append(([j1, j2, j3, j4], w4))

    print(f"Total Evaluated Words (U <= 4, J_pre <= 8): {len(words_u4)}")

    # Detect all endpoint collisions D_u == D_v
    seen_endpoints = {}
    collisions = []
    for seq, word in words_u4:
        ep = word[6]
        if ep in seen_endpoints:
            collisions.append((seen_endpoints[ep], seq, ep))
        else:
            seen_endpoints[ep] = seq

    print(f"Distinct Endpoints: {len(seen_endpoints)} | Collisions Found: {len(collisions)}")
    print("Sample Collisions:")
    for w1, w2, ep in collisions[:5]:
        print(f"  Word {w1} and Word {w2} produce D_u = {ep}")

    # Explore residue signature D_u mod 2^m vs E_2^32
    print("\n=== Modular Signature Exploration (D_u mod 2^9) ===")
    safe_d_mod9 = set()
    for seq, word in words_u4:
        d_rem = word[6] % 512
        safe_d_mod9.add(d_rem)

    print(f"Distinct D_u mod 512 Residues in Reachable Frontier: {len(safe_d_mod9)} / 512")
    
    # Check overlap of safe_d_mod9 with two-zero cylinder residues mod 512
    e2_mod9_residues = set()
    for j in range(33):
        Mj, Qj, _, _, _, Cj, Dj, _ = branches[j]
        Bj = 9 + 4 * j
        for k in range(33):
            Mk, Qk, _, _, _, Ck, Dk, _ = branches[k]
            Bk = 9 + 4 * k
            inv_Qj = pow(Qj % Mk, -1, Mk)
            a_jk = (inv_Qj * ((Ck - Dj) % Mk)) % Mk
            R_jk = Cj + Mj * a_jk
            # R_jk mod 512
            e2_mod9_residues.add(R_jk % 512)

    overlap = safe_d_mod9.intersection(e2_mod9_residues)
    print(f"Distinct E_2^32 residues mod 512: {len(e2_mod9_residues)} / 512")
    print(f"Residue overlap between Reachable R and E_2^32 mod 512: {len(overlap)}")

if __name__ == "__main__":
    main()
