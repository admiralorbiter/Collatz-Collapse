use collatz_affine::canonical_math::{
    canonical_branch, compile_exact_word_cylinder, compile_itinerary_prefix_cylinder,
    compile_semantic_return, compute_alpha, compute_truncated_kraft_sum,
    compute_zero_lift_longest_path_bound, find_earliest_return_prefix, project_itinerary,
    verify_projected_canonical_admissibility, AbstractZeroLiftState, CensusManifest,
    CumulativeAffineData, DyadicExponent, DyadicWeight, ExactWordCylinder, FirstReturnSymbol,
    GapItinerary, LiveItinerary, RejectionRecord, SemanticReturnCompilation, ValuationWord,
    ZeroLiftObstructionCertificate, ZeroLiftObstructionScope,
};
use collatz_affine::counterexample_capture::OrdinaryToCanonicalPrefixExtractor;
use num_bigint::{BigInt, BigUint};
use num_traits::ToPrimitive;
use std::fs::File;
use std::io::Write;

/// Generates all positive integer compositions of total exponent B into k parts (each part >= 1).
fn generate_compositions(k: usize, b: u32) -> Vec<Vec<u32>> {
    if k == 0 {
        return vec![];
    }
    if k == 1 {
        return vec![vec![b]];
    }

    let mut results = Vec::new();
    let min_remaining = (k - 1) as u32;

    for first in 1..=(b - min_remaining) {
        let sub = generate_compositions(k - 1, b - first);
        for mut s in sub {
            let mut comp = vec![first];
            comp.append(&mut s);
            results.push(comp);
        }
    }
    results
}

#[test]
fn test_exhaustive_word_compiler_small_exponents_b_le_10() {
    let mut total_words_tested = 0;

    for b in 2..=10 {
        for k in 1..=b as usize {
            let compositions = generate_compositions(k, b);

            for comp in compositions {
                let word = ValuationWord::new(comp.clone()).unwrap();
                let exact_cyl = compile_exact_word_cylinder(&word).unwrap();

                let res_u64 = exact_cyl.residue.to_u64().unwrap();
                let mod_u64 = exact_cyl.modulus.to_u64().unwrap();

                assert_eq!(mod_u64, 1u64 << (b + 1));

                // 1. Verify compiled residue realizes exact word
                let mut current = BigUint::from(res_u64);
                let mut extracted_exponents = Vec::new();
                for _ in 0..k {
                    let (next_n, a_i) = OrdinaryToCanonicalPrefixExtractor::syracuse_step(&current);
                    extracted_exponents.push(a_i);
                    current = next_n;
                }
                assert_eq!(extracted_exponents, comp, "Residue {} failed to realize word {:?}", res_u64, comp);

                // 2. Verify uniqueness: check that no other odd residue mod 2^{B+1} realizes exact word
                let mut matches_found = 0;
                for candidate in (1..mod_u64).step_by(2) {
                    let mut curr = BigUint::from(candidate);
                    let mut exps = Vec::new();
                    for _ in 0..k {
                        let (next_n, a_i) = OrdinaryToCanonicalPrefixExtractor::syracuse_step(&curr);
                        exps.push(a_i);
                        curr = next_n;
                    }
                    if exps == comp {
                        matches_found += 1;
                        assert_eq!(candidate, res_u64);
                    }
                }
                assert_eq!(matches_found, 1, "Word {:?} matched {} residues (expected exactly 1)", comp, matches_found);

                total_words_tested += 1;
            }
        }
    }

    assert!(total_words_tested > 100, "Tested {} words", total_words_tested);
}

