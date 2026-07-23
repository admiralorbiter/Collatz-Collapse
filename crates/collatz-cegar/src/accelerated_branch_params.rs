use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

/// Foundational Accelerated Branch Parameter Struct & Generator for Return Gap j >= 0.
#[derive(Debug, Clone)]
pub struct AcceleratedBranchParams {
    pub gap: u64,
    pub precision: u64,
    pub modulus: BigUint,      // M_j
    pub multiplier: BigUint,   // Q_j

    pub t_coordinate_residue: BigUint,   // c_j
    pub t_coordinate_endpoint: BigUint,  // d_j
    pub mu_mod_11: u8,                   // mu_j

    pub z_source_residue: BigUint,      // C_j (canonical z-source lift)
    pub z_endpoint: BigUint,            // D_j (canonical z-endpoint)
    pub affine_intercept: BigInt,       // beta_j
}

impl AcceleratedBranchParams {
    /// Compute exact canonical branch parameters for return gap j >= 0.
    pub fn for_gap(j: u64) -> Self {
        let precision = 9 + 4 * j;
        let modulus = BigUint::one() << precision;

        let q_val = BigUint::from(729u64) * BigUint::from(27u64).pow(j as u32);

        // 1. c_j = 729^{-1} * (81 * 2^{1+4j} * 27^{-j} - 231) mod M_j
        let inv_729 = Self::mod_inverse(&BigUint::from(729u64), &modulus);
        let pow27_j = BigUint::from(27u64).pow(j as u32);
        let inv_27_j = Self::mod_inverse(&pow27_j, &modulus);

        let term1 = (BigUint::from(81u64) << (1 + 4 * j)) * inv_27_j;
        let diff = if term1 >= BigUint::from(231u64) {
            (term1 - BigUint::from(231u64)) % &modulus
        } else {
            let sub = (BigUint::from(231u64) - term1) % &modulus;
            if sub.is_zero() {
                BigUint::zero()
            } else {
                &modulus - sub
            }
        };

        let t_coordinate_residue = (inv_729 * diff) % &modulus;

        // 2. mu_j = (1 - c_j) * M_j^{-1} mod 11
        let m_mod_11 = (&modulus % 11u32).to_u64_digits().first().cloned().unwrap_or(0);
        let inv_m_11 = Self::mod_inverse_u64(m_mod_11, 11);
        let c_mod_11 = (&t_coordinate_residue % 11u32).to_u64_digits().first().cloned().unwrap_or(0);
        let one_minus_c = (11 + 1 - (c_mod_11 % 11)) % 11;
        let mu_mod_11 = ((one_minus_c * inv_m_11) % 11) as u8;

        // 3. C_j = (c_j - 1 + M_j * mu_j) / 11
        let num_c = &t_coordinate_residue + &modulus * BigUint::from(mu_mod_11) - BigUint::one();
        assert_eq!(&num_c % 11u32, BigUint::zero(), "C_j numerator not divisible by 11");
        let z_source_residue = num_c / 11u32;

        // 4. beta_j recurrence: beta_0 = 26, beta_k = 27 * beta_{k-1} + 674 * 16^{k-1}
        let mut affine_intercept = BigInt::from(26i64);
        for k in 1..=j {
            let term_step = BigInt::from(674i64) * BigInt::from(BigUint::one() << (4 * (k - 1)));
            affine_intercept = BigInt::from(27i64) * affine_intercept + term_step;
        }

        // 5. D_j = (Q_j * C_j + beta_j) / M_j
        let q_c = BigInt::from(q_val.clone()) * BigInt::from(z_source_residue.clone());
        let num_d = q_c + &affine_intercept;
        let m_big = BigInt::from(modulus.clone());
        assert_eq!(&num_d % &m_big, BigInt::zero(), "D_j non-integer division");
        let z_endpoint = (num_d / m_big).to_biguint().unwrap();

        // 6. d_j = 11 * D_j + 1 - Q_j * mu_j
        let d_num = BigUint::from(11u64) * &z_endpoint + BigUint::one();
        let q_mu = &q_val * BigUint::from(mu_mod_11);
        assert!(d_num >= q_mu, "d_j underflow");
        let t_coordinate_endpoint = d_num - q_mu;

        let params = AcceleratedBranchParams {
            gap: j,
            precision,
            modulus,
            multiplier: q_val,
            t_coordinate_residue,
            t_coordinate_endpoint,
            mu_mod_11,
            z_source_residue,
            z_endpoint,
            affine_intercept,
        };

        assert!(params.verify_invariants(), "Branch parameter invariants failed for gap {}", j);
        params
    }

