// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct item {
    quantitat: i32,
    preu: i32
}

fn dispQ(I: &item){
    println!("La quantitat es: {:?}", I.quantitat);
}

fn dispP(I: &item){
    println!("El preu es: {:?}", I.preu);
}

fn main() {
    let Obj = item{
        quantitat: 10,
        preu: 15
    };
    dispQ(&Obj);
    dispP(&Obj);
}
