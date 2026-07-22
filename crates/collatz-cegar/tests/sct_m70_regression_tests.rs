use collatz_cegar::non_commuting_cycles::NonCommutingCycleAnalyzer;
use collatz_cegar::semantic_gate::{CylinderImage, SemanticGate, WordForcingStatus};

#[test]
fn test_m70_regression_reject_misaligned_target_residue() {
    // F_1(7) = 13 != 43 mod 64 (Q2 requirement)
    let w1 = collatz_affine::ValuationWord::new(vec![1, 1, 2]).unwrap();
    let img = CylinderImage::compute_exact_image(7, 5, &w1, 6).unwrap();

    assert_eq!(img.target_base_image, 13);
    assert_ne!(img.target_base_image % 64, 43);
}

#[test]
fn test_m70_regression_reject_non_odd_macrostep_image() {
    // F_2(11) = 10 (EVEN!), which violates odd-only macrostep assumption
    let w2 = collatz_affine::ValuationWord::new(vec![1, 2, 2]).unwrap();
    let img_res = CylinderImage::compute_exact_image(11, 5, &w2, 5);

    assert!(img_res.is_err() || img_res.unwrap().target_base_image % 2 == 0);
}

#[test]
fn test_m70_regression_reject_commutative_cycles() {
    // Simple cycle repetition [1, 2] and [1, 2, 1, 2] commute (uv == vu)
    assert!(NonCommutingCycleAnalyzer::do_words_commute(
        &[1, 2],
        &[1, 2, 1, 2]
    ));
}

#[test]
fn test_m70_regression_exact_word_forcing_status() {
    // [1, 1, 2] has total valuation A = 4. Minimally requires 2^5 = 32 (modulus_exponent >= 5) for exact word forcing.
    let w1 = collatz_affine::ValuationWord::new(vec![1, 1, 2]).unwrap();
    assert_eq!(
        SemanticGate::verify_word_forcing(7, 5, &w1),
        WordForcingStatus::ExactWord
    );
    assert_eq!(
        SemanticGate::verify_word_forcing(7, 4, &w1),
        WordForcingStatus::TerminalAtLeast
    );
}
