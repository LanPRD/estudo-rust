// ============================================================================
// O ENUM OPTION<T>: Lidando com Ausência de Valor
// ============================================================================
//
// Rust NÃO TEM NULL! Em vez disso, usa Option<T> para representar
// "talvez tenha um valor, talvez não".
//
// JS/TS:
//   let x: string | null = null;      // pode ser null
//   let y: string | undefined;        // pode ser undefined
//
// Rust:
//   let x: Option<String> = None;     // sem valor
//   let y: Option<String> = Some(String::from("valor")); // com valor
//
// A diferença crucial: você é OBRIGADO a lidar com o caso None!
// O compilador não deixa você usar um Option<T> como se fosse T.
// ============================================================================

pub fn main() {
    println!("--- 1. O problema do null ---");
    problema_do_null();

    println!("\n--- 2. Criando Option ---");
    criando_option();

    println!("\n--- 3. Option precisa de tratamento ---");
    option_precisa_tratamento();

    println!("\n--- 4. Métodos úteis de Option ---");
    metodos_option();

    println!("\n--- 5. Quando usar Option ---");
    quando_usar_option();
}

// ============================================================================
// O PROBLEMA DO NULL
// ============================================================================

fn problema_do_null() {
    // Em JS, null pode causar erros em runtime:
    //
    //   let user = getUser();  // pode retornar null
    //   console.log(user.name); // 💥 Cannot read property 'name' of null
    //
    // O inventor do null chamou isso de "erro de um bilhão de dólares".
    //
    // Rust resolve isso em tempo de compilação!
    // Se algo PODE ser nulo, o tipo OBRIGA você a tratar.

    println!("Em Rust, null não existe!");
    println!("Usamos Option<T> para representar presença/ausência de valor.");
    println!("O compilador GARANTE que você trate o caso de ausência.");
}

// ============================================================================
// CRIANDO OPTION
// ============================================================================

fn criando_option() {
    // Option é um enum definido assim na biblioteca padrão:
    //
    // enum Option<T> {
    //     Some(T),  // tem um valor do tipo T
    //     None,     // não tem valor
    // }
    //
    // <T> é um tipo genérico (veremos em detalhes no cap. 10)
    // Por enquanto: T pode ser qualquer tipo

    // Com valor
    let some_number = Some(5);           // Option<i32>
    let some_char = Some('e');           // Option<char>
    let some_string = Some(String::from("olá")); // Option<String>

    println!("some_number: {:?}", some_number);
    println!("some_char: {:?}", some_char);
    println!("some_string: {:?}", some_string);

    // Sem valor (precisa anotar o tipo!)
    let absent_number: Option<i32> = None;
    let absent_string: Option<String> = None;

    println!("absent_number: {:?}", absent_number);
    println!("absent_string: {:?}", absent_string);

    // Por que precisa anotar tipo no None?
    // Porque None sozinho não diz qual tipo SERIA o valor.
    // O compilador precisa saber para checar tipos depois.

    // Some e None são tão comuns que estão no "prelude"
    // (importados automaticamente). Não precisa de Option::Some()
}

// ============================================================================
// OPTION<T> NÃO É T!
// ============================================================================

fn option_precisa_tratamento() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // ❌ ERRO: não pode somar i8 com Option<i8>!
    // let sum = x + y;
    //
    // error[E0277]: cannot add `Option<i8>` to `i8`
    //  --> src/main.rs:5:17
    //  |
    //  | let sum = x + y;
    //  |             ^ no implementation for `i8 + Option<i8>`

    println!("x = {}", x);
    println!("y = {:?}", y);
    println!("x + y = ???  (não compila!)");

    // Para usar o valor de Option, você PRECISA extraí-lo
    // Isso força você a lidar com o caso None

    // Forma 1: match (veremos em detalhes)
    let sum = match y {
        Some(valor) => x + valor,
        None => x, // se y for None, só usa x
    };
    println!("Usando match: x + y = {}", sum);

    // Forma 2: unwrap_or (valor padrão se for None)
    let sum2 = x + y.unwrap_or(0);
    println!("Usando unwrap_or: x + y = {}", sum2);

    // Isso é a grande vantagem sobre null:
    // Em JS: você PODE esquecer de checar null
    // Em Rust: você é OBRIGADO a lidar com None
}

