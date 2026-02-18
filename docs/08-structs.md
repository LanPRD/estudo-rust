# Structs

Structs são como objetos/interfaces do TypeScript, mas com algumas diferenças importantes.

## O Básico

```rust
// Definição (como interface no TS)
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Instanciação (como criar objeto no JS)
let user1 = User {
    active: true,
    username: String::from("rust123"),
    email: String::from("rust@example.com"),
    sign_in_count: 1,
};

// Acesso (igual JS)
println!("{}", user1.email);
```

```
┌─────────────────────────────────────────────────────────┐
│  TypeScript                 │  Rust                     │
├─────────────────────────────┼───────────────────────────┤
│  interface User {           │  struct User {            │
│    active: boolean;         │    active: bool,          │
│    username: string;        │    username: String,      │
│  }                          │  }                        │
├─────────────────────────────┼───────────────────────────┤
│  const user: User = {       │  let user = User {        │
│    active: true,            │    active: true,          │
│    username: "foo"          │    username: String::from │
│  };                         │      ("foo"),             │
│                             │  };                       │
└─────────────────────────────────────────────────────────┘
```

---

## Mutabilidade

Para modificar campos, a **instância inteira** precisa ser `mut`:

```rust
let mut user1 = User {
    active: true,
    username: String::from("rust123"),
    email: String::from("old@email.com"),
    sign_in_count: 1,
};

user1.email = String::from("new@email.com");  // ✅ OK
```

**Importante:** Rust não permite marcar campos individuais como mutáveis:

```rust
// ❌ Isso NÃO existe em Rust:
struct User {
    mut email: String,  // ERRO!
}
```

---

## Funções que Retornam Structs

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

---

## Field Init Shorthand

Quando parâmetro e campo têm o mesmo nome, você pode abreviar (igual JS!):

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,        // ao invés de username: username
        email,           // ao invés de email: email
        sign_in_count: 1,
    }
}
```

```
┌────────────────────────────────────────────┐
│  JS:   const obj = { email, username };    │
│  Rust: User { email, username, ... }       │
│        (mesma ideia!)                      │
└────────────────────────────────────────────┘
```

---

## Struct Update Syntax (Spread do Rust)

Similar ao spread operator do JS, mas com uma diferença crucial: **pode mover dados**.

```rust
let user1 = User {
    active: true,
    username: String::from("original"),
    email: String::from("original@email.com"),
    sign_in_count: 10,
};

// Cria user2 usando valores de user1, mas com email diferente
let user2 = User {
    email: String::from("novo@email.com"),
    ..user1  // pega o resto de user1
};
```

```
┌────────────────────────────────────────────────────────┐
│  JS:   const user2 = { ...user1, email: "novo" };     │
│  Rust: let user2 = User { email: "...", ..user1 };    │
│                                            ↑          │
│                               (precisa vir no final)  │
└────────────────────────────────────────────────────────┘
```

### Cuidado com Move!

```rust
let user2 = User {
    email: String::from("novo@email.com"),
    ..user1
};

// ⚠️ user1.username foi MOVIDO para user2!
println!("{}", user1.username);  // ❌ ERRO!

// Mas campos Copy (bool, u64) ainda funcionam:
println!("{}", user1.active);        // ✅ OK (bool é Copy)
println!("{}", user1.sign_in_count); // ✅ OK (u64 é Copy)
```

```
ANTES do update:                DEPOIS do update:
┌─────────┐                     ┌─────────┐
│  user1  │                     │  user1  │
│  ───────┼──► "original"       │  (parcialmente inválido)
│  active │                     │  active ✅
│  count  │                     │  count ✅
└─────────┘                     │  username ❌ (movido)
                                └─────────┘

                                ┌─────────┐
                                │  user2  │
                                │  ───────┼──► "original" (agora é de user2)
                                │  ───────┼──► "novo@..."
                                └─────────┘
```

### Evitando o Move

Se você der novos valores para **todos** os campos não-Copy, o original continua válido:

```rust
let user2 = User {
    email: String::from("novo@email.com"),
    username: String::from("novo_user"),  // novo valor!
    ..user1  // só copia active e sign_in_count (são Copy)
};

println!("{}", user1.username);  // ✅ OK agora!
```

---

## Tuple Structs

Structs que parecem tuplas, mas com um nome/tipo próprio:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// Acesso por índice
println!("R: {}", black.0);
println!("X: {}", origin.0);

// Desestruturação
let Color(r, g, b) = black;
let Point(x, y, z) = origin;
```

**Por que usar?** Mesmo tendo a mesma estrutura (3 i32), `Color` e `Point` são tipos **diferentes**:

```rust
fn mover_ponto(p: Point) { ... }

mover_ponto(origin);  // ✅ OK
mover_ponto(black);   // ❌ ERRO! Color não é Point
```

