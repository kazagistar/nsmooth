extern crate slow_primes;
extern crate num;

use std::cmp::Ordering;
use num::integer::Integer;
use num::traits::One;
use std::collections::BinaryHeap;
use num::FromPrimitive;

#[derive(Clone, Eq, PartialEq)]
struct Composite <I : Integer + Ord> {
    product : I,
    cutoff: usize,
}

impl <I: Integer + Ord> Ord for Composite<I> {
    fn cmp(&self, other: &Composite<I>) -> Ordering {
        other.product.cmp(&self.product)
    }
}

impl <I: Integer + Ord> PartialOrd for Composite<I> {
    fn partial_cmp(&self, other: &Composite<I>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct NSmooth <I : Integer + Ord> {
    lookahead : BinaryHeap<Composite<I>>,
    factors : Box<[I]>,
}

pub fn nsmooth<I : Integer + Ord + FromPrimitive>(n : usize) -> NSmooth<I> {
    let primes: Vec<_> = slow_primes::Primes::sieve(n + 1).primes()
        .take_while(|x| x <= &n)
        .map(|x| <I as FromPrimitive>::from_usize(x).expect("Overflow while generating primes"))
        .collect();

    // for now, we ignore n, until I actually get a prime generator
    let mut ns = NSmooth {
        factors: primes.into_boxed_slice(),
        lookahead: BinaryHeap::new(),
    };
    ns.lookahead.push(Composite { product: <I as One>::one(), cutoff: 0 });
    return ns;
}

impl <I : Integer + Ord + Clone> Iterator for NSmooth<I> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        let prev = self.lookahead.pop().expect("Error: Number of products was finite?!?");

        let prime = self.factors[prev.cutoff].clone();
        // Push first child
        self.lookahead.push(Composite {
            product: prev.product.clone() * prime.clone(),
            cutoff: prev.cutoff,
        });

        // Push brother
        let brother_cutoff = prev.cutoff + 1;
        if brother_cutoff < self.factors.len() && prev.product != <I as One>::one() {
            let brother_prime = self.factors[brother_cutoff].clone();
            self.lookahead.push(Composite {
                product: prev.product.clone() / prime * brother_prime,
                cutoff: prev.cutoff + 1,
            });
        }
        return Some(prev.product);
    }
}
