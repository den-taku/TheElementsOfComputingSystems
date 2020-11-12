mod logic;
mod arithmetic;
mod sequential;

use logic::*;
use logic::bit::{O, I};
use arithmetic::*;
use sequential::*;

fn main() {
    println!("{}", And(I, O));
    println!("{}", Word::new([O; 16]));
    println!("{}", Word::new([I; 16]));
    println!("{}", Inc16(Word::new([I; 16])));
    println!("{:?}", 
        ALU(
            Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
            Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
            O, O, I, I, O, O
        ));
    
    // let mut clock = Clock::new();
    // loop {
    //     clock.next();
    //     println!("{:?}", clock.state());
    // }
}