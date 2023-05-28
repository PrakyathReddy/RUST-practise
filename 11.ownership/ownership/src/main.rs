fn main() {
    let s1: String = String::from("Rust"); //heap allocated string
    let s2: String = s1;
    let s3: String = s2.clone();
    println!("{s2}");
    println!("{s3}");

    let x: i32 = 10;
    let y: i32 = x;
    println!("{x}");
} //s1 will be dropped

/*
Ownership: A strategy for managing memory (and other resources) through a set of rules checked at compile time.
    
    Rules:
    a. Each value in Rust has a variable that's called it's owner.
    b. There can only be one owner at a time
    c. When the owner goes out of scope, the value will be dropped.

    Problems Ownership solves:
    a. Memory / resource leaks
    b. Double free errors
    c. Use after free errors
 */

/*
This string type comes from the Rust standard library and it's a UTF-8 encoded, 
growable string that's allocated on the heap.
We are creating the string be calling the "from" function using the "::" syntax.
s1 has 2 components - first one is a pointer stored on a stack, more specifically
stack frame for the main function. The second component is the actual string 
which is allocated on the heap. 
Following the ownership rules, s1 is the owner of the data stored on the heap.
So when s1 goes out of scope this data (on heap) will be cleaned up.
 */ 

/*
Moving ownership
When we create s2 equal to s1, we get an error with print statement for s1.
Because when we set s2 = s1, the value in s1 is moved to s2, and because the 
ownership rules state that we can only have 1 owner at a time, s2 is now the
owner of the string "Rust", and s1 is invalidated.
 */

/* 
Cloning instead of moving - can be done with .clone() method.
Now s3 has it's own copy of the "Rust" string
 */

/*
Value is usually moved when reassigning like s2=s1. However that is not true
for primitive types such a numbers, floating points, etc. When we move x to variable 
y, what actually happens is a clone of x to y. This is because primitives are
entirely stored on the stack such as integers, floating point numbers, booleans 
or characters are cloned by default. These types are cheap to clone, so there's no
material difference between cloning and moving the values.
 */