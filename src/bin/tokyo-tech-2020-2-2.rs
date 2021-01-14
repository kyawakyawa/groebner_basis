extern crate groebner_basis;

use groebner_basis::monomial::{Monomial, MonomialOrder};
use groebner_basis::polynomial::{Polynomial, PolynomialHandlers};
use std::time::Instant;

/*
(* A(0, 0) *) (* Aは原点に固定する(一般性は失わない)*)
(* B(b1, 0) *) (* 辺ABはx軸上にする(一般性は失わない)*)
(* C(c1, c2)*)
(* M(m1, m2) 外接円の中心 *)
(* P(p1, p2) 外接円上の点 *)
(* r 外接円の半径*)
(* d 答え*)

(* 辺ABと辺ACが同じ長さ *)
f1 = c1 ^2 + c2 ^2 - b1 ^2

(* 辺ABと辺BCが同じ長さ *)
f2 = (c1 - b1)^2 + c2 ^ 2 - b1^2

(* 点Aと点Xの距離はr *)
f3 = m1 ^2 + m2 ^2 - r ^2

(* 点Bと点Xの距離はr *)
f4 = (b1 - m1)^ 2 + (0 - m2)^2 - r ^2

(* 点Cと点Xの距離はr *)
f5 = (c1 - m1)^ 2 + (c2 - m2)^2 - r ^2

(* 点Pと点Xの距離はr *)
f6 = (p1 - m1)^ 2 + (p2 - m2)^2 - r ^2

(* b1 0 は でない *) (* この条件だけで三角形が潰れれることはなくなる *)
f7 = b1 * v - 1

(* AP^2+BP^2+CP^2==d1 (d1が求めたい式に等しい) *)
g = p1 ^2 + p2 ^2 + (p1 - b1)^ 2 + p2 ^ 2 + (p1 - c1)^ 2 + (p2 - c2)^2 - d1


(* AP^4+BP^4+CP^4==d2 (d2が求めたい式に等しい) *)
h = (p1 ^2 + p2 ^2)^2 + ((p1 - b1)^2 + p2 ^2)^ 2 + ((p1 - c1)^2 + (p2 - c2)^ 2)^2 - d2

(* 消去定理により イデアルに含まれる rとd1 or d2のみの変数からなる多項式を探す *)

(* AP^2+BP^2+CP^2 *)
GroebnerBasis[{f1, f2, f3, f4, f5, f6, f7, g}, {b1, c1, c2, m1, m2, p1, p2, v, r, d1}]

(* AP^4+BP^4+CP^4 *)
GroebnerBasis[{f1, f2, f3, f4, f5, f6, f7, h}, {b1, c1, c2, m1, m2, p1, p2, v, r, d2}]
*/

fn sq(v: Polynomial) -> Polynomial {
    &v * &v
}

fn main() {
    // b1 -> x1

    // c1 -> x2
    // c2 -> x3

    // m1 -> x4
    // m2 -> x5

    // p1 -> x6
    // p2 -> x7

    // v  -> x8

    // r  -> x9

    // d  -> x10

    let monomial_order = MonomialOrder::Lex;

    let degree: Vec<i64> = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let b1 = Polynomial::from(Monomial::from((degree, monomial_order)));

    let degree: Vec<i64> = vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0];
    let c1 = Polynomial::from(Monomial::from((degree, monomial_order)));

    let degree: Vec<i64> = vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0];
    let c2 = Polynomial::from(Monomial::from((degree, monomial_order)));

    let degree: Vec<i64> = vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0];
    let m1 = Polynomial::from(Monomial::from((degree, monomial_order)));

    let degree: Vec<i64> = vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0];
    let m2 = Polynomial::from(Monomial::from((degree, monomial_order)));

    let degree: Vec<i64> = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0];
    let p1 = Polynomial::from(Monomial::from((degree, monomial_order)));

    let degree: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0];
    let p2 = Polynomial::from(Monomial::from((degree, monomial_order)));

    let degree: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0];
    let v = Polynomial::from(Monomial::from((degree, monomial_order)));

    let degree: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0];
    let r = Polynomial::from(Monomial::from((degree, monomial_order)));

    let degree: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let d = Polynomial::from(Monomial::from((degree, monomial_order)));

    let degree: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let one = Polynomial::from(Monomial::from((degree, monomial_order)));

    let f1 = &c1 * &c1 + &c2 * &c2 - &b1 * &b1;

    let f2 = sq(&c1 - &b1) + &c2 * &c2 - &b1 * &b1;

    let f3 = &m1 * &m1 + &m2 * &m2 - &r * &r;

    let f4 = sq(&b1 - &m1) + &m2 * &m2 - &r * &r;

    let f5 = sq(&c1 - &m1) + sq(&c2 - &m2) - &r * &r;

    let f6 = sq(&p1 - &m1) + sq(&p2 - &m2) - &r * &r;

    let f7 = &b1 * &v - &one;

    let g = &p1 * &p1 + &p2 * &p2 + sq(&p1 - &b1) + &p2 * &p2 + sq(&p1 - &c1) + sq(&p2 - &c2) - &d;

    let h = sq(&p1 * &p1 + &p2 * &p2)
        + sq(sq(&p1 - &b1) + &p2 * &p2)
        + sq(sq(&p1 - &c1) + sq(&p2 - &c2))
        - &d;

    let fs = vec![
        f1.clone(),
        f2.clone(),
        f3.clone(),
        f4.clone(),
        f5.clone(),
        f6.clone(),
        f7.clone(),
        g.clone(),
    ];

    let start = Instant::now();
    let ps = groebner_basis::groebner_basis::compute_groebner_basis(fs);
    let end = start.elapsed();

    println!(
        "compute Groebner Basis in {}.{:03}sec",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
    println!("GroebnerBasis[f1, f2, f3, f4, f5, f6, f7, g]");
    for (i, p) in ps.iter().enumerate() {
        println!("  |  p_{} = {}", i + 1, p.clone().integer_coeff());
    }
    println!("\n\n");

    let fs = vec![
        f1.clone(),
        f2.clone(),
        f3.clone(),
        f4.clone(),
        f5.clone(),
        f6.clone(),
        f7.clone(),
        h.clone(),
    ];

    let start = Instant::now();
    let qs = groebner_basis::groebner_basis::compute_groebner_basis(fs);
    let end = start.elapsed();

    println!(
        "compute Groebner Basis in {}.{:03}sec",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
    println!("GroebnerBasis[f1, f2, f3, f4, f5, f6, f7, h]");
    for (i, q) in qs.iter().enumerate() {
        println!("  |  q_{} = {}", i + 1, q.clone().integer_coeff());
    }
    println!("\n\n");
}
