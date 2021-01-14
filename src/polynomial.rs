use crate::monomial::{Monomial, MonomialHandlers, MonomialOrder};
use crate::scalar::{Integer, Rational};
use crate::{monomial, scalar};
use std::collections::BTreeMap;

use std::cmp::Ordering;
use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Clone, Debug)]
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
        let monomial = Monomial::from((pair.0, pair.1));
        Self::from(monomial)
    }
}

impl From<Monomial> for Polynomial {
    fn from(v: Monomial) -> Self {
        let monomial_order = v.get_monomial_order();
        Polynomial::from((Rational::from(1), v, monomial_order))
    }
}

impl From<(Monomial, MonomialOrder)> for Polynomial {
    fn from(pair: (Monomial, MonomialOrder)) -> Self {
        Self::from((Rational::from(1), pair.0, pair.1))
    }
}

impl From<(Rational, usize, MonomialOrder)> for Polynomial {
    fn from(pair: (Rational, usize, MonomialOrder)) -> Self {
        let mut v = Vec::new();
        v.resize(pair.1, Integer::from(0));

        Self::from((pair.0, Monomial::from((v, pair.2)), pair.2))
    }
}

impl From<(Rational, usize)> for Polynomial {
    fn from(pair: (Rational, usize)) -> Self {
        let mut v = Vec::new();
        v.resize(pair.1, Integer::from(0));

        let monomial_order = MonomialOrder::Lex;
        Self::from((pair.0, Monomial::from((v, monomial_order)), monomial_order))
    }
}

impl From<(Rational, Monomial, MonomialOrder)> for Polynomial {
    fn from(tuple: (Rational, Monomial, MonomialOrder)) -> Self {
        let n_ = tuple.1.get_n();
        let mut ret = Self {
            terms: BTreeMap::new(),
            n: n_,
            monomial_order: tuple.2,
        };

        let mut monomial = tuple.1;
        monomial.set_monomial_order(tuple.2);

        ret.add_term(tuple.0, monomial);

        ret
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut output = String::new();

        let mut front = true;

        let terms = self.terms.iter().collect::<Vec<(&Monomial, &Rational)>>();

        for it in terms.iter().rev() {
            let monomial = it.0;
            let coeff = it.1;

            let is_show_abs_coeff = |abs_coeff: &Rational, monomial: &Monomial| {
                // 定数項であるまたは係数の絶対値が1で無い時
                monomial.fetch_total_degree() == Integer::from(0) || abs_coeff != &Rational::from(1)
            };

            match coeff.cmp(&Rational::from(0)) {
                Ordering::Equal => {
                    continue;
                }
                Ordering::Greater => {
                    if !front {
                        output.push_str(" + ");
                    }
                }
                Ordering::Less => {
                    output.push_str(" - ");
                }
            };
            front = false;

            let abs_coeff = coeff.abs();
            if is_show_abs_coeff(&abs_coeff, monomial) {
                output.push_str(&abs_coeff.to_string());
            }
            output.push_str(&monomial.to_string());
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
// TODO 共通化
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
        assert!(self.n == other.n);
        assert!(self.monomial_order == other.monomial_order);

        let mut ret = self;

        for (k, v) in other.terms.iter() {
            assert_ne!(v, &Rational::from(0));
            ret.add_term(v.clone(), k.clone());
        }

        ret
    }
}

impl Add<Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Polynomial {
        assert!(self.n == other.n);
        assert!(self.monomial_order == other.monomial_order);

        let mut ret = other;

        for (k, v) in self.terms.iter() {
            assert_ne!(v, &Rational::from(0));
            ret.add_term(v.clone(), k.clone());
        }

        ret
    }
}

impl Add<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Polynomial {
        assert!(self.n == other.n);
        assert!(self.monomial_order == other.monomial_order);

        let (longer, shorter) = match self.terms.len() >= other.terms.len() {
            true => (self, other),
            false => (other, self),
        };

        let mut ret = longer;

        for (k, v) in shorter.terms {
            assert_ne!(v, Rational::from(0));
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

        for (k, v) in other.terms.iter() {
            assert_ne!(v, &Rational::from(0));
            ret.sub_term(v.clone(), k.clone());
        }

        ret
    }
}

