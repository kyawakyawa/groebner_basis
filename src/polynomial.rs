use crate::monomial;
use crate::monomial::{Monomial, MonomialHandlers, MonomialOrder};
use crate::scalar::{Integer, Rational};
use std::collections::BTreeMap;

use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial {
    terms: BTreeMap<Monomial, Rational>,
    n: usize,
    monomial_order: MonomialOrder,
}

impl From<usize> for Polynomial {
    fn from(n: usize) -> Self {
        Self::from((n, MonomialOrder::Lex))
    }
}

impl From<(usize, MonomialOrder)> for Polynomial {
    fn from(pair: (usize, MonomialOrder)) -> Self {
        Self {
            terms: BTreeMap::new(),
            n: pair.0,
            monomial_order: pair.1,
        }
    }
}

impl From<Vec<Integer>> for Polynomial {
    fn from(v: Vec<Integer>) -> Self {
        Self::from((v, MonomialOrder::Lex))
    }
}

impl From<(Vec<Integer>, MonomialOrder)> for Polynomial {
    fn from(pair: (Vec<Integer>, MonomialOrder)) -> Self {
        let n_ = pair.0.len();
        let x = Monomial::from(pair.0);
        let mut terms_ = BTreeMap::new();
        terms_.insert(x, Rational::from(1));

        Self {
            terms: terms_,
            n: n_,
            monomial_order: pair.1,
        }
    }
}

impl From<Monomial> for Polynomial {
    fn from(v: Monomial) -> Self {
        let n_ = v.get_n();
        let monomial_order_ = v.get_monomial_order();
        let mut terms_ = BTreeMap::new();
        terms_.insert(v, Rational::from(1));
        Self {
            terms: terms_,
            n: n_,
            monomial_order: monomial_order_,
        }
    }
}

impl From<(Monomial, MonomialOrder)> for Polynomial {
    fn from(pair: (Monomial, MonomialOrder)) -> Self {
        let n_ = pair.0.get_n();
        let mut terms_ = BTreeMap::new();
        terms_.insert(pair.0, Rational::from(1));
        Self {
            terms: terms_,
            n: n_,
            monomial_order: pair.1,
        }
    }
}

impl From<(Rational, Monomial, MonomialOrder)> for Polynomial {
    fn from(tuple: (Rational, Monomial, MonomialOrder)) -> Self {
        let n_ = tuple.1.get_n();
        let mut terms_ = BTreeMap::new();
        if tuple.0 != Rational::zero() {
            terms_.insert(tuple.1, tuple.0);
        }
        Self {
            terms: terms_,
            n: n_,
            monomial_order: tuple.2,
        }
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut output = String::new();

        let mut front = true;

        let mut terms = Vec::new();

        for pair in &self.terms {
            terms.push(pair);
        }
        let terms = terms;

        for it in terms.iter().rev() {
            let monomial = it.0;
            let coeff = it.1;

            if front {
                if coeff != &Rational::from(0) {
                    output.push_str(&coeff.to_string());
                    output.push_str(&monomial.to_string());
                    front = false;
                }
            } else {
                if coeff != &Rational::from(0) {
                    if coeff > &Rational::from(0) {
                        output.push_str(" + ");
                        output.push_str(&coeff.to_string());
                    } else {
                        output.push_str(" - ");
                        let mv = &Rational::from(-1) * coeff;
                        output.push_str(&mv.to_string());
                    }
                    output.push_str(&monomial.to_string());
                }
            }
        }
        if front {
            output.push_str("0");
        }

        output.push_str(" ");

        output.push_str(match self.monomial_order {
            MonomialOrder::Lex => "Lex",
            MonomialOrder::Grlex => "Grlex",
        });

        write!(f, "{}", output)
    }
}

impl<'a, 'b> Add<&'a Polynomial> for &'b Polynomial {
    type Output = Polynomial;

