fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;  // z is an immutable reference to x
    *y += 1; // Modifies x through y
    println!("x = {}", x); // Prints 6
    println!("z = {}", *z); // Prints 6, but this is wrong in some context
}