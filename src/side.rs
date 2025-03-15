fn main() {

    name();
    mat_no();
    favorite_emoji();
    difference();
    formatted();
}

fn name(){
    // Printing my name
    let name: String = String::from("Leonard");

    println!("{name}");
}

fn mat_no() {
    // Printing my Mat No
    let mat_no: String = String::from("ENG2204594");

    println!("{mat_no}");
}

fn favorite_emoji() {
    // Printing my favorite Emoji
    let favorite_emoji = String::from("😎");

    println!("{favorite_emoji}");
}

fn difference() {
    // The difference of two entered numbers
    let num1 = 98;
    let num2 = 45;
    let difference = num1 - num2;

    println!("{difference}");
}

fn formatted() {
    //making the result of the difference into a fraction/dcimal
    let integer: i32 = difference;
    let float: f64 = integer as f64;
    let formatted = format!("{:.2}", float);

    println!("Formatted float: {formatted}");
}

