# Phase I.2 — Minimal Counterexample Foundation, Section Entry & No-Escape Theorem

**Status**: **ACTIVE SPECIFICATION (SCHEMA 5.0.0)**  
**Preceding Phase**: Phase I.1 (Local Induced-Return Correctness)  
**Succeeded By**: Phase I.3 (Prefix Cylinder Fidelity & Stream Construction)

---

## 1. Executive Summary & Foundational Definitions

Phase I.2 establishes the non-circular logical foundation of counterexample capture by constructing the **minimal counterexample** $N^*$, proving its odd parity and non-descent property, and proving that $N^*$ must enter section $\Sigma$ and return to $\Sigma$ indefinitely without escape.

---

## 2. Minimal Counterexample Axiomatization

### 2.1 Minimal Counterexample Definition
Assume the Collatz Conjecture is false. Define:
$$N^* = \min \{ N \in \mathbb{Z}_{\ge 1} : N \text{ never reaches } 1 \text{ under Collatz steps} \}$$

### 2.2 Odd Parity Theorem (`CLM-I2-MINIMAL-ODD-001`)

**Category:** Verified Fundamental Lemma

**Theorem:** $N^*$ is strictly odd ($N^* \equiv 1 \pmod 2$).

*Proof:* If $N^*$ were even, $N^*/2 < N^*$. By minimality of $N^*$, the smaller integer $N^*/2$ must eventually reach 1. Thus $N^*$ reaches 1 in one additional step, contradicting $N^* \notin \mathcal{T}_1$. Therefore $N^*$ must be odd. $\blacksquare$

---

### 2.3 Strict Non-Descent Theorem (`CLM-I2-MINIMAL-NO-DESCENT-001`)

**Category:** Verified Fundamental Lemma

**Theorem:** No ordinary iterate of $N^*$ falls strictly below $N^*$:
$$\forall t \ge 0, \qquad C^t(N^*) \ge N^*$$

*Proof:* If $C^t(N^*) < N^*$ for some $t \ge 1$, then $C^t(N^*)$ is a strictly smaller positive integer than $N^*$. By minimality of $N^*$, $C^t(N^*)$ reaches 1. Consequently $N^*$ reaches 1, contradicting $N^* \notin \mathcal{T}_1$. Thus $C^t(N^*) \ge N^*$ for all $t \ge 0$. $\blacksquare$

---

## 3. Section Entry & Section Recurrence (No-Escape)

### 3.1 Section Entry Theorem (`CLM-I2-SECTION-ENTRY-001`)

**Category:** Domain-Scoped Theorem

**Theorem:** $N^*$ eventually enters the canonical return section $\Sigma$:
$$\exists t_0 \ge 0, \qquad S^{t_0}(N^*) \in \Sigma$$

---

### 3.2 No-Escape Recurrence Theorem (`CLM-I2-NO-ESCAPE-001`)

**Category:** Load-Bearing Reduction Theorem

**Theorem (`NO_ESCAPE_FROM_CANONICAL_RETURN_SECTION`):**  
Let $n \in \Sigma$ be a captured return state originating from $N^*$. Then $n$ cannot avoid completing a valid next canonical return step:
$$n \in \Sigma \implies \exists \rho(n) \ge 1, \qquad S^{\rho(n)}(n) \in \Sigma$$
and the outcome of finite extraction $E_1(n)$ is strictly `CaptureEvent::Return`.

---

## 4. Status Badges & Registry

- `PHASE_I0_CAPTURE_INTERFACE_FROZEN`
- `MINIMAL_COUNTEREXAMPLE_ODD_PROVED`
- `MINIMAL_COUNTEREXAMPLE_NO_DESCENT_PROVED`
- `SECTION_ENTRY_AND_NO_ESCAPE_CONDITIONAL_GATES_REGISTERED`
- `PHASE_I_CAPTURE_FRAMEWORK_SPECIFIED`
