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

fn tuple_destruct() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}")
}

/*
*   You can directly access a tuple element by using a "." followed by the index
*   value we want to access.
*
*   In example, the first index of a tuple is 0 and we can access that value by
*   assigning a variable like so; "let five_hundred = x.0;". This will assign the
*   variable "five_hundred" to the first index of "x" which is the value 500.
*/

fn tuple_directAccess() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

/*  A tuple without any values is called a "unit" which is written as "()"
 *   and returns an empty value or an empty return type.
*/

fn tuple_empty() {
    let tup1: () = ();

    let tup2: (unit, unit, unit) = ();
}
