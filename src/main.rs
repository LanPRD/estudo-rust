mod conceitos_comuns;
mod jogo_de_advinhacao;

fn main() {
    println!("\nComeçando o programa!\n");

    // println!("\nJogo de advinhação!\n");
    // jogo_de_advinhacao::main();

    println!("\nVariáveis e mutabilidade\n");
    conceitos_comuns::variaveis_e_mutabilidade::main();

    println!("\nTipos de dados\n");
    conceitos_comuns::tipos_de_dados::main();

    println!("\nFunções\n");
    conceitos_comuns::funcoes::main();
}
