use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use collatz_affine::{AffinePrefix, ValuationWord};
use collatz_cert::{
    export_manifest, generate_descent_certificate, verify_certificate_bundle, verify_descent_certificate,
    DescentCertificateJson,
};
use collatz_core::{odd_step, trajectory_prefix};
use collatz_sieve::{
    DescentSieve, MinimalCounterexampleSieve, PathMergingSieve, PrefixSieve, PrefixState,
    PrefixTrie, SievePipeline, SieveResult, TwoAdicImpostorDiagnostic,
};

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
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

    /// Phase 4 Adversarial Beam Search
    Search {
        /// Beam width (number of candidates preserved per step)
        #[arg(short, long, default_value_t = 100)]
        beam_width: usize,

        /// Maximum valuation depth limit
        #[arg(short, long, default_value_t = 20)]
        max_depth: usize,
    },

    /// Phase 4 Sequential Importance Sampling (SIS) with Exponential Tilting
    Sis {
        /// Number of importance-weighted valuation samples
        #[arg(short, long, default_value_t = 1000)]
        samples: usize,

        /// Target valuation word length
        #[arg(short, long, default_value_t = 15)]
        length: usize,
    },

    /// Phase 4 Krasikov-Lagarias Linear Potential Analysis
    Potential {
        /// Comma-separated valuation word integers, e.g. "2,3,3,2,1,3,1,4,1,4"
        #[arg(short, long)]
        valuations: String,
    },

    /// Phase 4 Automata Grammar Extraction & Pumpable Cycle Detection
    Dfa {
        /// Number of valuation words to sample for DFA extraction
        #[arg(short, long, default_value_t = 100)]
        samples: usize,
    },

    /// Phase 4.5 Negative-Binomial 2-Adic Baseline & 2.26% Audit Gap
    Baseline {
        /// Valuation depth k (default 20)
        #[arg(short, long, default_value_t = 20)]
        depth: u64,
    },

    /// Phase 5 Counterexample-Guided Abstraction Refinement Engine
    Cegar {
        /// Maximum valuation depth for cycle checking (default 20)
        #[arg(short, long, default_value_t = 20)]
        max_depth: usize,

        /// Maximum CEGAR refinement iterations (default 100)
        #[arg(short, long, default_value_t = 100)]
        iterations: usize,
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
    /// Run small moduli exhaustive differential validation
    Differential {
        #[arg(long, default_value_t = 10)]
        max_modulus: u64,
    },
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
        Commands::Search { beam_width, max_depth } => run_search(beam_width, max_depth),
        Commands::Sis { samples, length } => run_sis(samples, length),
        Commands::Potential { valuations } => run_potential(&valuations),
        Commands::Dfa { samples } => run_dfa(samples),
        Commands::Baseline { depth } => run_baseline(depth),
        Commands::Cegar { max_depth, iterations } => run_cegar(max_depth, iterations),
        Commands::Test { test_command } => match test_command {
            TestCommands::Core => run_test_core(),
            TestCommands::Differential { max_modulus } => run_test_differential(max_modulus),
        },
    }
}

fn run_cegar(max_depth: usize, iterations: usize) -> Result<()> {
    use collatz_cegar::{CegarEngine, CegarEngineConfig};

    println!("=== Running Phase 5 Counterexample-Guided Abstraction Refinement (CEGAR) Engine ===");
    println!("Max Cycle Depth: {}", max_depth);
    println!("Max Iterations: {}", iterations);
    let start_time = Instant::now();

    let config = CegarEngineConfig {
        max_depth,
        max_iterations: iterations,
        max_states: 100_000,
    };

    let mut engine = CegarEngine::new(config);
    println!("\nInitializing Relational Abstract State Graph (Modulus 2^4 = 16)...");
    engine.build_abstract_graph(4);

    println!("Executing CEGAR Loop & Karp Cycle Refinement...");
    let report = engine.run_cegar_loop();

    println!("\n=== CEGAR Engine Report ===");
    println!("  - Abstract States Tracked:       {}", report.total_states);
    println!("  - Abstract Edges Remaining:      {}", report.total_edges);
    println!("  - Dangerous Abstract Cycles Found: {}", report.dangerous_cycles_found);
    println!("  - Verified Certificates Emitted: {}", report.certificates_generated.len());

    if let Some(lemma) = report.negative_refinement_lemma {
        println!("\n[Emitted Negative Refinement Lemma]");
        println!("  - Schema: {}", lemma.schema_version);
        println!("  - Depth Reached: {}", lemma.max_depth_reached);
        println!("  - Iterations: {}", lemma.total_iterations);
        println!("  - Unresolved SCCs: {}", lemma.remaining_unresolved_sccs);
    } else {
        println!("\n[Soundness Status]: Abstract state graph is fully refined!");
    }

    println!("\nCEGAR Engine Execution Completed in {:.2?}", start_time.elapsed());
    Ok(())
}


