use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::{Add, Sub};

use crate::scalar::Integer;

#[derive(Clone, Debug)]
pub struct Zn {
    v: Vec<Integer>,
}

impl From<Vec<Integer>> for Zn {
    fn from(v: Vec<Integer>) -> Self {
        Self { v }
    }
}

impl Into<Vec<Integer>> for Zn {
    fn into(self) -> Vec<Integer> {
        self.v
    }
}

impl Display for Zn {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();

        for i in 0..self.v.len() {
            comma_separated.push_str(&self.v[i].to_string());
            if i < self.v.len() - 1 {
                comma_separated.push_str(", ");
            } else {
                comma_separated.push_str(" ");
            }
        }

        write!(f, "{}", comma_separated)
    }
}

impl Add<Zn> for Zn {
    type Output = Zn;

    fn add(self, other: Zn) -> Zn {
        let mut ret = self.v.clone();

        let n = std::cmp::min(ret.len(), other.v.len());

        for i in 0..n {
            ret[i] = &ret[i] + &other.v[i];
        }

        Zn { v: ret }
    }
}

impl<'a, 'b> Add<&'a Zn> for &'b Zn {
    type Output = Zn;

    fn add(self, other: &Zn) -> Zn {
        let mut ret = self.v.clone();

        let n = std::cmp::min(ret.len(), other.v.len());

        for i in 0..n {
            ret[i] = &ret[i] + &other.v[i];
        }

        Zn { v: ret }
    }
}

impl Sub for Zn {
    type Output = Zn;

    fn sub(self, other: Zn) -> Zn {
        let mut ret = self.v.clone();

        let n = std::cmp::min(ret.len(), other.v.len());

        for i in 0..n {
            ret[i] = &ret[i] - &other.v[i];
        }

        Zn { v: ret }
    }
}

impl<'a, 'b> Sub<&'a Zn> for &'b Zn {
    type Output = Zn;

    fn sub(self, other: &Zn) -> Zn {
        let mut ret = self.v.clone();

        let n = std::cmp::min(ret.len(), other.v.len());

        for i in 0..n {
            ret[i] = &ret[i] - &other.v[i];
        }

        Zn { v: ret }
    }
}

impl PartialEq for Zn {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.v.len() == other.v.len());
        if self.v.len() != other.v.len() {
            return false;
        }
        let n = self.v.len();

        for i in 0..n {
            if self.v[i] != other.v[i] {
                return false;
            }
        }
        true
    }
}
impl Eq for Zn {}

pub trait Degree {
    fn get_ref_v<'a>(self: &'a Self) -> &'a Vec<Integer>;
    fn dim(&self) -> usize;
}

impl Degree for Zn {
    fn get_ref_v<'a>(self: &'a Self) -> &'a Vec<Integer> {
        &(self.v)
    }
    fn dim(&self) -> usize {
        return self.v.len();
    }
}
