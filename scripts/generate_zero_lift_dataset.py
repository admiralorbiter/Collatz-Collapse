#!/usr/bin/env python3
"""
Python Script to generate zero_lift_record_dataset.json and verify Phase 7.3D-R2 packet
"""

import json

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

def zero_lift_step(y_s, max_j_search=12):
    for j in range(max_j_search + 1):
        b = get_branch_normal_form(j)
        m_j = b["m_j"]
        if y_s % m_j == b["C_j"]:
            residual_e = (y_s - b["C_j"]) // m_j
            next_z = b["D_j"] + b["q_j"] * residual_e
            next_odd_mult = b["q_j"]
            next_prec_inc = 9 + 4 * j
            return {
                "j": j,
                "residual_e": residual_e,
                "next_z": next_z,
                "next_odd_mult": next_odd_mult,
                "prec_inc": next_prec_inc,
            }
    return None

def trace_dataset():
    records = []
    
    for start_j in range(9):
        b0 = get_branch_normal_form(start_j)
        initial_y = b0["C_j"]
        source_rho = initial_y
        
        gap_seq = [start_j]
        lift_digits = [0] # Zero-lift sequence after initial C_j setup
        endpoint_seq = [str(initial_y)]
        odd_mult_seq = ["1"]
        prec_seq = [9 + 4 * start_j]
        
        curr_y = initial_y
        curr_odd = 1
        curr_prec = 9 + 4 * start_j
        
        step_count = 0
        while step_count < 20:
            res = zero_lift_step(curr_y, max_j_search=12)
            if res is not None:
                step_count += 1
                curr_y = res["next_z"]
                curr_odd *= res["next_odd_mult"]
                curr_prec += res["prec_inc"]
                
                gap_seq.append(res["j"])
                lift_digits.append(0)
                endpoint_seq.append(str(curr_y))
                odd_mult_seq.append(str(curr_odd))
                prec_seq.append(curr_prec)
            else:
                break
                
        quot_guard = 61 + 512 * source_rho
        mod_exp = prec_seq[-1] + 9
        
        mod_3_sigs = [int(y) % 27 for y in endpoint_seq]
        
        rec = {
            "start_j": start_j,
            "gap_sequence": gap_seq,
            "lift_digits": lift_digits,
            "source_residue": str(source_rho),
            "endpoint_sequence": endpoint_seq,
            "odd_multiplier_sequence": odd_mult_seq,
            "precision_sequence": prec_seq,
            "quotient_guard": f"k = {quot_guard} (mod 2^{mod_exp})",
            "zero_suffix_length": len(gap_seq) - 1,
            "termination_reason": f"Endpoint z = {curr_y} left all zero-lift branch domains C_j (j <= 12)",
            "mod_3_signatures": mod_3_sigs,
        }
        records.append(rec)
        
    with open("scripts/zero_lift_record_dataset.json", "w") as f:
        json.dump(records, f, indent=2)
        
    print(f"Generated scripts/zero_lift_record_dataset.json with {len(records)} record entries.")

if __name__ == '__main__':
    trace_dataset()
