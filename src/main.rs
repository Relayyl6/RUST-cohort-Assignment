fn main() {

    
    
    name();
    mat_no();
    favorite_emoji();
    let value = difference();
    formatted(value);
}

fn name(){
    // Printing my name
    let name: String = String::from("Leonard");

    println!("My name is: {name}");
}

fn mat_no() {
    // Printing my Mat No
    let mat_no: String = String::from("ENG2204523");

    println!("My Matriculation number is: {mat_no}");
}

fn favorite_emoji() {
    // Printing my favorite Emoji
    let favorite_emoji = String::from("😎");

    println!("My favorite emoji is: {favorite_emoji}");
}

fn difference() -> i32 {
    // The difference of two entered numbers
    let num1: i32 = 78;
    let num2: i32 = 45;
    let difference = num1 - num2;

    println!("The difference between the two numbers entered is: {difference}");

    difference as i32
}

fn formatted(result: i32) {
    //making the result of the dfference into a fraction/dcimal
    let integer: i32 = result;
    let float: f64 = integer as f64;
    let formatted = format!("{:.2}", float);

    println!("Formatted float: {formatted}");
}