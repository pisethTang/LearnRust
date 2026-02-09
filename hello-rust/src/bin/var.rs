#![allow(unused)]

fn main() {
    // variables are by default immutable
    // signed integers 32-but
    let x: i32 = -123;
    // x += 1; // cannot mutate immutable variables

    // if we want to mutate the variable's internal value
    let mut y: i32 = 123;
    y += 1;

    let z = -123; // the rust compiler can implicitly infer types

    // if we want to figure out the type of a variable
    // let w: () = 123;
    // unsigned integers 
    const NUM: u32 = 1;

    // can redeclare variables 
    let x: i32 = -1;

    // ....
    let x: bool = true;

    //  a type placeholder 
    let v: Vec<_> = vec![1,2,3]; 

}
