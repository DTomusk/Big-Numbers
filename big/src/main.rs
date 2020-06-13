// we want to be able to add and multiply Bigs
// modular arithmetic is important
// want a way to convert ints into bigs
// want to randomly generate Bigs
// first digit will be the sign

use std::fmt;
use std::ops;
use std::cmp::Ordering;
use std::convert::TryInto;

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

    fn random_odd(s: Option<usize>) -> Big {
        let mut b = Big::random(s);
        b.0[1023] = true;
        b
    }

    fn zero() -> Big {
        Big([false; 1024])
    }

    // can only be used on numbers of limited size
    fn print_decimal(&self) {
        let mut i: i64 = 0;
        for x in 0..1024 {
            if self.0[x] {
                i += 2_i64.pow((1023-x).try_into().unwrap());
            }
        }
        println!("{:?}", i);
    }

    fn complement(&self) -> Big {
        let mut comp = Big(self.0.clone());
        let mut temp = Big([false; 1024]);
        temp.0[1023] = true;
        for x in 0..1024 {
            if comp.0[x] {
                comp.0[x] = false;
            } else {
                comp.0[x] = true;
            }
        }
        comp + temp
    }
}

// need to modify to show negative sign
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

impl ops::Add for Big {
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
            //panic!("Overflow");
        }
        res
    }
}

impl ops::Sub for Big {
    type Output = Big;

    fn sub(self, rhs: Big) -> Big {
        self + rhs.complement()
    }
}

impl ops::Rem for Big {
    type Output = Big;

    fn rem(self, modulus: Big) -> Big {
        if modulus > self {
            self
        } else if modulus == self {
            Big([false; 1024])
        } else {
            let mut temp = self - modulus;
            while temp > modulus {
                temp = temp - modulus;
            }
            // not quite right yet
            temp
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
            // if any bit differs then not equal
            if self.0[x] ^ other.0[x] {
                return false;
            }
        }
        true
    }
}

impl Copy for Big {}

impl Clone for Big {
    fn clone(&self) -> Self {
        Big(self.0.clone())
    }
}

fn main() {
    let those = Big::random_odd(Some(50));
    those.print_decimal();
    println!("{:#?}", those);
    let comp = those.complement();
    println!("{:#?}", comp);
    // should be zero (these things should be tests)
    println!("{:#?}", comp + those);
}
