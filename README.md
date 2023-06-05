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
24. Const variables do occupy any specific location, while static variables do occupy a specific location in memory, which means there is only one instance of
    the value.

Cargo Commands
Some common cargo commands are (see all commands with --list):
build, b Compile the current package
check, c Analyse the current package and report errors, but don't build object files
clean Remove the target directory
doc, d Build this package's and its dependencies' documentation
new Create a new cargo package
init Create a new cargo package in an existing directory
add Add dependencies to a manifest file
run, r Run a binary or example of the local package
test, t Run the tests
bench Run the benchmarks
update Update dependencies listed in Cargo.lock
search Search registry for crates
publish Package and upload this package to the registry
install Install a Rust binary. Default location is $HOME/.cargo/bin
uninstall Uninstall a Rust binary
See 'cargo help ' for more information on a specific command
cargo install # Installs a binary directly from crates.io

Create a keypair
mkdir ~/my-solana-wallet
solana-keygen new --outfile ~/my-solana-wallet/my-keypair.json
display the result with
solana-keygen pubkey ~/my-solana-wallet/my-keypair.json
verify your address
solana-keygen verify <PUBKEY> ~/my-solana-wallet/my-keypair.json

Connect to the dev network
solana config set --url https://api.devnet.solana.com
You can check the configuration with
solana config get
Get some tokens from dev net
solana airdrop 1 <RECIPIENT_ACCOUNT_ADDRESS> --url https://api.devnet.solana.com
you will receive the transaction ID, and can look for this on the dev net block explorer
You can also check your balance with
solana balance <ACCOUNT_ADDRESS> --url https://api.devnet.solana.com
See Docs
Note that you need to use the file system wallet we set up yesterday.
In the following <KEYPAIR> is the path to that wallet.

Transferring SOL to another account
solana transfer --from <KEYPAIR> <RECIPIENT_ACCOUNT_ADDRESS> <AMOUNT> --fee-payer
<KEYPAIR>
