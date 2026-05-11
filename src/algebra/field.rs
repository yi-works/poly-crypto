#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_field_properties() {
        let a = Zp(100);
        let b = Zp(PRIME - 1);
        // 交換法則
        assert_eq!(a.add(b), b.add(a));

        // 逆元
        let a_inv = a.inv();
        assert_eq!(a.mul(a_inv), Zp::one());

        // 剰余の性質
        assert_eq!(b.add(Zp::one()), Zp::zero());
    }

    #[test]
    fn test_zero_properties() {
        let a = Zp(100);
        assert_eq!(a.add(Zp::zero()), a);
        assert_eq!(a.mul(Zp::zero()), Zp::zero());
    }
}

pub trait Field: Sized + Copy + PartialEq {
    fn add(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn inv(self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}

pub const PRIME: i128 = 2147483647;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zp(pub i128);

impl Field for Zp {
    fn add(self, rhs: Self) -> Self {
        Zp((self.0 + rhs.0) % PRIME)
    }
    fn mul(self, rhs: Self) -> Self {
        Zp((self.0 * rhs.0) % PRIME)
    }
    fn inv(self) -> Self {
        //  Euclidean Algorithm
        let (mut a, mut b) = (self.0, PRIME);
        let (mut u, mut v) = (1, 0);
        while b != 0 {
            let t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);
            u -= t * v;
            std::mem::swap(&mut u, &mut v);
        }
        Zp((u + PRIME) % PRIME)
    }
    fn zero() -> Self {
        Zp(0)
    }
    fn one() -> Self {
        Zp(1)
    }
}
