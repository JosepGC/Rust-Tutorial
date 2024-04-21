// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
    
    let (x,y) = mapa();
    if y < 5 {
        println!("Y es mes petita que 5 donat que el seu valor es: {:?}", y);
    } else if y == 5 {
        println!("Y es igual que 5 donat que el seu valor es: {:?}", y);
    } else {
        println!("Y es mes gran que 5 donat que el seu valor es: {:?}", y);
    }
}

fn mapa() -> (i32, i32){
    (3, 3)
}