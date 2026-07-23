#!/usr/bin/env python3
"""
Phase 7.3S.2B Backward Tree Oracle (Schema 4.1.0)
Independent Python implementation of exact backward cylinder approximants E_0 = [0]_0, E_{n+1} = Phi_J(E_n),
trie sibling merging, and bounded-prefix reachability intersection I_{U,J} cap E_n.
"""

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

class PythonCylinderTrie:
    def __init__(self):
        self.root = {"is_end": False, "left": None, "right": None}

    def insert(self, r, p):
        curr = self.root
        for bit_idx in range(p):
            if curr["is_end"]:
                return
            bit = (r >> bit_idx) & 1
            key = "left" if bit == 0 else "right"
            if curr[key] is None:
                curr[key] = {"is_end": False, "left": None, "right": None}
            curr = curr[key]
        curr["is_end"] = True
        curr["left"] = None
        curr["right"] = None
        self._compact(self.root)

    def _compact(self, node):
        if node["is_end"]:
            return
        if node["left"]:
            self._compact(node["left"])
        if node["right"]:
            self._compact(node["right"])
        left_end = node["left"]["is_end"] if node["left"] else False
        right_end = node["right"]["is_end"] if node["right"] else False
        if left_end and right_end:
            node["is_end"] = True
            node["left"] = None
            node["right"] = None

    def contains(self, x):
        curr = self.root
        bit_idx = 0
        while curr:
            if curr["is_end"]:
                return True
            bit = (x >> bit_idx) & 1
            curr = curr["left"] if bit == 0 else curr["right"]
            bit_idx += 1
        return False

    def to_list(self):
        acc = []
        def _collect(node, p, r):
            if node["is_end"]:
                acc.append((r, p))
                return
            if node["left"]:
                _collect(node["left"], p + 1, r)
            if node["right"]:
                _collect(node["right"], p + 1, r | (1 << p))
        _collect(self.root, 0, 0)
        return acc

def pre_j(s, m, j):
    p_j = branches[j]
    M_j = p_j[0]
    Q_j = p_j[1]
    C_j = p_j[5]
    D_j = p_j[6]
    B_j = M_j.bit_length() - 1
    
    mod_2m = 1 << m
    inv_Q_j = pow(Q_j % mod_2m, -1, mod_2m)
    diff = (s - D_j) % mod_2m
    n_rem = (inv_Q_j * diff) % mod_2m
    
    r_pred = (C_j + M_j * n_rem) % (1 << (m + B_j))
    p_pred = m + B_j
    return r_pred, p_pred

def main():
    print("=== Phase 7.3S.2B Python Backward Tree Oracle ===")
    max_gap = 2
    
    # E_0 = [0]_0
    e_curr = PythonCylinderTrie()
    e_curr.insert(0, 0)
    
    print("Level 0 (E_0): 1 cylinder [0]_0")
    
    for level in range(1, 4):
        e_next = PythonCylinderTrie()
        for r, p in e_curr.to_list():
            for j in range(max_gap + 1):
                r_p, p_p = pre_j(r, p, j)
                e_next.insert(r_p, p_p)
        e_curr = e_next
        cyls = e_curr.to_list()
        print(f"Level {level} (E_{level}): {len(cyls)} cylinders in Trie")
        for c in cyls[:3]:
            print(f"  Sample: [{c[0]}]_{c[1]}")
            
    print("\nPYTHON ORACLE BACKWARD TREE GENERATION COMPLETED SUCCESSFULLY!")

if __name__ == "__main__":
    main()
