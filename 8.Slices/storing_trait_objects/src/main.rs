trait Doubler {
    fn double(&self, value: i32) -> i32;
}

struct Simple;

impl Doubler for Simple {
    fn double(&self, value: i32) -> i32 {
        value * 2
    }
}

struct Logged;

impl Doubler for Logged {
    fn double(&self, input: i32) -> i32 {
        let output = input * 2;
        println!("Doubling {input} gives {output}");
        output
    }
}

// The compiler will generate a new function for each type D.
// That also means that it can perform optimizations taking the implementation
// of D into account
fn static_double<D: Doubler>(doubler: D, value: i32) -> i32 {
    doubler.double(value)
}

// the compiler will only generate a simple implementation
// The function to be invoked is to be stored in a virtual method table
// the reference '&dyn Doubler' creates a pointer to the instance and to the vtable
fn dynamic_double(doubler: &dyn Doubler, value: i32) -> i32 {
    doubler.double(value)
}

fn main() {
    println!("{:?}", static_double(Simple, 1));
    println!("{:?}", static_double(Logged, 3));
    println!("{:?}", dynamic_double(&Simple, 5));
    println!("{:?}", dynamic_double(&Logged, 7));
}