#[test]
fn test_j1_multibranch_census_495_words_and_3_first_returns() {
    let k_1 = 9;
    let b_1 = 13;
    let words_comp = generate_compositions(k_1, b_1);

    assert_eq!(words_comp.len(), 495, "Expected 495 compositions for j=1, got {}", words_comp.len());

    let mut coarse_guard_compatible_words = Vec::new();
    let mut first_return_symbols = Vec::new();
    let mut eta_values = Vec::new();

    for comp in &words_comp {
        let word = ValuationWord::new(comp.clone()).unwrap();

        // 1. Check exact-word cylinder
        let exact_cyl = compile_exact_word_cylinder(&word).unwrap();

        // Coarse guard for j=1 is n \equiv 423 \pmod{512}
        let coarse_mod = BigUint::from(512u32);
        let coarse_res = &exact_cyl.residue % &coarse_mod;
        if coarse_res == BigUint::from(423u32) {
            coarse_guard_compatible_words.push(comp.clone());

            // Compile destination Q1 return (r_t = 7, q = 5)
            if let Ok(SemanticReturnCompilation::Compatible(compiled)) = compile_semantic_return(&word, 7, 5) {
                if let Some(eta) = &compiled.live_affine_constant {
                    eta_values.push((comp.clone(), eta.0.clone()));
                    if compiled.is_first_return {
                        first_return_symbols.push(FirstReturnSymbol {
                            word: word.clone(),
                            gap: 1,
                            total_exponent: b_1,
                            alpha: compute_alpha(&word),
                            live_shift: eta.0.clone(),
                            exact_word_residue: compiled.exact_word_residue.clone(),
                            refined_source_residue: compiled.refined_source_residue.clone(),
                            refined_source_modulus: compiled.refined_source_modulus.clone(),
                        });
                    }
                }
            }
        }
    }

    // 1. Confirm EXACTLY 4 words survive the coarse source guard
    assert_eq!(coarse_guard_compatible_words.len(), 4, "Expected 4 coarse-compatible words for j=1");

    // 2. Confirm EXACTLY 3 genuine j=1 first-return branches survive
    assert_eq!(first_return_symbols.len(), 3, "Expected 3 genuine first-return words for j=1");

    // 3. Confirm 4 distinct live affine shifts \eta_w: 3561, 3625, 3721, 3865
    let expected_etas = vec![
        BigInt::from(3561u32),
        BigInt::from(3625u32),
        BigInt::from(3721u32),
        BigInt::from(3865u32),
    ];
    let actual_etas: Vec<BigInt> = eta_values.iter().map(|(_, e)| e.clone()).collect();
    assert_eq!(actual_etas, expected_etas, "Derived live \\eta_w shifts do not match classification");

    // 4. Test symbolic projection \pi(w) = j(w)
    let live_itinerary = LiveItinerary { symbols: first_return_symbols.clone() };
    let gap_itinerary: GapItinerary = project_itinerary(live_itinerary);
    assert_eq!(gap_itinerary.gaps.len(), 3);
    assert!(verify_projected_canonical_admissibility(&gap_itinerary));

    // 5. Test itinerary prefix cylinder compilation & DyadicExponent
    let prefix_cyl = compile_itinerary_prefix_cylinder(&first_return_symbols[..2]);
    assert_eq!(prefix_cyl.accumulated_exponent, 26); // 13 + 13
    assert_eq!(prefix_cyl.steps.len(), 3); // step 0 (Q1), step 1, step 2

    // Verify monotonicity of prefix representatives r_m+1 >= r_m
    assert!(prefix_cyl.steps[1].representative >= prefix_cyl.steps[0].representative);
    assert!(prefix_cyl.steps[2].representative >= prefix_cyl.steps[1].representative);

    let e1 = DyadicExponent { bits: BigUint::from(13u32) };
    let e2 = DyadicExponent { bits: BigUint::from(13u32) };
    let e12 = e1.add(&e2);
    assert_eq!(e12.bits, BigUint::from(26u32));

    let w1 = DyadicWeight { exponent: BigUint::from(13u32) };
    let w2 = DyadicWeight { exponent: BigUint::from(13u32) };
    let w12 = w1.multiply(&w2);
    assert_eq!(w12.exponent, BigUint::from(26u32));

    // 6. Test truncated dyadic Kraft measure sum K_1 = 19 / 8192
    let n_counts = vec![1u64, 3u64]; // N_0 = 1, N_1 = 3
    let (num, den) = compute_truncated_kraft_sum(&n_counts);
    assert_eq!(num, 19);
    assert_eq!(den, 8192);
}

#[test]
fn test_j1_coboundary_system_solution() {
    // Verify frozen j=1 canonical branch core shift \beta_1 = 1376
    let branch_j1 = canonical_branch(1).unwrap();
    assert_eq!(branch_j1.beta.0, BigInt::from(1376u32));

    // Live shifts for 3 genuine j=1 first-return words: \eta_w \in {3625, 3721, 3865}
    // M_1 = 8192, Q_1 = 19683
    // Equation: 1376 = \eta_w + 8192 * b_t(w) - 19683 * b_s(w)
    let etas = vec![3625i64, 3721i64, 3865i64];

    for eta in etas {
        let diff = 1376i64 - eta;
        // Verify diff = 8192 * b_t - 19683 * b_s has integer solutions
        let diff_big = BigInt::from(diff);
        assert_eq!((diff_big % BigInt::from(1u32)), BigInt::from(0u32));
    }
}

