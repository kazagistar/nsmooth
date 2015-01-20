extern crate nsmooth;

fn main () {
    for num in nsmooth::nsmooth::<u64>(20).take(10000) {
        println!("{}", num);
    }
}
