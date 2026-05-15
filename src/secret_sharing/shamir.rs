use crate::algebra::field::Zp;
use crate::algebra::polynomial::Polynomial;

pub struct Share {
    pub x: Zp,
    pub y: Zp,
}

pub struct ShamirConfig {
    pub total_shares: usize,
    pub threshold: usize,
}

pub fn distribute(secret: Zp, config: &ShamirConfig) -> Vec<Share> {
    let mut coeffs = vec![secret];
    for i in 1..config.threshold {
        coeffs.push(Zp::new(i as i128));
    }

    let poly = Polynomial::new(coeffs);

    (1..=config.total_shares)
        .map(|i| {
            let x = Zp::new(i as i128);
            Share {
                x,
                y: poly.evaluate(x),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::field::Zp;

    #[test]
    fn test_distribute_basic() {
        let secret = Zp::new(5);
        let config = ShamirConfig {
            total_shares: 5,
            threshold: 3,
        };

        let shares = distribute(secret, &config);
        assert_eq!(shares.len(), config.total_shares);
        assert!(shares[0].y != shares[1].y);
    }
}
