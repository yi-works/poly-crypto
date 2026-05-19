use crate::algebra::field::NTTField;

pub fn ntt<F: NTTField>(a: &mut [F], invert: bool) {
    let n = a.len();

    // bit reverse
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            a.swap(i, j);
        }
    }

    let mut len = 2;
    while len <= n {
        let ang = (F::modulus() - 1) / len as i128;
        let mut wlen = F::primitive_root().pow(ang as u64);

        if invert {
            wlen = wlen.inv();
        }

        for i in (0..n).step_by(len) {
            let mut w = F::one();
            for j in 0..len / 2 {
                let u = a[i + j];
                let v = a[i + j + len / 2].mul(w);

                a[i + j] = u.add(v);
                a[i + j + len / 2] = u.sub(v);

                w = w.mul(wlen);
            }
        }

        len <<= 1;
    }

    if invert {
        let inv_n = F::from_i128(n as i128).inv();
        for x in a.iter_mut() {
            *x = x.mul(inv_n);
        }
    }
}
