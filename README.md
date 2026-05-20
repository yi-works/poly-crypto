# poly-crypto
A Rust library for Finite Field arithmetic, Shamir's Secret Sharing, and Number Theoretic Transform (NTT).
> **Note**: This project is a personal learning project.

## Motivation
- Rustによる有限体演算、秘密分散法、数論変換(NTT)を実装し、数学的背景を理解する

## Benchmark

多項式の畳み込みにおいて、ナイーブ実装（O(n²)）と  
NTT実装（O(n log n)）の性能比較を行った。

### Results

| n    | Naive (O(n²)) | NTT (O(n log n)) |
|------|------------------|------------------|
| 64   | 585.8 µs         | 342.8 µs         |
| 128  | 2.53 ms          | 0.87 ms          |
| 256  | 9.78 ms          | 1.41 ms          |
| 512  | 40.33 ms         | 2.96 ms          |
| 1024 | 150.69 ms        | 6.26 ms          |

### Analysis

- 小さいサイズでもNTTはナイーブ実装より高速であることが確認できた
- 入力サイズが大きくなるほど差が広がり、計算量の違いが顕著に現れる
- ナイーブ法はO(n²)で増加するのに対し、NTTはO(n log n)のため
  大規模入力で大きな性能差を生む


## Insights
- FFTもNTTも「原始N乗根」を使うが、
FFTでは複素数上で常に存在するのに対し、
NTTでは有限体上で条件付きでしか存在しないのが興味深い

- FFTは複素数の連続的な周期性を利用するが、浮動小数誤差が避けられない

- NTTは有限体の巡回群構造（modの周期性）を利用することで、完全に正確な計算を実現している
