use crate::monomial::{Monomial, MonomialHandlers, MonomialOrder};
use crate::scalar::{Integer, Rational};
use std::collections::BTreeMap;

use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug)]
pub struct Polynomial {
    terms: BTreeMap<Monomial, Rational>,
    n: usize,
    monomial_order: MonomialOrder,
}

impl From<usize> for Polynomial {
    fn from(n_: usize) -> Self {
        Self {
            terms: BTreeMap::new(),
            n: n_,
            monomial_order: MonomialOrder::Lex,
        }
    }
}

impl From<Vec<Integer>> for Polynomial {
    fn from(v: Vec<Integer>) -> Self {
        let n_ = v.len();
        let x = Monomial::from(v);
        let mut terms_ = BTreeMap::new();
        terms_.insert(x, Rational::from(1));

        Self {
            terms: terms_,
            n: n_,
            monomial_order: MonomialOrder::Lex,
        }
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
        let mut terms_ = BTreeMap::new();
        terms_.insert(v, Rational::from(1));
        Self {
            terms: terms_,
            n: n_,
            monomial_order: MonomialOrder::Lex,
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

impl Display for Polynomial {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();

        let mut front = true;

        for (k, v) in &self.terms {
            if front {
                if v != &Rational::from(0) {
                    comma_separated.push_str(&v.to_string());
                    comma_separated.push_str(&k.to_string());
                    front = true;
                }
            } else {
                if v != &Rational::from(0) {
                    if v > &Rational::from(0) {
                        comma_separated.push_str(" + ");
                        comma_separated.push_str(&v.to_string());
                    } else {
                        comma_separated.push_str(" - ");
                        let mv = &Rational::from(-1) * v;
                        comma_separated.push_str(&mv.to_string());
                    }
                    comma_separated.push_str(&k.to_string());
                }
            }
        }
        if front {
            comma_separated.push_str("0");
        }

        comma_separated.push_str(match self.monomial_order {
            MonomialOrder::Lex => "Lex",
            MonomialOrder::Grlex => "Grlex",
        });

        write!(f, "{}", comma_separated)
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
            ret.add_term(v.clone(), k.clone());
        }

        ret
    }
}

impl<'a, 'b> Sub<&'a Polynomial> for &'b Polynomial {
    type Output = Polynomial;

    fn sub(self, other: &Polynomial) -> Polynomial {
        assert!(self.n == other.n);
        assert!(self.monomial_order == other.monomial_order);

        let mut ret = self.clone();

        for (k, v) in &other.terms {
            ret.add_term(v.clone(), k.clone());
        }

        ret
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
            for (rx, rc) in &other.terms {
                ret.add_term(lc * rc, lx * rx);
            }
        }

        ret
    }
}

trait PolynomialHandlers {
    fn add_monomial(&mut self, x: Monomial);
    fn add_term(&mut self, c: Rational, x: Monomial);
    fn sub_monomial(&mut self, x: Monomial);
    fn sub_term(&mut self, c: Rational, x: Monomial);

    fn polynomial_divide(&self, rhs: &Vec<Polynomial>) -> (Polynomial, Polynomial);

    fn set_monomial_order(&mut self, o: MonomialOrder);
}

impl PolynomialHandlers for Polynomial {
    fn add_term(&mut self, c: Rational, x: Monomial) {
        let res = self.terms.get_mut(&x);

        let new_value = match res {
            Some(c_) => c_.clone() + c,
            None => c,
        };

        self.terms.insert(x, new_value);
    }
    fn add_monomial(&mut self, x: Monomial) {
        self.add_term(Rational::from(1), x);
    }

    fn sub_term(&mut self, c: Rational, x: Monomial) {
        let res = self.terms.get_mut(&x);

        let new_value = match res {
            Some(c_) => c_.clone() - c,
            None => -c,
        };

        self.terms.insert(x, new_value);
    }
    fn sub_monomial(&mut self, x: Monomial) {
        self.sub_term(Rational::from(1), x);
    }

    fn polynomial_divide(&self, rhs: &Vec<Polynomial>) -> (Polynomial, Polynomial) {
        // TODO
        (Polynomial::from(self.n), self.clone())
    }

    fn set_monomial_order(&mut self, o: MonomialOrder) {
        self.monomial_order = o;
    }
}