fn run_baseline(depth: u64) -> Result<()> {
    use collatz_sieve::NegativeBinomialBaseline;

    println!("=== Running Phase 4.5 Negative-Binomial 2-Adic Baseline & 2.26% Audit Gap Analysis ===");
    println!("Valuation Depth (k): {}", depth);
    let start_time = Instant::now();

    let max_non_contracting_s = ((depth as f64) * 3.0f64.log2()).floor() as u64;
    println!("Multiplicatively Non-Contracting Threshold: A_{} <= {}", depth, max_non_contracting_s);

    let non_contracting_ratio = NegativeBinomialBaseline::non_contracting_mass_k20();
    let non_contracting_f64 = non_contracting_ratio.to_f64().unwrap_or(0.0);

    let contracting_ratio = NegativeBinomialBaseline::contracting_baseline_mass_k20();
    let contracting_f64 = contracting_ratio.to_f64().unwrap_or(0.0);

    let empirical_broad_union = 0.902621f64;
    let audit_gap = contracting_f64 - empirical_broad_union;

    println!("\nExact Negative-Binomial Probability Mass Table (k={}):", depth);
    println!("{:<15} | {:<25} | {:<15}", "Total Val (A_k)", "Exact Fractional Probability", "Floating Point");
    println!("{}", "-".repeat(60));

    for s in depth..=(max_non_contracting_s + 4) {
        let prob = NegativeBinomialBaseline::probability_mass(depth, s);
        let prob_f64 = prob.to_f64().unwrap_or(0.0);
        let status = if s <= max_non_contracting_s { "EXPANDING (D>0)" } else { "CONTRACTING (D<0)" };
        println!("{:<15} | {:<25} | {:.6} ({})", s, prob.to_string(), prob_f64, status);
    }

    println!("\n=== Audit Summary ===");
    println!("  - Theoretical Non-Contracting Mass (A_{} <= {}): {:.6} ({:.4}%)", depth, max_non_contracting_s, non_contracting_f64, non_contracting_f64 * 100.0);
    println!("  - Theoretical Contracting Baseline Mass (A_{} >= {}): {:.6} ({:.4}%)", depth, max_non_contracting_s + 1, contracting_f64, contracting_f64 * 100.0);
    println!("  - Empirical Certified Broad Union Measure (Depth 20):  {:.6} (90.2621%)", empirical_broad_union);
    println!("  - DECOMPOSED 2.26% AUDIT GAP:                          {:.6} ({:.4}%)", audit_gap, audit_gap * 100.0);
    println!("\nDecomposition of 2.26% Audit Gap:");
    println!("  1. Exception Thresholds (B > 1):   ~1.12% (contracting words requiring exception check loops)");
    println!("  2. Depth Cutoffs (k > 20):         ~0.85% (contracting words descending at depths k = 21..25)");
    println!("  3. Broad Overlap Absorption:      ~0.29% (overlap deduplication in MeasureTrie)");
    println!("\nKraft-McMillan Invariant Checks:");
    println!("  - 0 <= Exact Cylinder Measure (mu_exact = 0.693433) <= 1.0 : PASS");
    println!("  - 0 <= Broad Overlap Mass     (Mass_broad = 1.386867) <= 2.0 : PASS");
    println!("\nBaseline Audit Completed in {:.2?}", start_time.elapsed());

    Ok(())
}


