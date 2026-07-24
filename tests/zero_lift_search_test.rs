use num_bigint::BigUint;

#[path = "../src/zero_lift_search.rs"]
mod zero_lift_search;

use zero_lift_search::{compute_diophantine_defect_diagnostic, compute_endpoint_compression_diagnostic, compute_guarded_top_ternary_window_diagnostic, compute_pathwise_defect_forcing_diagnostic, compute_prefix_signature, compute_scc_cycle_cone_diagnostic, compute_telescoping_defect_diagnostic, compute_top_ternary_window_diagnostic, compute_universal_certificate_diagnostic, export_concrete_defect_census, export_quotient_artifacts, export_reachable_scc_census_summary, export_recurrent_zero_lift_scc_census, search_counterexample_q1_traces, search_height_controlled_recurrent_defects, search_orbit_first_zero_lift_runs, CycleConeClassification, DiophantineDefectBounds, ZeroLiftSearchBounds};

#[test]
fn test_bounded_zero_lift_search_execution() {
    let bounds = ZeroLiftSearchBounds {
        max_start_value: BigUint::from(500u32),
        max_return_steps: 12,
        max_word_length: 12,
        max_exponent_sum: 20,
        max_depth: 5,
        target_run_length: 2,
    };

    let traces = search_orbit_first_zero_lift_runs(&bounds);
    assert!(!traces.is_empty(), "Zero-lift search should find valid traces for Q1 sources <= 500");
    for trace in &traces {
        assert_eq!(&trace.anchor % 32u32, BigUint::from(7u32));
        assert!(trace.steps.len() >= 2);
    }
}

#[test]
fn test_counterexample_q1_traces_diagnostics() {
    let bounds = ZeroLiftSearchBounds {
        max_start_value: BigUint::from(500u32),
        max_return_steps: 12,
        max_word_length: 12,
        max_exponent_sum: 20,
        max_depth: 5,
        target_run_length: 2,
    };

    let ce_traces = search_counterexample_q1_traces(&bounds);
    assert!(!ce_traces.is_empty());
    for trace in &ce_traces {
        assert_eq!(&trace.anchor % 32u32, BigUint::from(7u32));
        assert!(trace.endpoint_modulus3 > BigUint::from(1u32));
        assert!(&trace.endpoint_residue3 < &trace.endpoint_modulus3);
    }
}

#[test]
fn test_prefix_signature_exact_drift_computation() {
    let bounds = ZeroLiftSearchBounds {
        max_start_value: BigUint::from(500u32),
        max_return_steps: 12,
        max_word_length: 12,
        max_exponent_sum: 20,
        max_depth: 5,
        target_run_length: 2,
    };

    let traces = search_orbit_first_zero_lift_runs(&bounds);
    assert!(!traces.is_empty());
    let sig = compute_prefix_signature(&traces[0]);
    assert!(sig.step_time > 0);
    assert!(sig.exponent_sum > 0);
    assert!(sig.is_realizable);
}

#[test]
fn test_universal_certificate_diagnostic_execution() {
    let bounds = ZeroLiftSearchBounds {
        max_start_value: BigUint::from(500u32),
        max_return_steps: 12,
        max_word_length: 12,
        max_exponent_sum: 20,
        max_depth: 5,
        target_run_length: 2,
    };

    let traces = search_orbit_first_zero_lift_runs(&bounds);
    assert!(!traces.is_empty());
    let diag = compute_universal_certificate_diagnostic(&traces[0]);
    assert!(diag.is_source_congruence_satisfied);
    assert!(diag.is_endpoint_residue_satisfied);
}

#[test]
fn test_export_quotient_artifacts_execution() {
    let (candidate, cert) = export_quotient_artifacts();
    assert!(candidate.lean_soundness_verified);
    assert_eq!(candidate.states.len(), 2);
    assert_eq!(cert.schema_version, "v1.0");
}

#[test]
fn test_diophantine_defect_diagnostic_execution() {
    let bounds = ZeroLiftSearchBounds {
        max_start_value: BigUint::from(500u32),
        max_return_steps: 12,
        max_word_length: 12,
        max_exponent_sum: 20,
        max_depth: 5,
        target_run_length: 2,
    };

    let traces = search_orbit_first_zero_lift_runs(&bounds);
    let defect_bounds = DiophantineDefectBounds {
        convergent_p: 19,
        convergent_q: 12,
        max_step_time: 12,
    };
    let diag = compute_diophantine_defect_diagnostic(&traces[0], &defect_bounds);
    assert!(diag.step_time_t > 0);
    assert!(diag.exponent_sum_a > 0);
}