    /// Direct original gap return evaluation without re-calling for_gap.
    pub fn direct_original_gap_return(&self, source_c: &BigUint) -> BigUint {
        let num = BigInt::from(self.multiplier.clone()) * BigInt::from(source_c.clone()) + &self.affine_intercept;
        let m_big = BigInt::from(self.modulus.clone());
        assert_eq!(&num % &m_big, BigInt::zero(), "Direct return non-integer");
        (num / m_big).to_biguint().unwrap()
    }

    /// Verify full mandatory semantic freeze invariants:
    /// 1. mu_mod_11 < 11
    /// 2. Q_j * C_j + \beta_j == M_j * D_j
    /// 3. exact_successor_gap(&z_source_residue) == Some(gap)
    /// 4. direct_original_gap_return(&z_source_residue) == z_endpoint
    pub fn verify_invariants(&self) -> bool {
        if self.mu_mod_11 >= 11 {
            return false;
        }

        let lhs = BigInt::from(self.multiplier.clone()) * BigInt::from(self.z_source_residue.clone())
            + &self.affine_intercept;
        let rhs = BigInt::from(self.modulus.clone()) * BigInt::from(self.z_endpoint.clone());
        if lhs != rhs {
            return false;
        }

        if Self::exact_successor_gap(&self.z_source_residue) != Some(self.gap) {
            return false;
        }

        if self.direct_original_gap_return(&self.z_source_residue) != self.z_endpoint {
            return false;
        }

        true
    }

    /// Exact successor gap solver for a candidate source C.
    pub fn exact_successor_gap(source_c: &BigUint) -> Option<u64> {
        let t_val = BigUint::one() + BigUint::from(11u64) * source_c;
        let val_num = BigUint::from(231u64) + BigUint::from(729u64) * &t_val;
        let val2 = Self::count_trailing_zeros(&val_num);
        if val2 > 0 && (val2 - 1).is_multiple_of(4) {
            Some((val2 - 1) / 4)
        } else {
            None
        }
    }

    fn count_trailing_zeros(n: &BigUint) -> u64 {
        if n.is_zero() {
            return 0;
        }
        let mut count = 0u64;
        let digits = n.to_u64_digits();
        for &d in &digits {
            if d == 0 {
                count += 64;
            } else {
                count += d.trailing_zeros() as u64;
                break;
            }
        }
        count
    }

    fn mod_inverse(a: &BigUint, m: &BigUint) -> BigUint {
        let a_int = BigInt::from(a.clone());
        let m_int = BigInt::from(m.clone());
        let (g, x, _) = Self::extended_gcd(&a_int, &m_int);
        assert_eq!(g, BigInt::one(), "Mod inverse does not exist");
        let res = (x % &m_int + &m_int) % &m_int;
        res.to_biguint().unwrap()
    }

    fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
        if a.is_zero() {
            (b.clone(), BigInt::zero(), BigInt::one())
        } else {
            let (g, x1, y1) = Self::extended_gcd(&(b % a), a);
            let x = y1 - (b / a) * &x1;
            let y = x1;
            (g, x, y)
        }
    }

    fn mod_inverse_u64(a: u64, m: u64) -> u64 {
        for x in 1..m {
            if (a * x) % m == 1 {
                return x;
            }
        }
        1
    }
}
