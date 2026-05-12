use crate::algebra::field::Field;

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
}
