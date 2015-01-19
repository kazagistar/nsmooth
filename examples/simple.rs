extern crate nsmooth;
extern crate num;

fn main () {
    let result: num::BigInt = nsmooth::nsmooth(20).nth(999999).expect("");
    println!("The millionth number not divisible by any prime greater then 20 is {}.", result);
}
