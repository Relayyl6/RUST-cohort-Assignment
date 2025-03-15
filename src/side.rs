


fn main() {
    // Printing my name
    let name: String = String::from("Leonard");

    // Printing my Mat No
    let mat_no: String = String::from("ENG2204594");

    // Printing my favorite Emoji
    let favorite_emoji = String::from("😎");


    // The difference of two entered numbers
    let num1 = 98;
    let num2 = 45;

    let difference = num1 - num2;

    //making the result of the difference into a fraction/dcimal
    let integer: i32 = difference;
    let float: f64 = integer as f64;
    let formatted = format!("{:.2}", float);

    println!("{name}");
    println!("{mat_no}");
    println!("{favorite_emoji}");
    println!("{difference}");
    println!("Formatted float: {formatted}");
}