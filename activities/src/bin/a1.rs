// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    fun_nom();
    fun_cnom();
}

fn fun_nom(){
    let nom = "Josep ";
    print!("{}", nom);
}
fn fun_cnom(){
    println!("Garcia Caldero");
}