// ============================================================================
// MÉTODOS ÚTEIS DE OPTION
// ============================================================================

fn metodos_option() {
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    // is_some() e is_none()
    println!("some_value.is_some(): {}", some_value.is_some()); // true
    println!("some_value.is_none(): {}", some_value.is_none()); // false
    println!("none_value.is_some(): {}", none_value.is_some()); // false
    println!("none_value.is_none(): {}", none_value.is_none()); // true

    println!();

    // unwrap() - extrai o valor OU panic se for None
    // ⚠️ Use com cuidado! Só quando tem CERTEZA que não é None
    let valor = some_value.unwrap();
    println!("some_value.unwrap(): {}", valor);

    // ❌ Isso causa panic!
    // let crash = none_value.unwrap(); // panic: called `unwrap()` on None

    // unwrap_or() - valor padrão se for None (mais seguro)
    let valor1 = some_value.unwrap_or(0);
    let valor2 = none_value.unwrap_or(0);
    println!("some_value.unwrap_or(0): {}", valor1); // 42
    println!("none_value.unwrap_or(0): {}", valor2); // 0

    // unwrap_or_else() - closure para calcular valor padrão
    let valor3 = none_value.unwrap_or_else(|| {
        println!("  (calculando valor padrão...)");
        100
    });
    println!("none_value.unwrap_or_else: {}", valor3);

    // expect() - como unwrap, mas com mensagem personalizada
    let valor4 = some_value.expect("Deveria ter um valor!");
    println!("some_value.expect(): {}", valor4);

    // ❌ Melhor para debug:
    // let crash = none_value.expect("Valor era None quando não deveria!");
}

// ============================================================================
// QUANDO USAR OPTION
// ============================================================================

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: Option<u32>, // idade é opcional
}

/// Busca usuário por nome (pode não encontrar)
fn find_user(name: &str) -> Option<User> {
    // Simula uma busca
    if name == "admin" {
        Some(User {
            name: String::from("admin"),
            email: String::from("admin@example.com"),
            age: Some(30),
        })
    } else if name == "guest" {
        Some(User {
            name: String::from("guest"),
            email: String::from("guest@example.com"),
            age: None, // não informou idade
        })
    } else {
        None // usuário não encontrado
    }
}

/// Busca primeiro elemento de um vetor (pode estar vazio)
fn first_element(vec: &[i32]) -> Option<&i32> {
    if vec.is_empty() {
        None
    } else {
        Some(&vec[0])
    }
    // Nota: Vec já tem o método .first() que faz exatamente isso!
}

fn quando_usar_option() {
    // Cenário 1: Campos opcionais em structs
    let user1 = User {
        name: String::from("João"),
        email: String::from("joao@email.com"),
        age: Some(25),
    };

    let user2 = User {
        name: String::from("Maria"),
        email: String::from("maria@email.com"),
        age: None, // não quis informar
    };

    println!("User 1: {:?}", user1);
    println!("User 2: {:?}", user2);

    // Cenário 2: Funções que podem falhar em encontrar algo
    match find_user("admin") {
        Some(user) => println!("Encontrou: {:?}", user),
        None => println!("Usuário não encontrado"),
    }

    match find_user("inexistente") {
        Some(user) => println!("Encontrou: {:?}", user),
        None => println!("Usuário 'inexistente' não encontrado"),
    }

    // Cenário 3: Operações em coleções
    let numeros = vec![10, 20, 30];
    let vazio: Vec<i32> = vec![];

    println!("Primeiro de numeros: {:?}", first_element(&numeros));
    println!("Primeiro de vazio: {:?}", first_element(&vazio));

    // Rust já tem isso embutido:
    println!("numeros.first(): {:?}", numeros.first());
    println!("vazio.first(): {:?}", vazio.first());
}
