fn main(){
    println!("Hello function inside bin/");

    let sum = my_function(112, 323);
    println!("sum = {}", sum);
}



fn my_function(x: i32, y:i32) -> i32{
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let sum = x + y;
    return sum;
}


