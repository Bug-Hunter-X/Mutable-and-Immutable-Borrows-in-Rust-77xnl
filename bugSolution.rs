fn main() {
    let mut x = 5;
    { // Create a new scope to limit the lifetime of the mutable reference
        let y = &mut x;
        *y += 1;
    }
    let z = &x;
    println!("x = {}", x); // Prints 6
    println!("z = {}", *z); // Prints 6.  This is now safe.
}