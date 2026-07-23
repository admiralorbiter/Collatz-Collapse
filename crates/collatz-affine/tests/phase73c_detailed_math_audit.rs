use collatz_affine::{SymbolicLanguageEnumerator, SymbolicWordData};
use collatz_cegar::PeriodicNecklaceAnalyzer;
use std::collections::HashMap;

#[test]
fn run_phase73c_detailed_math_audit() {
    let words = SymbolicLanguageEnumerator::enumerate(12).unwrap();

    // 1. Depth Statistics & Child Counts
    let mut total_u_children = 0;
    let mut total_v_children = 0;
    let mut zero_lift_u_children = 0;
    let mut zero_lift_v_children = 0;

    let mut parent_zero_child_count: HashMap<Vec<u8>, usize> = HashMap::new();

    for w in &words {
        if let Some(is_zero) = w.is_zero_lift_from_parent {
            let is_u = w.word.as_slice().ends_with(&[1, 1, 2]);
            let is_v = w.word.as_slice().ends_with(&[1, 1, 2, 1, 2, 2]);

            if is_u {
                total_u_children += 1;
                if is_zero {
                    zero_lift_u_children += 1;
                }
            } else if is_v {
                total_v_children += 1;
                if is_zero {
                    zero_lift_v_children += 1;
                }
            }

            let parent_slice = if is_u {
                &w.word.as_slice()[..w.word.as_slice().len() - 3]
            } else {
                &w.word.as_slice()[..w.word.as_slice().len() - 6]
            };

            let entry = parent_zero_child_count.entry(parent_slice.to_vec()).or_insert(0);
            if is_zero {
                *entry += 1;
            }
        }
    }

    let mut parents_with_0_zero = 0;
    let mut parents_with_1_zero = 0;
    let mut parents_with_2_zero = 0;

    for &count in parent_zero_child_count.values() {
        match count {
            0 => parents_with_0_zero += 1,
            1 => parents_with_1_zero += 1,
            2 => parents_with_2_zero += 1,
            _ => panic!("Parent has > 2 zero children!"),
        }
    }

    println!("--- Zero-Lift Child Statistics (Depths 1..12) ---");
    println!("Total u-children: {}, Zero-lift u-children: {}", total_u_children, zero_lift_u_children);
    println!("Total v-children: {}, Zero-lift v-children: {}", total_v_children, zero_lift_v_children);
    println!("Parents with 0 zero-lift children: {}", parents_with_0_zero);
    println!("Parents with 1 zero-lift child: {}", parents_with_1_zero);
    println!("Parents with 2 zero-lift children: {}", parents_with_2_zero);

    // 2. Specific Depth-12 Witness with 2 Zero-Lift Edges (3 Nodes)
    // uuuvvvu -> uuuvvvuu -> uuuvvvuuu
    let mut word_map: HashMap<Vec<u8>, SymbolicWordData> = HashMap::new();
    for w in &words {
        word_map.insert(w.word.as_slice().to_vec(), w.clone());
    }

    // u = [1,1,2], v = [1,1,2,1,2,2]
    let u_slice = [1, 1, 2];
    let v_slice = [1, 1, 2, 1, 2, 2];

    let mut base = Vec::new();
    for _ in 0..3 { base.extend_from_slice(&u_slice); } // uuu
    for _ in 0..3 { base.extend_from_slice(&v_slice); } // vvv
    base.extend_from_slice(&u_slice);                  // uuuvvvu (depth 7)

    let mut step1 = base.clone();
    step1.extend_from_slice(&u_slice);                  // uuuvvvuu (depth 8)

    let mut step2 = step1.clone();
    step2.extend_from_slice(&u_slice);                  // uuuvvvuuu (depth 9)

    if let (Some(w0), Some(w1), Some(w2)) = (word_map.get(&base), word_map.get(&step1), word_map.get(&step2)) {
        println!("--- Certified Depth-12 Witness with 2 Zero-Lift Edges ---");
        println!("w0 (uuuvvvu):  guard = {}, A = {}", w0.guard_residue, w0.guard_modulus_exponent);
        println!("w1 (uuuvvvuu): guard = {}, A = {}, is_zero_lift = {:?}", w1.guard_residue, w1.guard_modulus_exponent, w1.is_zero_lift_from_parent);
        println!("w2 (uuuvvvuuu): guard = {}, A = {}, is_zero_lift = {:?}", w2.guard_residue, w2.guard_modulus_exponent, w2.is_zero_lift_from_parent);

        assert_eq!(w0.guard_residue, w1.guard_residue);
        assert_eq!(w1.guard_residue, w2.guard_residue);
        assert_eq!(w0.guard_residue.to_string(), "2673862933783");
    }

    // 3. Primitive Necklace Table Grouped by Canonical Root & Phases
    let necklaces = PeriodicNecklaceAnalyzer::extract_primitive_necklaces(&words).unwrap();
    println!("--- Primitive Necklace Table (Grouped by Phases) ---");
    for (idx, neck) in necklaces.iter().take(5).enumerate() {
        println!(
            "Necklace #{}: canonical={:?}, period={}",
            idx + 1,
            neck.canonical_root,
            neck.period_length
        );
        for rot in &neck.rotations {
            println!(
                "  Phase: word={:?}, fixed_point={}",
                rot.current_rotation, rot.fixed_point_rational_str
            );
        }
    }
}
