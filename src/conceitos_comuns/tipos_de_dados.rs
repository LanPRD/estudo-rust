// ---------------------------------------------
// 📚 TABELA COMPLETA DOS TIPOS PRIMITIVOS EM RUST
//
// ░░ Inteiros com e sem sinal ░░
//
// | Tipo   | Bits | Bytes | Sinal   | Intervalo                                |
// |--------|------|--------|---------|------------------------------------------|
// | i8     |  8   | 1 B    | Sim     | -128 a 127                                |
// | i16    | 16   | 2 B    | Sim     | -32.768 a 32.767                          |
// | i32    | 32   | 4 B    | Sim     | -2.147.483.648 a 2.147.483.647            |
// | i64    | 64   | 8 B    | Sim     | -9.223.372.036.854.775.808 a 9.223...807  |
// | i128   | 128  | 16 B   | Sim     | ±170 sextilhões...                        |
// | isize  | 32/64| 4/8 B  | Sim     | Depende da arquitetura (32 ou 64 bits)    |
// | u8     |  8   | 1 B    | Não     | 0 a 255                                   |
// | u16    | 16   | 2 B    | Não     | 0 a 65.535                                |
// | u32    | 32   | 4 B    | Não     | 0 a 4.294.967.295                         |
// | u64    | 64   | 8 B    | Não     | 0 a 18.446.744.073.709.551.615            |
// | u128   | 128  | 16 B   | Não     | 0 a 340 sextilhões...                     |
// | usize  | 32/64| 4/8 B  | Não     | Idem acima (usado para indexação)         |
//
// ░░ Ponto flutuante ░░
//
// | Tipo   | Bits | Bytes | Precisão     | Exemplo     |
// |--------|------|--------|---------------|-------------|
// | f32    | 32   | 4 B    | 6-7 dígitos   | let x: f32 = 3.14; |
// | f64    | 64   | 8 B    | 15 dígitos    | let y = 2.718;     |
//
// ░░ Booleano ░░
//
// | Tipo   | Bits | Valores possíveis |
// |--------|------|-------------------|
// | bool   | 8*   | true / false      |
//
// * Apesar de ser 1 bit logicamente, geralmente ocupa 1 byte na memória.
//
// ░░ Caracteres ░░
//
// | Tipo   | Bytes | Detalhe                      |
// |--------|--------|-----------------------------|
// | char   | 4 B    | Representa 1 caractere Unicode UTF-8 |
//
// Exemplo: let c = '😻'; // sempre aspas simples!
// ---------------------------------------------

use std::io;

pub fn main() {
    tipo_numerico_explicito();
    operacoes_basicas();
    tipos_logicos_e_chars();
    tuplas();
    arrays_validos();
    acesso_invalido(); // ⚠️ Demonstrando erro
}

// Exemplo de tipo explícito
fn tipo_numerico_explicito() {
    // Converte uma string para inteiro u8, com verificação
    let _guess: u8 = "42".parse().expect("Não é um número!");
}

// Operações matemáticas
fn operacoes_basicas() {
    let _x = 2.0; // f64 (float padrão)
    let _y: f32 = 3.0; // float de 32 bits

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;
}

// Booleanos e caracteres
fn tipos_logicos_e_chars() {
    let _t = true;
    let _f: bool = false;

    let _c = 'z';
    let _z = 'ℤ';
    let _emoji = '😻';
}

// Tuplas
fn tuplas() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Desestruturação
    let (_x, _y, _z) = tup;

    // Acesso direto
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
}

// Arrays válidos
fn arrays_validos() {
    let _a = [1, 2, 3, 4, 5]; // [i32; 5]
    let _months = ["Janeiro", "Fevereiro", "Dezembro"]; // [&str; 3]
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // equivalente a [3, 3, 3, 3, 3]

    let _first = a[0];
    let _second = a[1];
}

// Acesso inválido ao array
fn acesso_invalido() {
    let a = [1, 2, 3, 4, 5];

    println!("⚠️ Por favor, insira um índice do array:");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Erro ao ler a linha");

    let index: usize = index
        .trim()
        .parse()
        .expect("Índice inserido não é um número!");

    // ❌ Se o índice for >= 5, o programa entra em pânico (erro em tempo de execução)
    let element = a[index];

    println!("✅ O valor no índice {index} é: {element}");
}
