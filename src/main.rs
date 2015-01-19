use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Eq, PartialEq)]
struct Composite <'a> {
    product : u64,
    successors : &'a [u64],
}

impl <'a> Ord for Composite<'a> {
    fn cmp(&self, other: &Composite) -> Ordering {
        other.product.cmp(&self.product)
    }
}

impl <'a> PartialOrd for Composite<'a> {
    fn partial_cmp(&self, other: &Composite) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // Primes in reverse order is about 20% faster
    // This is because the "small and frequent" primes are near the "branches" of the "tree"
    //let legal_primes = [2,3,5,7,11,13,17,19];
    let legal_primes = [19,17,13,11,7,5,3,2];
    let mut heap = BinaryHeap::new();
    
    heap.push(Composite { product: 1, successors: &legal_primes });
    
    for _ in 1..10000000 {
        let x = heap.pop().expect("Error: Number of products was finite?!?");
        
        for (index, &prime) in x.successors.iter().enumerate() {
            let new = Composite {
                product: x.product * prime,
                successors: &x.successors[index..],
            };
            heap.push(new);
        }
    }
    
    let last = heap.pop().expect("Error: Number of products was finite?!?");
    
    println!("The millionth number not divisible by primes bigger then 20 is {}.", last.product);
}
