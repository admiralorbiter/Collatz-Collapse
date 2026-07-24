use num_bigint::BigUint;
use num_traits::{One, Zero};

#[derive(Debug, Clone)]
pub struct ZeroLiftSearchBounds {
    pub max_start_value: BigUint,
    pub max_return_steps: usize,
    pub max_word_length: usize,
    pub max_exponent_sum: usize,
    pub max_depth: usize,
    pub target_run_length: usize,
}

#[derive(Debug, Clone)]
pub struct ZeroLiftStep {
    pub index: usize,
    pub word: Vec<usize>,
    pub exponent_sum: usize,
    pub source_class: u8,
    pub destination_class: u8,
    pub lift_digit: BigUint,
    pub endpoint: BigUint,
    pub minimum_odd_state: BigUint,
}

#[derive(Debug, Clone)]
pub struct ZeroLiftRunTrace {
    pub anchor: BigUint,
    pub start_index: usize,
    pub steps: Vec<ZeroLiftStep>,
    pub precision_at_start: usize,
    pub no_descent: bool,
    pub terminated_within_horizon: bool,
}

#[derive(Debug, Clone)]
pub struct CounterexampleSearchTrace {
    pub anchor: BigUint,
    pub q1_source: BigUint,
    pub entry_time: usize,
    pub minimum_odd_state: BigUint,
    pub survived_horizon: bool,
    pub no_descent_below_anchor: bool,
    pub return_words: Vec<Vec<usize>>,
    pub endpoint_residue3: BigUint,
    pub endpoint_modulus3: BigUint,
}

pub fn syracuse_step(n: &BigUint) -> (BigUint, usize) {
    if n.is_zero() || n % 2u32 == BigUint::zero() {
        return (BigUint::zero(), 0);
    }
    let mut next = n * 3u32 + 1u32;
    let mut val = 0;
    while &next % 2u32 == BigUint::zero() {
        next /= 2u32;
        val += 1;
    }
    (next, val)
}

pub fn search_orbit_first_zero_lift_runs(bounds: &ZeroLiftSearchBounds) -> Vec<ZeroLiftRunTrace> {
    let mut traces = Vec::new();
    let mut current = BigUint::from(7u32);
    let step_32 = BigUint::from(32u32);

    while current <= bounds.max_start_value {
        let mut curr_val = current.clone();
        let mut min_state = curr_val.clone();
        let mut steps = Vec::new();
        let mut no_descent = true;
        let mut terminated = false;

        for depth in 0..bounds.max_depth {
            let mut word = Vec::new();
            let mut exp_sum = 0;
            let start_state = curr_val.clone();

            for _ in 0..bounds.max_return_steps {
                let (next, val) = syracuse_step(&curr_val);
                if next < min_state {
                    no_descent = false;
                    min_state = next.clone();
                }
                word.push(val);
                exp_sum += val;
                curr_val = next.clone();
                if &curr_val == &BigUint::one() {
                    terminated = true;
                    break;
                }
                if &curr_val % 32u32 == BigUint::from(7u32) {
                    break;
                }
            }

            let source_cls = ((&start_state / 32u32) % 16u32).to_u64_digits().first().cloned().unwrap_or(0) as u8;
            let dest_cls = ((&curr_val / 32u32) % 16u32).to_u64_digits().first().cloned().unwrap_or(0) as u8;

            steps.push(ZeroLiftStep {
                index: depth,
                word,
                exponent_sum: exp_sum,
                source_class: source_cls,
                destination_class: dest_cls,
                lift_digit: BigUint::zero(),
                endpoint: curr_val.clone(),
                minimum_odd_state: min_state.clone(),
            });

            if terminated {
                break;
            }
        }

        if steps.len() >= bounds.target_run_length {
            traces.push(ZeroLiftRunTrace {
                anchor: current.clone(),
                start_index: 0,
                steps,
                precision_at_start: 5,
                no_descent,
                terminated_within_horizon: terminated,
            });
        }

        current += &step_32;
    }

    traces
}

pub fn search_counterexample_q1_traces(bounds: &ZeroLiftSearchBounds) -> Vec<CounterexampleSearchTrace> {
    let mut traces = Vec::new();
    let runs = search_orbit_first_zero_lift_runs(bounds);

    for run in runs {
        let min_odd = run.steps.iter().map(|s| s.minimum_odd_state.clone()).min().unwrap_or(run.anchor.clone());
        let no_descent_anchor = min_odd >= run.anchor;
        let return_words = run.steps.iter().map(|s| s.word.clone()).collect();
        let last_endpoint = run.steps.last().map(|s| s.endpoint.clone()).unwrap_or(BigUint::zero());

        let total_time: usize = run.steps.iter().map(|s| s.word.len()).sum();
        let modulus3 = BigUint::from(3u32).pow(total_time as u32);
        let res3 = &last_endpoint % &modulus3;

        traces.push(CounterexampleSearchTrace {
            anchor: run.anchor.clone(),
            q1_source: run.anchor.clone(),
            entry_time: 0,
            minimum_odd_state: min_odd,
            survived_horizon: !run.terminated_within_horizon,
            no_descent_below_anchor: no_descent_anchor,
            return_words,
            endpoint_residue3: res3,
            endpoint_modulus3: modulus3,
        });
    }

    traces
}