    fn add(self, other: &Polynomial) -> Polynomial {
        assert!(self.n == other.n);
        assert!(self.monomial_order == other.monomial_order);

        let longer: &Polynomial = if self.terms.len() >= other.terms.len() {
            self
        } else {
            other
        };
        let shorter: &Polynomial = if self.terms.len() >= other.terms.len() {
            other
        } else {
            self
        };

        let mut ret = longer.clone();

        for (k, v) in &shorter.terms {
            assert_ne!(v, &Rational::from(0));
            ret.add_term(v.clone(), k.clone());
        }

        ret
    }
}

impl Add<&Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, other: &Polynomial) -> Polynomial {
        &self + other
    }
}

impl Add<Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Polynomial {
        self + &other
    }
}

impl Add<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Polynomial {
        &self + &other
    }
}

impl<'a, 'b> Sub<&'a Polynomial> for &'b Polynomial {
    type Output = Polynomial;

    fn sub(self, other: &Polynomial) -> Polynomial {
        assert!(self.n == other.n);
        assert!(self.monomial_order == other.monomial_order);

        let mut ret = self.clone();

        for (k, v) in &other.terms {
            assert_ne!(v, &Rational::from(0));
            ret.sub_term(v.clone(), k.clone());
        }

        ret
    }
}

impl Sub<&Polynomial> for Polynomial {
    type Output = Polynomial;

    fn sub(self, other: &Polynomial) -> Polynomial {
        &self - other
    }
}

impl Sub<Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn sub(self, other: Polynomial) -> Polynomial {
        self - &other
    }
}

impl Sub<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn sub(self, other: Polynomial) -> Polynomial {
        &self - &other
    }
}

impl<'a, 'b> Mul<&'a Polynomial> for &'b Polynomial {
    type Output = Polynomial;

    fn mul(self, other: &Polynomial) -> Polynomial {
        assert!(self.n == other.n);
        assert!(self.monomial_order == other.monomial_order);

        let mut ret = Polynomial {
            terms: BTreeMap::new(),
            n: self.n,
            monomial_order: self.monomial_order,
        };

        for (lx, lc) in &self.terms {
            assert_ne!(lc, &Rational::from(0));
            for (rx, rc) in &other.terms {
                assert_ne!(rc, &Rational::from(0));
                ret.add_term(lc * rc, lx * rx);
            }
        }

        ret
    }
}

impl Mul<&Polynomial> for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: &Polynomial) -> Polynomial {
        &self * other
    }
}

impl Mul<Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Polynomial {
        self * &other
    }
}

impl Mul<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Polynomial {
        &self * &other
    }
}

pub trait PolynomialHandlers {
    fn add_monomial(&mut self, x: Monomial);
    fn add_term(&mut self, c: Rational, x: Monomial);
    fn sub_monomial(&mut self, x: Monomial);
    fn sub_term(&mut self, c: Rational, x: Monomial);

    fn polynomial_divide(&self, rhses: &Vec<Polynomial>) -> (Vec<Polynomial>, Polynomial);

    fn set_monomial_order(&mut self, o: MonomialOrder);

    fn lt(&self) -> Option<Polynomial>;
    fn lm(&self) -> Option<Monomial>;
    fn lc(&self) -> Option<Rational>;
}

impl PolynomialHandlers for Polynomial {
    fn add_term(&mut self, c: Rational, x: Monomial) {
        let res = self.terms.get_mut(&x);

        let new_coeff = match res {
            Some(c_) => c_.clone() + c,
            None => c,
        };

        if new_coeff != Rational::zero() {
            self.terms.insert(x, new_coeff);
        } else {
            self.terms.remove(&x);
        }
    }
    fn add_monomial(&mut self, x: Monomial) {
        self.add_term(Rational::from(1), x);
    }

    fn sub_term(&mut self, c: Rational, x: Monomial) {
        let res = self.terms.get_mut(&x);

        let new_coeff = match res {
            Some(c_) => c_.clone() - c,
            None => -c,
        };
        if new_coeff != Rational::zero() {
            self.terms.insert(x, new_coeff);
        } else {
            self.terms.remove(&x);
        }
    }
    fn sub_monomial(&mut self, x: Monomial) {
        self.sub_term(Rational::from(1), x);
    }

