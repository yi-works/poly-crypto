use poly_crypto::algebra::field::Zp;
use poly_crypto::algebra::polynomial::Polynomial;
use std::time::Instant;

#[test]
fn benchmark_ntt_vs_naive() {
    let sizes = [64, 128, 256, 512, 1024];
    for &n in &sizes {
        println!("n = {}", n);
        let p1 = Polynomial::new((0..n).map(|i| Zp::new(i as i128)).collect());
        let p2 = Polynomial::new((0..n).map(|i| Zp::new((i * 2) as i128)).collect());

        let start = Instant::now();
        let _ = p1.mul_naive(&p2);
        println!("naive: {:?}", start.elapsed());

        let start = Instant::now();
        let _ = p2.mul_ntt(&p1);
        println!("ntt: {:?}", start.elapsed());
    }
}
