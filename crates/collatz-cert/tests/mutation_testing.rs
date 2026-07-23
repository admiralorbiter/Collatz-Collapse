use collatz_affine::ValuationWord;
use collatz_cert::descent::generate_descent_certificate;
use collatz_cert::schema::DescentCertificateJson;
use collatz_cert::verify_descent_certificate;

#[test]
fn test_mutation_valid_base_certificate() {
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let cert = generate_descent_certificate(word).unwrap();
    assert!(verify_descent_certificate(&cert).is_ok());
}

#[test]
fn test_mutate_valuation_zero() {
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let mut cert = generate_descent_certificate(word).unwrap();
    cert.valuation_word[0] = 0; // Corrupt valuation step to 0
    assert!(verify_descent_certificate(&cert).is_err());
}

#[test]
fn test_mutate_total_twos() {
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let mut cert = generate_descent_certificate(word).unwrap();
    cert.total_twos += 1; // Corrupt total twos
    assert!(verify_descent_certificate(&cert).is_err());
}

#[test]
fn test_mutate_starting_residue() {
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let mut cert = generate_descent_certificate(word).unwrap();
    cert.starting_residue = "999999".to_string(); // Corrupt residue
    assert!(verify_descent_certificate(&cert).is_err());
}

#[test]
fn test_mutate_modulus_exponent() {
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let mut cert = generate_descent_certificate(word).unwrap();
    cert.modulus_exponent += 5; // Corrupt exponent
    assert!(verify_descent_certificate(&cert).is_err());
}

#[test]
fn test_mutate_constant() {
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let mut cert = generate_descent_certificate(word).unwrap();
    cert.constant = "12345".to_string(); // Corrupt constant c_k
    assert!(verify_descent_certificate(&cert).is_err());
}

#[test]
fn test_mutate_threshold() {
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let mut cert = generate_descent_certificate(word).unwrap();
    cert.descent_threshold = "999".to_string(); // Corrupt threshold B
    assert!(verify_descent_certificate(&cert).is_err());
}

#[test]
fn test_mutate_semantics() {
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let mut cert = generate_descent_certificate(word).unwrap();
    cert.valuation_semantics = Some("invalid_semantics".to_string());
    assert!(verify_descent_certificate(&cert).is_err());
}

#[test]
fn test_mutate_schema_version() {
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let mut cert = generate_descent_certificate(word).unwrap();
    cert.schema_version = "descent_v99".to_string();
    assert!(verify_descent_certificate(&cert).is_err());
}

#[test]
fn test_mutate_exceed_max_valuation_step() {
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let mut cert = generate_descent_certificate(word).unwrap();
    cert.valuation_word[0] = 256; // Exceeds MAX_VALUATION_STEP (255)
    assert!(verify_descent_certificate(&cert).is_err());
}

#[test]
fn test_mutate_exceed_max_modulus_exponent() {
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let mut cert = generate_descent_certificate(word).unwrap();
    cert.modulus_exponent = 4097; // Exceeds MAX_MODULUS_EXPONENT (4096)
    assert!(verify_descent_certificate(&cert).is_err());
}

#[test]
fn test_mutate_exceed_max_digits() {
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let mut cert = generate_descent_certificate(word).unwrap();
    cert.constant = "9".repeat(4097); // Exceeds MAX_DIGITS (4096)
    assert!(verify_descent_certificate(&cert).is_err());
}

#[test]
fn test_mutate_unknown_fields_rejection() {
    let json_str = r#"{
        "schema_version": "descent_v1",
        "valuation_word": [1, 1, 2, 1, 3],
        "total_twos": 8,
        "odd_steps": 5,
        "starting_residue": "39",
        "modulus_exponent": 8,
        "constant": "251",
        "descent_threshold": "20",
        "checked_exceptions": [],
        "malicious_extra_field": "attack"
    }"#;

    let res: Result<DescentCertificateJson, _> = serde_json::from_str(json_str);
    assert!(res.is_err());
}
