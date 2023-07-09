use std::path::{Path, PathBuf};

fn print_path<P: AsRef<Path>>(Path: P) {
    let path = Path.as_ref();

    println!("{}", path.display());
}

fn main() {
    let a: &'static str = "static_str";
    print_path(a);

    let b: String = String::from("owned_string");
    print_path(b);

    let c: PathBuf = PathBuf::from("owned path");
    print_path(c);
}
