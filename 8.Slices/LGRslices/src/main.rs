/*
Slices are references to a contiguous sequence of elements in a collection
Slices are specially used when you want to refer a part of a collection instead
of the entire collection.
*/

fn main() {

    let s: &str = "my string slice";

    let tweet: String = String::from(
        "This is my tweet and it's very long"
    );
    let trimmed_tweet: &str = trim_tweet(&tweet); //String slice
    // although tweet is String and trim_tweet is expecting &str, it still works
    // because of a feature in rust called deref coercion. When we call a trim_tweet
    // with a reference to a String, Rust will automatically coerce it to a string slice. 
    println!("tweet 1 is {trimmed_tweet}");

    let tweet2: &str = "This is my tweet and it's very long";
    let trimmed_tweet2: &str = trim_tweet(tweet2);
    println!("tweet 2 is {trimmed_tweet2}");

    let a: [i32;6] = [1,2,3,4,5,6];
    let a_slice: &[i32] = &a[..3]; 
    println!("{:?}", a_slice);
    /*
    This syntax of :? will print a string out with debug format. 
     */
}

/*
String types:
1. String: Growable, heap allocated string (UTF-8 encoded)
2. str: Immutable sequence of UTF-8 bytes somewhere in memory (stack, heap or
static memory). The length of the sequence is unknown at compile time, so when 
you see str, u usually see str behind a reference like &str

When you need to own the string because you want to mutate it or pass it to
other threads, then use the String type. But if only need a immutable view of
a string or a subset of a string, then use a string slice (&str).

In Rust, all string literals are string slices, and the strings themselves are
stored in application's binary.

let s: &str = "my string slice";
In this case s is a string slice pointing to a specific location in your program's
binary. 

Slices work with other collection as well such as arrays and vectors.
 */

fn trim_tweet(tweet: &str) -> &str {
    &tweet[..20]
}

/*
Summary: If you need a function that accepts a string but does not need ownership
of that string, then you should use &str. That way, callers of your function can 
pass in references to a string or string slices.
 */