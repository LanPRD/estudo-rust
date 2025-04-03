pub fn main() {
    println!("Início do programa!");

    // Chamada de função sem parâmetros e sem retorno
    mensagem_simples();

    // Chamada de função com parâmetro
    imprimir_valor(42);

    // Chamada de função com múltiplos parâmetros
    imprimir_medida(10, 'm');

    // Chamada de função que retorna um valor
    let cinco = retorna_cinco(); // recebe o valor retornado (5)
    println!("A função 'retorna_cinco' retornou: {cinco}");

    // Usando o valor de retorno de outra função
    let seis = soma_um(5);
    println!("Resultado de 'soma_um(5)': {seis}");

    // Bloco de código como expressão (retorna um valor)
    let resultado = {
        let x = 2;
        x * 3 // sem ponto e vírgula = retorna 6
    };

    println!("Valor retornado pelo bloco: {resultado}");

    // ERRO se tentássemos colocar ponto e vírgula no bloco acima:
    // let resultado = { let x = 2; x * 3; }; // retornaria `()` e daria erro se esperássemos `i32`
}

fn mensagem_simples() {
    println!("Executando uma função sem parâmetros!");
}

fn imprimir_valor(x: i32) {
    println!("Valor recebido: {x}");
}

fn imprimir_medida(valor: i32, unidade: char) {
    println!("A medida é: {valor}{unidade}");
}

fn retorna_cinco() -> i32 {
    5
}

fn soma_um(x: i32) -> i32 {
    x + 1
}
