enum Color {
    Blue,
    Yellow,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Color::Blue => "blue",
            Color::Yellow => "yellow",
        })
    }
}

fn main() {
    println!("{}", Color::Yellow);
}
