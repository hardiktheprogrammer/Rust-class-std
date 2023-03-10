// unsigned integer can only be a positive number
// signed integer can be positive or negative numbers
// types rust integers
// decimals , Hex , Octal , Binary, byte(u8 only)

// let a: = 98_222;// decimal
// let b:  = 0xff; // Hex
// let c: = 0o77; // Octal

// compound Types
fn main() {
    let tup: (&str, i32) = ("let's Build in solana!",100_00);
    let (channel, sub_count: i32) = tup;
    let sub_count: i32 = tup.1;

    let error_code: [i32;_] = [200,404,2000];
    let not_found: i32 = error_code[1];
    let x: i32 = error_code[0];

    let bytes: [i32;_] = [0;1];

}