fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // works because x is mutable
    println!("The value of x is: {x}");

    // constants are always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    //Shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");
}
