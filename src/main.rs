fn main() {
    //okay here we are going to review some fundamentals
    //Rust checks for the byte size it sends to Assenbly (guess)
    //of course "byte size" is used as a logical understanding ONLY - will review rust's implementation one of thess days
    //so it offers mechanisms to check a lot of out of range, overflow
    //type situations which are not easy to check and figure out in C
    //some things to play with, getting this right will make you a better
    //programmer by just using Rust instead of trying to solve just in 
    //context, as a C programmer, one situation at a time
    
    //using i8 to illustrate 
    //Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive
    //where n is the number of bits that variant uses
    //So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127
    //some of the code here is written to avoid warnings that will distract from learning concepts
    let ani8variable001: i8 = 100;
    let ani8variable002: i8 = 2;
    
    // the idea is to first force an i8 variable to hold a value greater than 127
    let mut ani8variable003: i8 = 0;
    ani8variable003 = ani8variable003 + ani8variable001 + ani8variable002;
    //compile time observation
    // attempt to compute `100_i8 + 50_i8`, which would overflow
    // error: this arithmetic operation will overflow
    // change the values to compile to less than 127
    println!("value of the 003 variable that adds the other 2 i8 variables {ani8variable003}");
}
