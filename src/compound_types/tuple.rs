// Example of the Tuple Type

/*
*   A tuple is a general way of grouping together a number of values with a variety
*   of types into one compound type. Tuples have a fixed length: once declared,
*   they cannot grow or shrink in size.
*/

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

/*
*   A tuple is considered a single compound element. To get individual values out
*   of a tuple, we can use pattern matching to destructure a tuple value like below.
*
*   The "_" infront of the variables "x" and "z" indicate they are not being used.
*/

fn destruct() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}")
}
