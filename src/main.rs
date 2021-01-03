use gmp::mpq::Mpq;
use std::time::Instant;

fn factorial(n: &Mpq) -> Mpq {
    if n == &Mpq::zero() {
        Mpq::one()
    } else {
        n * &factorial(&(n - &Mpq::one()))
    }
}

fn main() {
    let start = Instant::now();
    for _i in 0..101 {
        println!("{}!={}", _i, factorial(&Mpq::from(_i)));
    }
    let end = start.elapsed();
    println!("{}.{:03}sec", end.as_secs(), end.subsec_nanos() / 1_000_000)
}
