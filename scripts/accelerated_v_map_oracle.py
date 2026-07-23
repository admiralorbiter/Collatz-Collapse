#!/usr/bin/env python3
"""
Python Differential Oracle for Phase 7.3D:
u-Phase Acceleration, Induced v-to-v Map, Dyadic Branch Normal Form,
and Exact Normalized z-Coordinates
"""

import sys

def compute_valuation_x(k):
    val_expr = 11 * k + 3
    trailing_zeros = (val_expr & -val_expr).bit_length() - 1
    return 5 + trailing_zeros

def arbitrary_u_countdown_length(x):
    if x < 9:
        return 0
    return (x - 5) // 4

def accelerate_u(k):
    x = compute_valuation_x(k)
    if x < 9:
        raise ValueError(f"State k={k} not in Q_1 u-execution guard (x={x} < 9)")
    
    l = arbitrary_u_countdown_length(x)
    expr_0 = 11 * k + 3
    pow_27 = 27**l
    pow_16 = 16**l
    
    num = pow_27 * expr_0
    assert num % pow_16 == 0
    expr_l = num // pow_16
    assert (expr_l - 3) % 11 == 0
    final_k = (expr_l - 3) // 11
    final_x = x - 4 * l
    final_unit = expr_l >> (final_x - 5)
    
    return {
        "initial_k": str(k),
        "initial_valuation_x": x,
        "arbitrary_u_count_l": l,
        "final_valuation_x": final_x,
        "final_k": str(final_k),
        "final_unit_u": str(final_unit),
    }

def is_positive_realizable(t):
    return (t % 11) == 1

def t_to_z(t):
    return (t - 1) // 11 if is_positive_realizable(t) else None

def z_to_t(z):
    return 1 + 11 * z

def eval_induced_v_step(t):
    realizable = is_positive_realizable(t)
    z_val = t_to_z(t)
    val_expr = 231 + 729 * t
    delta = (val_expr & -val_expr).bit_length() - 1
    if delta < 1 or (delta - 1) % 4 != 0:
        return {
            "input_t": str(t),
            "input_z": str(z_val) if z_val is not None else None,
            "is_positive_realizable": realizable,
            "valuation_delta": delta,
            "u_step_count_j": delta // 4,
            "is_valid_v_return": False,
            "next_unit_u": str(val_expr >> delta),
            "next_t": None,
            "next_z": None,
            "next_t_realizable": False,
            "q_signature_mod27": 2,
        }
    
    j = (delta - 1) // 4
    pow_val = 1 << (1 + 4 * j)
    base_unit = val_expr // pow_val
    pow_27 = 27**j
    u_next = base_unit * pow_27
    
    is_valid_v = (u_next % 256) == 81
    next_t = ((u_next - 81) // 256) if is_valid_v else None
    next_z = t_to_z(next_t) if next_t is not None else None
    next_t_realizable = next_z is not None
    
    return {
        "input_t": str(t),
        "input_z": str(z_val) if z_val is not None else None,
        "is_positive_realizable": realizable,
        "valuation_delta": delta,
        "u_step_count_j": j,
        "is_valid_v_return": is_valid_v,
        "next_unit_u": str(u_next),
        "next_t": str(next_t) if next_t is not None else None,
        "next_z": str(next_z) if next_z is not None else None,
        "next_t_realizable": next_t_realizable,
        "q_signature_mod27": 2,
    }

def compute_mu_j(c_j, m_j):
    target = (1 - (c_j % 11)) % 11
    m_mod = m_j % 11
    for mu in range(11):
        if (m_mod * mu) % 11 == target:
            return mu
    raise ValueError("mu_j not found")

def main():
    print("--- Phase 7.3D Differential Oracle Test ---")
    
    # 1. u-countdown test
    k_test = 7
    u_res = accelerate_u(k_test)
    assert u_res['arbitrary_u_count_l'] == 1

    # 2. Verified immediate vv step (t=3763 => 5358; z=342 => 487)
    t_vv = 3763
    v_res_vv = eval_induced_v_step(t_vv)
    print(f"vv step (t={t_vv}, z={t_to_z(t_vv)}): delta={v_res_vv['valuation_delta']}, next_t={v_res_vv['next_t']}, next_z={v_res_vv['next_z']}")
    assert v_res_vv['is_positive_realizable'] == True
    assert v_res_vv['valuation_delta'] == 1
    assert v_res_vv['u_step_count_j'] == 0
    assert v_res_vv['is_valid_v_return'] == True
    assert v_res_vv['next_t'] == '5358'
    assert v_res_vv['next_z'] == '487'
    assert v_res_vv['next_t_realizable'] == True

    # 3. Verified vuv step (t=81313 => 195372; z=7392 => 17761)
    t_vuv = 81313
    v_res_vuv = eval_induced_v_step(t_vuv)
    print(f"vuv step (t={t_vuv}, z={t_to_z(t_vuv)}): delta={v_res_vuv['valuation_delta']}, next_t={v_res_vuv['next_t']}, next_z={v_res_vuv['next_z']}")
    assert v_res_vuv['is_positive_realizable'] == True
    assert v_res_vuv['valuation_delta'] == 5
    assert v_res_vuv['u_step_count_j'] == 1
    assert v_res_vuv['is_valid_v_return'] == True
    assert v_res_vuv['next_t'] == '195372'
    assert v_res_vuv['next_z'] == '17761'
    assert v_res_vuv['next_t_realizable'] == True

    # 4. Property test for exact z-normalization across j=0..3
    branch_data = [
        (0, 179, 512, 255, 729, 7, 342, 487),
        (1, 7585, 8192, 18225, 19683, 9, 7392, 17761),
        (2, 30785, 131072, 124821, 531441, 7, 86208, 349537),
        (3, 529985, 2097152, 3626208, 14348907, 9, 1764032, 12069670),
    ]

    for j, c_j, m_j, d_j, q_j, expected_mu, expected_C, expected_D in branch_data:
        mu = compute_mu_j(c_j, m_j)
        assert mu == expected_mu, f"j={j}: mu={mu} != {expected_mu}"
        
        C_calc = (c_j - 1 + m_j * mu) // 11
        D_calc = (d_j - 1 + q_j * mu) // 11
        assert C_calc == expected_C, f"j={j}: C={C_calc} != {expected_C}"
        assert D_calc == expected_D, f"j={j}: D={D_calc} != {expected_D}"

        # Property test for n=0 and n=1:
        for n in [0, 1]:
            z = C_calc + m_j * n
            t = 1 + 11 * z
            assert t % m_j == c_j % m_j
            m = (t - c_j) // m_j
            t_prime = d_j + q_j * m
            z_prime_expected = D_calc + q_j * n
            assert t_prime == 1 + 11 * z_prime_expected, f"j={j}, n={n}: t_prime fail"

    print("ALL ORACLE & PROPERTY TEST ASSERTIONS PASSED (0 DIFF).")

if __name__ == '__main__':
    main()
