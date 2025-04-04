pub fn main() {
    stack_com_tipo_primario();
    heap_com_string();
    dado_estatico_com_str_literal();
    array_na_stack();
    slice_estatica();
}

/// STACK: valores de tipo simples e tamanho fixo são armazenados diretamente na stack.
/// Isso inclui `i32`, `bool`, `char`, arrays fixos, etc.
fn stack_com_tipo_primario() {
    let x: i32 = 10;
    let y: bool = true;
    let z: char = 'A';

    // Todos esses valores estão na stack — são rápidos de acessar e descartados automaticamente.
    println!("x: {}, y: {}, z: {}", x, y, z);
}

/// HEAP: quando usamos tipos como `String`, os dados são alocados na heap.
/// A variável na stack contém apenas uma estrutura com ponteiro, tamanho e capacidade.
fn heap_com_string() {
    let texto = String::from("alocado na heap");

    // O conteúdo ("alocado na heap") está na heap.
    // A stack contém apenas a estrutura String com metadados.
    println!("String na heap: {}", texto);
}

/// ESTÁTICO: literais de string (`"texto"`) são armazenados em memória estática.
/// Essa memória está embutida no binário e vive durante toda a execução do programa.
fn dado_estatico_com_str_literal() {
    let s: &str = "sou um dado estático";

    // A variável `s` está na stack, mas o conteúdo "sou um dado estático" está na área estática.
    println!("&str estático: {}", s);
}

/// STACK: arrays de tamanho fixo (como [u8; 5]) são inteiramente armazenados na stack.
/// Isso inclui o conteúdo real — não há alocação na heap.
fn array_na_stack() {
    let arr: [u8; 5] = [1, 2, 3, 4, 5];

    // Tanto o array quanto seus elementos estão na stack.
    println!("Array na stack: {:?}", arr);
}

/// ESTÁTICO: slices fixas com `b"..."` ou `&[u8]` em geral podem apontar para dados estáticos.
/// `b"..."` é um literal de bytes com duração `'static`.
fn slice_estatica() {
    let slice: &[u8] = b"static slice";

    // slice aponta para dados em área estática
    println!("Slice estática: {:?}", slice);
}
