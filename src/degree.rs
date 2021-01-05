use std::fmt::Display;
use std::ops::{Add, Sub};

use crate::scalar::Integer;

trait Degree:
    Clone
    + From<Vec<Integer>>
    + Into<Vec<Integer>>
    + Display
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + Add
    + Sub
{
}

struct ZLex {
    alpha: Vec<Integer>,
}

// impl Degree for ZLex {}
