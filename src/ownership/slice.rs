pub fn main() {
    println!("--- 1. Diferença entre String e &str ---");
    exemplo_string_vs_str();

    println!("\n--- 2. Por que aceitar &str e não &String ---");
    exemplo_aceitar_str();

    println!("\n--- 3. Problema de referência imutável travando mutável ---");
    exemplo_bloqueio();

    println!("\n--- 4. Como liberar referência imutável ---");
    exemplo_liberar_referencia();

    println!("\n--- 5. Como contornar usando .clone() ---");
    exemplo_clone();

    println!("\n--- 6. &mut str: slice mutável ---");
    exemplo_mut_str();

    println!("\n--- 7. Onde os dados de &str vivem ---");
    exemplo_armazenamento_str();
}

/// 1. Mostra diferença de onde estão os dados e os tipos
fn exemplo_string_vs_str() {
    let literal: &str = "oi"; // dados no binário
    let dinamica: String = String::from("oi"); // dados na heap
    let fatia: &str = &dinamica[..]; // slice na heap, tipo &str

    println!("literal (&str) -> {}", literal);
    println!("dinâmica (String) -> {}", dinamica);
    println!("fatia (&str) de String -> {}", fatia);
}

/// 2. Mostra porque sempre preferimos &str nas funções
fn exemplo_aceitar_str() {
    imprime(&String::from("sou String")); // coerção automática
    imprime("sou literal &str"); // literal já é &str
}

fn imprime(s: &str) {
    println!("Recebi: {}", s);
}

/// 3. Mostra o erro de tentar mutar enquanto tem &str ativa
fn exemplo_bloqueio() {
    let s: String = String::from("hello");

    let slice: &str = &s[..];
    println!("Primeira palavra (slice): {}", slice);

    // s.push_str(" world"); // ❌ erro se descomentar
    println!("Se eu tentar alterar aqui, Rust não deixa!");
}

/// 4. Mostra como liberar referência imutável (usando escopo)
fn exemplo_liberar_referencia() {
    let mut s: String = String::from("hello");

    {
        let slice: &str = &s[..];
        println!("Usei o slice: {}", slice);
    } // slice morre aqui

    s.push_str(" world"); // agora posso mutar
    println!("Agora alterei s: {}", s);
}

/// 5. Mostra como .clone() cria uma cópia e evita o problema
fn exemplo_clone() {
    let mut s: String = String::from("hello");

    let copia: String = s.clone(); // nova String, heap separada
    s.push_str(" world"); // posso mutar livremente

    println!("Original alterado: {}", s);
    println!("Cópia intocada: {}", copia);
}

/// 6. &mut str — slice mutável da String
fn exemplo_mut_str() {
    let mut s: String = String::from("hello");

    let slice: &mut str = &mut s[..]; // válido se s é mutável
    println!("Slice mutável de str: {}", slice);

    // ⚠️ Não podemos alterar diretamente slice porque &mut str exige UTF-8 válido
    // slice[0] = b'X'; // ❌ não permitido diretamente
}

/// 7. Demonstra onde os dados de um &str podem estar
fn exemplo_armazenamento_str() {
    let binario: &str = "fixo"; // no binário
    let heap: String = String::from("alocado");
    let slice: &str = &heap[..]; // na heap

    println!("&str do binário: {}", binario);
    println!("&str da heap: {}", slice);
}
