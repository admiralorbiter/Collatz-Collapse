# Phase I.1 — Local Induced-Return Correctness & Intertwining

**Status**: **FROZEN SPECIFICATION (SCHEMA 5.0.0)**  
**Preceding Phase**: Phase I.0 (Capture Interface Freeze)  
**Succeeded By**: Phase I.2 (Entry and No-Escape Theorem)

---

## 1. Executive Summary & Core Intertwining Theorem

Phase I.1 establishes the local mathematical intertwining connecting ordinary Syracuse iterates $S^{\rho(n)}(n)$ on section $\Sigma$ to the exact forward cylinder return map $F_{h(n)}$ on endpoint coordinates $\mathcal{D}$.

---

## 2. Theoretical Statements

### 2.1 Ordinary-to-Canonical Intertwining Theorem (`CLM-I1-INTERTWINE-001`)

**Category:** Verified Algebraic Identity

For any state $n \in \Sigma$ that completes a canonical return block with gap symbol $h(n) = j \in \mathcal{A}$ and return time $\rho(n)$, let $n' = S^{\rho(n)}(n) \in \Sigma$ be the next return iterate.

Then the coordinate map $\iota: \Sigma \to \mathcal{D}$ strictly satisfies:
$$\iota(S^{\rho(n)}(n)) = F_{h(n)}(\iota(n)) = \frac{Q_{h(n)} \cdot \iota(n) + \beta_{h(n)}}{M_{h(n)}}$$

Furthermore:
1. **Source Residue Compatibility:** $n \equiv r_1(h(n)) \pmod{2^{B_{h(n)}}}$.
2. **Exact Denominator Divisibility:** $Q_{h(n)} \cdot \iota(n) + \beta_{h(n)} \equiv 0 \pmod{M_{h(n)}}$.
3. **Target Positivity:** $\iota(n') > 0$ strictly.

---

### 2.2 Uniqueness of Gap Labeling (`CLM-I1-LABEL-UNIQUE-001`)

**Category:** Verified Finite Property

For any state $n \in \Sigma$, the returned gap symbol $h(n) \in \mathcal{A}$ and return exponent slice $(a_0, \dots, a_{\rho(n)-1})$ are **uniquely determined** by $n \pmod{2^{B_{\max}}}$. No state $n \in \Sigma$ can simultaneously satisfy the return guards of two distinct gap symbols $j \neq j'$.

---

### 2.3 Online Deterministic Extraction (`CLM-I1-ONLINE-DETERMINISM-001`)

**Category:** Algorithmic Guarantee

The extraction map $E_1(n)$ depends strictly on observed valuation exponents $(a_0, \dots, a_{\tau-1})$ up to the return step $\tau = \rho(n)$. It uses **zero future information**, hindsight, or hindsight search.

---

## 3. Status Badges & Registry

- `ORDINARY_TO_CANONICAL_RETURN_INTERTWINING_PROVED`
- `CANONICAL_RETURN_LABEL_UNIQUENESS_PROVED`
- `CANONICAL_RETURN_EXTRACTION_ONLINE_PROVED`
