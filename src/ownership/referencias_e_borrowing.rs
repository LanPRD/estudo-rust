pub fn main() {
    referencia_basica();
    referencia_com_funcao();
    tentativa_modificacao_imutavel();
    referencia_mutavel_valida();
    referencias_mutaveis_simultaneas();
    mistura_referencias();
    escopo_encerra_referencia();
    referencia_pendurada();
}

/// Demonstra uma referência imutável básica, sem mover a posse.
fn referencia_basica() {
    let s1: String = String::from("hello");

    // Cria uma referência imutável a s1
    let len: usize = calcula_tamanho(&s1);

    // s1 ainda é válido, pois apenas emprestamos o valor
    println!("O tamanho de '{s1}' é {len}.");
}

fn calcula_tamanho(s: &String) -> usize {
    s.len()
}

/// Demonstra como referências funcionam com funções e escopos
fn referencia_com_funcao() {
    let s: String = String::from("Rustacean");
    exibe(&s); // empresta a string, não move
    println!("s ainda é válido após exibe: {}", s);
}

fn exibe(s: &String) {
    println!("valor recebido por referência: {}", s);
}

/// Tenta modificar um valor emprestado de forma imutável (gera erro se descomentado)
fn tentativa_modificacao_imutavel() {
    let _s: String = String::from("hello");
    // change(&_s); // ERRO: não é permitido modificar via referência imutável
}

// fn change(s: &String) {
//     s.push_str(", world");
// }

/// Mostra como modificar valores com referências mutáveis corretamente
fn referencia_mutavel_valida() {
    let mut s: String = String::from("hello");
    modifica(&mut s);
    println!("Após modificação: {}", s);
}

fn modifica(s: &mut String) {
    s.push_str(", world");
}

/// Demonstra erro ao tentar ter duas referências mutáveis simultâneas (comentado)
fn referencias_mutaveis_simultaneas() {
    let mut _s: String = String::from("olá");

    // let r1 = &mut _s;
    // let r2 = &mut _s; // ERRO: não pode haver duas referências mutáveis ao mesmo tempo

    // println!("{}, {}", r1, r2);
}

/// Demonstra erro ao misturar referência mutável com imutáveis simultâneas
fn mistura_referencias() {
    let mut _s: String = String::from("olá");

    // let r1 = &_s;
    // let r2 = &_s;
    // let r3 = &mut _s; // ERRO: referência mutável com imutáveis ativas

    // println!("{}, {}, {}", r1, r2, r3);
}

/// Mostra que o escopo termina na última vez que a referência é usada
fn escopo_encerra_referencia() {
    let mut s: String = String::from("olá");

    let r1: &String = &s;
    let r2: &String = &s;
    println!("{} e {}", r1, r2); // última vez que r1 e r2 são usados

    // Agora é seguro criar uma referência mutável
    let r3: &mut String = &mut s;
    r3.push_str("!!!");
    println!("{}", r3);
}

/// Demonstra tentativa inválida de retornar referência para variável local
fn referencia_pendurada() {
    // let r = dangle(); // ERRO: tentativa de retornar referência para valor que será descartado
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // ERRO: s será dropado aqui e r apontaria para memória inválida

// Correto: retorna a String, transferindo a posse
// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s
// }
