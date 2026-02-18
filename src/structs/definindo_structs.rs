// ============================================================================
// STRUCTS: Definindo e Instanciando
// ============================================================================
//
// Structs são como "objetos" do JS/TS, mas com tipos fixos.
// Diferente de tuplas, cada campo tem um nome.
//
// JS/TS equivalente:
//   interface User { active: boolean; username: string; ... }
//   const user: User = { active: true, username: "foo" };
//
// Rust:
//   struct User { active: bool, username: String, ... }
//   let user = User { active: true, username: String::from("foo") };
// ============================================================================

pub fn main() {
    println!("--- 1. Struct básica ---");
    struct_basica();

    println!("\n--- 2. Struct mutável ---");
    struct_mutavel();

    println!("\n--- 3. Função que retorna struct ---");
    funcao_retorna_struct();

    println!("\n--- 4. Field init shorthand ---");
    field_init_shorthand();

    println!("\n--- 5. Struct update syntax ---");
    struct_update_syntax();

    println!("\n--- 6. Tuple structs ---");
    tuple_structs();

    println!("\n--- 7. Unit-like structs ---");
    unit_structs();

    println!("\n--- 8. Ownership em structs ---");
    ownership_em_structs();
}

// ============================================================================
// DEFINIÇÃO DE STRUCT
// ============================================================================

/// Define a struct User com 4 campos tipados
/// Isso é como um `interface` ou `type` no TypeScript
struct User {
    active: bool,
    username: String, // String, não &str (a struct é dona dos dados)
    email: String,
    sign_in_count: u64,
}

/// Criando e acessando uma instância
fn struct_basica() {
    // Cria uma instância (como criar um objeto no JS)
    let user1 = User {
        active: true,
        username: String::from("rustacean123"),
        email: String::from("rust@example.com"),
        sign_in_count: 1,
    };

    // Acessa campos com notação de ponto (igual JS)
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Ativo: {}", user1.active);
    println!("Logins: {}", user1.sign_in_count);
}

// ============================================================================
// STRUCT MUTÁVEL
// ============================================================================

/// Para modificar campos, a instância INTEIRA precisa ser `mut`
/// Rust não permite marcar apenas alguns campos como mutáveis
fn struct_mutavel() {
    // mut na instância, não na struct
    let mut user1 = User {
        active: true,
        username: String::from("rustacean123"),
        email: String::from("rust@example.com"),
        sign_in_count: 1,
    };

    println!("Email original: {}", user1.email);

    // Modifica o campo (só funciona porque user1 é mut)
    user1.email = String::from("novo@example.com");

    println!("Email novo: {}", user1.email);

    // ❌ Isso não existe em Rust:
    // struct User {
    //     mut email: String,  // ERRO! Não pode ter campo mut individual
    // }
}

// ============================================================================
// FUNÇÃO QUE RETORNA STRUCT
// ============================================================================

/// Funções podem criar e retornar instâncias de structs
/// É como uma factory function no JS
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, // parâmetro → campo
        email: email,       // parâmetro → campo
        sign_in_count: 1,
    }
}

fn funcao_retorna_struct() {
    let user =
        build_user(String::from("test@email.com"), String::from("testuser"));

    println!("Criado via função: {} <{}>", user.username, user.email);
}

// ============================================================================
// FIELD INIT SHORTHAND
// ============================================================================

/// Quando o nome do parâmetro é igual ao nome do campo,
/// você pode usar a sintaxe abreviada (igual JS!)
fn build_user_short(email: String, username: String) -> User {
    User {
        active: true,
        username, // shorthand: username: username
        email,    // shorthand: email: email
        sign_in_count: 1,
    }
}

fn field_init_shorthand() {
    let user = build_user_short(
        String::from("short@email.com"),
        String::from("shortuser"),
    );

    println!("Criado com shorthand: {} <{}>", user.username, user.email);

    // Comparação com JS:
    // const email = "a@b.com";
    // const obj = { email };  // mesmo que { email: email }
}

