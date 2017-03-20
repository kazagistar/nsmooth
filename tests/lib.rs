extern crate nsmooth;
extern crate slow_primes;

use std::collections::BTreeSet;

// All smooth numbers are counted as monotonically increasing
// This means there are no repeats, and no "out of order" problems
// Test up to a million
#[test]
fn strictly_monotonically_increasing() {
    let mut prev = 0;
    for now in nsmooth::nsmooth::<u64>(20).take(1000000) {
        assert!(now > prev);
        prev = now;
    }
}

// Makes sure generated numbers are actually nsmooth by factoring them
// Tests up to a million
#[test]
fn valid_factorizations() {
    let primes = slow_primes::Primes::sieve(20);

    for now in nsmooth::nsmooth(20).take(1000000) {
        match primes.factor(now) {
            Ok(factors) => { assert!(factors.iter().all(|&(x,_)| x<20)) },
            Err(_)      => { panic!("Bad factor") },
        }
    }
}

// Make sure generated numbers are unique
// Tests up to ten thousand
#[test]
fn unique_results() {
    let unique: BTreeSet<i64> = nsmooth::nsmooth(5).take(10000).collect();
    assert!(unique.len() == 10000);
}
