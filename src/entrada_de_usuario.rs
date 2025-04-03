use std::io;

fn convert_to_int(data_input: &str) -> Result<i32, std::num::ParseIntError> {
    data_input.trim().parse::<i32>()
}

pub fn main() {
    let mut num1: String = String::new();
    let mut num2: String = String::new();

    println!("Digite o primeiro número:");

    if io::stdin().read_line(&mut num1).is_err() {
        eprintln!("Erro ao ler a primeira entrada.");
        return;
    }

    println!("Digite o segundo número:");

    if io::stdin().read_line(&mut num2).is_err() {
        eprintln!("Erro ao ler a segunda entrada.");
        return;
    }

    let int1: i32 = match convert_to_int(&num1) {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Primeira entrada inválida. Digite um número inteiro.");
            return;
        }
    };

    let int2: i32 = match convert_to_int(&num2) {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Segunda entrada inválida. Digite um número inteiro.");
            return;
        }
    };

    if int1 > int2 {
        println!("{} é maior que {}", int1, int2);
    } else {
        println!("{} é menor ou igual que {}", int1, int2);
    }
}