---

## Unit-Like Structs

Structs sem campos:

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

Parece inútil, mas é útil para:
- Implementar traits sem dados
- Criar tipos marcadores
- Estados ou singletons

Você vai entender melhor quando estudar traits (capítulo 10).

---

## Ownership em Structs

### Por que `String` e não `&str`?

Usamos `String` nos campos para que a struct seja **dona** dos dados:

```rust
// ✅ Struct é dona dos dados
struct User {
    username: String,
    email: String,
}
```

Se tentarmos usar `&str`:

```rust
// ❌ ERRO: precisa de lifetime
struct User {
    username: &str,
    email: &str,
}
```

O compilador reclama:

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
```

### Regra Prática

| Situação | Use |
|----------|-----|
| Campo de struct | `String` (struct é dona) |
| Parâmetro de função | `&str` (só empresta) |
| Precisa de referência em struct | Lifetimes (capítulo 10) |

---

---

## Evoluindo o Código: Variáveis → Tuplas → Structs

Um exemplo prático de porque structs são melhores:

### Versão 1: Variáveis Soltas (ruim)

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height
}

let width = 30;
let height = 50;
area(width, height);
```

**Problema:** Não fica claro que `width` e `height` pertencem ao mesmo retângulo.

### Versão 2: Tuplas (melhor, mas confuso)

```rust
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1  // 0 é largura ou altura?
}

let rect = (30, 50);
area(rect);
```

**Problema:** `dimensions.0` e `dimensions.1` não têm significado claro.

### Versão 3: Structs (ideal)

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height  // claro!
}

let rect = Rectangle { width: 30, height: 50 };
area(&rect);  // passa por referência
```

**Por que referência?** Para não mover ownership. Assim `rect` continua válido depois.

---

## Debug Trait: Imprimindo Structs

Por padrão, você **não pode** imprimir structs com `println!`:

```rust
struct Rectangle { width: u32, height: u32 }

let rect = Rectangle { width: 30, height: 50 };
println!("{}", rect);    // ❌ ERRO: não implementa Display
println!("{:?}", rect);  // ❌ ERRO: não implementa Debug
```

### Solução: `#[derive(Debug)]`

```rust
#[derive(Debug)]  // adiciona isso acima da struct
struct Rectangle {
    width: u32,
    height: u32,
}

let rect = Rectangle { width: 30, height: 50 };
println!("{:?}", rect);   // ✅ Rectangle { width: 30, height: 50 }
println!("{:#?}", rect);  // ✅ formatação bonita (pretty print)
```

### Comparação de Saída

```
{:?}   → Rectangle { width: 30, height: 50 }

{:#?}  → Rectangle {
             width: 30,
             height: 50,
         }
```

Use `{:#?}` para structs grandes - fica mais legível.

---

## Macro `dbg!()`

Uma forma poderosa de debugar:

```rust
let scale = 2;
let rect = Rectangle {
    width: dbg!(30 * scale),  // imprime E retorna o valor
    height: 50,
};

dbg!(&rect);  // use & para não mover ownership
```

**Saída:**

```
[src/main.rs:4:12] 30 * scale = 60
[src/main.rs:8:5] &rect = Rectangle {
    width: 60,
    height: 50,
}
```

### `println!` vs `dbg!`

| Característica | `println!` | `dbg!` |
|----------------|------------|--------|
| Recebe | Referência | Ownership (use `&`) |
| Saída | stdout | stderr |
| Mostra arquivo/linha | Não | Sim |
| Retorna valor | Não | Sim |

**Quando usar `dbg!`?**
- Debug rápido inline
- Ver valor de expressões intermediárias
- Quando precisa do arquivo e linha

---

## Resumo Visual

```
┌────────────────────────────────────────────────────────────────┐
│  TIPOS DE STRUCTS                                              │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  1. Struct com campos nomeados (mais comum)                    │
│     struct User {                                              │
│         username: String,                                      │
│         active: bool,                                          │
│     }                                                          │
│                                                                │
│  2. Tuple struct (campos sem nome)                             │
│     struct Point(i32, i32, i32);                               │
│                                                                │
│  3. Unit struct (sem campos)                                   │
│     struct Marker;                                             │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│  SINTAXE                                                       │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  Criar:      let u = User { field: value, ... };               │
│  Acessar:    u.field                                           │
│  Modificar:  u.field = novo (u precisa ser mut)                │
│  Shorthand:  User { email, username }                          │
│  Update:     User { email: "x", ..outro }                      │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│  DEBUG                                                         │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  #[derive(Debug)]   → habilita {:?} e {:#?}                    │
│  println!("{:?}")   → debug compacto                           │
│  println!("{:#?}")  → debug bonito (pretty)                    │
│  dbg!(valor)        → debug com arquivo/linha                  │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```
