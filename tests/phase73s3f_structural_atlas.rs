use collatz_cegar::witness_family_atlas::WitnessFamilyAtlasEngine;

#[test]
fn test_phase73s3f_witness_structural_atlas() {
    let atlas = WitnessFamilyAtlasEngine::build_witness_atlas();

    println!("\n=======================================================");
    println!("PHASE 7.3S.3F STRUCTURAL WITNESS FAMILY ATLAS:");
    println!(" - Total Certified Witnesses: {}", atlas.total_witnesses);
    println!(" - Depth 6 Count: {}", atlas.depth_6_count);
    println!(" - Depth 7 Count: {}", atlas.depth_7_count);
    println!(" - Distinct Endpoint Count: {}", atlas.distinct_endpoints_count);

    println!("\n(j, k) PAIR DISTRIBUTION AT EXACT DEPTH 6:");
    println!(" - Pairs: {:?}", atlas.depth_6_jk_distribution);

    println!("\n(j, k) PAIR DISTRIBUTION AT EXACT DEPTH 7:");
    println!(" - Pairs: {:?}", atlas.depth_7_jk_distribution);

    println!("\nANCESTRAL SUBTREE DISTRIBUTION (First Gap Symbol h_0):");
    println!(" - Subtrees: {:?}", atlas.ancestral_tree_distribution);

    println!("\nDETAILED 25-WITNESS STRUCTURAL RECORD MATRIX:");
    println!("| # | Depth | Word Suffix(3) | (j, k) | Ancestral Root h_0 | Exponent Phase (e mod 64) |");
    for (idx, rec) in atlas.witness_records.iter().enumerate() {
        println!(
            "| {:^3} | {:^5} | {:^14?} | (j={}, k={}) | {:^18} | {:^25} |",
            idx + 1, rec.depth, rec.suffix_3, rec.first_gap_j, rec.second_gap_k, rec.first_symbol, rec.exponent_phase_mod64
        );
    }

    assert_eq!(atlas.total_witnesses, 25);
    assert_eq!(atlas.distinct_endpoints_count, 25, "All 25 witnesses must have distinct endpoints");

    println!("\nBADGES REGISTERED:");
    println!(" - WITNESS_STRUCTURAL_FAMILY_ATLAS_BUILT");
    println!(" - ALL_25_WITNESSES_PROVED_DISTINCT_ENDPOINTS");
    println!("=======================================================\n");
}
