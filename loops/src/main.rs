fn main() {
    //create a mutable variable to serve as counter
    let mut counter = 0;

    // create a loop and assigns its terminal value to result variable
    let result = loop {
        counter += 1;

        if counter == 10 {
            // if counter is 10, break loop and return some value
            break counter * 2;
        }
    };
    println!("The result is {result}");
}
