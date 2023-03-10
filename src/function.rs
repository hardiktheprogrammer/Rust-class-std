fn main() {

    let sum = first_function(11, 2);
    println!("this is the sum of: {}",sum);
}
/*
fn first_function(x:i32, y:i32) -> i32 {
        println!("this is the x function: {}", x);
    println!("this is the y value:{}",y);
    let z = x * y;
    z;
    }

 */
// new way of writing functions
fn first_function(x:i32,y:i32) -> i32 {
    println!("this is the x function:{}",x);
    println!("this is the y value:{}",y);
    x + y
}

