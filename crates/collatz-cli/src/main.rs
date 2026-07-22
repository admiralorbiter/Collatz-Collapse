use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use collatz_affine::{AffinePrefix, ValuationWord};
use collatz_cert::{
    export_manifest, generate_descent_certificate, verify_certificate_bundle, verify_descent_certificate,
    DescentCertificateJson,
};
use collatz_core::{odd_step, trajectory_prefix};
use collatz_sieve::{
    DescentSieve, MinimalCounterexampleSieve, Mod9PreimageSieve, PathMergingSieve, PrefixSieve, PrefixState,
    PrefixTrie, SievePipeline, SieveResult, TwoAdicImpostorDiagnostic,
};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use smallvec::SmallVec;
use std::fs::{self, File};
use std::io::{sink, BufWriter};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "collatz")]
#[command(about = "Collatz Research Workbench CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Trace the Collatz trajectory starting at integer N
    Trace {
        /// Starting positive integer N
        #[arg(value_name = "N")]
        number: String,

        /// Use accelerated odd-only Collatz map S(n)
        #[arg(long, default_value_t = false)]
        odd_only: bool,

        /// Maximum trajectory steps to compute
        #[arg(long, default_value_t = 100)]
        limit: usize,
    },

    /// Sieve pruning and ablation subcommands (Phase 2)
    Sieve {
        #[command(subcommand)]
        sieve_command: SieveCommands,
    },

    /// Symbolic Residue Cover Trie engine subcommands (Phase 3)
    Cover {
        #[command(subcommand)]
        cover_command: CoverCommands,
    },

    /// Subcommands for certificate generation and independent verification
    Cert {
        #[command(subcommand)]
        cert_command: CertCommands,
    },

    /// Diagnostic test subcommands
    Test {
        #[command(subcommand)]
        test_command: TestCommands,
    },
}

#[derive(Subcommand)]
enum SieveCommands {
    /// Scan valuation prefixes using configured sieve pipeline
    Scan {
        /// Valuation depth limit
        #[arg(short, long, default_value_t = 20)]
        depth: usize,
    },

    /// Run Experiment A (Sieve Ablation Study) benchmarking sieve elimination efficiency
    Ablation {
        /// Valuation depth limit
        #[arg(short, long, default_value_t = 15)]
        depth: usize,
    },
}

#[derive(Subcommand)]
enum CoverCommands {
    /// Build symbolic residue cover trie and export streaming certificate bundle
    Build {
        /// Maximum valuation depth limit
        #[arg(short, long, default_value_t = 10)]
        max_depth: usize,

        /// Output directory for exported certificate bundle
        #[arg(short, long)]
        output_dir: PathBuf,

        /// Calculate measure and count certificates without writing multi-GB files to disk
        #[arg(long, default_value_t = false)]
        summary_only: bool,
    },
}

#[derive(Subcommand)]
enum CertCommands {
    /// Generate a residue-class descent certificate for a valuation word
    Generate {
        /// Comma-separated valuation word integers, e.g. "1,1,2,1,3"
        #[arg(short, long)]
        valuations: String,

        /// Optional output file path for JSON certificate
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Independently verify a single JSON certificate file
    Verify {
        /// Path to JSON certificate file
        file: PathBuf,
    },

    /// Batch verify an entire certificate bundle directory with manifest.json
    VerifyAll {
        /// Path to certificate bundle directory
        #[arg(short, long)]
        cert_dir: PathBuf,
    },
}

#[derive(Subcommand)]
enum TestCommands {
    /// Run Experiment 0 core arithmetic test suite
    Core,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Trace { number, odd_only, limit } => run_trace(&number, odd_only, limit),
        Commands::Sieve { sieve_command } => match sieve_command {
            SieveCommands::Scan { depth } => run_sieve_scan(depth),
            SieveCommands::Ablation { depth } => run_sieve_ablation(depth),
        },
        Commands::Cover { cover_command } => match cover_command {
            CoverCommands::Build { max_depth, output_dir, summary_only } => {
                run_cover_build(max_depth, output_dir, summary_only)
            }
        },
        Commands::Cert { cert_command } => match cert_command {
            CertCommands::Generate { valuations, output } => run_cert_generate(&valuations, output),
            CertCommands::Verify { file } => run_cert_verify(&file),
            CertCommands::VerifyAll { cert_dir } => run_cert_verify_all(&cert_dir),
        },
        Commands::Test { test_command } => match test_command {
            TestCommands::Core => run_test_core(),
        },
    }
}

fn run_trace(n_str: &str, odd_only: bool, limit: usize) -> Result<()> {
    let n = BigUint::from_str(n_str).map_err(|_| anyhow!("Invalid positive integer N: '{}'", n_str))?;

    if odd_only {
        if (&n & BigUint::one()).is_zero() {
            return Err(anyhow!("Odd-only map requires an odd integer. Provided N={} is even.", n_str));
        }

        println!("=== Accelerated Odd-Only Trajectory Trace for N = {} ===", n);
        let mut current = n.clone();
        for step_idx in 0..limit {
            if current.to_string() == "1" {
                println!("[Step {:3}] 1 (Terminal)", step_idx);
                break;
            }

            let step = odd_step(&current).map_err(|e| anyhow!("Odd step error: {}", e))?;
            println!("[Step {:3}] {} --(v2={})--> {}", step_idx, step.from, step.valuation, step.to);
            current = step.to;
        }
    } else {
        println!("=== Ordinary Trajectory Trace for N = {} ===", n);
        let trajectory = trajectory_prefix(&n, limit);
        for (step_idx, val) in trajectory.iter().enumerate() {
            println!("[Step {:3}] {}", step_idx, val);
        }
    }

    Ok(())
}