    fn polynomial_divide(&self, rhses: &Vec<Polynomial>) -> (Vec<Polynomial>, Polynomial) {
        let monomial_order = self.monomial_order;
        let n = self.n;

        let mut p = self.clone();
        let zero = Polynomial::from(self.n);

        let s = rhses.len();

        let mut a = Vec::new();
        a.resize(s, Polynomial::from(n));
        let mut r = Polynomial::from(n);

        while &p != &zero {
            let mut divisionoccurred = false;

            for i in 0..s {
                let fi = &rhses[i];
                let lm_fi = fi.lm();
                let lm_p = p.lm();

                let lm_pair = (lm_p, lm_fi);

                match lm_pair {
                    (Some(lm_p), Some(lm_fi)) => {
                        if lm_p.is_divisible_by(&lm_fi) {
                            let lc_fi = fi.lc();
                            let lc_p = p.lc();

                            let lc_pair = (lc_p, lc_fi);
                            match lc_pair {
                                (Some(lc_p), Some(lc_fi)) => {
                                    let d = Polynomial::from((
                                        &lc_p / &lc_fi,
                                        &lm_p / &lm_fi,
                                        monomial_order,
                                    ));
                                    let ai = &a[i] + &d;
                                    a[i] = ai;

                                    p = &p - (&d * fi);
                                }
                                (_, _) => {
                                    assert!(false);
                                }
                            }

                            divisionoccurred = true;
                            break;
                        }
                    }
                    (_, _) => {
                        assert!(false);
                    }
                }
            }
            if !divisionoccurred {
                let lt_p = p.lt();
                match lt_p {
                    Some(lt_p) => {
                        r = &r + &lt_p;
                        p = &p - &lt_p;
                    }
                    None => {
                        assert!(false);
                    }
                }
            }
        }

        (a, r)
    }

    fn set_monomial_order(&mut self, o: MonomialOrder) {
        self.monomial_order = o;
    }

    fn lt(&self) -> Option<Polynomial> {
        let last = self.terms.iter().last();
        match last {
            Some((monomial, coeff)) => Some(Polynomial::from((
                coeff.clone(),
                monomial.clone(),
                self.monomial_order,
            ))),
            None => None,
        }
    }

    fn lm(&self) -> Option<Monomial> {
        let last = self.terms.iter().last();
        match last {
            Some((monomial, _)) => Some(monomial.clone()),
            None => None,
        }
    }

    fn lc(&self) -> Option<Rational> {
        let last = self.terms.iter().last();
        match last {
            Some((_, coeff)) => Some(coeff.clone()),
            None => None,
        }
    }
}

pub fn s_polynomial(f: &Polynomial, g: &Polynomial) -> Option<Polynomial> {
    assert_eq!(f.monomial_order, g.monomial_order);
    let lm_f = f.lm();
    let lm_g = g.lm();

    match (lm_f, lm_g) {
        (Some(lm_f), Some(lm_g)) => {
            let lcm_fg = monomial::lcm(&lm_f, &lm_g);

            let lc_f = f.lc();
            let lc_g = g.lc();
            match (lc_f, lc_g) {
                (Some(lc_f), Some(lc_g)) => {
                    let a = Polynomial::from((lc_f.invert(), &lcm_fg / &lm_f, f.monomial_order));
                    let b = Polynomial::from((lc_g.invert(), &lcm_fg / &lm_g, g.monomial_order));

                    Some(&a * f - &b * g)
                }
                (_, _) => None,
            }
        }
        (_, _) => None,
    }
}

#[cfg(test)]
mod test {
    use super::{s_polynomial, Polynomial, PolynomialHandlers};
    use crate::monomial;
    use crate::monomial::{Monomial, MonomialHandlers};
    use crate::scalar::{Integer, Rational};

