pub fn main() {
    ownership_basico();
    ownership_com_funcao();
    clone_vs_move();
    referencia_imutavel();
    referencia_mutavel();
    tipos_copy_vs_clone();
}

/// Ownership básico: todo valor em Rust tem um "dono" (owner).
/// Quando esse valor é atribuído a outra variável, o ownership pode ser:
/// - **movido**, no caso de tipos que **não** implementam `Copy`
/// - **copiado automaticamente**, no caso de tipos que **são `Copy`**
///
/// Tipos como `String`, `Vec`, `Box` não implementam `Copy`, então são movidos.
/// Tipos primitivos como `i32`, `bool`, `char` **implementam `Copy`**, então são duplicados.
fn ownership_basico() {
    // Exemplo 1: tipo não-Copy (String)
    let s1 = String::from("olá");

    // `String` é movido para s2. s1 perde a validade.
    let s2 = s1;

    // println!("{}", s1); // ERRO! s1 foi movido
    println!("s2 ainda tem a string: {}", s2);

    // Exemplo 2: tipo Copy (i32)
    let x = 42;

    // `i32` é `Copy`, então x é duplicado para y.
    let y = x;

    // Ambas as variáveis são válidas
    println!("x ainda é válido: {}", x);
    println!("y também: {}", y);
}

/// Funções também podem tomar posse de um valor.
/// Quando passamos um valor diretamente para uma função (sem &), o dono muda.
/// Isso também acontece com valores não-Copy como `String`.
fn ownership_com_funcao() {
    let nome = String::from("Rustacean");

    // nome é movido para a função e não pode mais ser usado aqui
    imprime_nome(nome);

    // println!("{}", nome); // ERRO! nome foi movido
}

/// A função recebe um `String` por valor, ou seja, ela se torna a nova dona.
/// Ao fim da função, o valor será descartado.
fn imprime_nome(s: String) {
    println!("Nome recebido: {}", s);
}

/// O método `.clone()` permite criar uma **cópia profunda** dos dados.
/// Com `String`, isso significa criar uma nova alocação na heap,
/// copiando todos os caracteres do original.
fn clone_vs_move() {
    let a = String::from("clone-me");

    // Aqui, b recebe uma cópia completa dos dados de a.
    let b = a.clone();

    // Como usamos `.clone()`, `a` ainda é válido.
    println!("a: {}, b: {}", a, b);
}

/// Em vez de mover o valor, podemos apenas **emprestá-lo** com `&`.
/// Isso cria uma **referência imutável**: a função pode ler, mas não modificar nem tomar posse.
///
/// Isso é conhecido como "borrowing" em Rust.
fn referencia_imutavel() {
    let s = String::from("leitura");

    // Empresta o valor para a função sem mover
    mostra_tamanho(&s);

    // s ainda é válido após o uso
    println!("s ainda é acessível: {}", s);
}

/// A função recebe uma referência imutável (`&String`) e apenas lê.
fn mostra_tamanho(texto: &String) {
    println!("Tamanho do texto: {}", texto.len());
}

/// Podemos também emprestar valores de forma **mutável** com `&mut`,
/// permitindo modificações sem mover o ownership.
///
/// Regras importantes:
/// - Só pode existir **uma referência mutável por vez**.
/// - Isso evita condições de corrida (data races) em tempo de compilação.
fn referencia_mutavel() {
    let mut s = String::from("editável");

    // Empresta de forma mutável
    adiciona_exclamacao(&mut s);

    // Valor foi alterado pela função
    println!("Depois da modificação: {}", s);
}

/// A função recebe uma referência mutável e modifica o conteúdo da string.
fn adiciona_exclamacao(s: &mut String) {
    s.push('!');
}

/// Exibe uma tabela resumida de quais tipos possuem `Copy` e quais não.
///
/// Tipos `Copy`:
/// - São simples, leves, e com tamanho fixo conhecido em tempo de compilação.
/// - São duplicados automaticamente sem `clone()`.
/// - Ex: i32, f64, bool, char, arrays fixos, referências (&T).
///
/// Tipos **não-Copy**:
/// - Envolvem dados alocados na heap ou recursos exclusivos.
/// - Exigem `.clone()` para duplicação ou `&` para uso temporário.
/// - Ex: String, Vec, Box, HashMap, structs que contêm esses tipos.
fn tipos_copy_vs_clone() {
    println!("\n=== Tabela de Tipos: Copy vs Não-Copy ===");
    println!("{:<25} | {:<10}", "Tipo", "É Copy?");
    println!("{}", "-".repeat(40));

    // Tipos simples: Copy
    println!("{:<25} | {:<10}", "i32", "Sim");
    println!("{:<25} | {:<10}", "f64", "Sim");
    println!("{:<25} | {:<10}", "bool", "Sim");
    println!("{:<25} | {:<10}", "char", "Sim");
    println!("{:<25} | {:<10}", "[i32; 3]", "Sim");
    println!("{:<25} | {:<10}", "&str", "Sim");
    println!("{:<25} | {:<10}", "&String", "Sim");

    // Tipos compostos: não são Copy
    println!("{:<25} | {:<10}", "String", "Não");
    println!("{:<25} | {:<10}", "Vec<T>", "Não");
    println!("{:<25} | {:<10}", "Box<T>", "Não");
    println!("{:<25} | {:<10}", "HashMap<K, V>", "Não");
    println!("{:<25} | {:<10}", "struct com String", "Não");
}
