use crate::algebra::field::Field;

pub fn lagrange_at_zero<F: Field>(points: &[(F, F)]) -> F {
    let mut ans = F::zero();

    for i in 0..points.len() {
        let (xi, yi) = points[i];
        let mut num = F::one();
        let mut den = F::one();

        for (j, &(xj, _)) in points.iter().enumerate() {
            if i != j {
                num = num.mul(xj);
                den = den.mul(xj.sub(xi));
            }
        }
        let li = num.mul(den.inv());
        ans = ans.add(yi.mul(li));
    }
    ans
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::field::Zp;
    use crate::secret_sharing::shamir::{distribute, ShamirConfig};

    #[test]
    fn test_lagrange() {
        let points = [(Zp::new(1), Zp::new(5)), (Zp::new(2), Zp::new(7))];
        let result = lagrange_at_zero(&points);
        assert_eq!(result, Zp::new(3));
    }

    #[test]
    fn test_reconstruct() {
        let secret = Zp::new(5);
        let config = ShamirConfig {
            total_shares: 5,
            threshold: 3,
        };
        let shares = distribute(secret, &config);

        let points: Vec<_> = shares.iter().map(|s| (s.x, s.y)).collect();
        let recovered = lagrange_at_zero(&points[..3]);

        assert_eq!(recovered, secret);
    }
}
