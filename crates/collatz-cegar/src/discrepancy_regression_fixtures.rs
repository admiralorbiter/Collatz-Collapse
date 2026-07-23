use crate::witness_certification_engine::{WitnessCertificate, WitnessCertificationEngine};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionFixtureReport {
    pub total_fixtures: usize,
    pub verified_fixtures_count: usize,
    pub shortest_depth_6_fixtures: Vec<Vec<u64>>,
    pub is_discrepancy_resolved: bool,
}

pub struct DiscrepancyRegressionFixturesEngine;

impl DiscrepancyRegressionFixturesEngine {
    /// Document Historical Discrepancy Resolution:
    /// The earlier Phase 7.3S.3D oracle returned zero double-zero matches because it evaluated
    /// shell signatures using un-normalized rational roots rather than normalizing by the 2-adic
    /// odd multiplier Q_j on the centered carry coordinate X = 2673 * n.
    /// Phase 7.3S.3E-B/F corrected this identity (x_{j,\infty} = 2673 * a_{j,\infty}), revealing
    /// the true 25 double-zero witnesses.
    pub fn historical_discrepancy_documentation() -> &'static str {
        "HISTORICAL_DISCREPANCY_RESOLVED: S.3D un-normalized rational root shell evaluation defect corrected."
    }

    /// Verify all 25 regression fixtures
    pub fn verify_all_regression_fixtures() -> (RegressionFixtureReport, Vec<WitnessCertificate>) {
        let stream_report = crate::streaming_falsification_engine::StreamingFalsificationEngine::run_streaming_falsification(7, 8);

        let mut certificates = Vec::new();
        let mut shortest_depth_6_fixtures = Vec::new();

        for lvl in &stream_report.level_reports {
            for (_endpoint_big, word) in &lvl.one_zero_witness_data {
                if let Some(cert) = WitnessCertificationEngine::certify_witness(word) {
                    if cert.depth == 6 {
                        shortest_depth_6_fixtures.push(word.clone());
                    }
                    certificates.push(cert);
                }
            }
        }

        let total_fixtures = certificates.len();
        assert_eq!(total_fixtures, 25, "Must certify exactly 25 regression fixtures");
        assert_eq!(shortest_depth_6_fixtures.len(), 2, "Must have exactly 2 depth-6 regression fixtures");

        let report = RegressionFixtureReport {
            total_fixtures,
            verified_fixtures_count: total_fixtures,
            shortest_depth_6_fixtures,
            is_discrepancy_resolved: true,
        };

        (report, certificates)
    }
}
