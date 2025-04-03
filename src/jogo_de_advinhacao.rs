// Importa a trait Rng (para gerar números aleatórios)
// e outras coisas padrão: comparação (Ordering) e entrada (io)
use rand::Rng;
use std::{cmp::Ordering, io};

pub fn main() {
    // Gera um número aleatório entre 1 e 100 (inclusive)
    let secret_number: i8 = rand::rng().random_range(1..=100);

    println!("O número secreto é um número entre 1 e 100.");

    loop {
        println!("Insira um número:");

        // Cria uma string mutável para armazenar a entrada do usuário
        let mut guess: String = String::new();

        // Lê a entrada do usuário e armazena em guess
        // `&mut guess` passa uma **referência mutável** (tipo um ponteiro para `guess`)
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a entrada.");

        // Converte a string lida para número (`i8`)
        // Se falhar, `continue` pula pra próxima iteração do loop
        let guess: i8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Mostra o valor que o usuário digitou
        println!("Você chutou: {}", guess);

        // Compara o palpite com o número secreto
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("O número secreto é maior."), // palpite < secreto
            Ordering::Greater => println!("O número secreto é menor."), // palpite > secreto
            Ordering::Equal => {
                println!("Parabéns! Você acertou.");
                break; // Sai do loop, fim do jogo
            }
        }
    }
}
