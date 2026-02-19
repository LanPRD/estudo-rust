// ============================================================================
// ENUMS: Definindo e Usando Enumerações
// ============================================================================
//
// Enums permitem definir um tipo listando suas variantes possíveis.
// Diferente de structs (que agrupam campos), enums dizem "é um OU outro".
//
// JS/TS equivalente:
//   type IpKind = "V4" | "V6";           // Union type básico
//   type IpAddr = { kind: "V4", addr: string } | { kind: "V6", addr: string };
//
// Rust:
//   enum IpAddrKind { V4, V6 }           // Variantes simples
//   enum IpAddr { V4(String), V6(String) }  // Com dados associados
//
// A grande vantagem: cada variante pode ter tipos DIFERENTES de dados!
// ============================================================================

pub fn main() {
    println!("--- 1. Enum básico ---");
    enum_basico();

    println!("\n--- 2. Enum como parâmetro ---");
    enum_como_parametro();

    println!("\n--- 3. Enum com dados (simples) ---");
    enum_com_dados_simples();

    println!("\n--- 4. Enum com dados diferentes por variante ---");
    enum_dados_diferentes();

    println!("\n--- 5. Enum complexo (tipo Message) ---");
    enum_message();

    println!("\n--- 6. Métodos em enums ---");
    metodos_em_enums();
}

// ============================================================================
// ENUM BÁSICO
// ============================================================================

/// Define as possíveis versões de IP
/// Cada valor só pode ser V4 OU V6, nunca ambos
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn enum_basico() {
    // Cria instâncias usando :: (namespace)
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Ambos são do mesmo tipo: IpAddrKind
    println!("IPv4: {:?}", four);
    println!("IPv6: {:?}", six);

    // Comparação com JS:
    // const four: IpKind = "V4";
    // const six: IpKind = "V6";
    //
    // Diferença: em Rust, as variantes são namespaced (IpAddrKind::V4)
}

// ============================================================================
// ENUM COMO PARÂMETRO DE FUNÇÃO
// ============================================================================

/// Função que aceita qualquer variante de IpAddrKind
fn route(ip_kind: IpAddrKind) {
    println!("Roteando para tipo: {:?}", ip_kind);
}

fn enum_como_parametro() {
    // Podemos passar qualquer variante
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // É como uma função que aceita o union type no TS:
    // function route(ip: "V4" | "V6") { ... }
}

// ============================================================================
// ENUM + STRUCT (forma menos elegante)
// ============================================================================

/// Primeira tentativa: struct com enum
/// Funciona, mas é verboso
#[derive(Debug)]
enum IpAddrKindV1 {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrStruct {
    kind: IpAddrKindV1,
    address: String,
}

fn enum_com_dados_simples() {
    let home = IpAddrStruct {
        kind: IpAddrKindV1::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKindV1::V6,
        address: String::from("::1"),
    };

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    // Funciona, mas podemos fazer melhor!
    // Rust permite colocar dados DIRETO nas variantes do enum
}

// ============================================================================
// ENUM COM DADOS ASSOCIADOS (forma elegante)
// ============================================================================

/// Melhor: dados associados diretamente às variantes
/// Cada variante vira um "construtor" que retorna o tipo do enum
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

/// Ainda melhor: cada variante pode ter tipos DIFERENTES!
#[derive(Debug)]
enum IpAddrDiff {
    V4(u8, u8, u8, u8), // 4 números separados
    V6(String),         // uma string
}

fn enum_dados_diferentes() {
    // Sintaxe: NomeEnum::Variante(dados)
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home (String): {:?}", home);
    println!("Loopback (String): {:?}", loopback);

    // Com tipos diferentes por variante
    let home_diff = IpAddrDiff::V4(127, 0, 0, 1);
    let loopback_diff = IpAddrDiff::V6(String::from("::1"));

    println!("Home (4 u8): {:?}", home_diff);
    println!("Loopback (String): {:?}", loopback_diff);

    // Isso é IMPOSSÍVEL com struct!
    // struct IpAddrStruct {
    //     address: ??? // não pode ser u8,u8,u8,u8 OU String
    // }

    // Comparação com TS (discriminated union):
    // type IpAddr =
    //   | { kind: "V4"; octets: [number, number, number, number] }
    //   | { kind: "V6"; address: string };
}

// ============================================================================
// ENUM MESSAGE (exemplo complexo)
// ============================================================================

/// Enum com variantes de tipos completamente diferentes
/// Mostra todo o poder dos enums em Rust
#[derive(Debug)]
enum Message {
    Quit,                       // sem dados (como unit struct)
    Move { x: i32, y: i32 },    // campos nomeados (como struct)
    Write(String),              // uma String (como tuple struct)
    ChangeColor(i32, i32, i32), // três i32 (como tuple struct)
}

fn enum_message() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("Olá!"));
    let m4 = Message::ChangeColor(255, 128, 0);

    println!("Quit: {:?}", m1);
    println!("Move: {:?}", m2);
    println!("Write: {:?}", m3);
    println!("ChangeColor: {:?}", m4);

    // Todas são do tipo Message!
    // Isso permite criar funções que aceitam qualquer mensagem:
    processar_mensagem(m1);
    processar_mensagem(m2);
    processar_mensagem(m3);
    processar_mensagem(m4);
}

fn processar_mensagem(msg: Message) {
    println!("  -> Processando: {:?}", msg);
}

// Se fossem structs separadas, precisaríamos de funções diferentes:
// fn processar_quit(msg: QuitMessage) { ... }
// fn processar_move(msg: MoveMessage) { ... }
// Ou um trait (veremos depois)

// ============================================================================
// MÉTODOS EM ENUMS
// ============================================================================

/// Assim como structs, enums podem ter métodos
/// Usamos impl igual
impl Message {
    fn call(&self) {
        println!("Método call() em: {:?}", self);
    }

    fn descricao(&self) -> &str {
        // Veremos match em detalhes depois
        match self {
            Message::Quit => "Comando para sair",
            Message::Move { .. } => "Comando para mover",
            Message::Write(_) => "Comando para escrever",
            Message::ChangeColor(_, _, _) => "Comando para mudar cor",
        }
    }
}

fn metodos_em_enums() {
    let m = Message::Write(String::from("hello"));

    // Chama método na instância
    m.call();
    println!("Descrição: {}", m.descricao());

    // self é a variante específica (Write("hello") neste caso)
}
