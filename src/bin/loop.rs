fn main(){
    // control flow 
    let mut counter = 10;

    let number = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }

        // println!("again");
        // break;
    };

    print!("The number is {}", number);
}