//! A super simple implementation of a Random Number Generator for WASI.
//! Implements [RngCore] and [CryptoRng] by using [random_get] to
//! [try_fill_bytes](RngCore::try_fill_bytes).
//!
//! ```
//! use rand_core::RngCore;
//! use wasi_rng::WasiRng;
//!
//! let mut rng = WasiRng;
//! println!("random number: {}", rng.next_u32());
//! ```

#![no_std]

use core::num::NonZeroU32;
use rand_core::impls::{next_u32_via_fill, next_u64_via_fill};
use rand_core::{CryptoRng, Error, RngCore};
use wasi::random_get;

#[derive(Clone, Copy, Debug, Default)]
pub struct WasiRng;

impl RngCore for WasiRng {
    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        unsafe {
            random_get(dest.as_mut_ptr(), dest.len())
                .map_err(|err| Error::from(NonZeroU32::new_unchecked(err.raw_error() as u32)))
        }
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.try_fill_bytes(dest).unwrap()
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        next_u32_via_fill(self)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        next_u64_via_fill(self)
    }
}

impl CryptoRng for WasiRng {}
