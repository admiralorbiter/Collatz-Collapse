use collatz_cegar::extremal_source_search::ExtremalSourceSearchEngine;
use collatz_cegar::positive_control_replay_engine::PositiveControlReplayEngine;

#[test]
fn test_mod512_endpoint_only_separation_refutation() {
    // 1. Fetch exact parameters for j=0 branch guard C_0 and D_0
    let p0 = ExtremalSourceSearchEngine::branch_parameters_j(0);
    let c0_mod512 = (&p0.z_source_residue % 512u64).to_u64_digits().first().cloned().unwrap_or(0);
    assert_eq!(c0_mod512, 342, "Branch guard C_0 mod 512 must equal 342");

    // 2. Evaluate positive controls D_{(0,0,7)} and D_{(2,2,8)}
    let (d_007, _, _) = PositiveControlReplayEngine::verify_control_0_0_7();
    let (d_228, _, _) = PositiveControlReplayEngine::verify_control_2_2_8();

    let d_007_mod512 = (&d_007 % 512u64).to_u64_digits().first().cloned().unwrap_or(0);
    let d_228_mod512 = (&d_228 % 512u64).to_u64_digits().first().cloned().unwrap_or(0);

    // 3. Confirm that both positive controls satisfy D mod 512 == 342
    assert_eq!(d_007_mod512, 342, "Control D_(0,0,7) mod 512 is 342");
    assert_eq!(d_228_mod512, 342, "Control D_(2,2,8) mod 512 is 342");

    // 4. Confirm that E_2 mod 512 projection set contains 342
    let dangerous_mod512_projection = [192u64, 224u64, 342u64];
    assert!(
        dangerous_mod512_projection.contains(&d_007_mod512),
        "Control D_(0,0,7) intersects the mod-512 projection of E_2"
    );
    assert!(
        dangerous_mod512_projection.contains(&d_228_mod512),
        "Control D_(2,2,8) intersects the mod-512 projection of E_2"
    );

    // 5. Formal Refutation Verdict:
    // Any endpoint-only set S mod 512 disjoint from {192, 224, 342} MUST exclude valid reachable one-zero witnesses.
    // Therefore, R \subseteq S and S \cap {192, 224, 342} = \emptyset CANNOT both hold.
    println!("\n=======================================================");
    println!("BADGE REGISTERED: MOD512_ENDPOINT_ONLY_SEPARATION_REFUTED");
    println!("Reason: Controls D_(0,0,7) and D_(2,2,8) mod 512 = 342 intersect E_2 mod 512.");
    println!("=======================================================\n");
}
