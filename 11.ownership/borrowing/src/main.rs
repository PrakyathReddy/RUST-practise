/*
Borrowing is the act of creating a reference to a value.
A reference is like a regular pointer with some rules/restrictions.
Refereneces do not take ownership of values. This is why creatig references is
called borrowing, because your borrowing the value instead of taking ownership
of it.

Why Borrow ?
1. Performance
2. When ownership is not needed / desired.

Borrowing rules:
1. At any given time, you can have either one mutable reference or any number
of immutable references.
2. All references must be valid.

Problems these rules solves:
1. Data races - which happens when 2 threads try to read and write to the same
memory location and results are non-deterministic.
2. Dangling references - which is when a reference is pointing to an invalid 
memory.
 */

fn main() {
    let mut s1: String = String::from("Rust"); //heap allocated string
    //print_string(s1.clone());
    //instead of cloning s1, create a reference
    let r1: &String = &s1;
    print_string(r1);
    let r2: &mut String = &mut s1;
    add_to_string(r2);
    println!("s1 is {s1}");
    let _s2 = generate_string();
}

/*update the String to accept a reference of a string instead of a String like 
&String. References are defined with an '&'. Instead of taking ownership, 
print_string is now bowworing a string.
*/
fn print_string(p1: &String) {
    println!("p1 is {p1}");
}

/*
Following function we are able to do push str on p1 even though it's a 
reference because Rust has a feature called Automatic Dereferencing. This 
means we don't have to specifically deference p1 in this function. 
However if we wanted to, it would look like this: (*p1) - the * is
a dereference operator.
 */
fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome");
}

/*
What if instead of returning a string, we return a reference to a string ?
As a general rule, if you create an owned value inside a function and want 
to return a value, you have to move ownership. You can't return a reference,
because the owned value will be cleaned up at the end of the function, so
the reference will be invalid.
 */
fn generate_string() -> String {
    String::from("Ferris")
    //let s: String = String::from("Ferris");
    //&s
}