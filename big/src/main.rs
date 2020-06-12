// we want to be able to add and multiply Bigs
// modular arithmetic is important
// want a way to convert ints into bigs
// want to randomly generate Bigs

use rand::prelude::*;
use std::fmt;
use std::ops;

struct Big {
    // use bools to simulate bits
    // num stores a 1024 "bit" number
    num: [bool; 1024],
}

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
        Big {
            num: arr,
        }
    }

    fn zero() -> Big {
        Big { num: [false; 1024] }
    }
}

impl fmt::Debug for Big {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sting = "".to_string();
        for x in self.num.iter() {
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
            inter = self.num[x] ^ rhs.num[x];
            res.num[x] = carry ^ inter;
            carry = (self.num[x] & rhs.num[x]) | (inter & carry);
        }
        if carry {
            panic!("Overflow");
        }
        res
    }
}

fn main() {
    let this = Big::random(Some(3));
    let that = Big::random(Some(3));
    println!("{:#?}", this);
    println!("{:#?}", that);
    println!("{:#?}", this + that);
}