#[test]
fn test_search_height_controlled_recurrent_defects_execution() {
    let bounds = ZeroLiftSearchBounds {
        max_start_value: BigUint::from(500u32),
        max_return_steps: 12,
        max_word_length: 12,
        max_exponent_sum: 20,
        max_depth: 5,
        target_run_length: 2,
    };

    let traces = search_orbit_first_zero_lift_runs(&bounds);
    assert!(!traces.is_empty());
    let defect_bounds = DiophantineDefectBounds {
        convergent_p: 19,
        convergent_q: 12,
        max_step_time: 12,
    };
    let trace = search_height_controlled_recurrent_defects(&traces[0], &defect_bounds);
    assert!(trace.step_time_t > 0);
    assert_eq!(trace.defect_bound_k, "10");
}

#[test]
fn test_export_concrete_defect_census_execution() {
    let census = export_concrete_defect_census();
    assert_eq!(census.len(), 11);
    let excluded: Vec<_> = census.iter().filter(|e| e.allowed_time_mod_12 == 6).collect();
    assert!(excluded.is_empty());
}

#[test]
fn test_compute_telescoping_defect_diagnostic_execution() {
    let diag = compute_telescoping_defect_diagnostic(7, 11, 2, 3, 5);
    assert!(diag.shifted_height_bound_verified);
}

#[test]
fn test_export_recurrent_zero_lift_scc_census_execution() {
    let census = export_recurrent_zero_lift_scc_census(2, 3, 12, 18);
    assert_eq!(census.len(), 1);
    assert!(census[0].is_zero_lift_accepted);
}

#[test]
fn test_compute_scc_cycle_cone_diagnostic_execution() {
    let diag = compute_scc_cycle_cone_diagnostic(12, 19);
    assert_eq!(diag.classification, CycleConeClassification::IntersectsNeutralBand);
    let diag2 = compute_scc_cycle_cone_diagnostic(12, 18);
    assert_eq!(diag2.classification, CycleConeClassification::StrictlyBelowNeutralBand);
}

#[test]
fn test_export_reachable_scc_census_summary_execution() {
    let summary = export_reachable_scc_census_summary(2, 3, 12, 18);
    assert_eq!(summary.reachable_zero_lift_sccs, 1);
}

#[test]
fn test_compute_pathwise_defect_forcing_diagnostic_execution() {
    let diag = compute_pathwise_defect_forcing_diagnostic(0, 100, 12, 18);
    assert!(diag.is_pathwise_defect_linear_verified);
}

#[test]
fn test_compute_endpoint_compression_diagnostic_execution() {
    let diag = compute_endpoint_compression_diagnostic(12, 5, 2);
    assert!(diag.is_endpoint_compressed_verified);
}

#[test]
fn test_compute_top_ternary_window_diagnostic_execution() {
    let diag = compute_top_ternary_window_diagnostic(2, 12, 5);
    assert!(diag.is_top_window_zero_verified);
}

#[test]
fn test_compute_guarded_top_ternary_window_diagnostic_execution() {
    let diag = compute_guarded_top_ternary_window_diagnostic(2, 12, 5);
    assert!(diag.is_guarded_and_zero_verified);
}

