// ============================================================================
// EXEMPLO PRÁTICO: Calculando Área de Retângulo
// ============================================================================
//
// Este arquivo mostra a evolução do código:
//   1. Variáveis soltas → funciona, mas confuso
//   2. Tuplas → agrupa, mas sem nomes
//   3. Structs → agrupa COM nomes (melhor!)
//
// Também introduz:
//   - #[derive(Debug)] para imprimir structs
//   - {:?} e {:#?} para formatação de debug
//   - dbg!() macro para debugging
//
// ============================================================================

pub fn main() {
    println!("--- 1. Com variáveis soltas (ruim) ---");
    com_variaveis_soltas();

    println!("\n--- 2. Com tuplas (melhor, mas ainda confuso) ---");
    com_tuplas();

    println!("\n--- 3. Com structs (ideal) ---");
    com_structs();

    println!("\n--- 4. Debug trait e formatação ---");
    debug_trait();

    println!("\n--- 5. Macro dbg!() ---");
    macro_dbg();
}

// ============================================================================
// VERSÃO 1: VARIÁVEIS SOLTAS
// ============================================================================

/// Funciona, mas os parâmetros não têm relação clara entre si
fn com_variaveis_soltas() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "Área do retângulo: {} pixels quadrados",
        area_v1(width1, height1)
    );

    // Problema: olhando a assinatura da função, não fica claro
    // que width e height pertencem ao MESMO retângulo
}

fn area_v1(width: u32, height: u32) -> u32 {
    width * height
}

// ============================================================================
// VERSÃO 2: COM TUPLAS
// ============================================================================

/// Melhor: agrupa os valores, mas perde clareza nos nomes
fn com_tuplas() {
    let rect1 = (30, 50); // (largura, altura)

    println!(
        "Área do retângulo: {} pixels quadrados",
        area_v2(rect1)
    );

    // Problema: dimensions.0 é largura ou altura?
    // Fácil de confundir e introduzir bugs
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 // 0 = largura? altura? quem sabe...
}

// ============================================================================
// VERSÃO 3: COM STRUCTS (IDEAL)
// ============================================================================

/// Struct sem Debug (ainda não podemos imprimir com {:?})
struct RectangleSemDebug {
    width: u32,
    height: u32,
}

/// Struct COM Debug - permite imprimir com {:?}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn com_structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Passa por REFERÊNCIA (&rect1) para não mover ownership
    println!(
        "Área do retângulo: {} pixels quadrados",
        area_v3(&rect1)
    );

    // rect1 ainda é válido porque só emprestamos
    println!("Largura: {}", rect1.width);
    println!("Altura: {}", rect1.height);
}

/// Recebe referência imutável - não toma posse, só lê
fn area_v3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
    // Acessar campos de uma referência NÃO move os valores
}

// ============================================================================
// DEBUG TRAIT E FORMATAÇÃO
// ============================================================================

fn debug_trait() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // ❌ Isso NÃO funciona sem #[derive(Debug)]:
    // let rect_sem = RectangleSemDebug { width: 10, height: 20 };
    // println!("{:?}", rect_sem);  // ERRO: não implementa Debug

    // ✅ Com #[derive(Debug)], podemos usar {:?}
    println!("Debug simples:  {:?}", rect);

    // ✅ {:#?} formata de forma mais legível (pretty print)
    println!("Debug bonito:   {:#?}", rect);

    // Comparação de saída:
    // {:?}  → Rectangle { width: 30, height: 50 }
    // {:#?} → Rectangle {
    //             width: 30,
    //             height: 50,
    //         }
}

// ============================================================================
// MACRO dbg!()
// ============================================================================

fn macro_dbg() {
    let scale = 2;

    // dbg!() imprime e RETORNA o valor
    // Útil para debug inline sem quebrar o código
    let rect = Rectangle {
        width: dbg!(30 * scale), // imprime: [arquivo:linha] 30 * scale = 60
        height: 50,
    };

    // Para structs, use referência para não mover
    dbg!(&rect);

    // Diferenças entre println! e dbg!:
    //
    // | println!          | dbg!                    |
    // |-------------------|-------------------------|
    // | Recebe referência | Toma ownership (ou &)   |
    // | Saída: stdout     | Saída: stderr           |
    // | Sem arquivo/linha | Mostra arquivo e linha  |
    // | Não retorna valor | Retorna o valor         |

    println!("\nValor final de rect.width: {}", rect.width);
}
