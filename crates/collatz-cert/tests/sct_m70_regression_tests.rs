use collatz_cert::schema::SctEdgeCertificateJson;

#[test]
fn test_cert_regression_schema_validation() {
    let raw_json = r#"{
        "schema_version": "sct_edge_v1",
        "edge_id": "E12-INVALIDATED",
        "source_state": "Q1",
        "target_state": "Q2",
        "valuation_word": [1, 1, 2],
        "source_guard": {
            "residue": "7",
            "modulus_exponent": 5,
            "positivity_required": true
        },
        "affine_map": {
            "odd_steps": 3,
            "total_twos": 4,
            "constant": "19"
        },
        "features": [],
        "proved_relations": [],
        "proof_kind": "fixed_point_linear_form"
    }"#;

    let cert: Result<SctEdgeCertificateJson, _> = serde_json::from_str(raw_json);
    assert!(cert.is_ok());
}
