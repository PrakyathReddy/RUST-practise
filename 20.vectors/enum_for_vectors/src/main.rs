fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let _row = vec![SpreadsheetCell::Int(3), 
                    SpreadsheetCell::Float(2.4), 
                    SpreadsheetCell::Text(String::from("multiple type of values"))
                    ];
}