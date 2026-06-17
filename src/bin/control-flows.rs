fn main(){
    let number = 5;
    if number < 12 {
        println!("first condition was true");
    } else if number < 12 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }


    // control flows. 
    let condition = true;
    let number = if condition {5} else {12};

    println!("The value of number is: {}", number);

}