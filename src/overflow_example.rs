// Example function showing a arithmetic overflow by subtracting outside of the u8 bound
/*
* You can bypass the compiler warning by enabling #![enable{arithmetic_overflow}]
* This is because #[deny(arithmetic_overflow)] on by default
*/
fn main() {
    // defining the initial u8 variable x
    let x: u8 = 0;
    // causing an overflow by trying to computer 0_u8 - 1_u8, which is out of bounds
    let test = x - 1;
    // print result to console
    print!("{test}");
}
