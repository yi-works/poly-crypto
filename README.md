# poly-crypto
A Rust library for Finite Field arithmetic, Shamir's Secret Sharing, and Number Theoretic Transform (NTT).
> **Note**: This project is a personal learning project.

## Motivation
- Rustによる有限体演算、秘密分散法、数論変換(NTT)を実装し、数学的背景を理解する

## Insights
- FFTもNTTも「原始N乗根」を使うが、
FFTでは複素数上で常に存在するのに対し、
NTTでは有限体上で条件付きでしか存在しないのが興味深い

- FFTは複素数の連続的な周期性を利用するが、浮動小数誤差が避けられない

- NTTは有限体の巡回群構造（modの周期性）を利用することで、完全に正確な計算を実現している
