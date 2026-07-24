# Phase I.0 — Capture Interface Freeze & Syracuse Ground Trace

**Status**: **FROZEN SPECIFICATION (SCHEMA 5.0.0)**  
**Preceding Phase**: Phase H.3 (Sturmian Gap Elimination Version 1.0)  
**Succeeded By**: Phase I.1 (Local Induced-Return Correctness)

---

## 1. Executive Summary & Purpose

Phase I.0 freezes the ground-level definitions, alphabet conventions, return section semantics, and finite capture outcome types connecting ordinary odd Collatz (Syracuse) iterations to the canonical 2-adic return subsystem.

---

## 2. Standard Syracuse Ground Trace

For any odd positive integer $n_0 \in 2\mathbb{N}+1$:
1. $3n_k + 1$ is even, admitting a unique 2-adic valuation $a_k = v_2(3n_k + 1) \ge 1$.
2. The next odd Syracuse iterate is $n_{k+1} = S(n_k) = \frac{3n_k + 1}{2^{a_k}}$.
3. The infinite sequence of valuation steps $a(n_0) = (a_0, a_1, a_2, \dots)$ is the **Syracuse ground trace** of $n_0$.

---

## 3. Canonical Section, Return Time & Gap Alphabet

### 3.1 Return Section $\Sigma$
The canonical return section $\Sigma \subset 2\mathbb{N}+1$ consists of odd integers satisfying the canonical quotient register entry condition $n \equiv 1 \pmod 2$. In endpoint coordinates $\mathcal{D}$, the coordinate map $\iota: \Sigma \to \mathcal{D}$ assigns:
$$\iota(n) = D_0(n)$$

### 3.2 Gap Alphabet $\mathcal{A}$
The canonical gap alphabet is:
$$\mathcal{A} = \mathbb{Z}_{\ge 0} = \{0, 1, 2, 3, \dots\}$$
For any gap symbol $j \in \mathcal{A}$:
- The total 2-adic exponent block length is $B_j = 9 + 4j$.
- The affine multiplier is $Q_j = 3^{k_j}$ (where $k_j$ is the odd step count associated with gap $j$).
- The affine modulus is $M_j = 2^{B_j}$.

### 3.3 Return Time $\rho$ and Induced Return Map $R$
For an odd state $n \in \Sigma$, the canonical return time $\rho(n) \in \mathbb{Z}_{>0}$ is the minimal number of ordinary Syracuse steps required to complete a valid canonical return block:
$$\rho(n) = \min \{ \tau \ge 1 : S^\tau(n) \in \Sigma \text{ and semantic return guard } j = h(n) \text{ is satisfied} \}$$
The induced return map $R: \Sigma \to \Sigma$ is defined by:
$$R(n) = S^{\rho(n)}(n)$$

---

## 4. Finite Capture Outcome Type

In executable and formal models, every finite extraction step evaluates to one of 4 exhaustive mathematical outcomes (plus `SearchLimitReached` for computational bounds):

```rust
pub enum CaptureEvent {
    Return {
        gap: u32,
        ordinary_steps: usize,
        next_odd: BigUint,
        next_state: CanonicalState,
        witness: ReturnWitness,
    },
    HitOne,
    DescendedBelowBase {
        value: BigUint,
    },
    Escape {
        witness: EscapeWitness,
    },
    SearchLimitReached,
}
```

Where `ReturnWitness` carries full empirical proof of the intertwining step:
```rust
pub struct ReturnWitness {
    pub source_odd: BigUint,
    pub target_odd: BigUint,
    pub ordinary_exponents: Vec<u32>,
    pub gap: u32,
    pub source_state: CanonicalState,
    pub target_state: CanonicalState,
    pub source_residue: BigUint,
    pub modulus: BigUint,
}
```

---

## 5. Status Badges & Registry

- `PHASE_I0_CAPTURE_INTERFACE_FROZEN`
- `SYRACUSE_GROUND_TRACE_SPECIFICATION_FROZEN`
- `GAP_ALPHABET_NONNEGATIVE_INTEGERS_FROZEN`
