//Mutable Example:
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
//Immutable Example: Outputs Error
/*fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
} */

//Constant Example:
//Naming Convention for Constant UPPERCASE = words, UNDERSCORES = spaces
//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

//Shadowing Example:
/*fn main() {
    let x = 5; //first variable

    let x = x + 1; //second variable shadowed the first

    {
        let x = x * 2; //third variable shadowed the second
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
} */


/* Immutable and Mutable Examples in different datatypes
//Able to Output because of shadowing concept
let spaces = "   "; //String
let spaces = spaces.len(); //Numerical  

//Unable to Output because both are different datatypes
let mut spaces = "   ";
    spaces = spaces.len();
*/

