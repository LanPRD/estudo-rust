# Slices

Slices são **referências para uma parte** de uma coleção (como strings ou arrays).

## String Slices (`&str`)

Uma string slice é uma referência para parte de uma `String`:

```rust
let s = String::from("hello world");

let hello = &s[0..5];   // "hello"
let world = &s[6..11];  // "world"
```

```
    s = "hello world"
         ↓    ↓
         0    6

    &s[0..5]  → "hello"
    &s[6..11] → "world"

┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐
│ h │ e │ l │ l │ o │   │ w │ o │ r │ l │ d │
└───┴───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘
  0   1   2   3   4   5   6   7   8   9   10

  └─────────────┘       └─────────────────┘
     &s[0..5]              &s[6..11]
```

### Atalhos de Range

```rust
let s = String::from("hello");

let slice = &s[0..2];   // "he"
let slice = &s[..2];    // "he" (mesmo, omite 0)

let slice = &s[3..5];   // "lo"
let slice = &s[3..];    // "lo" (mesmo, vai até o fim)

let slice = &s[0..5];   // "hello"
let slice = &s[..];     // "hello" (string inteira)
```

---

## String vs &str

Essa é uma confusão comum. Vamos esclarecer:

```
┌─────────────────────────────────────────────────────────────────┐
│  String                         │  &str                         │
├─────────────────────────────────┼───────────────────────────────┤
│  Dona dos dados                 │  Referência (empresta)        │
│  Vive na heap                   │  Aponta para dados            │
│  Pode crescer/mudar             │  Tamanho fixo                 │
│  Tipo: String                   │  Tipo: &str                   │
├─────────────────────────────────┼───────────────────────────────┤
│  let s = String::from("hi");    │  let s: &str = "hi";          │
│  let s = "hi".to_string();      │  let s = &outra_string[..];   │
└─────────────────────────────────────────────────────────────────┘
```

### Onde vivem os dados?

```rust
// &str literal - dados no BINÁRIO (estático, imutável)
let literal: &str = "olá";

// String - dados na HEAP
let heap: String = String::from("olá");

// &str de String - aponta para HEAP
let slice: &str = &heap[..];
```

```
┌────────────────┐
│    BINÁRIO     │  ← literais "olá" vivem aqui
├────────────────┤
│     STACK      │  ← variáveis (ponteiros, tamanhos)
├────────────────┤
│      HEAP      │  ← String::from("olá") vive aqui
└────────────────┘
```

---

## Por que Funções Devem Aceitar `&str`

**Sempre** prefira `&str` em parâmetros de função:

```rust
// ✅ BOM - aceita String E &str
fn imprime(s: &str) {
    println!("{}", s);
}

// ❌ RUIM - só aceita String
fn imprime(s: &String) {
    println!("{}", s);
}
```

```rust
let heap = String::from("hello");
let literal = "world";

imprime(&heap);    // ✅ String → &str (coerção automática)
imprime(literal);  // ✅ já é &str
```

---

## O Problema: Referência Bloqueia Mutação

Se você tem uma `&str` apontando para uma `String`, não pode modificar a `String`:

```rust
let mut s = String::from("hello");

let slice = &s[..];       // cria referência imutável
s.push_str(" world");     // ❌ ERRO! Não pode mutar enquanto slice existe

println!("{}", slice);
```

### Solução 1: Escopo

```rust
let mut s = String::from("hello");

{
    let slice = &s[..];
    println!("{}", slice);
}  // slice morre aqui

s.push_str(" world");  // ✅ agora pode
```

### Solução 2: Clone

```rust
let mut s = String::from("hello");

let copia = s.clone();     // cópia independente
s.push_str(" world");      // ✅ s é modificável
println!("{}", copia);     // copia não foi afetada
```

---

## Slices de Arrays

Funciona igual para arrays:

```rust
let arr = [1, 2, 3, 4, 5];

let slice = &arr[1..3];  // [2, 3]

for x in slice {
    println!("{}", x);
}
```

O tipo é `&[i32]` (slice de i32).

---

## `&mut str` - Slice Mutável

Existe, mas é limitada:

```rust
let mut s = String::from("hello");
let slice: &mut str = &mut s[..];

// Você pode chamar métodos que modificam in-place
// Mas NÃO pode mudar o tamanho (adicionar/remover caracteres)
```

Na prática, `&mut String` é mais útil.

---

## Resumo Visual

```
┌────────────────────────────────────────────────────────────────┐
│  TIPOS DE STRING                                               │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  String                                                        │
│  ├── Dona dos dados                                            │
│  ├── Pode crescer: s.push_str("...")                          │
│  └── Criação: String::from("...") ou "...".to_string()        │
│                                                                │
│  &str                                                          │
│  ├── Referência/slice                                          │
│  ├── Tamanho fixo                                              │
│  └── Criação: "literal" ou &string[..]                        │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│  CONVERSÕES                                                    │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  String → &str   :  &s  ou  &s[..]  ou  s.as_str()            │
│  &str → String   :  s.to_string()  ou  String::from(s)        │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│  RANGES                                                        │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  &s[0..5]  → do índice 0 até 5 (não inclui 5)                 │
│  &s[..5]   → do início até 5                                   │
│  &s[3..]   → do índice 3 até o fim                            │
│  &s[..]    → string inteira                                    │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

## Dica Final

Pense assim:
- **String** = você é dono de uma caixa com texto
- **&str** = você está olhando para o texto de alguém (ou do binário)

```
┌─────────────────┐
│  Em funções:    │
│  Use &str       │  ← aceita tudo
└─────────────────┘

┌─────────────────┐
│  Em structs:    │
│  Use String     │  ← precisa ser dono
└─────────────────┘
```
