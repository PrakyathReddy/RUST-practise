fn main() {
    //boolean
    let b1: bool = true;

    //unsigned integers
    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 2;
    let i4: u64 = 2;
    let i5: u128 = 1;

    //signed integers
    let i6: i8 = 1;
    let i7: i16 = 1;
    let i8: i32 = 1;
    let i9: i64 = 1;
    let i10: i128 = 1;

    //floating point numbers
    let i11: f32 = 1.0;
    let i12: f64 = 1.0;

    //platform specific integers
    let i13: usize = 1;
    let i14: isize = 1;

    //characters, &str and string
    let i15: char = 'c'; // for a single character
    let i16: &str = "hello";
    let i17: String = String::from("hello");

    //arrays
    let i18: [i32;5] = [1,2,3,4,5];
    let i19: i32 = i18[3];

    //tuples
    let i20 = (2,3,4);
    let i20 = (2,3.0,"five");

    let i21: &str = i20.2;
    let (i19: i32, i12: f64, i16: &str) = i21;

    let unit: () = ();

    //type aliasing
    type age = u8;
    let i22: age = 24;

}