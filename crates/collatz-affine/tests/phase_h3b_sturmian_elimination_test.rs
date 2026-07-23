use collatz_affine::{
    SturmianGapEmbedding, SturmianTemplateExtractor, SturmianTransitionGraph, ValuationWord,
};

#[test]
fn test_sturmian_period_extension_invariance() {
    let word_w = ValuationWord::from_slice(&[1, 2]); // Period 2

    // Verify invariance for observed length L = 6 and k = 1, 2, 5 extended periods
    assert!(SturmianTransitionGraph::verify_period_extension_invariance(&word_w, 6, 1));
    assert!(SturmianTransitionGraph::verify_period_extension_invariance(&word_w, 6, 2));
    assert!(SturmianTransitionGraph::verify_period_extension_invariance(&word_w, 6, 5));
}

#[test]
fn test_sturmian_14_primitive_necklaces_generation() {
    let embedding = SturmianGapEmbedding::ordered_1_2();
    let necklaces = SturmianTransitionGraph::generate_14_primitive_necklaces(&embedding);

    assert_eq!(
        necklaces.len(),
        14,
        "Must generate exactly 14 primitive binary necklaces of lengths 1..=5"
    );
}

#[test]
fn test_length_32_balanced_template_enumeration() {
    let templates = SturmianTemplateExtractor::generate_length_32_balanced_templates(1, 2);

    assert!(
        !templates.is_empty(),
        "Must generate length-32 balanced Sturmian templates"
    );
    for t in &templates {
        assert_eq!(t.as_slice().len(), 32);
        assert!(SturmianTemplateExtractor::is_balanced(t.as_slice(), 2));
    }
}

#[test]
fn test_sturmian_return_phase_graph_construction() {
    let embedding = SturmianGapEmbedding::ordered_1_2();
    let graph = SturmianTransitionGraph::build(embedding);

    assert!(
        graph.nodes.len() > 0,
        "Sturmian graph must instantiate valid return core nodes"
    );
    assert!(
        !graph.edges.is_empty(),
        "Sturmian graph must have transition edges between phase nodes"
    );
}

#[test]
fn test_sturmian_ordered_embedding_1_2_negative_cycle_certificate() {
    let embedding = SturmianGapEmbedding::ordered_1_2(); // (1, 2)
    let graph = SturmianTransitionGraph::build(embedding);

    let cert = graph.certify_negative_cycle_potential(1);
    assert!(
        cert.is_some(),
        "Sturmian transition graph must certify negative cycle potential for ordered embedding (1, 2)"
    );

    let (_potential, min_slack) = cert.unwrap();
    println!("Certified ordered embedding (1, 2) min_slack epsilon* = {}", min_slack);
    assert!(
        min_slack >= 1,
        "Certified minimum integer potential slack epsilon* must be >= 1"
    );
}

#[test]
fn test_sturmian_ordered_embedding_2_1_negative_cycle_certificate() {
    let embedding = SturmianGapEmbedding::ordered_2_1(); // (2, 1)
    let graph = SturmianTransitionGraph::build(embedding);

    let cert = graph.certify_negative_cycle_potential(1);
    assert!(
        cert.is_some(),
        "Sturmian transition graph must certify negative cycle potential for ordered embedding (2, 1)"
    );

    let (_potential, min_slack) = cert.unwrap();
    println!("Certified ordered embedding (2, 1) min_slack epsilon* = {}", min_slack);
    assert!(
        min_slack >= 1,
        "Certified minimum integer potential slack epsilon* must be >= 1"
    );
}
