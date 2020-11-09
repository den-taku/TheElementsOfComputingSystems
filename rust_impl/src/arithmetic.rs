#![allow(dead_code, non_snake_case)]

use crate::logic::*;
// use crate::logic::Bit::{I, O};

// a, b -> curry, sum
pub fn HalfAdder(a: Bit, b: Bit) -> [Bit; 2] {
    [
        And(
            a,
            b
        ),
        Xor(
            a,
            b
        )
    ]
}

// a, b, curry -> curry, sum
// c = a * b + b * curry + curry * a
pub fn FullAdder(a: Bit, b: Bit, c: Bit) -> [Bit; 2] {
    let half_adder1 = HalfAdder(
        a,
        b
    );
    let half_adder2 = HalfAdder(
        c,
        half_adder1[1]
    );
    [
        Or(
            half_adder1[0],
            half_adder2[0]
        ),
        half_adder2[1]
    ]
}

pub fn Add16(a: Word, b: Word) -> Word {
    let fulladder15 = HalfAdder(
        a[15],
        b[15]
    );
    let fulladder14 = FullAdder(
        a[14],
        b[14],
        fulladder15[0]
    );
    let fulladder13 = FullAdder(
        a[13],
        b[13],
        fulladder14[0]
    );
    let fulladder12 = FullAdder(
        a[12],
        b[12],
        fulladder13[0]
    );
    let fulladder11 = FullAdder(
        a[11],
        b[11],
        fulladder12[0]
    );
    let fulladder10 = FullAdder(
        a[10],
        b[10],
        fulladder11[0]
    );
    let fulladder9 = FullAdder(
        a[9],
        b[9],
        fulladder10[0]
    );
    let fulladder8 = FullAdder(
        a[8],
        b[8],
        fulladder9[0]
    );
    let fulladder7 = FullAdder(
        a[7],
        b[7],
        fulladder8[0]
    );
    let fulladder6 = FullAdder(
        a[6],
        b[6],
        fulladder7[0]
    );
    let fulladder5 = FullAdder(
        a[5],
        b[5],
        fulladder6[0]
    );
    let fulladder4 = FullAdder(
        a[4],
        b[4],
        fulladder5[0]
    );
    let fulladder3 = FullAdder(
        a[3],
        b[3],
        fulladder4[0]
    );
    let fulladder2 = FullAdder(
        a[2],
        b[2],
        fulladder3[0]
    );
    let fulladder1 = FullAdder(
        a[1],
        b[1],
        fulladder2[0]
    );
    let fulladder0 = FullAdder(
        a[0],
        b[0],
        fulladder1[0]
    );
    Word::new([
        fulladder0[1],
        fulladder1[1],
        fulladder2[1],
        fulladder3[1],
        fulladder4[1],
        fulladder5[1],
        fulladder6[1],
        fulladder7[1],
        fulladder8[1],
        fulladder9[1],
        fulladder10[1],
        fulladder11[1],
        fulladder12[1],
        fulladder13[1],
        fulladder14[1],
        fulladder15[1],
    ])
}

#[cfg(test)]
mod tests{
    use crate::logic::Word;
    use crate::logic::Bit::{I, O};
    use super::{HalfAdder, FullAdder, Add16};

    #[test]
    fn for_halfadder() {
        assert_eq!(HalfAdder(O, O), [O, O]);
        assert_eq!(HalfAdder(O, I), [O, I]);
        assert_eq!(HalfAdder(I, O), [O, I]);
        assert_eq!(HalfAdder(I, I), [I, O]);
    }

    #[test]
    fn for_fulladder() {
        assert_eq!(FullAdder(O, O, O), [O, O]);
        assert_eq!(FullAdder(O, O, I), [O, I]);
        assert_eq!(FullAdder(O, I, O), [O, I]);
        assert_eq!(FullAdder(O, I, I), [I, O]);
        assert_eq!(FullAdder(I, O, O), [O, I]);
        assert_eq!(FullAdder(I, O, I), [I, O]);
        assert_eq!(FullAdder(I, I, O), [I, O]);
        assert_eq!(FullAdder(I, I, I), [I, I]);
    }

    #[test]
    fn for_add16() {
        assert_eq!(
            Add16(
                Word::new([I; 16]), 
                Word::new([O; 16])),
            Word::new([I; 16])
        );
        assert_eq!(
            Add16(
                Word::new([O, O, O, O, I, I, I, I, O, I, O, I, O, O, I, I]), 
                Word::new([O, I, I, O, O, I, O, O, I, O, O, I, I, O, I, O])),
            Word::new([O, I, I, I, O, O, I, I, I, I, I, O, I, I, O, I])
        );
        assert_eq!(
            Add16(
                Word::new([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]), 
                Word::new([O, I, I, O, O, I, O, O, I, O, O, I, I, O, I, O])),
            Word::new([O, I, I, O, O, I, O, O, I, O, O, I, I, O, O, I])
        );
    }
}