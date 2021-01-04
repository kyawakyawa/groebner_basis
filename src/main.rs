mod scalar;

use scalar::Integer;
use std::time::Instant;

fn factorial(n: &Integer) -> Integer {
    if n == &Integer::zero() {
        Integer::one()
    } else {
        n * &factorial(&(n - &Integer::one()))
    }
}

fn main() {
    let start = Instant::now();
    for _i in 0..101 {
        println!("{}!={}", _i, factorial(&Integer::from(_i)));
    }
    let end = start.elapsed();
    println!("{}.{:03}sec", end.as_secs(), end.subsec_nanos() / 1_000_000)
}