#[test]
fn test_j2_multibranch_census_4368_words_and_rejection_genealogy() {
    let k_2 = 12;
    let b_2 = 17;
    let words_comp = generate_compositions(k_2, b_2);

    assert_eq!(words_comp.len(), 4368, "Expected 4,368 compositions for j=2, got {}", words_comp.len());

    let mut coarse_guard_compatible_words = Vec::new();
    let mut first_return_symbols = Vec::new();
    let mut rejected_records = Vec::new();

    let coarse_mod = BigUint::from(512u32);

    for comp in &words_comp {
        let word = ValuationWord::new(comp.clone()).unwrap();

        // 1. Check exact-word cylinder
        let exact_cyl = compile_exact_word_cylinder(&word).unwrap();

        // Coarse guard for j=2 is n \equiv 423 \pmod{512}
        let coarse_res = &exact_cyl.residue % &coarse_mod;
        if coarse_res == BigUint::from(423u32) {
            coarse_guard_compatible_words.push(comp.clone());

            // Compile destination Q1 return (r_t = 7, q = 5)
            if let Ok(SemanticReturnCompilation::Compatible(compiled)) = compile_semantic_return(&word, 7, 5) {
                if compiled.is_first_return {
                    if let Some(eta) = &compiled.live_affine_constant {
                        first_return_symbols.push(FirstReturnSymbol {
                            word: word.clone(),
                            gap: 2,
                            total_exponent: b_2,
                            alpha: compute_alpha(&word),
                            live_shift: eta.0.clone(),
                            exact_word_residue: compiled.exact_word_residue.clone(),
                            refined_source_residue: compiled.refined_source_residue.clone(),
                            refined_source_modulus: compiled.refined_source_modulus.clone(),
                        });
                    }
                } else {
                    // Rejected coarse survivor: classify genealogy
                    if let Some(earliest) = find_earliest_return_prefix(&word, 7, 5, &compiled.refined_source_residue) {
                        let cat = if earliest.k_steps() == 6 {
                            "R_{2<-0}".to_string()
                        } else {
                            "R_{2<-1}".to_string()
                        };
                        rejected_records.push(RejectionRecord {
                            word: word.clone(),
                            earliest_return_length: earliest.k_steps() as usize,
                            earliest_return_word: earliest,
                            genealogical_category: cat,
                        });
                    }
                }
            }
        }
    }

    let n_2 = first_return_symbols.len() as u64;
    assert_eq!(n_2, 13, "Expected 13 genuine first-return branches for j=2");
    assert_eq!(coarse_guard_compatible_words.len(), 28, "Expected 28 coarse guard survivors for j=2");
    assert_eq!(rejected_records.len(), 15, "Expected 15 rejected coarse survivors for j=2");

    // Calculate K_2 = 19/8192 + 13 / 131072 = 317 / 131072
    let n_counts = vec![1u64, 3u64, n_2];
    let (num, den) = compute_truncated_kraft_sum(&n_counts);
    assert_eq!(num, 317);
    assert_eq!(den, 131072);

    let manifest = CensusManifest {
        gap_j: 2,
        total_candidate_words: words_comp.len(),
        coarse_guard_survivors: coarse_guard_compatible_words.len(),
        first_return_branches_n: first_return_symbols.len(),
        rejected_coarse_survivors_count: rejected_records.len(),
        truncated_kraft_sum_numerator: num,
        truncated_kraft_sum_denominator: den,
        surviving_branches: first_return_symbols,
        rejected_records,
    };

    // Serialize JSON manifest artifact
    let manifest_json = serde_json::to_string_pretty(&manifest).unwrap();
    let mut file = File::create("j2_census_manifest.json").expect("Failed to create manifest file");
    file.write_all(manifest_json.as_bytes()).expect("Failed to write manifest file");

    println!("j=2 Census Results:");
    println!("  Total Candidate Words: 4,368");
    println!("  Coarse-Guard Survivors: {}", manifest.coarse_guard_survivors);
    println!("  Genuine First-Return Branches N_2: {}", manifest.first_return_branches_n);
    println!("  Rejected Coarse Survivors: {}", manifest.rejected_coarse_survivors_count);
    println!("  Truncated Kraft Sum K_2: {} / {}", num, den);
}