    #[test]
    fn test_polynomial_divide() {
        let mut f = Polynomial::from(2);
        f.add_term(
            Rational::from(1),
            Monomial::from(vec![Integer::from(2), Integer::from(1)]),
        );
        f.add_term(
            Rational::from(1),
            Monomial::from(vec![Integer::from(1), Integer::from(2)]),
        );
        f.add_term(
            Rational::from(1),
            Monomial::from(vec![Integer::from(0), Integer::from(2)]),
        );

        let mut f0 = Polynomial::from(2);
        f0.add_term(
            Rational::from(1),
            Monomial::from(vec![Integer::from(0), Integer::from(2)]),
        );
        f0.add_term(
            Rational::from(-1),
            Monomial::from(vec![Integer::from(0), Integer::from(0)]),
        );

        let mut f1 = Polynomial::from(2);
        f1.add_term(
            Rational::from(1),
            Monomial::from(vec![Integer::from(1), Integer::from(1)]),
        );
        f1.add_term(
            Rational::from(-1),
            Monomial::from(vec![Integer::from(0), Integer::from(0)]),
        );

        let (a, r) = f.polynomial_divide(&vec![f0.clone(), f1.clone()]);

        let correct = &a[0] * &f0 + &a[1] * &f1 + &r;
        assert_eq!(f, correct);

        let lm_r = r.lm();
        let lm_f0 = f0.lm();
        let lm_f1 = f1.lm();

        match (lm_r, lm_f0, lm_f1) {
            (Some(lm_r), Some(lm_f0), Some(lm_f1)) => {
                assert!(!lm_r.is_divisible_by(&lm_f0));
                assert!(!lm_r.is_divisible_by(&lm_f1));
            }
            (_, _, _) => {
                panic!("failed to calc lm");
            }
        }

        // a0
        let mut correct = Polynomial::from(2);
        correct.add_term(
            Rational::from(1),
            Monomial::from(vec![Integer::from(1), Integer::from(0)]),
        );
        correct.add_term(
            Rational::from(1),
            Monomial::from(vec![Integer::from(0), Integer::from(0)]),
        );
        assert_eq!(a[0], correct);

        // a1
        let mut correct = Polynomial::from(2);
        correct.add_term(
            Rational::from(1),
            Monomial::from(vec![Integer::from(1), Integer::from(0)]),
        );
        assert_eq!(a[1], correct);

        // r
        let mut correct = Polynomial::from(2);
        correct.add_term(
            Rational::from(2),
            Monomial::from(vec![Integer::from(1), Integer::from(0)]),
        );
        correct.add_term(
            Rational::from(1),
            Monomial::from(vec![Integer::from(0), Integer::from(0)]),
        );
        assert_eq!(r, correct);
    }

    #[test]
    fn test_s_polynomial() {
        let mut f = Polynomial::from((2, monomial::MonomialOrder::Grlex));
        f.add_term(
            Rational::from(1),
            Monomial::from(vec![Integer::from(3), Integer::from(2)]),
        );
        f.add_term(
            Rational::from(-1),
            Monomial::from(vec![Integer::from(2), Integer::from(3)]),
        );
        f.add_term(
            Rational::from(1),
            Monomial::from(vec![Integer::from(1), Integer::from(0)]),
        );

        let mut g = Polynomial::from((2, monomial::MonomialOrder::Grlex));
        g.add_term(
            Rational::from(3),
            Monomial::from(vec![Integer::from(4), Integer::from(1)]),
        );
        g.add_term(
            Rational::from(1),
            Monomial::from(vec![Integer::from(0), Integer::from(2)]),
        );

        let s_fg = s_polynomial(&f, &g);

        match s_fg {
            Some(s_fg) => {
                println!("S({}, {}) = {}", f, g, s_fg);

                let mut correct = Polynomial::from((2, monomial::MonomialOrder::Grlex));

                correct.add_term(
                    Rational::from(-1),
                    Monomial::from(vec![Integer::from(3), Integer::from(3)]),
                );
                correct.add_term(
                    Rational::from(1),
                    Monomial::from(vec![Integer::from(2), Integer::from(0)]),
                );
                correct.add_term(
                    &Rational::from(-1) / &Rational::from(3),
                    Monomial::from(vec![Integer::from(0), Integer::from(3)]),
                );
                println!("correct {}", correct);

                assert_eq!(s_fg, correct);
            }
            None => {
                panic!("Failed to compute s polynomial.");
            }
        }
    }
}
