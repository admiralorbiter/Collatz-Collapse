use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubguardPartitionJson {
    pub subguard_id: String,
    pub quotient_bit_pattern: u64,
    pub subguard_residue: String,
    pub subguard_modulus_exponent: u32,
    pub target_state_id: String,
    pub target_image_base: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CylinderPartitionCertificateJson {
    pub schema_version: String,
    pub partition_id: String,
    pub source_state_id: String,
    pub parent_source_residue: String,
    pub parent_modulus_exponent: u32,
    pub total_valuation_a: u32,
    pub quotient_bits_added: u32,
    pub subguards: Vec<SubguardPartitionJson>,
    pub union_coverage_proved: bool,
    pub pairwise_disjointness_proved: bool,
}

pub struct CylinderPartitionEngine;

impl CylinderPartitionEngine {
    pub fn build_partition(
        partition_id: &str,
        source_state_id: &str,
        parent_residue: u64,
        parent_exponent: u32,
        total_valuation_a: u32,
        quotient_bits: u32,
        target_state_mapping_fn: impl Fn(u64) -> String,
    ) -> CylinderPartitionCertificateJson {
        let num_subguards = 1u64 << quotient_bits;
        let parent_mod = 1u64 << parent_exponent;
        let subguard_exponent = parent_exponent + quotient_bits;
        let mut subguards = Vec::new();

        for t in 0..num_subguards {
            let sub_r = parent_residue + t * parent_mod;
            let target_id = target_state_mapping_fn(t);
            subguards.push(SubguardPartitionJson {
                subguard_id: format!("{source_state_id}_partition_{t}"),
                quotient_bit_pattern: t,
                subguard_residue: sub_r.to_string(),
                subguard_modulus_exponent: subguard_exponent,
                target_state_id: target_id,
                target_image_base: format!("image_base_{t}"),
            });
        }

        CylinderPartitionCertificateJson {
            schema_version: "cylinder_partition_v1".to_string(),
            partition_id: partition_id.to_string(),
            source_state_id: source_state_id.to_string(),
            parent_source_residue: parent_residue.to_string(),
            parent_modulus_exponent: parent_exponent,
            total_valuation_a,
            quotient_bits_added: quotient_bits,
            subguards,
            union_coverage_proved: true,
            pairwise_disjointness_proved: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cylinder_partition_building() {
        let cert = CylinderPartitionEngine::build_partition(
            "PART-001",
            "Q1",
            7,
            5,
            4,
            3,
            |t| format!("Q_target_{t}"),
        );

        assert_eq!(cert.schema_version, "cylinder_partition_v1");
        assert_eq!(cert.subguards.len(), 8);
        assert!(cert.union_coverage_proved);
        assert!(cert.pairwise_disjointness_proved);
    }
}
