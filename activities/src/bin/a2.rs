// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn suma(a: i32, b:i32) -> i32 {
    a + b
}
fn res(a: i32){
    println!("{:?}", a);
}
fn main() {
    let resultat;
    let num1 = 2;
    let num2 = 2;
    resultat = suma(num1, num2);
    res(resultat);
}
