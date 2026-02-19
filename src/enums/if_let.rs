// ============================================================================
// IF LET e LET...ELSE: Controle de Fluxo Conciso
// ============================================================================
//
// Quando você só quer tratar UM caso de um enum, `match` é verboso.
// `if let` e `let...else` são atalhos para esses cenários.
//
// JS/TS equivalente:
//   // if let ~= checagem simples
//   if (valor !== null) { usa(valor); }
//
//   // let...else ~= early return / guard clause
//   if (!valor) return;
//   usa(valor);
//
// ============================================================================

pub fn main() {
    println!("--- 1. Match verboso ---");
    match_verboso();

    println!("\n--- 2. if let ---");
    if_let_basico();

    println!("\n--- 3. if let com else ---");
    if_let_com_else();

    println!("\n--- 4. let...else ---");
    let_else_basico();

    println!("\n--- 5. Exemplo prático ---");
    exemplo_pratico();
}

// ============================================================================
// MATCH VERBOSO
// ============================================================================

fn match_verboso() {
    let config_max = Some(3u8);

    // Queremos agir só quando é Some
    // Com match, precisamos lidar com TODOS os casos
    match config_max {
        Some(max) => println!("Máximo configurado: {}", max),
        _ => (), // não faz nada, mas é obrigatório 😒
    }

    println!("Esse `_ => ()` é boilerplate irritante!");
}

// ============================================================================
// IF LET BÁSICO
// ============================================================================

fn if_let_basico() {
    let config_max = Some(3u8);

    // Mesmo comportamento, menos código
    if let Some(max) = config_max {
        println!("Máximo configurado: {}", max);
    }

    // Sintaxe: if let PADRÃO = EXPRESSÃO { ... }
    // Se o padrão casar, executa o bloco

    // Funciona com qualquer enum, não só Option
    #[derive(Debug)]
    enum Cor {
        Rgb(u8, u8, u8),
        Hex(String),
    }

    let cor = Cor::Rgb(255, 128, 0);

    if let Cor::Rgb(r, g, b) = cor {
        println!("Cor RGB: ({}, {}, {})", r, g, b);
    }

    // Se não casar, simplesmente não executa
    let outra_cor = Cor::Hex(String::from("#FF8800"));

    if let Cor::Rgb(r, g, b) = outra_cor {
        // Isso não vai executar
        println!("Nunca imprime: ({}, {}, {})", r, g, b);
    }

    println!("(outra_cor era Hex, então if let não executou)");
}

// ============================================================================
// IF LET COM ELSE
// ============================================================================

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn if_let_com_else() {
    let mut count = 0;

    let coins = vec![
        Coin::Quarter(String::from("Alaska")),
        Coin::Penny,
        Coin::Dime,
        Coin::Quarter(String::from("Texas")),
        Coin::Nickel,
    ];

    for coin in coins {
        // Anuncia quarters, conta o resto
        if let Coin::Quarter(state) = coin {
            println!("Quarter do estado: {}!", state);
        } else {
            count += 1;
        }
    }

    println!("Total de moedas não-quarter: {}", count);

    // Equivalente com match:
    // match coin {
    //     Coin::Quarter(state) => println!("Quarter de {}!", state),
    //     _ => count += 1,
    // }
}

// ============================================================================
// LET...ELSE
// ============================================================================

fn let_else_basico() {
    // let...else é útil para "early return" / "guard clauses"
    // Extrai o valor OU sai do fluxo

    fn process_number(maybe_num: Option<i32>) -> i32 {
        // Se não for Some, retorna 0 imediatamente
        let Some(num) = maybe_num else {
            println!("  Nenhum número, retornando 0");
            return 0;
        };

        // Aqui, `num` está disponível (extraído do Some)
        println!("  Processando número: {}", num);
        num * 2
    }

    println!("Com Some(5): {}", process_number(Some(5)));
    println!("Com None: {}", process_number(None));

    // A diferença para if let:
    // - if let: código útil DENTRO do bloco if (aninhado)
    // - let...else: código útil DEPOIS do let (nível principal)
}

// ============================================================================
// EXEMPLO PRÁTICO: Múltiplas Validações
// ============================================================================

fn exemplo_pratico() {
    // let...else brilha com várias checagens em sequência

    fn parse_config(input: Option<&str>) -> Result<i32, &'static str> {
        // Validação 1: input existe?
        let Some(text) = input else {
            return Err("Input vazio");
        };

        // Validação 2: não está em branco?
        let text = text.trim();
        if text.is_empty() {
            return Err("Input em branco");
        }

        // Validação 3: é um número?
        let Ok(number) = text.parse::<i32>() else {
            return Err("Não é um número válido");
        };

        // Validação 4: está no range?
        if number < 0 || number > 100 {
            return Err("Número fora do range 0-100");
        }

        // Caminho feliz: todas as validações passaram!
        Ok(number)
    }

    let testes = vec![
        None,
        Some(""),
        Some("   "),
        Some("abc"),
        Some("150"),
        Some("42"),
    ];

    for teste in testes {
        match parse_config(teste) {
            Ok(n) => println!("{:?} => Ok({})", teste, n),
            Err(e) => println!("{:?} => Err(\"{}\")", teste, e),
        }
    }
}
