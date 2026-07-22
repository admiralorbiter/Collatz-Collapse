use crate::schema::DescentCertificateJson;
use crate::verify::verify_descent_certificate;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleManifest {
    pub schema_version: String,
    pub certificate_count: usize,
    pub total_2adic_measure: String,
    pub sha256_checksum: String,
    pub cert_file: String,
}

pub fn export_manifest(
    output_dir: &Path,
    count: usize,
    measure_str: &str,
    checksum_str: &str,
) -> Result<BundleManifest, String> {
    let manifest = BundleManifest {
        schema_version: "bundle_v1".to_string(),
        certificate_count: count,
        total_2adic_measure: measure_str.to_string(),
        sha256_checksum: checksum_str.to_string(),
        cert_file: "certs.jsonl".to_string(),
    };

    let manifest_path = output_dir.join("manifest.json");
    let manifest_json = serde_json::to_string_pretty(&manifest).map_err(|e| format!("Manifest serialize error: {}", e))?;
    fs::write(&manifest_path, manifest_json).map_err(|e| format!("Manifest write error: {}", e))?;

    Ok(manifest)
}

pub fn export_certificate_bundle(
    output_dir: &Path,
    certificates: &[DescentCertificateJson],
    measure_str: &str,
) -> Result<BundleManifest, String> {
    fs::create_dir_all(output_dir).map_err(|e| format!("Failed to create output dir: {}", e))?;

    let jsonl_path = output_dir.join("certs.jsonl");
    let mut file = fs::File::create(&jsonl_path).map_err(|e| format!("Failed to create certs.jsonl: {}", e))?;

    let mut byte_contents = Vec::new();
    for cert in certificates {
        let line = serde_json::to_string(cert).map_err(|e| format!("Serialization error: {}", e))?;
        writeln!(file, "{}", line).map_err(|e| format!("Write error: {}", e))?;
        byte_contents.extend_from_slice(line.as_bytes());
        byte_contents.push(b'\n');
    }

    let mut hash_accumulator: u64 = 0xCBF2_9CE4_8422_2325;
    for &b in &byte_contents {
        hash_accumulator ^= b as u64;
        hash_accumulator = hash_accumulator.wrapping_mul(0x0000_0100_0000_01B3);
    }
    let checksum_str = format!("{:016x}", hash_accumulator);

    export_manifest(output_dir, certificates.len(), measure_str, &checksum_str)
}

pub fn verify_certificate_bundle(bundle_dir: &Path) -> Result<usize, String> {
    let manifest_path = bundle_dir.join("manifest.json");
    if !manifest_path.exists() {
        return Err(format!("Manifest file not found: {:?}", manifest_path));
    }

    let manifest_str = fs::read_to_string(&manifest_path).map_err(|e| format!("Failed to read manifest: {}", e))?;
    let manifest: BundleManifest = serde_json::from_str(&manifest_str).map_err(|e| format!("Manifest parse error: {}", e))?;

    let jsonl_path = bundle_dir.join(&manifest.cert_file);
    let file = fs::File::open(&jsonl_path).map_err(|e| format!("Failed to open certs.jsonl: {}", e))?;
    let reader = BufReader::new(file);

    let mut verified_count = 0;
    for (line_num, line_res) in reader.lines().enumerate() {
        let line = line_res.map_err(|e| format!("Read error at line {}: {}", line_num + 1, e))?;
        if line.trim().is_empty() {
            continue;
        }

        let cert: DescentCertificateJson = serde_json::from_str(&line)
            .map_err(|e| format!("Deserialization error at line {}: {}", line_num + 1, e))?;

        verify_descent_certificate(&cert)
            .map_err(|e| format!("Verification failed at line {}: {}", line_num + 1, e))?;

        verified_count += 1;
    }

    if verified_count != manifest.certificate_count {
        return Err(format!(
            "Certificate count mismatch: manifest expected {}, verified {}",
            manifest.certificate_count, verified_count
        ));
    }

    Ok(verified_count)
}