#[test]
fn test_compute_864_state_diagnostic_census() {
    let state_index = |r: usize, mu: usize, beta: usize| -> usize {
        r * 54 + mu * 6 + beta
    };

    let mut transition_exists = [[[false; 16]; 6]; 16];
    for r in 0..16 {
        for m in 0..2048 {
            let x: u64 = 2 * r as u64 + 1 + 32 * m as u64;
            let val = (3 * x + 1).trailing_zeros() as usize;
            let k_mod = val % 6;
            let target = (3 * x + 1) >> val;
            let r_next = (target % 32 / 2) as usize;
            if r_next < 16 {
                transition_exists[r][k_mod][r_next] = true;
            }
        }
    }

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); 864];
    let mut edge_count = 0;

    for r in 0..16 {
        if r == 3 { continue; } // Q1 excluded state
        for mu in 0..9 {
            for beta in 0..6 {
                let u = state_index(r, mu, beta);
                for r_next in 0..16 {
                    if r_next == 3 { continue; } // Target avoids Q1
                    for k_mod in 0..6 {
                        if transition_exists[r][k_mod][r_next] {
                            let beta_next = (beta + k_mod) % 6;
                            let lhs = (3 * mu + 1) % 9;
                            let pow2 = match k_mod {
                                0 => 1, 1 => 2, 2 => 4, 3 => 8, 4 => 7, 5 => 5,
                                _ => unreachable!(),
                            };
                            for mu_next in 0..9 {
                                if lhs == (pow2 * mu_next) % 9 {
                                    let v = state_index(r_next, mu_next, beta_next);
                                    adj[u].push(v);
                                    edge_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut index = 0;
    let mut indices = vec![None; 864];
    let mut lowlink = vec![0; 864];
    let mut on_stack = vec![false; 864];
    let mut stack = Vec::new();
    let mut sccs = Vec::new();

    fn strongconnect(
        u: usize,
        index: &mut usize,
        indices: &mut Vec<Option<usize>>,
        lowlink: &mut Vec<usize>,
        on_stack: &mut Vec<bool>,
        stack: &mut Vec<usize>,
        sccs: &mut Vec<Vec<usize>>,
        adj: &Vec<Vec<usize>>,
    ) {
        indices[u] = Some(*index);
        lowlink[u] = *index;
        *index += 1;
        stack.push(u);
        on_stack[u] = true;

        for &v in &adj[u] {
            if indices[v].is_none() {
                strongconnect(v, index, indices, lowlink, on_stack, stack, sccs, adj);
                lowlink[u] = lowlink[u].min(lowlink[v]);
            } else if on_stack[v] {
                lowlink[u] = lowlink[u].min(indices[v].unwrap());
            }
        }

        if lowlink[u] == indices[u].unwrap() {
            let mut scc = Vec::new();
            loop {
                let v = stack.pop().unwrap();
                on_stack[v] = false;
                scc.push(v);
                if v == u { break; }
            }
            sccs.push(scc);
        }
    }

    for u in 0..864 {
        let r = u / 54;
        if r != 3 && indices[u].is_none() {
            strongconnect(u, &mut index, &mut indices, &mut lowlink, &mut on_stack, &mut stack, &mut sccs, &adj);
        }
    }

    let mut largest_scc_size = 0;
    let mut cyclic_sccs = 0;
    for scc in &sccs {
        largest_scc_size = largest_scc_size.max(scc.len());
        let is_cyclic = if scc.len() > 1 {
            true
        } else {
            let u = scc[0];
            adj[u].contains(&u)
        };
        if is_cyclic {
            cyclic_sccs += 1;
        }
    }

    let mut reachable = vec![false; 864];
    let mut q = Vec::new();
    for r in 0..16 {
        if r == 3 { continue; }
        for mu in 0..9 {
            let u = state_index(r, mu, 0);
            reachable[u] = true;
            q.push(u);
        }
    }

    let mut head = 0;
    while head < q.len() {
        let u = q[head];
        head += 1;
        for &v in &adj[u] {
            if !reachable[v] {
                reachable[v] = true;
                q.push(v);
            }
        }
    }

    let reachable_state_count = reachable.iter().filter(|&&r| r).count();

    let mut reachable_sccs = 0;
    let mut reachable_cyclic_sccs = 0;
    for scc in &sccs {
        let is_reachable = scc.iter().any(|&u| reachable[u]);
        if is_reachable {
            reachable_sccs += 1;
            let is_cyclic = if scc.len() > 1 {
                true
            } else {
                let u = scc[0];
                adj[u].contains(&u)
            };
            if is_cyclic {
                reachable_cyclic_sccs += 1;
            }
        }
    }

    println!("\n=== 864-STATE AVOIDANCE GRAPH DIAGNOSTIC CENSUS ===");
    println!("States (Total): 864 (810 avoiding, 54 Q1)");
    println!("Boolean edges: {}", edge_count);
    println!("SCC count: {}", sccs.len());
    println!("Largest SCC size: {}", largest_scc_size);
    println!("Cyclic SCCs: {}", cyclic_sccs);
    println!("Reachable states: {}", reachable_state_count);
    println!("Reachable SCCs: {}", reachable_sccs);
    println!("Reachable cyclic SCCs: {}", reachable_cyclic_sccs);
    println!("====================================================\n");

    assert!(sccs.len() > 0);
}












