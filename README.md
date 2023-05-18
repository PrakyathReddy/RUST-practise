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
14. const VARIABLE_NAME:dataType = value; - syntax to declare a constant. Const declaration must declare a datatype unlike variables where it's optional. Always Immutable, not mut option. Should always be named in capitals.