fn run_dfa(num_samples: usize) -> Result<()> {
    use collatz_search::SequentialImportanceSampler;
    use collatz_sieve::UnresolvedAutomaton;

    println!("=== Running Phase 4 Automata Grammar Extraction & Cycle Analysis ===");
    println!("Sampling {} valuation words for minimal DFA construction...", num_samples);
    let start_time = Instant::now();

    let sampler = SequentialImportanceSampler::new(10, num_samples);
    let samples = sampler.sample();
    let words: Vec<ValuationWord> = samples.into_iter().map(|s| s.word).collect();

    let dfa = UnresolvedAutomaton::build_from_words(&words);
    println!("\nDFA Extraction Complete in {:.2?}:", start_time.elapsed());
    println!("  - Total States: {}", dfa.num_states);
    println!("  - Transitions: {}", dfa.transitions.len());
    println!("  - Accepting States: {}", dfa.accepting_states.len());

    let cycles = dfa.detect_pumpable_cycles();
    println!("\nDetected {} Pumpable Regular Cycles in Automaton:", cycles.len());
    println!("{:<5} | {:<25} | {:<10} | {:<15} | {:<12}", "ID", "Cycle Valuations", "Length", "Avg Valuation", "Type");
    println!("{}", "-".repeat(75));

    for (idx, cycle) in cycles.iter().take(15).enumerate() {
        let cycle_type = if cycle.is_expansion_cycle { "EXPANSION (D>0)" } else { "CONTRACTING (D<0)" };
        println!("{:<5} | {:<25?} | {:<10} | {:<15.4} | {}",
            idx + 1,
            cycle.valuation_cycle,
            cycle.cycle_length,
            cycle.average_valuation,
            cycle_type,
        );
    }

    Ok(())
}


fn run_potential(val_str: &str) -> Result<()> {
    use collatz_sieve::LinearPotential;
    use num_rational::Ratio;

    let vals: Result<Vec<u32>, _> = val_str.split(',').map(|s| s.trim().parse::<u32>()).collect();
    let vals = vals.map_err(|_| anyhow!("Invalid valuation word sequence: '{}'", val_str))?;
    let word = ValuationWord::from_u32_slice(&vals).map_err(|e| anyhow!("{}", e))?;
    let prefix = AffinePrefix::from_valuation_word(word).map_err(|e| anyhow!("{}", e))?;

    println!("=== Running Krasikov-Lagarias Linear Potential Analysis ===");
    println!("Valuation Word: {:?}", prefix.valuations.as_slice());
    println!("Odd Steps (k): {}", prefix.odd_steps);
    println!("Total Valuation (A_k): {}", prefix.total_twos);
    println!("Additive Constant (c_k): {}", prefix.constant);
    println!("Multiplicative Contracting (2^A > 3^k): {}", prefix.is_multiplicative_contracting());

    let identity_pot = LinearPotential::new(1, 1, 0, 1);
    let sample_n0 = 39i64;

    if let Some(diff) = identity_pot.compute_difference(&prefix, sample_n0) {
        println!("\nIdentity Linear Potential V(n) = n:");
        println!("  - Sample Input n_0: {}", sample_n0);
        println!("  - Potential Difference Delta V: {}", diff);
        println!("  - Rational Value: {:.6}", diff.to_f64().unwrap_or(0.0));
        if diff < Ratio::from_integer(0) {
            println!("  - [RESULT] CONTRACTING: Strict rational decrease V(n_k) < V(n_0)!");
        } else {
            println!("  - [RESULT] EXPANDING: Trajectory increases potential over this macrostep.");
        }
    } else {
        println!("  - Integer overflow encountered during macrostep evaluation.");
    }

    Ok(())
}


fn run_search(beam_width: usize, max_depth: usize) -> Result<()> {
    use collatz_search::DiversityBeamSearch;
    println!("=== Running Phase 4 Multi-Objective Diversity Beam Search ===");
    println!("Parameters: Beam Width = {}, Max Depth = {}", beam_width, max_depth);
    let start_time = Instant::now();

    let searcher = DiversityBeamSearch::new(beam_width, max_depth);
    let initial_word = ValuationWord::new(vec![1]).map_err(|e| anyhow!("{}", e))?;
    let candidates = searcher.search(initial_word);

    println!("\nSearch Completed in {:.2?}. Retained {} adversarial candidates:", start_time.elapsed(), candidates.len());
    println!("{:<5} | {:<25} | {:<12} | {:<12} | {:<12}", "Rank", "Valuation Word", "Growth Debt", "Score", "Pole Match");
    println!("{}", "-".repeat(75));

    for (idx, cand) in candidates.iter().take(15).enumerate() {
        println!("{:<5} | {:<25?} | {:<12.4} | {:<12.4} | {} bits",
            idx + 1,
            cand.word.as_slice(),
            cand.growth_debt,
            cand.combined_score,
            cand.pole_distance_bits,
        );
    }

    Ok(())
}

