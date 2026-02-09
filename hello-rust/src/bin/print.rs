#![allow(unused)]

// struct variables

#[derive(Debug)] // tells Rust to auto-generate the code so that Lang struct variable can be printed out nicely 
struct Lang {
    language: String,
    version: String,
}

fn main() {
    let lang = "rust";
    println!("hello {}", lang);
    println!("hello {} {}", lang, lang);
    // cargo fmt: formats our code

    println!("hello {lang}");

    // can also pass in positional argument
    let x = 2;
    println!("{0} x {0} = {1}", x, x * x);

    let lang = Lang {
        language: "rust".to_string(),
        version: "1.83".to_string(),
    };

    // debug feature
    println!("{:?}", lang); // "Lang" does not implement debug 
    println!("{:#?}", lang); // prints with some line break 
}
