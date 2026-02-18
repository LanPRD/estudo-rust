// ============================================================================
// MÉTODOS: Funções associadas a Structs
// ============================================================================
//
// Em JS/TS, métodos são funções dentro de classes:
//   class Rectangle {
//     area() { return this.width * this.height; }
//   }
//
// Em Rust, métodos ficam em blocos `impl`:
//   impl Rectangle {
//     fn area(&self) -> u32 { self.width * self.height }
//   }
//
// Diferença chave: `self` é explícito e tem variantes (&self, &mut self, self)
//
// ============================================================================

pub fn main() {
    println!("--- 1. Método básico com &self ---");
    metodo_basico();

    println!("\n--- 2. Tipos de self ---");
    tipos_de_self();

    println!("\n--- 3. Método com mesmo nome de campo ---");
    metodo_mesmo_nome_campo();

    println!("\n--- 4. Métodos com mais parâmetros ---");
    metodos_com_parametros();

    println!("\n--- 5. Associated functions (construtores) ---");
    associated_functions();

    println!("\n--- 6. Múltiplos blocos impl ---");
    multiplos_impl();
}

// ============================================================================
// STRUCT BASE PARA OS EXEMPLOS
// ============================================================================

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ============================================================================
// BLOCO IMPL - ONDE DEFINIMOS MÉTODOS
// ============================================================================

impl Rectangle {
    /// Método que só lê dados (&self = referência imutável)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Método que modifica dados (&mut self = referência mutável)
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    /// Método que consome a instância (self = toma ownership)
    /// Raramente usado, mas útil para transformações
    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle {
            width: side,
            height: side,
        }
    }

    /// Método com mesmo nome de um campo (getter)
    fn width(&self) -> bool {
        self.width > 0
    }

    /// Método que recebe outro Rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// Associated function (sem self) - funciona como construtor
    /// Chamada com :: ao invés de .
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Outro construtor: cria um quadrado
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// ============================================================================
// EXEMPLOS DE USO
// ============================================================================

fn metodo_basico() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // Chamada de método: instância.método()
    // Diferente de função: area(&rect)
    println!("Área: {} px²", rect.area());

    // Rust faz "automatic referencing" automaticamente
    // Não precisa de -> como em C/C++
    // Essas duas linhas são equivalentes:
    // rect.area()
    // (&rect).area()
}

fn tipos_de_self() {
    // &self - apenas lê (mais comum)
    let rect1 = Rectangle::new(10, 20);
    println!("Área (só leitura): {}", rect1.area());

    // &mut self - modifica
    let mut rect2 = Rectangle::new(10, 20);
    println!("Antes: {:?}", rect2);
    rect2.double_size();
    println!("Depois de double_size: {:?}", rect2);

    // self - consome (toma ownership)
    let rect3 = Rectangle::new(10, 20);
    let square = rect3.into_square();
    println!("Transformado em quadrado: {:?}", square);
    // println!("{:?}", rect3);  // ❌ ERRO! rect3 foi consumido
}

fn metodo_mesmo_nome_campo() {
    let rect = Rectangle::new(30, 50);

    // rect.width  → acessa o CAMPO (u32)
    // rect.width() → chama o MÉTODO (bool)

    if rect.width() {
        println!("O retângulo tem largura não-zero: {}", rect.width);
    }

    // Isso é útil para criar getters que fazem mais do que
    // simplesmente retornar o valor
}

fn metodos_com_parametros() {
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);

    // Método que recebe outro Rectangle por referência
    println!("rect1 pode conter rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 pode conter rect3? {}", rect1.can_hold(&rect3));

    // rect2 e rect3 ainda são válidos (passamos por &)
    println!("rect2 ainda existe: {:?}", rect2);
}

fn associated_functions() {
    // Associated functions são chamadas com :: (não com .)
    // São como métodos estáticos em outras linguagens

    // Construtor padrão
    let rect1 = Rectangle::new(30, 50);
    println!("Criado com new: {:?}", rect1);

    // Construtor especializado
    let square = Rectangle::square(25);
    println!("Criado com square: {:?}", square);

    // Você já conhece uma associated function:
    // String::from("texto")
    //        ↑↑
    //   :: indica associated function
}

fn multiplos_impl() {
    // Você pode ter vários blocos impl para a mesma struct
    // Útil para organização ou quando usar generics/traits
    println!("Múltiplos blocos impl são válidos (veja o código)");
}

// ============================================================================
// MÚLTIPLOS BLOCOS IMPL (válido!)
// ============================================================================

// Podemos separar métodos em blocos diferentes
// Isso é equivalente a ter tudo em um bloco só

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// Na prática, você usa múltiplos impl quando:
// - Implementa traits diferentes
// - Usa generics com bounds diferentes
// - Quer organizar código de forma específica
