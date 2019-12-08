# wasi-rng

A super simple implementation of a Random Number Generator for WASI.
Implements [RngCore] and [CryptoRng] by using [random_get] to
[try_fill_bytes](RngCore::try_fill_bytes).

```rust
use rand::rng_core::RngCore;
use wasi_rng::WasiRng;

let mut rng = WasiRng;
println!("random number: {}", rng.next_u32());
```
