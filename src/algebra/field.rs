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
impl Zp {
    pub fn new(value: i128) -> Self {
        let v = (value % PRIME + PRIME) % PRIME;
        Zp(v)
    }

    pub fn value(self) -> i128 {
        self.0
    }
}
//
impl Field for Zp {
    fn add(self, rhs: Self) -> Self {
        Zp::new(self.0 + rhs.0)
    }

    fn mul(self, rhs: Self) -> Self {
        Zp::new(self.0 * rhs.0)
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
        Zp::new(u)
    }

    fn zero() -> Self {
        Zp::new(0)
    }

    fn one() -> Self {
        Zp::new(1)
    }
}