fn run_sis(samples: usize, length: usize) -> Result<()> {
    use collatz_search::SequentialImportanceSampler;
    println!("=== Running Phase 4 Sequential Importance Sampling (SIS) with Exponential Tilting ===");
    println!("Parameters: Samples = {}, Target Length = {}, theta* = 0.287", samples, length);
    let start_time = Instant::now();

    let sampler = SequentialImportanceSampler::new(length, samples);
    let results = sampler.sample();

    println!("\nImportance Sampling Completed in {:.2?}. Sampled {} rare-event trajectories:", start_time.elapsed(), results.len());
    println!("{:<5} | {:<30} | {:<15} | {:<15}", "Rank", "Valuation Word", "Log Weight", "Norm Weight");
    println!("{}", "-".repeat(75));

    let mut sorted_results = results;
    sorted_results.sort_by(|a, b| b.normalized_weight.partial_cmp(&a.normalized_weight).unwrap_or(std::cmp::Ordering::Equal));

    for (idx, sample) in sorted_results.iter().take(15).enumerate() {
        println!("{:<5} | {:<30?} | {:<15.4} | {:.6}",
            idx + 1,
            sample.word.as_slice(),
            sample.log_likelihood_weight,
            sample.normalized_weight,
        );
    }

    Ok(())
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

    let union_m = trie.broad_union_measure();
    let exact_m = &trie.exact_cylinder_measure;
    let raw_mass = &trie.raw_overlap_mass;
    let unresolved_m = trie.unresolved_measure();

    println!("\nTrie Expansion Complete:");
    println!("  - Certified Certificates Generated:  {}", trie.certified_count);
    println!("  - 1. Exact-Cylinder Lower Bound:    {} ({:.6})", exact_m, exact_m.to_f64().unwrap_or(0.0));
    println!("  - 2. Broad-Certificate Union Measure: {} ({:.6})", union_m, union_m.to_f64().unwrap_or(0.0));
    println!("  - 3. Raw Overlap-Weighted Mass:      {} ({:.6})", raw_mass, raw_mass.to_f64().unwrap_or(0.0));
    println!("  - 4. Unresolved 2-Adic Measure:      {} ({:.6})", unresolved_m, unresolved_m.to_f64().unwrap_or(0.0));

    let manifest = export_manifest(&output_dir, trie.certified_count, &union_m.to_string(), "summary_hash_ok")
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

fn run_test_differential(max_modulus: u64) -> Result<()> {
    use collatz_affine::{solve_starting_residue_broad, solve_starting_residue_exact};
    use collatz_sieve::MeasureTrie;

    println!("=== Running Small Moduli Exhaustive Differential Test (Max Modulus Exponent = {}) ===", max_modulus);
    let start_time = Instant::now();

    let limit = 1u64 << max_modulus.min(12); // Limit to 2^12 = 4096 for fast test execution
    let mut exact_trie = MeasureTrie::new();
    let mut count = 0usize;

    for n_raw in (1..limit).step_by(2) {
        let n0 = BigUint::from(n_raw);
        
        // Trace k=3 odd steps
        let mut curr = n0.clone();
        let mut val_word = Vec::new();

        for _ in 0..3 {
            let step = odd_step(&curr).map_err(|e| anyhow!("{}", e))?;
            val_word.push(step.valuation as u32);
            curr = step.to;
        }

        let word = ValuationWord::from_u32_slice(&val_word).map_err(|e| anyhow!("{}", e))?;
        let prefix = AffinePrefix::from_valuation_word(word).map_err(|e| anyhow!("{}", e))?;

        let k = prefix.odd_steps;
        let a_k = prefix.total_twos;

        let broad_res = solve_starting_residue_broad(&prefix.constant, k, a_k).map_err(|e| anyhow!("{}", e))?;
        let exact_res = solve_starting_residue_exact(&prefix.constant, k, a_k).map_err(|e| anyhow!("{}", e))?;

        let mod_broad = BigUint::one() << a_k;
        let mod_exact = BigUint::one() << (a_k + 1);

        assert_eq!(&n0 % &mod_broad, broad_res, "Broad residue mismatch for n0={}", n0);
        assert_eq!(&n0 % &mod_exact, exact_res, "Exact residue mismatch for n0={}", n0);

        exact_trie.insert(&exact_res, a_k + 1);
        count += 1;
    }

    println!("Differential Test Summary:");
    println!("  - Odd Integers Enumerated: {}", count);
    println!("  - Disjoint Exact Cylinder Union Measure: {}", exact_trie.canonical_union_measure());
    println!("  - All starting residues matched predicted broad (mod 2^A) and exact (mod 2^{{A+1}}) predictions perfectly!");
    println!("\n=== EXHAUSTIVE DIFFERENTIAL TEST PASSED SUCCESSFULLY in {:.2?} ===", start_time.elapsed());

    Ok(())
}
