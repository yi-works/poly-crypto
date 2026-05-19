use crate::algebra::field::Field;
use crate::algebra::field::NTTField;
use crate::algebra::ntt::ntt;

pub struct Polynomial<F: Field> {
    pub coeffs: Vec<F>,
}

impl<F: Field> Polynomial<F> {
    pub fn new(coeffs: Vec<F>) -> Self {
        Self { coeffs }
    }

    pub fn evaluate(&self, x: F) -> F {
        self.coeffs // fx = a0 + a1 * x + an * x^n
            .iter() // [a0, a1, an]
            .rev() // [an, a1, a0]
            .fold(F::zero(), |acc, &c| acc.mul(x).add(c)) // acc_new = acc * x + c
    }
}

impl<F: NTTField> Polynomial<F> {
    pub fn mul_ntt(&self, other: &Self) -> Self {
        let mut a = self.coeffs.clone();
        let mut b = other.coeffs.clone();

        let mut n = 1;
        while n < a.len() + b.len() {
            n <<= 1;
        }

        a.resize(n, F::zero());
        b.resize(n, F::zero());

        ntt(&mut a, false);
        ntt(&mut b, false);

        for i in 0..n {
            a[i] = a[i].mul(b[i]);
        }

        ntt(&mut a, true);

        Self::new(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::field::Zp;

    #[test]
    fn test_poly_eval() {
        let p = Polynomial {
            coeffs: vec![Zp::new(1), Zp::new(2)],
        };
        assert_eq!(p.evaluate(Zp::new(3)), Zp::new(7));
    }

    #[test]
    fn test_ntt_mul() {
        use crate::algebra::field::Zp;

        let p1 = Polynomial::new(vec![Zp::new(1), Zp::new(2)]);
        let p2 = Polynomial::new(vec![Zp::new(3), Zp::new(4)]);

        let res = p1.mul_ntt(&p2);

        // (1 + 2x)(3 + 4x) = 3 + 10x + 8x^2
        let expected = [Zp::new(3), Zp::new(10), Zp::new(8)];

        assert_eq!(res.coeffs[..3], expected);
    }
}
