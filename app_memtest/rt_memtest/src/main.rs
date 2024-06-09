#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;
use rand::{rngs::SmallRng, RngCore, SeedableRng};

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Hello from rt_memtest");

    let mut rng = SmallRng::seed_from_u64(0xdead_beef);
    println!("{:?}",rng);
}

