#!/usr/bin/env python3
"""
Python Differential Oracle for Phase 7.3S Experiment 3:
Periodic Ghost Orbit Atlas over Accelerated Gap Words (Fixed Points z_w^* < 0, Positive Representatives z_{w,r}, 2-Adic Valuations).
"""

import sys

def gap_branch_parameters(j):
    b_exp = 9 + 4 * j
    a = 729 * (27 ** j)
    if j == 0:
        b = 26
    else:
        c_j = 7
        for _ in range(1, j):
            c_j = (c_j * 27 + 3) >> 4
        b = c_j
    return a, b, b_exp

def word_parameters(word):
    a_comp = 1
    b_comp = 0
    shift_comp = 0
    for j in word:
        a_j, b_j, shift_j = gap_branch_parameters(j)
        a_comp *= a_j
        b_comp = a_j * b_comp + b_j * (1 << shift_comp)
        shift_comp += shift_j
    return a_comp, b_comp, shift_comp

def fixed_point(word):
    a_w, b_w, b_exp = word_parameters(word)
    pow_b = 1 << b_exp
    denom = pow_b - a_w
    return b_w, denom

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

def positive_representative(word, r):
    num, denom = fixed_point(word)
    _, _, b_exp = word_parameters(word)
    N = b_exp * (r + 1)
    modulus = 1 << N
    denom_abs = abs(denom)
    inv = mod_inverse(denom_abs, modulus)
    
    num_mod = num % modulus
    if denom < 0:
        rep = ((modulus - (num_mod % modulus)) * inv) % modulus
    else:
        rep = (num_mod * inv) % modulus
    return rep

def test_fixed_points():
    print("[+] Testing Experiment 3 Periodic Ghost Orbit Oracle...")
    
    # Test j=0 fixed point (pure v-branch)
    a_0, b_0, shift_0 = gap_branch_parameters(0)
    assert a_0 == 729 and b_0 == 26 and shift_0 == 9, f"j=0 params: {a_0}, {b_0}, {shift_0}"
    
    num_0, denom_0 = fixed_point([0])
    # z_0^* = 26 / (512 - 729) = 26 / -217 = -26/217
    assert num_0 == 26 and denom_0 == -217, f"j=0 fixed point: {num_0}/{denom_0}"
    
    # Test positive representatives for j=0
    z_r1 = positive_representative([0], 1)
    assert z_r1 == 200534, f"z_r1 mismatch: expected 200534, got {z_r1}"
    
    z_r3 = positive_representative([0], 3)
    assert z_r3 == 23750971222, f"z_r3 mismatch: expected 23750971222, got {z_r3}"

    # Test mixed gap word w = (0, 1)
    a_01, b_01, shift_01 = word_parameters([0, 1])
    assert shift_01 == 22, f"j=(0,1) shift mismatch: {shift_01}"

    print("[OK] All Experiment 3 Periodic Ghost Orbit Oracle tests PASSED!")

if __name__ == "__main__":
    test_fixed_points()