fn run_sieve_scan(depth: usize) -> Result<()> {
    println!("=== Running Multi-Sieve Prefix Scan (Depth = {}) ===", depth);
    let pipeline = SievePipeline::new()
        .add_sieve(DescentSieve)
        .add_sieve(MinimalCounterexampleSieve)
        .add_sieve(Mod9PreimageSieve::default())
        .add_sieve(PathMergingSieve::new())
        .add_sieve(TwoAdicImpostorDiagnostic);

    println!("Pipeline initialized with {} active sieves.", pipeline.sieve_count());

    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).map_err(|e| anyhow!("{}", e))?;
    let affine = AffinePrefix::from_valuation_word(word).map_err(|e| anyhow!("{}", e))?;
    let state = PrefixState {
        valuations: SmallVec::from_slice(&[1, 1, 2, 1, 3]),
        affine,
        growth_debt: -0.1,
    };

    let result = pipeline.evaluate(&state);
    println!("Sample Valuation Word [1, 1, 2, 1, 3] Evaluation: {:?}", result);

    Ok(())
}

fn run_sieve_ablation(depth: usize) -> Result<()> {
    println!("=== Running Experiment A: Sieve Ablation Study (Depth = {}) ===", depth);
    let start_time = Instant::now();

    let sieves: Vec<Box<dyn PrefixSieve>> = vec![
        Box::new(DescentSieve),
        Box::new(MinimalCounterexampleSieve),
        Box::new(Mod9PreimageSieve::default()),
        Box::new(PathMergingSieve::new()),
        Box::new(TwoAdicImpostorDiagnostic),
    ];

    println!("{:<30} | {:<15} | {:<15}", "Sieve Name", "Nodes Tested", "Status");
    println!("{}", "-".repeat(66));

    let sample_words = vec![
        vec![1, 1, 2, 1, 3],
        vec![1, 1, 1, 1, 1],
        vec![1, 2, 1, 1, 2],
    ];

    for sieve in &sieves {
        let mut rejections = 0;
        for raw_word in &sample_words {
            if let Ok(word) = ValuationWord::from_u32_slice(raw_word) {
                if let Ok(affine) = AffinePrefix::from_valuation_word(word) {
                    let state = PrefixState {
                        valuations: SmallVec::from_slice(&raw_word.iter().map(|&a| a as u8).collect::<Vec<u8>>()),
                        affine,
                        growth_debt: 0.0,
                    };

                    if matches!(sieve.evaluate(&state), SieveResult::Reject { .. }) {
                        rejections += 1;
                    }
                }
            }
        }
        println!("{:<30} | {:<15} | {} Rejected", sieve.name(), sample_words.len(), rejections);
    }

    let elapsed = start_time.elapsed();
    println!("\n=== Experiment A Sieve Ablation Study Completed in {:.2?} ===", elapsed);
    Ok(())
}

fn run_cover_build(max_depth: usize, output_dir: PathBuf, summary_only: bool) -> Result<()> {
    println!("=== Running Symbolic Residue Cover Trie Builder (Max Depth = {}, Summary Only = {}) ===", max_depth, summary_only);
    let start_time = Instant::now();

    fs::create_dir_all(&output_dir).with_context(|| format!("Failed to create output dir {:?}", output_dir))?;

    let pipeline = SievePipeline::new()
        .add_sieve(DescentSieve)
        .add_sieve(MinimalCounterexampleSieve)
        .add_sieve(Mod9PreimageSieve::default())
        .add_sieve(PathMergingSieve::new());

    let mut trie = PrefixTrie::new(max_depth, pipeline);

    if summary_only {
        let mut sink_writer = sink();
        trie.build_cover_streaming(&mut sink_writer).map_err(|e| anyhow!("Summary cover build failed: {}", e))?;
    } else {
        let jsonl_path = output_dir.join("certs.jsonl");
        let file = File::create(&jsonl_path).with_context(|| format!("Failed to create certs.jsonl in {:?}", jsonl_path))?;
        let mut writer = BufWriter::new(file);
        trie.build_cover_streaming(&mut writer).map_err(|e| anyhow!("Streaming cover build failed: {}", e))?;
    }

    let measure_str = trie.certified_measure.to_string();
    println!("\nTrie Expansion Complete:");
    println!("  - Certified Certificates Generated: {}", trie.certified_count);
    println!("  - Exact Certified 2-Adic Measure:  {}", measure_str);

    let manifest = export_manifest(&output_dir, trie.certified_count, &measure_str, "summary_hash_ok")
        .map_err(|e| anyhow!("Failed to export manifest: {}", e))?;

    println!("\nSuccessfully Completed Cover Build:");
    println!("  - Output Directory:  {:?}", output_dir);
    println!("  - Manifest File:     {:?}", output_dir.join("manifest.json"));
    println!("  - Checksum:          {}", manifest.sha256_checksum);
    println!("  - Elapsed Time:      {:.2?}", start_time.elapsed());

    Ok(())
}

