// we want to be able to add and multiply Bigs
// modular arithmetic is important
// want a way to convert ints into bigs
// want to randomly generate Bigs

use rand::prelude::*;
use std::fmt;
use std::ops;
use std::cmp::Ordering;

struct Big([bool; 1024]);

impl Big {
    // s is the length of the number in bits
    fn random(s: Option<usize>) -> Big {
        let mut arr: [bool; 1024] = [false; 1024];
        // this part doesn't feel idiomatic
        let mut a = 0;
        if let Some(i) = s {
            a = 1024 - i
        }
        for x in a..1024 {
            arr[x] = rand::random();
        }
        Big(arr)
    }

    fn zero() -> Big {
        Big([false; 1024])
    }
}

impl fmt::Debug for Big {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sting = "".to_string();
        for x in self.0.iter() {
            if *x {
                sting.push_str("1");
            } else {
                sting.push_str("0");
            }
        }
        f.debug_struct("Big").field("num", &sting).finish()
    }
}

impl ops::Add<Big> for Big {
    type Output = Big;
    // want to write a full adder
    fn add(self, rhs: Big) -> Big {
        let mut carry = false;
        let mut res = Big::zero();
        let mut inter = false;
        for x in (0..1024).rev() {
            inter = self.0[x] ^ rhs.0[x];
            res.0[x] = carry ^ inter;
            carry = (self.0[x] & rhs.0[x]) | (inter & carry);
        }
        if carry {
            // if the carry digit at the end is true then we've overflown
            panic!("Overflow");
        }
        res
    }
}

impl ops::Rem<Big> for Big {
    type Output = Big;

    fn rem(self, modulus: Big) -> Big {
        if modulus > self {
            self
        } else if modulus == self {
            Big([false; 1024])
        } else {
            self
        }
    }
}

impl PartialOrd for Big {
    fn partial_cmp(&self, other: &Big) -> Option<Ordering> {
        for x in 0..1024 {
            if self.0[x] & !other.0[x] {
                return Some(Ordering::Greater);
            } else if !self.0[x] & other.0[x] {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }
}

impl PartialEq for Big {
    fn eq(&self, other: &Big) -> bool {
        for x in 0..1024 {
            if self.0[x] ^ other.0[x] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let this = Big::random(Some(3));
    let that = Big::random(Some(3));
    println!("{:#?}", this);
    println!("{:#?}", that);
    println!("{:#?}", this + that);
}
