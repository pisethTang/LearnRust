fn main() {
    let x: i32 = 5;
    println!("The value of x is: {}", x);
    // let x = 10; // this shadows 
    // can change types (e.g., from integer to string.)
    let x = "i am a string";

    println!("The value of x is: {}", x);

    // Integers 
    let a = 98_222; // decimal 
    let b = 0xff; // hex 
    let c = 0o77; // octal 
    let d = 0b1111_0000; // binary 
    let e = b'A'; // byte (u8 only)

    println!("The value of a is: {}", a);

    // rust performs 2-complement computations.
    let f: u8 = 255;
    // 256 --> 0 
    // 257 --> 1

    let g: f32 = 3.0;

    // booleans 

    let t = true;
    let f = false; 


    // Characters 
    let x = "z";
    let z = "Z";
    let heart_eyed_cat = "🍰";


    // arithmetics 
    let sum = 10 + 12;
    let difference = 12 - 383;
    let multiplication = 39 * 10;
    let quotient = 2193 / 12;
    let remainder = 19 % 10;


    const SUBSCRIBER_COUNT: u32 = 100_000; // Rust allows us to add "_" (underscores) between zeros to improve readability. 
    println!("The value of SUBSCRIBER_COUNT is: {}", SUBSCRIBER_COUNT);


}
