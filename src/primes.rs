extern crate primes;
extern crate rand;

use primes::PrimeSet;
use rand::Rng;

// This is a really hacky way of generating the prime
// and coprime.  Would like to do it properly.

fn brute_force_root() -> u64 {
    let mut rng = rand::thread_rng();
    let close: u64 = rng.gen_range(1, 1000);

    let mut pset = PrimeSet::new();
    let (_, prime) = pset.find(close);

    return prime;
}

fn brute_force_prime(root: u64) -> u64 {
    let mut pset = PrimeSet::new();
    let prime: u64 = pset.iter().skip(root as usize).next().unwrap();

    return prime;
}

pub fn generate_prime_and_root() -> (u64, u64) {
    let root: u64 = brute_force_root();
    let prime: u64 = brute_force_prime(root);
    return (prime, root);
}
