use crate::accelerated_branch_params::AcceleratedBranchParams;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

/// Phase 7.3S.2A.0: Exact Precision-Aware Cylinder [r]_p = { D in Z_2 : D == r (mod 2^p) }.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cylinder {
    pub residue: BigUint,
    pub precision: u32,
}

impl Cylinder {
    pub fn new(residue: BigUint, precision: u32) -> Self {
        let mod_p = BigUint::one() << precision;
        Self {
            residue: residue % mod_p,
            precision,
        }
    }

    /// Exact Forward Cylinder Transformer: Post_j([r]_p) = [D_j + Q_j * t]_{p - B_j}
    pub fn post_j(&self, gap_j: u64) -> Option<Cylinder> {
        let p_j = AcceleratedBranchParams::for_gap(gap_j);
        let b_j = p_j.precision as u32;

        if self.precision < b_j {
            return None;
        }

        let mod_b_j = BigUint::one() << b_j;
        if (&self.residue % &mod_b_j) != (&p_j.z_source_residue % &mod_b_j) {
            return None;
        }

        let t = (&self.residue - &p_j.z_source_residue) >> b_j;
        let succ_num = BigInt::from(p_j.z_endpoint.clone()) + BigInt::from(p_j.multiplier.clone()) * BigInt::from(t);
        let p_succ = self.precision - b_j;
        Some(Cylinder::new(succ_num.to_biguint().unwrap(), p_succ))
    }

    /// Exact Backward Cylinder Transformer: Pre_j([s]_m) = [C_j + M_j * (Q_j^{-1} * (s - D_j) mod 2^m)]_{m + B_j}
    pub fn pre_j(target: &Cylinder, gap_j: u64) -> Cylinder {
        let p_j = AcceleratedBranchParams::for_gap(gap_j);
        let b_j = p_j.precision as u32;
        let m = target.precision;

        let mod_2m = BigUint::one() << m;
        let inv_q = Self::mod_inverse(&p_j.multiplier, &mod_2m);

        let s_int = BigInt::from(target.residue.clone());
        let d_int = BigInt::from(p_j.z_endpoint.clone());
        let diff = (s_int - d_int) % BigInt::from(mod_2m.clone());
        let diff_pos = (diff + BigInt::from(mod_2m.clone())) % BigInt::from(mod_2m.clone());

        let n_rem = (BigInt::from(inv_q) * diff_pos) % BigInt::from(mod_2m);
        let r_pred = BigInt::from(p_j.z_source_residue.clone()) + BigInt::from(p_j.modulus.clone()) * n_rem;
        let p_pred = m + b_j;

        Cylinder::new(r_pred.to_biguint().unwrap(), p_pred)
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
}
