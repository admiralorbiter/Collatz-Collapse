use crate::witness_certification_engine::WitnessCertificationEngine;
use num_bigint::{BigInt, BigUint};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WitnessFamilyRecord {
    pub word: Vec<u64>,
    pub depth: usize,
    pub endpoint_d_u: BigInt,
    pub multiplier_q_u: BigUint,
    pub first_gap_j: u64,
    pub second_gap_k: u64,
    pub first_symbol: u64,
    pub prefix_2: (u64, u64),
    pub suffix_3: Vec<u64>,
    pub exponent_phase_mod64: u64,
}

#[derive(Debug, Clone)]
pub struct WitnessAtlasReport {
    pub total_witnesses: usize,
    pub depth_6_count: usize,
    pub depth_7_count: usize,
    pub distinct_endpoints_count: usize,
    pub depth_6_jk_distribution: HashMap<String, usize>,
    pub depth_7_jk_distribution: HashMap<String, usize>,
    pub ancestral_tree_distribution: HashMap<u64, usize>,
    pub witness_records: Vec<WitnessFamilyRecord>,
}

pub struct WitnessFamilyAtlasEngine;

impl WitnessFamilyAtlasEngine {
    /// Compute exponent phase e_u mod 64 from multiplier Q_u = 3^{e_u}
    pub fn compute_exponent_phase_mod64(q_u: &BigUint) -> u64 {
        // 3^e mod 64 has period 16
        // Count power of 3
        let mod64 = BigUint::from(64u64);
        let q_mod64 = (q_u % &mod64).to_u64_digits().first().cloned().unwrap_or(0);

        for e in 0..64u64 {
            let p3 = BigUint::from(3u64).pow(e as u32);
            let p3_mod64 = (&p3 % &mod64).to_u64_digits().first().cloned().unwrap_or(0);
            if p3_mod64 == q_mod64 {
                return e % 64;
            }
        }
        0
    }

    /// Build structural atlas for all certified 25 witnesses
    pub fn build_witness_atlas() -> WitnessAtlasReport {
        let stream_report = crate::streaming_falsification_engine::StreamingFalsificationEngine::run_streaming_falsification(7, 8);

        let mut witness_records = Vec::new();
        let mut depth_6_count = 0;
        let mut depth_7_count = 0;
        let mut distinct_endpoints = std::collections::HashSet::new();

        let mut depth_6_jk_distribution = HashMap::new();
        let mut depth_7_jk_distribution = HashMap::new();
        let mut ancestral_tree_distribution = HashMap::new();

        for lvl in &stream_report.level_reports {
            for (_endpoint_big, word) in &lvl.one_zero_witness_data {
                if let Some(cert) = WitnessCertificationEngine::certify_witness(word) {
                    distinct_endpoints.insert(cert.endpoint_d_u.clone());

                    if cert.depth == 6 {
                        depth_6_count += 1;
                        let key = format!("(j={}, k={})", cert.first_gap_j, cert.second_gap_k);
                        *depth_6_jk_distribution.entry(key).or_insert(0) += 1;
                    } else if cert.depth == 7 {
                        depth_7_count += 1;
                        let key = format!("(j={}, k={})", cert.first_gap_j, cert.second_gap_k);
                        *depth_7_jk_distribution.entry(key).or_insert(0) += 1;
                    }

                    let first_symbol = word[0];
                    *ancestral_tree_distribution.entry(first_symbol).or_insert(0) += 1;

                    let prefix_2 = if word.len() >= 2 { (word[0], word[1]) } else { (word[0], 0) };
                    let suffix_3 = if word.len() >= 3 {
                        word[word.len() - 3..].to_vec()
                    } else {
                        word.clone()
                    };

                    let exp_phase = Self::compute_exponent_phase_mod64(&cert.multiplier_q_u);

                    witness_records.push(WitnessFamilyRecord {
                        word: word.clone(),
                        depth: cert.depth,
                        endpoint_d_u: cert.endpoint_d_u,
                        multiplier_q_u: cert.multiplier_q_u,
                        first_gap_j: cert.first_gap_j,
                        second_gap_k: cert.second_gap_k,
                        first_symbol,
                        prefix_2,
                        suffix_3,
                        exponent_phase_mod64: exp_phase,
                    });
                }
            }
        }

        WitnessAtlasReport {
            total_witnesses: witness_records.len(),
            depth_6_count,
            depth_7_count,
            distinct_endpoints_count: distinct_endpoints.len(),
            depth_6_jk_distribution,
            depth_7_jk_distribution,
            ancestral_tree_distribution,
            witness_records,
        }
    }
}
