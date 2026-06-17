fn main(){
    // compount types 
    let tup = ("Let's get rusty", 1991, [1, 1,3]);

    println!("{:?}", tup);

    let (channel, sub_count, more) = tup;
    let channel  = tup.0;
    let sub_count = tup.1;
    let more = tup.2;


    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    let x = error_codes[3];

    let byte = [0; 8];// create an array of length 8 with values equal 0


}