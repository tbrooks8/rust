// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!

A Big decimal (signed version: `BigDecimal`, unsigned version: `BigUdecimal`).

A `BigUdecimal` is represented as
A `BigDecimal` is a combination of `BigUdecimal` and `Sign`.
*/

use super::bigint::{BigDigit, BigUint};

#[deriving(Clone)]
pub struct BigUdecimal {
    priv value: ~[BigDigit],
    priv scale: uint,
    priv precision: uint
}

impl Eq for BigUdecimal {
    #[inline]
    fn eq(&self, other: &BigUdecimal) -> bool { self.equals(other) }
}

impl TotalEq for BigUdecimal {
    #[inline]
    fn equals(&self, other: &BigUdecimal) -> bool {
        match self.cmp(other) { Equal => true, _ => false }
    }
}

impl Ord for BigUdecimal {
    #[inline]
    fn lt(&self, other: &BigUdecimal) -> bool {
        match self.cmp(other) {Less => true, _ => false}
    }
}

// Unsure
impl TotalOrd for BigUdecimal {
    #[inline]
    fn cmp(&self, other: &BigUdecimal) -> Ordering {
        let (s_scale, o_scale) = ((self.scale as int), (other.scale as int));
        let s_diff: int = s_scale - (self.value.len() as int);
        let o_diff: int = o_scale - (other.value.len() as int);
        if s_diff < o_diff {
            return Greater;
        } else if s_diff > o_diff {
            return Less;
        } else {
            for (&self_i, &other_i) in self.value.iter().zip(other.value.iter()) {
                println!("{}", BigUint::new(~[0, 1]));

                if self_i < other_i { return Less; }
                if self_i > other_i { return Greater; }
            }
        }
        return Equal;
    }
}


impl BigUdecimal {
    #[inline]
    pub fn new(value: ~[BigDigit], scale: uint, precision: uint) -> BigUdecimal {

        return BigUdecimal { value: value, scale: scale, precision: precision };
    }
}

#[cfg(test)]
mod bigudecimal_tests {
    use super::{BigUdecimal};

    #[test]
    fn test_cmp_bigudecimal1() {
        let (grt, less) = (BigUdecimal::new(~[1, 2, 3], 2, 3), BigUdecimal::new(~[1, 2, 3], 3, 3));
        assert!(grt > less);
    }

    #[test]
    fn test_cmp_bigudecimal2() {
        let (grt, less) = (BigUdecimal::new(~[2, 2, 3], 2, 3), BigUdecimal::new(~[1, 2, 3], 2, 3));
        assert!(grt > less);
    }

    #[test]
    fn test_cmp_bigudecimal3() {
        let (grt, less) = (BigUdecimal::new(~[1, 2, 3, 0], 3, 3), BigUdecimal::new(~[1, 2, 3], 2, 3));
        assert!(grt > less);
    }

}