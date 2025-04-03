pub fn main() {
    let number1: i8 = 24;
    let number2: i8 = 24;

    if number1 > number2 {
        println!("{} > {}", number1, number2);
    } else if number2 > number1 {
        println!("{} > {}", number2, number1);
    } else {
        println!("{} = {}", number1, number2);
    }
}
