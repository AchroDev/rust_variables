// Data Types

/* Integer Types
Length	Signed	Unsigned
8-bit	i8	    u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
128-bit	i128	u128
arch	isize	usize
*/

//Each signed variant can store numbers from -(2^(n - 1)) to 2^(n - 1) - 1 inclusive
//Unsigned variants can store numbers from 0 to 2n - 1

/* Integer Literals
Number literals	Example
Decimal	        98_222
Hex	            0xff
Octal	        0o77
Binary	        0b1111_0000
Byte (u8 only)	b'A'
*/

// Ignores unused warning
#![allow(unused)]
fn main() {
    let x = 2.0; //f64

    let y: f32 = 3.0; //f32
}
