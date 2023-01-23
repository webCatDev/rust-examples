fn main() {
    // integers
    // decimal, hex, octal, binary, byte(u8 only)
    // sizes: 8, 16, 32, 64, 128 bit or isize, usize
    // signed integer can store -2^(n-1) to 2^(n-1)-1
    // unsigned integer can store 0 to 2^(n-1)
    // u32 is default integer type
    let a: u32 = 32;

    // we can use types at the end of the integer to be explicit
    let b = 43u32;

    println!("a = {a}");
    println!("b = {b}");
}
