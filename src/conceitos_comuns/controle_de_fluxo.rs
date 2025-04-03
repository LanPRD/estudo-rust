pub fn main() {
    if_examples();
    if_expression_result();
    else_if_chain();
    loop_with_break_return();
    labeled_loops();
    while_loop_countdown();
    while_array_iteration();
    for_array_iteration();
    for_reverse_countdown();
}

// ====== IF / ELSE simples ======
fn if_examples() {
    let number = 7;

    if number < 5 {
        println!("A condição era verdadeira");
    } else {
        println!("A condição era falsa");
    }

    if number != 0 {
        println!("Número diferente de zero");
    }

    // ❌ Isso não compila:
    // if number {
    //     println!("Isso dá erro: `number` precisa ser bool!");
    // }
}

fn if_expression_result() {
    let condition = true;
    let result = if condition { 10 } else { 20 };
    println!("Resultado do if expression: {result}");
}

fn else_if_chain() {
    let number = 6;

    if number % 4 == 0 {
        println!("Divisível por 4");
    } else if number % 3 == 0 {
        println!("Divisível por 3");
    } else if number % 2 == 0 {
        println!("Divisível por 2");
    } else {
        println!("Não é divisível por 4, 3 ou 2");
    }
}

// ====== loop com break que retorna valor ======
fn loop_with_break_return() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 3 {
            println!("Interrompendo loop!");
            break counter * 2;
        }
    };

    println!("Valor retornado pelo loop: {result}");
}

// ====== Loops aninhados com rótulo (label) ======
fn labeled_loops() {
    let mut count = 0;

    'outer: loop {
        println!("count = {count}");
        let mut inner = 10;

        loop {
            println!("inner = {inner}");
            if inner == 9 {
                break;
            }
            if count == 2 {
                break 'outer; // quebra o loop rotulado
            }
            inner -= 1;
        }

        count += 1;
    }

    println!("Fim do loop com labels, count = {count}");
}

// ====== while para contagem regressiva ======
fn while_loop_countdown() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("DECOLAR!!!");
}

// ====== Iteração de array com while (não recomendada) ======
fn while_array_iteration() {
    let array = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < array.len() {
        println!("Array[{}] = {}", index, array[index]);
        index += 1;
    }
}

// ====== Iteração de array com for (recomendada) ======
fn for_array_iteration() {
    let array = [10, 20, 30, 40, 50];

    for element in array {
        println!("Elemento do array: {element}");
    }
}

// ====== Contagem regressiva com for + rev + range ======
fn for_reverse_countdown() {
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("DECOLAR COM RANGE!");
}
