use collatz_affine::{
    classify_guarded_return, compose_guarded_path, CanonicalCylinder, MacrostepData, Q1Quotient,
    QuotientRegisterMachine, ReturnTransitionOutcome, ValuationWord,
};
use num_bigint::BigUint;
use num_traits::ToPrimitive;

#[test]
fn generate_math_audit_tables_and_deep_exploration() {
    let u_word = ValuationWord::from_u32_slice(&[1, 1, 2]).unwrap();
    let v_word = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2]).unwrap();

    let m_u = MacrostepData::from_word(u_word).unwrap();
    let m_v = MacrostepData::from_word(v_word).unwrap();

    let q1_base = CanonicalCylinder::new(BigUint::from(7u32), 5);

    // 1. Corrected Table 1 for Length 1 to 3 words
    let words_1to3 = vec![
        ("u", vec![m_u.clone()]),
        ("v", vec![m_v.clone()]),
        ("uu", vec![m_u.clone(), m_u.clone()]),
        ("uv", vec![m_u.clone(), m_v.clone()]),
        ("vu", vec![m_v.clone(), m_u.clone()]),
        ("vv", vec![m_v.clone(), m_v.clone()]),
        ("uuu", vec![m_u.clone(), m_u.clone(), m_u.clone()]),
        ("uuv", vec![m_u.clone(), m_u.clone(), m_v.clone()]),
        ("uvu", vec![m_u.clone(), m_v.clone(), m_u.clone()]),
        ("uvv", vec![m_u.clone(), m_v.clone(), m_v.clone()]),
        ("vuu", vec![m_v.clone(), m_u.clone(), m_u.clone()]), // Corrected vuu: v, u, u
        ("vuv", vec![m_v.clone(), m_u.clone(), m_v.clone()]),
        ("vvu", vec![m_v.clone(), m_v.clone(), m_u.clone()]), // vvu: v, v, u
        ("vvv", vec![m_v.clone(), m_v.clone(), m_v.clone()]),
    ];

    println!("\n=== CORRECTED TABLE 1: LENGTH 1 TO 3 WORDS ===");
    println!("Word | A(s) | Guard (k mod 2^A) | Least k | Least n = 32k+7 | Trajectory (k0 -> k1 -> ...)");

    for (label, steps) in &words_1to3 {
        let path = compose_guarded_path(steps, &q1_base).unwrap();
        let guard_res = &path.quotient_guard.residue;
        let guard_exp = path.quotient_guard.modulus_exponent;

        let least_k = guard_res.clone();
        let least_n = (&least_k << 5) + 7u32;

        let mut traj = vec![least_k.to_string()];
        let mut curr_k = least_k.clone();
        for step in steps {
            let q = Q1Quotient::from_k(curr_k.clone());
            if let Ok(ReturnTransitionOutcome::BasedReturn { next_k, .. }) =
                QuotientRegisterMachine::eval_transition(step, &q)
            {
                traj.push(next_k.value().to_string());
                curr_k = next_k.value().clone();
            }
        }

        println!(
            "{:4} | {:5} | {} (mod 2^{}) | {:8} | {:10} | {}",
            label,
            guard_exp,
            guard_res,
            guard_exp,
            least_k,
            least_n,
            traj.join(" -> ")
        );
    }

    // 2. Direct vs Recursive Agreement test for vuu
    let vuu_steps = vec![m_v.clone(), m_u.clone(), m_u.clone()];
    let path_vuu = compose_guarded_path(&vuu_steps, &q1_base).unwrap();
    assert_eq!(path_vuu.quotient_guard.residue, BigUint::from(98365u32));
    assert_eq!(path_vuu.quotient_guard.modulus_exponent, 17);

    // Direct check for vuu
    let concat_vuu = ValuationWord::from_u32_slice(&[1, 1, 2, 1, 2, 2, 1, 1, 2, 1, 1, 2]).unwrap();
    let m_concat_vuu = MacrostepData::from_word(concat_vuu).unwrap();
    let direct_vuu = classify_guarded_return(&m_concat_vuu, &q1_base).unwrap();
    assert_eq!(
        direct_vuu.quotient_guard.residue,
        BigUint::from(98365u32)
    );
    assert_eq!(direct_vuu.quotient_guard.modulus_exponent, 17);

    // 3. Deep Exploration of M_r and Minimizing Words up to depth 15
    println!("\n=== DEEP EXPLORATION: M_r, MINIMIZING WORDS & RATIOS (DEPTH 1 TO 15) ===");
    println!("r | A_min | Min k (M_r) | Minimizing Word | 2nd Min k | Min n = 32k+7 | Ratio r_s / 2^A");

    let mut current_level: Vec<(String, Vec<MacrostepData>, BigUint, u64)> = vec![
        ("u".to_string(), vec![m_u.clone()], BigUint::from(7u32), 4),
        ("v".to_string(), vec![m_v.clone()], BigUint::from(61u32), 9),
    ];

    for depth in 1..=15 {
        current_level.sort_by(|a, b| a.2.cmp(&b.2));
        let min_entry = &current_level[0];
        let second_min_entry = if current_level.len() > 1 {
            &current_level[1]
        } else {
            &current_level[0]
        };

        let min_k = &min_entry.2;
        let min_word = &min_entry.0;
        let min_exp = min_entry.3;
        let min_n = (min_k << 5) + 7u32;

        let ratio = min_k.to_f64().unwrap() / (1u64 << min_exp) as f64;

        println!(
            "{:2} | {:5} | {:15} | {:18} | {:15} | {:17} | {:.8}",
            depth,
            min_exp,
            min_k,
            min_word,
            second_min_entry.2,
            min_n,
            ratio
        );

        if depth < 15 {
            let mut next_level = Vec::with_capacity(current_level.len() * 2);
            for (w_str, steps, _k, _exp) in &current_level {
                // Extend with u
                let mut u_steps = steps.clone();
                u_steps.push(m_u.clone());
                let path_u = compose_guarded_path(&u_steps, &q1_base).unwrap();
                next_level.push((
                    format!("{}u", w_str),
                    u_steps,
                    path_u.quotient_guard.residue,
                    path_u.quotient_guard.modulus_exponent,
                ));

                // Extend with v
                let mut v_steps = steps.clone();
                v_steps.push(m_v.clone());
                let path_v = compose_guarded_path(&v_steps, &q1_base).unwrap();
                next_level.push((
                    format!("{}v", w_str),
                    v_steps,
                    path_v.quotient_guard.residue,
                    path_v.quotient_guard.modulus_exponent,
                ));
            }
            current_level = next_level;
        }
    }

    // 4. Verify length 10 counterexample: uuuuuuuuuu vs uuuuvuuuuu
    let mut u10_steps = Vec::new();
    for _ in 0..10 {
        u10_steps.push(m_u.clone());
    }
    let path_u10 = compose_guarded_path(&u10_steps, &q1_base).unwrap();

    let mut mix10_steps = Vec::new();
    for _ in 0..4 {
        mix10_steps.push(m_u.clone());
    }
    mix10_steps.push(m_v.clone());
    for _ in 0..5 {
        mix10_steps.push(m_u.clone());
    }
    let path_mix10 = compose_guarded_path(&mix10_steps, &q1_base).unwrap();

    println!("\n=== LENGTH 10 COMPARISON ===");
    println!(
        "guard(u^10)         = {} (mod 2^{})",
        path_u10.quotient_guard.residue, path_u10.quotient_guard.modulus_exponent
    );
    println!(
        "guard(u^4 v u^5)    = {} (mod 2^{})",
        path_mix10.quotient_guard.residue, path_mix10.quotient_guard.modulus_exponent
    );
    assert!(path_mix10.quotient_guard.residue < path_u10.quotient_guard.residue);
}
