use crate::algebra::field::Zp;
use crate::algebra::polynomial::Polynomial;

pub struct Share {
    pub x: Zp,
    pub y: Zp,
}

pub fn distribute(secret: Zp, n: usize, k: usize) -> Vec<Share> {
    let mut coeffs = vec![secret];
    for i in 1..k {
        coeffs.push(Zp::new(i as i128));
    }

    let poly = Polynomial::new(coeffs);

    (1..=n)
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
        let n = 5;
        let k = 3;

        let shares = distribute(secret, n, k);
        assert_eq!(shares.len(), n);
        assert!(shares[0].y != shares[1].y);
    }
}
