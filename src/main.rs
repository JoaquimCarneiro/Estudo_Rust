fn main() {
    // Variables are imutable by default
    let y = 10;
    // y = 15; //thi causes a compile error.
    println!("The value of Y is {y}, changing this value is not permited.");
    // mutable variable
    let mut x = 5;
    println!("The value of X is {x}");
    x = 6;
    println!("and now X is {x}");
}