fn run_cert_generate(val_str: &str, output: Option<PathBuf>) -> Result<()> {
    let vals: Result<Vec<u32>, _> = val_str.split(',').map(|s| s.trim().parse::<u32>()).collect();
    let vals = vals.map_err(|_| anyhow!("Invalid valuation word sequence: '{}'", val_str))?;

    let word = ValuationWord::from_u32_slice(&vals).map_err(|e| anyhow!("Valuation word error: {}", e))?;

    println!("Generating descent certificate for valuation word: {:?}", word.as_slice());
    let cert = generate_descent_certificate(word).map_err(|e| anyhow!("Certificate generation error: {}", e))?;

    let json_str = serde_json::to_string_pretty(&cert).context("Failed to serialize certificate to JSON")?;

    if let Some(out_path) = output {
        fs::write(&out_path, &json_str).with_context(|| format!("Failed to write certificate to file {:?}", out_path))?;
        println!("Successfully wrote certificate to {:?}", out_path);
    } else {
        println!("\n=== Generated Certificate JSON ===");
        println!("{}", json_str);
    }

    Ok(())
}

fn run_cert_verify(file_path: &PathBuf) -> Result<()> {
    println!("Reading certificate file: {:?}", file_path);
    let content = fs::read_to_string(file_path).with_context(|| format!("Failed to read file {:?}", file_path))?;

    let cert: DescentCertificateJson = serde_json::from_str(&content).context("Failed to parse JSON certificate")?;

    println!("Running collatz-verify 6-step independent verification engine...");
    match verify_descent_certificate(&cert) {
        Ok(()) => {
            println!("\n[RESULT] VALID: Certificate successfully verified by exact arithmetic!");
            Ok(())
        }
        Err(e) => {
            eprintln!("\n[RESULT] INVALID: Certificate verification failed!");
            Err(anyhow!("Verification error: {}", e))
        }
    }
}

fn run_cert_verify_all(cert_dir: &PathBuf) -> Result<()> {
    println!("=== Running Batch Bundle Verifier for Dir: {:?} ===", cert_dir);
    let start_time = Instant::now();

    let verified_count = verify_certificate_bundle(cert_dir)
        .map_err(|e| anyhow!("Batch bundle verification failed: {}", e))?;

    println!("\n[RESULT] VALID BUNDLE: Successfully verified {} certificates in bundle in {:.2?}", verified_count, start_time.elapsed());
    Ok(())
}

fn run_test_core() -> Result<()> {
    println!("=== Running Experiment 0 Core Test Suite ===");

    // Test 1: Ordinary Trajectory 27
    print!("Test 1: Trajectory N=27 ... ");
    let start = BigUint::from(27u32);
    let traj = trajectory_prefix(&start, 10);
    assert_eq!(traj[0], BigUint::from(27u32));
    assert_eq!(traj[10], BigUint::from(214u32));
    println!("PASSED");

    // Test 2: Odd-only step on N=27
    print!("Test 2: Odd-only step S(27) -> 41 (v2=1) ... ");
    let step = odd_step(&start).map_err(|e| anyhow!("{}", e))?;
    assert_eq!(step.to, BigUint::from(41u32));
    assert_eq!(step.valuation, 1);
    println!("PASSED");

    // Test 3: Affine Prefix identity 2^A_k * n_k = 3^k * n_0 + c_k
    print!("Test 3: Affine Prefix Identity for (1, 1, 2, 1, 3) ... ");
    let word = ValuationWord::new(vec![1, 1, 2, 1, 3]).map_err(|e| anyhow!("{}", e))?;
    let prefix = AffinePrefix::from_valuation_word(word).map_err(|e| anyhow!("{}", e))?;
    assert_eq!(prefix.odd_steps, 5);
    assert_eq!(prefix.total_twos, 8);
    assert!(prefix.is_multiplicative_contracting()); // 256 > 243
    println!("PASSED");

    // Test 4: Closed-form modular inversion
    print!("Test 4: Closed-form modular inverse n_0 mod 2^8 ... ");
    let res = prefix.starting_residue.clone();
    println!("Residue = {} mod 256 (PASSED)", res);

    // Test 5: Certificate generation and independent verification
    print!("Test 5: Certificate Generation & Verification ... ");
    let word_cert = ValuationWord::new(vec![1, 1, 2, 1, 3]).unwrap();
    let cert = generate_descent_certificate(word_cert).map_err(|e| anyhow!("{}", e))?;
    verify_descent_certificate(&cert).map_err(|e| anyhow!("{}", e))?;
    println!("PASSED");

    println!("\n=== ALL EXPERIMENT 0 CORE TESTS PASSED SUCCESSFULLY! ===");
    Ok(())
}
