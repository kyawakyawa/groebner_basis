extern crate groebner_basis;

use groebner_basis::monomial::{Monomial, MonomialOrder};
use groebner_basis::polynomial::{Polynomial, PolynomialHandlers};
use groebner_basis::scalar::Rational;

/*
 *
(* = >の証明*)

(*複素数を2つの実数を用いて表す*)
alpha = a1 + I * a2
beta = b1 + I * b2
gamma = c1 + I * c2


(*仮定*)
(*「 A,B,Cは異なり、同一線上に無い」 < = > ベクトルABとベクトルCBとベクトルの外積がでない*)
f3 = Expand[((a1 - b1) * (c2 - b2) - (a2 - b2) * (c1 - b1)) * v - 1]
=> -1 + a2 b1 v - a1 b2 v - a2 c1 v + b2 c1 v + a1 c2 v - b1 c2 v


(*仮定(結論)*)

(*仮定の式を 左辺-右辺*)
f = alpha^2 + beta ^2 + gamma ^ 2 - alpha * beta - beta * gamma - gamma * alpha

(*実部だけ取り出す
f1 = ComplexExpand[Re[f]]
=> a1^2 - a2^2 - a1 b1 + b1^2 + a2 b2 - b2^2 - a1 c1 - b1 c1 + c1^2 + a2 c2 + b2 c2 - c2^2

(*虚部だけをを取り出す*)
f2 = ComplexExpand[Im[f]]
=> 2 a1 a2 - a2 b1 - a1 b2 + 2 b1 b2 - a2 c1 - b2 c1 - a1 c2 - b1 c2 + 2 c1 c2


(*結論(仮定)*)
(*辺AB BC と辺 の長さが同じ*)
g1 = ComplexExpand[ (alpha - beta) * Conjugate[alpha - beta] - (beta - gamma) * Conjugate[beta - gamma]]
=> a1^2 + a2^2 - 2 a1 b1 - 2 a2 b2 + 2 b1 c1 - c1^2 + 2 b2 c2 - c2^2

(*辺 と辺 の長さが同じ BC CA *)
g2 = ComplexExpand[ (beta - gamma) *  Conjugate[beta - gamma] - (gamma - alpha) * Conjugate[gamma - alpha]]
=> -a1^2 - a2^2 + b1^2 + b2^2 + 2 a1 c1 - 2 b1 c1 + 2 a2 c2 - 2 b2 c2

(*ヒルベルトの零点定理によるチェック*)

(*g1が<f1,f2,f3>の根基イデアルの含まれるか調べる*)
GroebnerBasis[{f1, f2, f3, 1 - y * g1}, {a1, a2, b1, b2, c1, c2, v, y}]
=> {1}
(*g2が<f1,f2,f3>の根基イデアルの含まれるか調べる*)
グレブナー基底 GroebnerBasis[{f1, f2, f3, 1 - y * g2}, {a1, a2, b1, b2, c1, c2, v, y}]
=> {1}
(* = > の証明終了*)

(*< =の証明*)
(*ヒルベルトの零点定理によるチェック*)
(*f1が<g1,g2,f3>の根基イデアルの含まれるか調べる*)
GroebnerBasis[{g1, g2, f3, 1 - y * f1}, {a1, a2, b1, b2, c1, c2, v, y}]
=> {1}
(*f2が<g1,g2,f3>の根基イデアルの含まれるか調べる*)
GroebnerBasis[{g1, g2, f3, 1 - y * f2}, {a1, a2, b1, b2, c1, c2, v, y}]
=> {1}
*
*/
fn main() {
    // a1 -> x1
    // a2 -> x2

    // b1 -> x3
    // b2 -> x4

    // c1 -> x5
    // c2 -> x6

    // v -> x7

    // y -> x8

    // f3 = -1 + a2 b1 v - a1 b2 v - a2 c1 v + b2 c1 v + a1 c2 v - b1 c2 v

    let monomial_order = MonomialOrder::Grlex;

    let mut f3 = Polynomial::from((8, monomial_order));

    let degree: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    f3.add_term(Rational::from(-1), Monomial::from(degree)); // -1

    let degree: Vec<i64> = vec![0, 1, 1, 0, 0, 0, 1, 0];
    f3.add_term(Rational::from(1), Monomial::from(degree)); // a2 b1 v

    let degree: Vec<i64> = vec![1, 0, 0, 1, 0, 0, 1, 0];
    f3.add_term(Rational::from(-1), Monomial::from(degree)); // -a1 b2 v

    let degree: Vec<i64> = vec![0, 1, 0, 0, 1, 0, 1, 0];
    f3.add_term(Rational::from(-1), Monomial::from(degree)); // -a2 c1 v

    let degree: Vec<i64> = vec![0, 0, 0, 1, 1, 0, 1, 0];
    f3.add_term(Rational::from(1), Monomial::from(degree)); // b2 c1 v

    let degree: Vec<i64> = vec![1, 0, 0, 0, 0, 1, 1, 0];
    f3.add_term(Rational::from(1), Monomial::from(degree)); // a1 c2 v

    let degree: Vec<i64> = vec![0, 0, 1, 0, 0, 1, 1, 0];
    f3.add_term(Rational::from(-1), Monomial::from(degree)); // -b1 c2 v

    // f1 = a1^2 - a2^2 - a1 b1 + b1^2 + a2 b2 - b2^2 - a1 c1 - b1 c1 + c1^2 + a2 c2 + b2 c2 - c2^2
    let mut f1 = Polynomial::from((8, monomial_order));

    let degree: Vec<i64> = vec![2, 0, 0, 0, 0, 0, 0, 0];
    f1.add_term(Rational::from(1), Monomial::from(degree)); // a1^2

    let degree: Vec<i64> = vec![0, 2, 0, 0, 0, 0, 0, 0];
    f1.add_term(Rational::from(-1), Monomial::from(degree)); // -a2^2

    let degree: Vec<i64> = vec![1, 0, 1, 0, 0, 0, 0, 0];
    f1.add_term(Rational::from(-1), Monomial::from(degree)); // -a1 b1

    let degree: Vec<i64> = vec![0, 0, 2, 0, 0, 0, 0, 0];
    f1.add_term(Rational::from(1), Monomial::from(degree)); // b1^2

    let degree: Vec<i64> = vec![0, 1, 0, 1, 0, 0, 0, 0];
    f1.add_term(Rational::from(1), Monomial::from(degree)); // a2 b2

    let degree: Vec<i64> = vec![0, 0, 0, 2, 0, 0, 0, 0];
    f1.add_term(Rational::from(-1), Monomial::from(degree)); // -b2^2

    let degree: Vec<i64> = vec![1, 0, 0, 0, 1, 0, 0, 0];
    f1.add_term(Rational::from(-1), Monomial::from(degree)); // -a1 c1

    let degree: Vec<i64> = vec![0, 0, 1, 0, 1, 0, 0, 0];
    f1.add_term(Rational::from(-1), Monomial::from(degree)); // -b1 c1

    let degree: Vec<i64> = vec![0, 0, 0, 0, 2, 0, 0, 0];
    f1.add_term(Rational::from(1), Monomial::from(degree)); // c1^2

    let degree: Vec<i64> = vec![0, 1, 0, 0, 0, 1, 0, 0];
    f1.add_term(Rational::from(1), Monomial::from(degree)); // a2 c2

    let degree: Vec<i64> = vec![0, 0, 0, 1, 0, 1, 0, 0];
    f1.add_term(Rational::from(1), Monomial::from(degree)); // b2 c2

    let degree: Vec<i64> = vec![0, 0, 0, 0, 0, 2, 0, 0];
    f1.add_term(Rational::from(-1), Monomial::from(degree)); // -c2^2

    // f2 = 2 a1 a2 - a2 b1 - a1 b2 + 2 b1 b2 - a2 c1 - b2 c1 - a1 c2 - b1 c2 + 2 c1 c2
    let mut f2 = Polynomial::from((8, monomial_order));

    let degree: Vec<i64> = vec![1, 1, 0, 0, 0, 0, 0, 0];
    f2.add_term(Rational::from(2), Monomial::from(degree)); // 2 a1 a2

    let degree: Vec<i64> = vec![0, 1, 1, 0, 0, 0, 0, 0];
    f2.add_term(Rational::from(-1), Monomial::from(degree)); // - a2 b1

    let degree: Vec<i64> = vec![1, 0, 0, 1, 0, 0, 0, 0];
    f2.add_term(Rational::from(-1), Monomial::from(degree)); // - a1 b2

    let degree: Vec<i64> = vec![0, 0, 1, 1, 0, 0, 0, 0];
    f2.add_term(Rational::from(2), Monomial::from(degree)); // 2 b1 b2

    let degree: Vec<i64> = vec![0, 1, 0, 0, 1, 0, 0, 0];
    f2.add_term(Rational::from(-1), Monomial::from(degree)); // - a2 c1

    let degree: Vec<i64> = vec![0, 0, 0, 1, 1, 0, 0, 0];
    f2.add_term(Rational::from(-1), Monomial::from(degree)); // - b2 c1

    let degree: Vec<i64> = vec![1, 0, 0, 0, 0, 1, 0, 0];
    f2.add_term(Rational::from(-1), Monomial::from(degree)); // - a1 c2

    let degree: Vec<i64> = vec![0, 0, 1, 0, 0, 1, 0, 0];
    f2.add_term(Rational::from(-1), Monomial::from(degree)); // - b1 c2

    let degree: Vec<i64> = vec![0, 0, 0, 0, 1, 1, 0, 0];
    f2.add_term(Rational::from(2), Monomial::from(degree)); // 2 c1 c2

    // g1 = a1^2 + a2^2 - 2 a1 b1 - 2 a2 b2 + 2 b1 c1 - c1^2 + 2 b2 c2 - c2^2
    let mut g1 = Polynomial::from((8, monomial_order));

    let degree: Vec<i64> = vec![2, 0, 0, 0, 0, 0, 0, 0];
    g1.add_term(Rational::from(1), Monomial::from(degree)); // a1^2

    let degree: Vec<i64> = vec![0, 2, 0, 0, 0, 0, 0, 0];
    g1.add_term(Rational::from(1), Monomial::from(degree)); // a2^2

    let degree: Vec<i64> = vec![1, 0, 1, 0, 0, 0, 0, 0];
    g1.add_term(Rational::from(-2), Monomial::from(degree)); // -2 a1 b1

    let degree: Vec<i64> = vec![0, 1, 0, 1, 0, 0, 0, 0];
    g1.add_term(Rational::from(-2), Monomial::from(degree)); // -2 a2 b2

    let degree: Vec<i64> = vec![0, 0, 1, 0, 1, 0, 0, 0];
    g1.add_term(Rational::from(2), Monomial::from(degree)); // 2 b1 c1

    let degree: Vec<i64> = vec![0, 0, 0, 0, 2, 0, 0, 0];
    g1.add_term(Rational::from(-1), Monomial::from(degree)); // -c1^2

    let degree: Vec<i64> = vec![0, 0, 0, 1, 0, 1, 0, 0];
    g1.add_term(Rational::from(2), Monomial::from(degree)); // 2 b2 c2

    let degree: Vec<i64> = vec![0, 0, 0, 0, 0, 2, 0, 0];
    g1.add_term(Rational::from(-1), Monomial::from(degree)); // -c2^2

    // g2 = -a1^2 - a2^2 + b1^2 + b2^2 + 2 a1 c1 - 2 b1 c1 + 2 a2 c2 - 2 b2 c2
    let mut g2 = Polynomial::from((8, monomial_order));

    let degree: Vec<i64> = vec![2, 0, 0, 0, 0, 0, 0, 0];
    g2.add_term(Rational::from(-1), Monomial::from(degree)); // -a1^2

    let degree: Vec<i64> = vec![0, 2, 0, 0, 0, 0, 0, 0];
    g2.add_term(Rational::from(-1), Monomial::from(degree)); // -a2^2

    let degree: Vec<i64> = vec![0, 0, 2, 0, 0, 0, 0, 0];
    g2.add_term(Rational::from(1), Monomial::from(degree)); // b1^2

    let degree: Vec<i64> = vec![0, 0, 0, 2, 0, 0, 0, 0];
    g2.add_term(Rational::from(1), Monomial::from(degree)); // b2^2

    let degree: Vec<i64> = vec![1, 0, 0, 0, 1, 0, 0, 0];
    g2.add_term(Rational::from(2), Monomial::from(degree)); // 2 a1 c1

    let degree: Vec<i64> = vec![0, 0, 1, 0, 1, 0, 0, 0];
    g2.add_term(Rational::from(-2), Monomial::from(degree)); // -2 b1 c1

    let degree: Vec<i64> = vec![0, 1, 0, 0, 0, 1, 0, 0];
    g2.add_term(Rational::from(2), Monomial::from(degree)); // 2 a2 c2

    let degree: Vec<i64> = vec![0, 0, 0, 1, 0, 1, 0, 0];
    g2.add_term(Rational::from(-2), Monomial::from(degree)); // -2 b2 c2

    // 1
    let degree: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let one = Polynomial::from((Monomial::from(degree), monomial_order));

    // y
    let degree: Vec<i64> = vec![0, 0, 0, 0, 0, 0, 0, 1];
    let y = Polynomial::from((Monomial::from(degree), monomial_order));

    // h1 = 1 - y * g1
    let h1 = &one - &y * &g1;
    // h2 = 1 - y * g2
    let h2 = &one - &y * &g2;

    let fs = vec![f1.clone(), f2.clone(), f3.clone(), h1];
    let gs = groebner_basis::groebner_basis::compute_groebner_basis(fs);

    println!("GroebnerBasis[f1, f2, f3, 1 - y * g1]");
    for (i, g) in gs.iter().enumerate() {
        println!("  |  p_{} = {}", i + 1, g);
    }
    println!("\n\n");

    let fs = vec![f1.clone(), f2.clone(), f3.clone(), h2];
    let gs = groebner_basis::groebner_basis::compute_groebner_basis(fs);
    println!("GroebnerBasis[f1, f2, f3,1 - y * g2]");
    for (i, g) in gs.iter().enumerate() {
        println!("  |  q_{} = {}", i + 1, g);
    }
    println!("\n\n");

    let h1 = &one - &y * &f1;
    let h2 = &one - &y * &f2;

    let fs = vec![g1.clone(), g2.clone(), f3.clone(), h1];
    let gs = groebner_basis::groebner_basis::compute_groebner_basis(fs);

    println!("GroebnerBasis[g1, g2, f3, 1 - y * f1]");
    for (i, g) in gs.iter().enumerate() {
        println!("  |  r_{} = {}", i + 1, g);
    }
    println!("\n\n");

    let fs = vec![g1.clone(), g2.clone(), f3.clone(), h2];
    let gs = groebner_basis::groebner_basis::compute_groebner_basis(fs);
    println!("GroebnerBasis[g1, g2, f3,1 - y * f2]");
    for (i, g) in gs.iter().enumerate() {
        println!("  |  s_{} = {}", i + 1, g);
    }
    println!("\n\n");
}
