mod conceitos_comuns;
mod enums;
mod jogo_de_advinhacao;
mod ownership;
mod structs;

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

    println!("\nControle de fluxo\n");
    conceitos_comuns::controle_de_fluxo::main();

    println!("\nOwnership\n");
    ownership::memoria::main();

    println!("\nOwnership\n");
    ownership::o_que_e::main();

    ownership::referencias_e_borrowing::main();
    ownership::slice::main();

    println!("\nStructs\n");
    structs::definindo_structs::main();

    println!("\nStructs - Exemplo Retângulo\n");
    structs::exemplo_retangulo::main();

    println!("\nMétodos\n");
    structs::metodos::main();

    println!("\nEnums\n");
    enums::definindo_enums::main();

    println!("\nOption<T>\n");
    enums::option_enum::main();

    println!("\nif let e let...else\n");
    enums::if_let::main();
}