// ============================================================================
// STRUCT UPDATE SYNTAX (spread operator do Rust)
// ============================================================================

/// Similar ao spread operator do JS: { ...user1, email: "novo" }
/// Mas com uma diferença importante: pode MOVER campos!
fn struct_update_syntax() {
    let user1 = User {
        active: true,
        username: String::from("original"),
        email: String::from("original@email.com"),
        sign_in_count: 10,
    };

    // Cria user2 copiando valores de user1, mas com email diferente
    let user2 = User {
        email: String::from("novo@email.com"),
        ..user1 // pega o resto dos campos de user1
    };

    println!("user2.email: {}", user2.email);
    println!("user2.username: {}", user2.username);
    println!("user2.active: {}", user2.active);

    // ⚠️ CUIDADO: user1.username foi MOVIDO para user2!
    // println!("{}", user1.username);  // ❌ ERRO! username foi movido

    // Mas campos Copy (bool, u64) ainda são válidos em user1:
    println!("user1.active ainda é válido: {}", user1.active);
    println!(
        "user1.sign_in_count ainda é válido: {}",
        user1.sign_in_count
    );

    // Se tivéssemos dado um NOVO username também, user1 continuaria válido:
    let user3 = User {
        active: false,
        username: String::from("user3"),
        email: String::from("user3@email.com"),
        sign_in_count: 1,
    };

    let user4 = User {
        email: String::from("user4@email.com"),
        username: String::from("user4"), // novo username (não move de user3)
        ..user3                          // só copia active e sign_in_count
    };

    // Agora user3 ainda é totalmente válido!
    println!("user3 ainda válido: {}", user3.username);
    println!("user4: {}", user4.username);
}

// ============================================================================
// TUPLE STRUCTS
// ============================================================================

/// Structs que parecem tuplas, mas têm um nome/tipo próprio
/// Útil quando você quer tipos distintos sem nomear cada campo

// Ambas têm 3 valores i32, mas são tipos DIFERENTES
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Acesso por índice (como tuplas)
    println!("Color R: {}", black.0);
    println!("Point X: {}", origin.0);

    // Desestruturação (precisa do nome do tipo)
    let Color(r, g, b) = black;
    println!("RGB: {}, {}, {}", r, g, b);

    let Point(x, y, z) = origin;
    println!("XYZ: {}, {}, {}", x, y, z);

    // ❌ Não pode passar Color onde espera Point:
    // fn move_point(p: Point) { ... }
    // move_point(black);  // ERRO! Color não é Point
}

// ============================================================================
// UNIT-LIKE STRUCTS (structs sem campos)
// ============================================================================

/// Structs sem nenhum campo, usadas para implementar traits
/// Comportam-se como () (unit type)
struct AlwaysEqual;

fn unit_structs() {
    let _subject = AlwaysEqual;

    // Parece inútil agora, mas é útil quando você quer:
    // - Implementar traits sem guardar dados
    // - Criar tipos marcadores (marker types)
    // - Singletons ou estados

    println!("Unit struct criada (sem dados)");
}

// ============================================================================
// OWNERSHIP EM STRUCTS
// ============================================================================

/// Por que usamos String e não &str nos campos?
/// Porque queremos que a struct seja DONA dos seus dados.
fn ownership_em_structs() {
    // ✅ CORRETO: struct é dona dos dados
    let user = User {
        active: true,
        username: String::from("dono"),
        email: String::from("dono@email.com"),
        sign_in_count: 1,
    };

    println!("User é dono de: {}", user.username);

    // ❌ ERRO: usar &str sem lifetime
    // struct UserRef {
    //     username: &str,  // ERRO: precisa de lifetime
    //     email: &str,     // ERRO: precisa de lifetime
    // }

    // Para usar referências, precisamos de lifetimes (capítulo 10):
    // struct UserRef<'a> {
    //     username: &'a str,
    //     email: &'a str,
    // }

    // Por enquanto, use String para structs que precisam ser donas dos dados.
    // Use &str em parâmetros de funções.
}
