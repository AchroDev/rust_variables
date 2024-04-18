// Shadowing Example

fn main() {
      // Initial x value
      let x = 5;
  
      // Modified x value (What the compiler will see first)
      let x = x + 1;
      
      // Inner scope
      {
         // Modified x value in the inner scope
          let x = x * 2;
          println!("The value of x in the inner scope is: {x}");
      }
  
      println!("The value of x is: {x}");
}
