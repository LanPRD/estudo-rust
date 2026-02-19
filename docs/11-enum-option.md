# O Enum Option<T>

Rust **não tem null**! Em vez disso, usa `Option<T>` para representar valores que podem estar ausentes.

## O Problema do Null

```javascript
// JS: null pode causar erros em runtime
let user = getUser(); // pode retornar null
console.log(user.name); // 💥 Cannot read property 'name' of null
```

Tony Hoare, inventor do null, chamou isso de "erro de um bilhão de dólares".

Rust resolve em **tempo de compilação**!

---

## Definição de Option

```rust
// Definido na biblioteca padrão:
enum Option<T> {
    Some(T),  // tem um valor do tipo T
    None,     // não tem valor
}
```

`<T>` é um tipo genérico (capítulo 10). Por agora: T pode ser qualquer tipo.

---

## Criando Option

```rust
// Com valor
let some_number = Some(5);                    // Option<i32>
let some_char = Some('e');                    // Option<char>
let some_string = Some(String::from("olá"));  // Option<String>

// Sem valor (precisa anotar o tipo!)
let absent_number: Option<i32> = None;
let absent_string: Option<String> = None;
```

**Por que precisa anotar tipo no None?**

`None` sozinho não diz qual tipo _seria_ o valor. O compilador precisa saber para checar tipos depois.

`Some` e `None` estão no "prelude" (importados automaticamente) - não precisa de `Option::Some()`.

---

## Option<T> NÃO é T

Essa é a grande diferença para null:

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;  // ❌ ERRO!
```

```rust
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
  | let sum = x + y;
  |             ^ no implementation for `i8 + Option<i8>`
```

**Você é OBRIGADO a extrair o valor antes de usar!**

```rust
// Para usar, precisa tratar o caso None:
let sum = match y {
    Some(valor) => x + valor,
    None => x,  // se y for None, só usa x
};
```

```text
┌─────────────────────────────────────────────────────────────────┐
│  DIFERENÇA CRUCIAL                                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  JavaScript:                                                    │
│    let x = getValue();  // pode ser null                        │
│    let y = x + 5;       // ✅ compila, 💥 erro em runtime       │
│                                                                 │
│  Rust:                                                          │
│    let x: Option<i32> = get_value();                            │
│    let y = x + 5;       // ❌ NÃO COMPILA!                      │
│                         // Você é forçado a tratar None         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Métodos Úteis de Option

```rust
let some_value: Option<i32> = Some(42);
let none_value: Option<i32> = None;

// Checagem
some_value.is_some()  // true
some_value.is_none()  // false
none_value.is_some()  // false
none_value.is_none()  // true

// Extração segura
some_value.unwrap_or(0)     // 42 (valor existe)
none_value.unwrap_or(0)     // 0  (usa valor padrão)

// Extração com closure
none_value.unwrap_or_else(|| calcular_padrao())

// ⚠️ Extração PERIGOSA (panic se None!)
some_value.unwrap()          // 42
none_value.unwrap()          // 💥 PANIC!

// Com mensagem de erro melhor
some_value.expect("Deveria ter valor!")  // 42
none_value.expect("Deveria ter valor!")  // 💥 PANIC com mensagem
```

---

## Quando Usar Option

### 1. Campos Opcionais em Structs

```rust
struct User {
    name: String,
    email: String,
    age: Option<u32>,  // idade é opcional
}

let user = User {
    name: String::from("João"),
    email: String::from("joao@email.com"),
    age: None,  // não quis informar
};
```

### 2. Funções que Podem Não Encontrar

```rust
fn find_user(name: &str) -> Option<User> {
    if encontrou {
        Some(user)
    } else {
        None
    }
}

// Usando:
match find_user("admin") {
    Some(user) => println!("Encontrou: {}", user.name),
    None => println!("Não encontrado"),
}
```

### 3. Operações em Coleções

```rust
let numeros = vec![10, 20, 30];
let vazio: Vec<i32> = vec![];

numeros.first()  // Some(&10)
vazio.first()    // None
```

---

## Resumo Visual

```text
┌────────────────────────────────────────────────────────────────┐
│  ENUMS                                                         │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  Definir:    enum Nome { Var1, Var2(tipo), Var3 { campo: T } } │
│  Criar:      Nome::Variante ou Nome::Variante(dados)           │
│  Método:     impl Nome { fn metodo(&self) { } }                │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│  OPTION<T>                                                     │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  Some(valor)  → tem valor                                      │
│  None         → sem valor (substitui null)                     │
│                                                                │
│  .unwrap_or(padrão)   → valor ou padrão (SEGURO)               │
│  .unwrap()            → valor ou PANIC (PERIGOSO)              │
│  .is_some() / .is_none() → checagem                            │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│  VANTAGEM SOBRE NULL                                           │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  Option<T> ≠ T   →   O compilador FORÇA você a tratar None!    │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```
