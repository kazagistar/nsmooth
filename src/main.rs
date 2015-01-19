#![feature(box_syntax)]

use std::cmp::Ordering;
use std::num::Int;
use std::collections::BinaryHeap;
use std::num::NumCast;

#[derive(Copy, Eq, PartialEq)]
struct Composite <I : Int> {
    product : I,
    cutoff: usize,
}

impl <I: Int> Ord for Composite<I> {
    fn cmp(&self, other: &Composite<I>) -> Ordering {
        other.product.cmp(&self.product)
    }
}

impl <I: Int> PartialOrd for Composite<I> {
    fn partial_cmp(&self, other: &Composite<I>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


struct NSmooth <I : Int> {
    lookahead : BinaryHeap<Composite<I>>,
    factors : Box<[I]>,
}

fn nsmooth<I : Int>(n : usize) -> NSmooth<I> {
    // for now, we ignore n, until I actually get a prime generator
    let mut ns = NSmooth {
        factors: primes(n).into_boxed_slice(),
        lookahead: BinaryHeap::new(),
    };
    ns.lookahead.push(Composite { product: <I as Int>::one(), cutoff: 0 });
    return ns;
}

impl <I : Int> Iterator for NSmooth<I> {
    type Item = I;

    fn next(&mut self) -> Option<I> {
        let prev = self.lookahead.pop().expect("Error: Number of products was finite?!?");
        
        let prime = self.factors[prev.cutoff];
        // Push first child
        self.lookahead.push(Composite {
            product: prev.product * prime,
            cutoff: prev.cutoff,
        });
      
        // Push brother
        let brother_cutoff = prev.cutoff + 1;
        if brother_cutoff < self.factors.len() && prev.product != <I as Int>::one() {
            let brother_prime = self.factors[brother_cutoff];
            self.lookahead.push(Composite {
                product: prev.product / prime * brother_prime,
                cutoff: prev.cutoff + 1,
            });
        }
        return Some(prev.product);
    }
}


// Simple and slow prime finder
// generic numbers in rust are wierd and add a lot of boilerplate :(
fn primes <I: Int>(n : usize) -> Vec<I> {
    let mut primes = vec![];
    let one: I = Int::one();
    let mut candidate: I = one + one;
    loop {
        if !primes.iter().any(|&x| candidate % x == <I as Int>::zero()) {
           primes.push(candidate)
        }
        if primes.len() == n {
            return primes;
        }
        candidate = candidate + one;
    }
}


fn main() {
    let x: u64 = nsmooth(8).nth(9999999).expect("");
    println!("{}", x);
}
