pub fn main() {
    let name: &str = "Allan";
    // name = "Prado"; // This line will cause a compile-time error

    let mut age: u8 = 42;
    age += 1; // This line will not cause a compile-time error

    let test: f32 = 255.9;

    println!("Tamanho de bool: {} byte(s)", std::mem::size_of::<bool>());

    println!("Hello {} you are {}! Test {}", name, age, test);
}
