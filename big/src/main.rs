// we want to be able to add and multiply Bigs
// modular arithmetic is important
// want a way to convert ints into bigs
// want to randomly generate Bigs
// first digit will be the sign

// I have a sneaking suspicion that none of this is very memory efficient

use std::fmt;
use std::ops;
use std::cmp::Ordering;
use std::convert::TryInto;

const BIGSIZE: usize = 256;

struct Big([bool; BIGSIZE]);

impl Big {
    // s is the length of the number in bits
    fn random(s: Option<usize>) -> Big {
        let mut arr: [bool; BIGSIZE] = [false; BIGSIZE];
        // this part doesn't feel idiomatic
        let mut a = 0;
        if let Some(i) = s {
            a = BIGSIZE - i
        }
        for x in a..BIGSIZE {
            arr[x] = rand::random();
        }
        Big(arr)
    }

    fn random_odd(s: Option<usize>) -> Big {
        let mut b = Big::random(s);
        b.0[BIGSIZE-1] = true;
        b
    }

    fn zero() -> Big {
        Big([false; BIGSIZE])
    }

    fn big_to_int(&self) -> i64 {
        let mut i: i64 = 0;
        for x in 0..BIGSIZE {
            if self.0[x] {
                i += 2_i64.pow((BIGSIZE-1-x).try_into().unwrap());
            }
        }
        i
    }

    fn int_to_big(mut int: i64) -> Big {
        let mut temp = Big::zero();
        let mut i = BIGSIZE-1;
        while int > 0 {
            if int % 2 == 1 {
                temp.0[i] = true;
            }
            int /= 2;
            i -= 1;
        }
        temp
    }

    fn complement(&self) -> Big {
        let mut comp = Big(self.0.clone());
        let mut temp = Big([false; BIGSIZE]);
        temp.0[BIGSIZE-1] = true;
        for x in 0..BIGSIZE {
            if comp.0[x] {
                comp.0[x] = false;
            } else {
                comp.0[x] = true;
            }
        }
        comp + temp
    }

    // the same as multiplying by two a number of times
    // have to take into account sign bit
    fn shift(&self, times: usize) -> Big {
        let mut new = Big::zero();
        for x in times..BIGSIZE {
            new.0[x-times] = self.0[x];
        }
        new
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
        for x in (0..BIGSIZE).rev() {
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

impl ops::Mul for Big {
    type Output = Big;

    fn mul(self, rhs: Big) -> Big {
        // for every one in rhs create a temp of self shifted that many times across and add it to the running total
        let mut total = Big::zero();
        for (i, x) in rhs.0.iter().enumerate() {
            if *x {
                println!("{:?}", i);
                let temp = self.shift(BIGSIZE-i-1);
                println!("{:#?}", temp);
                total = total + temp;
            }
        }
        total
    }
}

impl ops::Rem for Big {
    type Output = Big;

    fn rem(self, modulus: Big) -> Big {
        if modulus == Big([false; BIGSIZE]) {
            panic!("Mod 0 is undefined");
        }
        if modulus > self {
            self
        } else if modulus == self {
            Big([false; BIGSIZE])
        } else {
            // i feel this might be the cause of the problems
            let mut temp = self - modulus;
            while temp >= modulus {
                temp = temp - modulus;
            }
            temp
        }
    }
}

impl PartialOrd for Big {
    fn partial_cmp(&self, other: &Big) -> Option<Ordering> {
        for x in 0..BIGSIZE {
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
        for x in 0..BIGSIZE {
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
    let this = Big::random(Some(3));
    let that = Big::random(Some(3));
    println!("{:#?}", this);
    println!("{:#?}", that);
    println!("{:#?}", this * that);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complement() {
        let this = Big::random(None);
        assert_eq!(this + this.complement(), Big::zero());
    }

    #[test]
    fn test_conversion() {
        // test ints should be randomly chosen
        let i: i64 = 154329877;
        assert_eq!(i, Big::int_to_big(i).big_to_int());
    }

    #[test]
    fn test_mul() {
        let this = Big::random(Some(10));
        let that = Big::random(Some(10));
        assert_eq!(this * that, Big::int_to_big(this.big_to_int() * that.big_to_int()));
    }

    #[test]
    fn test_mod() {
        let i: i64 = 120;
        let j: i64 = 7;
        assert_eq!(i % j, (Big::int_to_big(i) % Big::int_to_big(j)).big_to_int());
    }
}
