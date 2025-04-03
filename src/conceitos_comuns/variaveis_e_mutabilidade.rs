pub fn main() {
    // Declara uma variável imutável (como um `const` em TypeScript)
    let x = 5;
    println!("The value of x is: {x}");

    // ⚠️ O Rust **não permite** reatribuir valor a uma variável imutável
    // Descomentar as linhas abaixo causaria erro de compilação:
    // x = 6;
    // println!("The value of x is: {x}");

    // Declara uma variável **mutável** com `mut` (como um `let` em TS)
    let mut y = 5;
    println!("The value of y is: {y}");

    // Agora podemos alterar o valor de `y` livremente
    y = 6;
    println!("The value of y is: {y}");

    // Declaração de uma constante
    // Sempre imutável, precisa de anotação de tipo e deve conter uma expressão constante
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Demonstração de shadowing
    let x = 5; // x₁
    println!("x₁ endereço: {:p}", &x);

    let x = x + 1; // x₂ (sombra x₁)
    println!("x₂ valor: {x}");
    println!("x₂ endereço: {:p}", &x);

    {
        // Novo escopo → shadowing de novo
        let x = x * 2; // x₃ (sombra x₂ temporariamente)
        println!("O valor de x no escopo interno é: {x}");
        println!("x₃ (interno) endereço: {:p}", &x);
    }

    // Fora do escopo, x₃ morreu, volta a valer x₂
    println!("O valor de x no escopo externo é: {x}");
    println!("x₂ (ainda válido) endereço: {:p}", &x);

    // Shadowing para mudar tipo: de &str → usize
    let spaces = "   "; // tipo: &str

    let spaces = spaces.len(); // tipo: usize (muda tipo com shadowing)
    println!("spaces (len): {spaces}");

    // Mutável, mas tentando mudar tipo — causa erro!
    let mut _spaces = "   "; // tipo: &str
    // spaces = _spaces.len();     // ❌ Erro: não pode mudar tipo com `mut`
    // println!("spaces: {_spaces}");

    // O Rust não permite mudar o tipo de uma variável `mut` após sua criação.
    // Para isso, usamos shadowing.
}
