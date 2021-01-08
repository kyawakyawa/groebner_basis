use std::cmp::Ordering;
use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{Add, Sub};

use crate::scalar::Integer;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MonomialOrder {
    Lex,
    GrLex,
}

#[derive(Clone, Debug)]
pub struct Zn {
    alpha: Vec<Integer>,
    monomial_order: MonomialOrder,
}

impl From<Vec<Integer>> for Zn {
    fn from(v: Vec<Integer>) -> Self {
        Self {
            alpha: v,
            monomial_order: MonomialOrder::Lex,
        }
    }
}
impl From<(Vec<Integer>, MonomialOrder)> for Zn {
    fn from(pair: (Vec<Integer>, MonomialOrder)) -> Self {
        Self {
            alpha: pair.0,
            monomial_order: pair.1,
        }
    }
}

impl Into<Vec<Integer>> for Zn {
    fn into(self) -> Vec<Integer> {
        self.alpha
    }
}

impl Display for Zn {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();

        for i in 0..self.alpha.len() {
            comma_separated.push_str(&self.alpha[i].to_string());
            if i < self.alpha.len() - 1 {
                comma_separated.push_str(", ");
            } else {
                comma_separated.push_str(" ");
            }
        }

        comma_separated.push_str(match self.monomial_order {
            MonomialOrder::Lex => "Lex",
            MonomialOrder::GrLex => "grLex",
        });

        write!(f, "{}", comma_separated)
    }
}

impl Add<Zn> for Zn {
    type Output = Zn;

    fn add(self, other: Zn) -> Zn {
        let mut ret = self.alpha.clone();

        let n = std::cmp::min(ret.len(), other.alpha.len());

        for i in 0..n {
            ret[i] = &ret[i] + &other.alpha[i];
        }

        Zn {
            alpha: ret,
            monomial_order: self.monomial_order,
        }
    }
}

impl<'a, 'b> Add<&'a Zn> for &'b Zn {
    type Output = Zn;

    fn add(self, other: &Zn) -> Zn {
        let mut ret = self.alpha.clone();

        let n = std::cmp::min(ret.len(), other.alpha.len());

        for i in 0..n {
            ret[i] = &ret[i] + &other.alpha[i];
        }

        Zn {
            alpha: ret,
            monomial_order: self.monomial_order,
        }
    }
}

impl Sub for Zn {
    type Output = Zn;

    fn sub(self, other: Zn) -> Zn {
        let mut ret = self.alpha.clone();

        let n = std::cmp::min(ret.len(), other.alpha.len());

        for i in 0..n {
            ret[i] = &ret[i] - &other.alpha[i];
        }

        Zn {
            alpha: ret,
            monomial_order: self.monomial_order,
        }
    }
}

impl<'a, 'b> Sub<&'a Zn> for &'b Zn {
    type Output = Zn;

    fn sub(self, other: &Zn) -> Zn {
        let mut ret = self.alpha.clone();

        let n = std::cmp::min(ret.len(), other.alpha.len());

        for i in 0..n {
            ret[i] = &ret[i] - &other.alpha[i];
        }

        Zn {
            alpha: ret,
            monomial_order: self.monomial_order,
        }
    }
}

impl PartialEq for Zn {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.alpha.len() == other.alpha.len());
        if self.alpha.len() != other.alpha.len() {
            return false;
        }
        let n = self.alpha.len();

        for i in 0..n {
            if self.alpha[i] != other.alpha[i] {
                return false;
            }
        }
        true
    }
}
impl Eq for Zn {}

fn lex(lhs: &Zn, rhs: &Zn) -> Ordering {
    let n = std::cmp::min(lhs.alpha.len(), rhs.alpha.len());

    for i in 0..n {
        if lhs.alpha[i] != rhs.alpha[i] {
            return lhs.alpha[i].cmp(&rhs.alpha[i]);
        }
    }

    lhs.alpha.len().cmp(&rhs.alpha.len())
}

fn grlex(lhs: &Zn, rhs: &Zn) -> Ordering {
    let l_sum: Integer = lhs.alpha.iter().fold(Integer::from(0), |sum, a| sum + a);
    let r_sum: Integer = rhs.alpha.iter().fold(Integer::from(0), |sum, a| sum + a);
    if l_sum != r_sum {
        return l_sum.cmp(&r_sum);
    }
    lex(lhs, rhs)
}

impl PartialOrd for Zn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Zn {
    fn cmp(&self, other: &Self) -> Ordering {
        assert!(self.monomial_order == other.monomial_order);
        match self.monomial_order {
            MonomialOrder::Lex => lex(self, other),
            MonomialOrder::GrLex => grlex(self, other),
        }
    }
}