impl Sub<&Polynomial> for Polynomial {
    type Output = Polynomial;

    fn sub(self, other: &Polynomial) -> Polynomial {
        assert!(self.n == other.n);
        assert!(self.monomial_order == other.monomial_order);

        let mut ret = self;

        for (k, v) in other.terms.iter() {
            assert_ne!(v, &Rational::from(0));
            ret.sub_term(v.clone(), k.clone());
        }

        ret
    }
}

impl Sub<Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn sub(self, other: Polynomial) -> Polynomial {
        // TODO clone を無くす
        self - &other
    }
}

impl Sub<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn sub(self, other: Polynomial) -> Polynomial {
        assert!(self.n == other.n);
        assert!(self.monomial_order == other.monomial_order);

        let mut ret = self;

        for (k, v) in other.terms {
            assert_ne!(v, Rational::from(0));
            ret.sub_term(v, k);
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

impl AddAssign for Polynomial {
    fn add_assign(&mut self, other: Self) {
        for (k, v) in other.terms {
            assert_ne!(v, Rational::from(0));
            self.add_term(v, k);
        }
    }
}

impl AddAssign<&Polynomial> for Polynomial {
    fn add_assign(&mut self, other: &Self) {
        for (k, v) in other.terms.iter() {
            assert_ne!(v, &Rational::from(0));
            self.add_term(v.clone(), k.clone());
        }
    }
}

impl SubAssign for Polynomial {
    fn sub_assign(&mut self, other: Self) {
        for (k, v) in other.terms {
            assert_ne!(v, Rational::from(0));
            self.sub_term(v, k);
        }
    }
}

impl SubAssign<&Polynomial> for Polynomial {
    fn sub_assign(&mut self, other: &Self) {
        for (k, v) in other.terms.iter() {
            assert_ne!(v, &Rational::from(0));
            self.sub_term(v.clone(), k.clone());
        }
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.n, other.n);
        assert_eq!(self.monomial_order, other.monomial_order);
        self.terms.eq(&other.terms)
    }
}

impl Eq for Polynomial {}

pub trait PolynomialHandlers {
    fn add_monomial(&mut self, x: Monomial);
    fn add_term(&mut self, c: Rational, x: Monomial);
    fn sub_monomial(&mut self, x: Monomial);
    fn sub_term(&mut self, c: Rational, x: Monomial);

    fn polynomial_divide(&self, rhses: &Vec<Polynomial>) -> (Vec<Polynomial>, Polynomial);
    fn polynomial_divide_ref(&self, rhses: &Vec<&Polynomial>) -> (Vec<Polynomial>, Polynomial);

    fn get_n(&self) -> usize;
    fn get_monomial_order(&self) -> MonomialOrder;

    fn set_monomial_order(&mut self, o: MonomialOrder);

    fn fetch_lt(&self) -> Option<Polynomial>;
    fn fetch_lm(&self) -> Option<Monomial>;
    fn fetch_lc(&self) -> Option<Rational>;

    fn fetch_total_degree(&self) -> Option<Integer>;

    fn normalize(self) -> Self;

