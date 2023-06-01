This repo contains all the programs I used, to practise and master Rust. Also includes some notes on syntax, tips and candid learnings.

Keywords:

1. fn - to define a function
2. main() - special function that acts as an entrypoint to the program. Does not take any arguments and does not return any value. When you run a Rust program, execution begins in the main function.
3. macros - are a way of writing code that writes other code, which is known as metaprogramming.
4. println!() - to print to the console. Macro calls are always marked with an exclamation mark – !.
5. rustc - terminal command to compile a rust program
6. statically typed - means that when you write code, you must specify what data type each variable is.
7. let - to declare a variable
8. println!("the icon is {}",icon_char); - here {} is the placeholder
9. let age:u32 = 20; - here u32 refers to unsigned integer of size 32 bits. Can be bool/char also for example.
10. let float*with_separator = 11_000.555_001; - here '*' is used to separate numbers for better readability.
11. By default, variables are immutable − read only in Rust. In other words, the variable's value cannot be changed once a value is bound to a variable name.
12. Concurrency is a property of systems in which several tasks are executing in overlapping time intervals.
13. mut - to make a variable mutable
14. const VARIABLE_NAME:dataType = value; - syntax to declare a constant. Const declaration must declare a datatype unlike variables where it's optional. Always Immutable, not mut option. Should always be named in screaming snake capitals. Same with static variables. Both can be declared in any scope including the global scope outside the main function. Unlike const, static variables can be marked mutable.
15. Rust allows programmers to declare variables with the same name. In such a case, the new variable overrides the previous variable.
16. Difference between print and println - println adds a new line at the end
17. Use & whenever referencing
18. String::from("Hello World"): This is a static method call. The :: syntax is used to call associated functions on a type. An associated function is a function that is associated with a type, rather than an instance of a type. In other languages, these might be called static methods. In this case, from is a function associated with the String type, and it takes a string literal (&str) and converts it to an instance of String.
19. &str an immutable reference to a string slice. String a mutable string buffer.
20. In Rust, the -> operator is used to denote the return type of a function.
21. To generate documentation for a Rust package, you can use the cargo doc command, which is a wrapper around rustdoc. The contents are treated as Markdown.
22. impl is a keyword used to define methods associated with a particular struct or enum (or to provide an implementation of a trait for a particular type). Example:impl Rectangle begins an implementation block for the Rectangle struct.
23. In Rust the final expression in a function will be used as a return value. Expressions are things that evaluate to a return value, whereas statements are instructions that do not return a value.
24.

Cargo Commands
Here is a list of commonly utilized cargo commands and their purpose.

cargo new # Create a new binary executable crate
cargo new --lib # Create a new library crate

cargo build # Compiles our crate
cargo build --release # Compiles our crate with optimizations
cargo run # Compiles our crate and runs the compiled executable

cargo test # Run all tests in a crate
cargo doc --open # Build and open our crate's documentation in a web browser
cargo clean # Cleans up temporary files created during compilation
cargo publish # Publishes your crate to `crates.io`

cargo install # Installs a binary directly from crates.io
