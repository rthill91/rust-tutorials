const MAX_POINTS: u32 = 100_100;
fn main() {
    // Compiler Error
    // let x = 5
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