    fn integer_coeff(self) -> Self;
}

impl PolynomialHandlers for Polynomial {
    fn add_term(&mut self, c: Rational, x: Monomial) {
        // fix monomial order
        let mut x = x;
        x.set_monomial_order(self.monomial_order);

        let res = self.terms.get_mut(&x);

        let new_coeff = match res {
            Some(c_) => {
                let c_: &Rational = c_;
                let mut c = c;
                c += c_;
                c
            }
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
            Some(c_) => {
                let c_: &Rational = c_;
                let mut c = c;
                c *= Rational::from(-1);
                c += c_;
                c
            }
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
        self.polynomial_divide_ref(&rhses.iter().map(|f| f).collect::<Vec<&Polynomial>>())
    }

    fn polynomial_divide_ref(&self, rhses: &Vec<&Polynomial>) -> (Vec<Polynomial>, Polynomial) {
        let monomial_order = self.monomial_order;
        let n = self.n;

        let mut p = self.clone();
        let zero = Polynomial::from((n, monomial_order));

        let s = rhses.len();

        let mut a = Vec::new();
        a.resize(s, zero.clone());
        let mut r = zero.clone();

        while &p != &zero {
            let mut divisionoccurred = false;

            for i in 0..s {
                let fi = rhses[i];
                let lm_fi = fi.fetch_lm();
                let lm_p = p.fetch_lm();

                let lm_pair = (lm_p, lm_fi);

                match lm_pair {
                    (Some(lm_p), Some(lm_fi)) => {
                        if lm_p.is_divisible_by(&lm_fi) {
                            let lc_fi = fi.fetch_lc();
                            let lc_p = p.fetch_lc();

                            let lc_pair = (lc_p, lc_fi);
                            match lc_pair {
                                (Some(lc_p), Some(lc_fi)) => {
                                    let d = Polynomial::from((
                                        lc_p / lc_fi,
                                        &lm_p / &lm_fi,
                                        monomial_order,
                                    ));

                                    p -= &d * fi;
                                    a[i] += d;
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
                        panic!("found 0 polynomial\n p -> {:?}\n fi -> {:?},", p, fi);
                    }
                }
            }
            if !divisionoccurred {
                let lt_p = p.fetch_lt();
                match lt_p {
                    Some(lt_p) => {
                        p -= &lt_p;
                        r += lt_p;
                    }
                    None => {
                        assert!(false);
                    }
                }
            }
        }

        (a, r)
    }

    fn get_n(&self) -> usize {
        self.n
    }

    fn get_monomial_order(&self) -> MonomialOrder {
        self.monomial_order
    }

    fn set_monomial_order(&mut self, o: MonomialOrder) {
        self.monomial_order = o;
        // TODO termsにも伝搬させる
    }

    fn fetch_lt(&self) -> Option<Polynomial> {
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

    fn fetch_lm(&self) -> Option<Monomial> {
        let last = self.terms.iter().last();
        match last {
            Some((monomial, _)) => Some(monomial.clone()),
            None => None,
        }
    }

    fn fetch_lc(&self) -> Option<Rational> {
        let last = self.terms.iter().last();
        match last {
            Some((_, coeff)) => Some(coeff.clone()),
            None => None,
        }
    }

    fn fetch_total_degree(&self) -> Option<Integer> {
        if self.terms.is_empty() {
            None
        } else {
            Some(
                self.terms
                    .iter()
                    .map(|(x, _)| x.fetch_total_degree())
                    .fold(Integer::from(0), |mx, d| std::cmp::max(mx, d)),
            )
        }
    }

    fn normalize(self) -> Self {
        let mut f = self;
        let lc = f.fetch_lc();
        match lc {
            Some(lc) => {
                f.terms = f
                    .terms
                    .iter()
                    .map(|(monomial, coeff)| (monomial.clone(), coeff / &lc))
                    .collect::<BTreeMap<Monomial, Rational>>();
                f
            }
            None => f,
        }
    }
    fn integer_coeff(self) -> Self {
        if self.terms.is_empty() {
            return self;
        }
        let lcm_den =
            self.terms
                .iter()
                .map(|(_, coeff)| coeff)
                .fold(Integer::from(1), |lcm_den, a| {
                    let den = a.get_den();

                    scalar::lcm(&lcm_den, &den)
                });

        let lcm_den = Rational::from(lcm_den);

        let it = self
            .terms
            .into_iter()
            .map(|(term, coeff)| (term, coeff * &lcm_den));

        let mut ret = Polynomial::from((self.n, self.monomial_order));

        for pair in it {
            ret.add_term(pair.1, pair.0);
        }

        ret
    }
}

pub fn s_polynomial(f: &Polynomial, g: &Polynomial) -> Option<Polynomial> {
    assert_eq!(f.n, g.n);
    assert_eq!(f.monomial_order, g.monomial_order);
    let lm_f = f.fetch_lm();
    let lm_g = g.fetch_lm();

    match (lm_f, lm_g) {
        (Some(lm_f), Some(lm_g)) => {
            let lcm_fg = monomial::lcm(&lm_f, &lm_g);

            let lc_f = f.fetch_lc();
            let lc_g = g.fetch_lc();
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

mod test;
