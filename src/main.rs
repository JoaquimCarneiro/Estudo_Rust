fn main() {
    // #### Variables, mutability, shadowing and constants ####
    //
    // Variables are imutable by default
    let y = 10;
    // y = 15; //this causes a compile error.
    println!("The value of Y is {y}, changing this value is not permited.");
    // mutable variable
    let mut x = 5;
    println!("The value of X is {x}");
    x = 6;
    println!("and now X is {x}");
    // #### constants ####
    // can't be changed - always immutable
    // cant't use mut
    // type must be annotated
    // can be declared in any scope
    // can be a computed expression at compile time, not at runtime.
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("Three hours in seconds = {}", THREE_HOURS_IN_SECONDS);
    // #### Shadowing ####
}
