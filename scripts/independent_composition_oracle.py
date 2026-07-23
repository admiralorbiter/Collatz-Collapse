#!/usr/bin/env python3
"""
Genuinely Independent Differential Python Oracle for Phase 7.3-0.

Inputs: Raw valuation words and base residue specs.
Derives all affine transforms, exact valuation-word cylinders, and
complete guarded-path cylinders independently using pure Python arithmetic.

Emits artifacts/phase73_0/python_semantic_results.json and verifies canonical diff against Rust.
"""

import json
import os

def pow_mod_inv(a, m):
    g, x, _ = extended_gcd(a, m)
    if g != 1:
        raise ValueError(f"No modular inverse for {a} mod {m}")
    return x % m

def extended_gcd(a, b):
    if a == 0:
        return b, 0, 1
    gcd, x1, y1 = extended_gcd(b % a, a)
    x = y1 - (b // a) * x1
    y = x1
    return gcd, x, y

def compute_affine_prefix(word):
    c_k = 0
    partial_sum = 0
    for a_i in word:
        c_k = (3 * c_k) + (1 << partial_sum)
        partial_sum += a_i
    return len(word), partial_sum, c_k

def solve_starting_residue_exact(c_k, k, a_k):
    mod_exp = a_k + 1
    modulus = 1 << mod_exp
    pow3_k = 3**k
    inv3_k = pow_mod_inv(pow3_k, modulus)
    starting_residue = ((1 << a_k) - c_k) * inv3_k % modulus
    return starting_residue, mod_exp

def apply_affine_map(n_0, k, a_k, c_k):
    pow3_k = 3**k
    num = (pow3_k * n_0) + c_k
    denom = 1 << a_k
    if num % denom != 0:
        raise ValueError(f"n_0={n_0} does not execute exact word (remainder {num % denom})")
    return num // denom

def compute_guarded_path_cylinder(sequence_words, base_residue, base_mod_exp):
    flattened = []
    for w in sequence_words:
        flattened.extend(w)
    
    k, a_k, c_k = compute_affine_prefix(flattened)
    min_exp = a_k + base_mod_exp
    base_mod = 1 << base_mod_exp

    step_prefixes = [compute_affine_prefix(w) for w in sequence_words]

    for exp in range(min_exp, min_exp + 6):
        num_candidates = 1 << exp
        valid_residues = []

        for r in range(num_candidates):
            if r % base_mod != base_residue:
                continue
            
            curr = r
            path_valid = True
            for sk, sa_k, sc_k in step_prefixes:
                try:
                    curr = apply_affine_map(curr, sk, sa_k, sc_k)
                    if curr % base_mod != base_residue:
                        path_valid = False
                        break
                except ValueError:
                    path_valid = False
                    break
            
            if path_valid:
                valid_residues.append(r)
        
        if len(valid_residues) == 1:
            return valid_residues[0], exp

    raise RuntimeError("Failed to find unique guarded path cylinder")

def run_oracle():
    u = [1, 1, 2]
    v = [1, 1, 2, 1, 2, 2]
    q1_residue = 7
    q1_mod_exp = 5  # Q1 = 7 mod 32

    # Sequence [u, v]
    k_uv, a_uv, c_uv = compute_affine_prefix(u + v)
    exact_res_uv, exact_mod_uv = solve_starting_residue_exact(c_uv, k_uv, a_uv)
    guarded_res_uv, guarded_mod_uv = compute_guarded_path_cylinder([u, v], q1_residue, q1_mod_exp)

    # Sequence [v, u]
    k_vu, a_vu, c_vu = compute_affine_prefix(v + u)
    exact_res_vu, exact_mod_vu = solve_starting_residue_exact(c_vu, k_vu, a_vu)
    guarded_res_vu, guarded_mod_vu = compute_guarded_path_cylinder([v, u], q1_residue, q1_mod_exp)

    oracle_output = {
        "execution_semantics": "left_to_right_v1",
        "sequence_uv": {
            "flattened_word": u + v,
            "affine_form": {"k": k_uv, "total_twos": a_uv, "constant": str(c_uv)},
            "exact_word_cylinder": {"residue": str(exact_res_uv), "modulus_exponent": exact_mod_uv},
            "guarded_path_cylinder": {"residue": str(guarded_res_uv), "modulus_exponent": guarded_mod_uv},
        },
        "sequence_vu": {
            "flattened_word": v + u,
            "affine_form": {"k": k_vu, "total_twos": a_vu, "constant": str(c_vu)},
            "exact_word_cylinder": {"residue": str(exact_res_vu), "modulus_exponent": exact_mod_vu},
            "guarded_path_cylinder": {"residue": str(guarded_res_vu), "modulus_exponent": guarded_mod_vu},
        },
        "commutator_constant_diff": str(c_vu - c_uv)
    }

    # Verify expected headline values
    assert exact_res_uv == 1767 and exact_mod_uv == 14, f"UV exact word mismatch: {exact_res_uv} mod 2^{exact_mod_uv}"
    assert guarded_res_uv == 214759 and guarded_mod_uv == 18, f"UV guarded path mismatch: {guarded_res_uv} mod 2^{guarded_mod_uv}"
    assert exact_res_vu == 1959 and exact_mod_vu == 14, f"VU exact word mismatch: {exact_res_vu} mod 2^{exact_mod_vu}"
    assert guarded_res_vu == 1959 and guarded_mod_vu == 18, f"VU guarded path mismatch: {guarded_res_vu} mod 2^{guarded_mod_vu}"
    assert (c_vu - c_uv) == 5568, f"Commutator diff mismatch: {c_vu - c_uv}"

    artifacts_dir = "artifacts/phase73_0"
    os.makedirs(artifacts_dir, exist_ok=True)
    
    python_artifact = os.path.join(artifacts_dir, "python_semantic_results.json")
    with open(python_artifact, "w") as f:
        json.dump(oracle_output, f, indent=2)

    rust_artifact = os.path.join(artifacts_dir, "rust_semantic_results.json")
    if os.path.exists(rust_artifact):
        with open(rust_artifact, "r") as f:
            rust_data = json.load(f)
        assert rust_data == oracle_output, f"Canonical diff failure between Rust and Python outputs!\nRust: {rust_data}\nPython: {oracle_output}"
        print("Canonical diff between Rust and Python: EMPTY (100% Identical)")

    print("Independent Python Differential Oracle PASSED successfully!")
    print(json.dumps(oracle_output, indent=2))

if __name__ == "__main__":
    run_oracle()
