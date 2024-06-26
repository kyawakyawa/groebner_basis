use crate::degree::{Degree, Zn};
use crate::scalar::Integer;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{Div, Mul};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MonomialOrder {
    Lex,
    Grlex,
}

#[derive(Clone, Debug)]
pub struct Monomial {
    alpha: Zn,
    n: usize,
    monomial_order: MonomialOrder,
}

impl From<(usize, MonomialOrder)> for Monomial {
    fn from((n_, monomial_order): (usize, MonomialOrder)) -> Self {
        let mut v = Vec::new();
        v.resize(n_, Integer::from(0));

        Monomial::from((v, monomial_order))
    }
}

impl From<Vec<i64>> for Monomial {
    fn from(v: Vec<i64>) -> Self {
        Monomial::from((v, MonomialOrder::Lex))
    }
}

impl From<Vec<Integer>> for Monomial {
    fn from(v: Vec<Integer>) -> Self {
        Monomial::from((v, MonomialOrder::Lex))
    }
}

impl From<(Vec<i64>, MonomialOrder)> for Monomial {
    fn from(pair: (Vec<i64>, MonomialOrder)) -> Self {
        let v = pair
            .0
            .into_iter()
            .map(|i| Integer::from(i))
            .collect::<Vec<_>>();
        Self::from((v, pair.1))
    }
}

impl From<(Vec<Integer>, MonomialOrder)> for Monomial {
    fn from(pair: (Vec<Integer>, MonomialOrder)) -> Self {
        let n_ = pair.0.len();
        Self {
            alpha: Zn::from(pair.0),
            n: n_,
            monomial_order: pair.1,
        }
    }
}

impl Into<Vec<Integer>> for Monomial {
    fn into(self) -> Vec<Integer> {
        self.alpha.into()
    }
}

impl Display for Monomial {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();

        let v: &Vec<Integer> = self.alpha.get_ref_v();

        for i in 0..v.len() {
            if v[i] != Integer::from(0) {
                comma_separated.push_str("(x_");
                comma_separated.push_str(&(i + 1).to_string());
                comma_separated.push_str(")");
                if v[i] != Integer::from(1) {
                    comma_separated.push_str("^");
                    comma_separated.push_str(&v[i].to_string());
                }
            }
        }

        //comma_separated.push_str(match self.monomial_order {
        //    MonomialOrder::Lex => "Lex",
        //    MonomialOrder::Grlex => "Grlex",
        //});

        write!(f, "{}", comma_separated)
    }
}

fn lex(lhs: &Vec<Integer>, rhs: &Vec<Integer>) -> Ordering {
    let n = std::cmp::min(lhs.len(), rhs.len());

    for i in 0..n {
        if lhs[i] != rhs[i] {
            return lhs[i].cmp(&rhs[i]);
        }
    }

    lhs.len().cmp(&rhs.len())
}

fn grlex(lhs: &Vec<Integer>, rhs: &Vec<Integer>) -> Ordering {
    let l_sum: Integer = lhs.iter().fold(Integer::from(0), |sum, a| sum + a);
    let r_sum: Integer = rhs.iter().fold(Integer::from(0), |sum, a| sum + a);
    if l_sum != r_sum {
        return l_sum.cmp(&r_sum);
    }
    lex(lhs, rhs)
}

impl<'a, 'b> Mul<&'a Monomial> for &'b Monomial {
    type Output = Monomial;

    fn mul(self, other: &Monomial) -> Monomial {
        assert!(self.n == other.n);
        assert!(self.monomial_order == other.monomial_order);

        Monomial {
            alpha: &self.alpha + &other.alpha,
            n: self.n,
            monomial_order: self.monomial_order,
        }
    }
}

impl<'a, 'b> Div<&'a Monomial> for &'b Monomial {
    type Output = Monomial;

    fn div(self, other: &Monomial) -> Monomial {
        assert!(self.n == other.n);
        assert!(self.monomial_order == other.monomial_order);

        Monomial {
            alpha: &self.alpha - &other.alpha,
            n: self.n,
            monomial_order: self.monomial_order,
        }
    }
}

impl PartialEq for Monomial {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.monomial_order == other.monomial_order);
        self.cmp(&other) == Ordering::Equal
    }
}

impl Eq for Monomial {}

impl PartialOrd for Monomial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Monomial {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.monomial_order {
            MonomialOrder::Lex => lex(self.alpha.get_ref_v(), other.alpha.get_ref_v()),
            MonomialOrder::Grlex => grlex(self.alpha.get_ref_v(), other.alpha.get_ref_v()),
        }
    }
}

pub trait MonomialHandlers {
    fn set_monomial_order(&mut self, o: MonomialOrder);
    fn get_monomial_order(&self) -> MonomialOrder;
    fn get_n(&self) -> usize;
    fn is_divisible_by(&self, rhs: &Monomial) -> bool;

    fn fetch_total_degree(&self) -> Integer;
}

impl MonomialHandlers for Monomial {
    fn set_monomial_order(&mut self, o: MonomialOrder) {
        self.monomial_order = o;
    }
    fn get_monomial_order(&self) -> MonomialOrder {
        self.monomial_order
    }
    fn get_n(&self) -> usize {
        self.n
    }
    fn is_divisible_by(&self, rhs: &Monomial) -> bool {
        assert_eq!(self.get_n(), rhs.get_n());

        let lv = self.alpha.get_ref_v();
        let rv = rhs.alpha.get_ref_v();
        for i in 0..(lv.len()) {
            if lv[i] < rv[i] {
                return false;
            }
        }
        true
    }

    fn fetch_total_degree(&self) -> Integer {
        self.alpha
            .get_ref_v()
            .iter()
            .fold(Integer::from(0), |sum, a| sum + a)
    }
}

pub fn lcm(a: &Monomial, b: &Monomial) -> Monomial {
    assert_eq!(a.n, b.n);
    assert_eq!(a.monomial_order, b.monomial_order);

    let va = a.alpha.get_ref_v();
    let vb = b.alpha.get_ref_v();

    assert_eq!(va.len(), vb.len());

    let mut v = Vec::new();
    for (ai, bi) in va.iter().zip(vb.iter()) {
        v.push(std::cmp::max(ai, bi).clone());
    }

    Monomial::from((v, a.monomial_order))
}