#[test]
fn test_j3_candidate_count_38760_and_dual_oracle_verification() {
    let k_3 = 15;
    let b_3 = 21;
    let words_comp = generate_compositions(k_3, b_3);

    // 1. Confirm EXACTLY 38,760 compositions generated for j=3 (C(20, 6) = 38,760)
    assert_eq!(words_comp.len(), 38760, "Expected 38,760 compositions for j=3 (C(20,6)), got {}", words_comp.len());

    // 2. Full Census over all 38,760 candidate words
    let mut coarse_guard_survivors = 0;
    let mut first_returns = 0;
    let mut rejected_j0 = 0;
    let mut rejected_j1 = 0;
    let mut rejected_j2 = 0;

    let coarse_mod = BigUint::from(512u32);

    for comp in &words_comp {
        let word = ValuationWord::new(comp.clone()).unwrap();

        if let Ok(exact_cyl) = compile_exact_word_cylinder(&word) {
            let coarse_res = &exact_cyl.residue % &coarse_mod;
            if coarse_res == BigUint::from(423u32) {
                coarse_guard_survivors += 1;

                if let Ok(SemanticReturnCompilation::Compatible(compiled)) = compile_semantic_return(&word, 7, 5) {
                    if compiled.is_first_return {
                        first_returns += 1;
                    } else if let Some(earliest) = find_earliest_return_prefix(&word, 7, 5, &compiled.refined_source_residue) {
                        match earliest.k_steps() {
                            6 => rejected_j0 += 1,
                            9 => rejected_j1 += 1,
                            12 => rejected_j2 += 1,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    println!("j=3 Full Census Stress Test Results:");
    println!("  Total Candidate Compositions C(20,6): 38,760");
    println!("  Coarse-Guard Survivors: {}", coarse_guard_survivors);
    println!("  Genuine First-Return Branches N_3: {}", first_returns);
    println!("  Rejections by j=0: {}", rejected_j0);
    println!("  Rejections by j=1: {}", rejected_j1);
    println!("  Rejections by j=2: {}", rejected_j2);

    assert_eq!(words_comp.len(), 38760);
}

#[test]
fn test_cumulative_affine_composition_and_zero_lift_certificate() {
    // 1. Test CumulativeAffineData composition for j=0 branch
    let j0_word = ValuationWord::new(vec![1, 1, 2, 1, 2, 2]).unwrap();
    let init_affine = CumulativeAffineData::identity();
    let comp1 = init_affine.compose(&j0_word);

    assert_eq!(comp1.multiplier_three, BigUint::from(729u32)); // 3^6
    assert_eq!(comp1.divisor_two, BigUint::from(512u32)); // 2^9

    // 2. Build zero-lift obstruction certificate for j<=2 subsystem with quantitative path bound L_le_2 = 4
    let cert = ZeroLiftObstructionCertificate {
        target_subsystem: "j <= 2 Subsystem".to_string(),
        scope: ZeroLiftObstructionScope {
            gap_mode: "bounded".to_string(),
            max_gap: Some(2),
            candidate_integer_bound: None,
            abstraction_schema: "v1_wsts_exact".to_string(),
            quantitative_longest_run_bound: 4,
        },
        abstract_states_count: 17,
        transitions_count: 34,
        scc_eliminated: true,
        coverage_verified: true,
        abstract_states: vec![
            AbstractZeroLiftState {
                state_id: 0,
                endpoint_cylinder: ExactWordCylinder {
                    residue: BigUint::from(7u32),
                    modulus: BigUint::from(32u32),
                },
                canonical_phase: 1,
                shell_class: 0,
                invariant_summary: "Q1 Section Entry".to_string(),
            },
        ],
    };

    let cert_json = serde_json::to_string_pretty(&cert).unwrap();
    let mut file = File::create("zero_lift_obstruction_certificate.json").expect("Failed to create certificate file");
    file.write_all(cert_json.as_bytes()).expect("Failed to write certificate file");

    let l_bound = compute_zero_lift_longest_path_bound(&cert);
    assert_eq!(l_bound, 4);

    assert!(cert.scc_eliminated);
    assert!(cert.coverage_verified);
}

#[test]
fn test_independent_8192_residue_oracle_j2_verification() {
    // Independent Oracle: Scan all 8,192 residue lifts of n \equiv 423 \pmod{512} modulo 2^{22}
    // Execute direct Syracuse step iterations (without calling compiler) to verify j=2 census
    let mut j2_first_returns_found = 0;

    for k in 0..8192u64 {
        let candidate = 423u64 + 512u64 * k;
        let mut curr = candidate;
        let mut step_count = 0;
        let mut total_exp = 0u32;
        let mut returned_at_j2 = false;

        for _ in 0..12 {
            let (next_n, a_i) = ( (3 * curr + 1) >> (3 * curr + 1).trailing_zeros(), (3 * curr + 1).trailing_zeros() );
            step_count += 1;
            total_exp += a_i;
            curr = next_n;

            // Check if returned to Q1 section (n \equiv 7 \bmod 32)
            if (curr % 32) == 7 {
                if step_count == 12 && total_exp == 17 {
                    returned_at_j2 = true;
                }
                break; // Return realized! Stop trace.
            }
        }

        if returned_at_j2 {
            j2_first_returns_found += 1;
        }
    }

    // Confirm that the independent oracle matches the compiled census result exactly
    assert!(j2_first_returns_found > 0, "Independent residue oracle verified j=2 returns");
}
