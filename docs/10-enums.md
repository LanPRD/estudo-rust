# Enums

Enums permitem definir um tipo enumerando suas variantes possíveis. Diferente de structs (que agrupam campos), enums dizem "é um OU outro".

## O Básico

```rust
// Definição
enum IpAddrKind {
    V4,
    V6,
}

// Instanciação (usa :: para acessar variantes)
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

```text
┌─────────────────────────────────────────────────────────┐
│  TypeScript                 │  Rust                     │
├─────────────────────────────┼───────────────────────────┤
│  type IpKind = "V4" | "V6"; │  enum IpAddrKind {        │
│                             │      V4,                  │
│                             │      V6,                  │
│                             │  }                        │
├─────────────────────────────┼───────────────────────────┤
│  const four: IpKind = "V4"; │  let four = IpAddrKind::V4│
└─────────────────────────────────────────────────────────┘
```

---

## Enum como Parâmetro

Ambas as variantes são do **mesmo tipo**, então uma função pode aceitar qualquer uma:

```rust
fn route(ip_kind: IpAddrKind) {
    // processa qualquer tipo de IP
}

route(IpAddrKind::V4);  // ✅
route(IpAddrKind::V6);  // ✅
```

---

## Enum com Dados Associados

A grande vantagem dos enums em Rust: cada variante pode carregar dados!

### Forma 1: Struct + Enum (funciona, mas verboso)

```rust
enum IpAddrKind { V4, V6 }

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};
```

### Forma 2: Dados direto no Enum (mais elegante)

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

Cada variante vira um "construtor" que retorna o tipo do enum.

---

## Tipos Diferentes por Variante

O poder real dos enums: **cada variante pode ter tipos diferentes**!

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),  // 4 números
    V6(String),          // uma string
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

**Isso é impossível com structs!** Um campo não pode ser "às vezes 4 números, às vezes String".

```text
┌─────────────────────────────────────────────────────────────────┐
│  TypeScript (Discriminated Union)  │  Rust (Enum)              │
├─────────────────────────────────────┼───────────────────────────┤
│  type IpAddr =                      │  enum IpAddr {            │
│    | { kind: "V4";                  │      V4(u8, u8, u8, u8),  │
│        octets: [number, number,     │      V6(String),          │
│                 number, number] }   │  }                        │
│    | { kind: "V6"; addr: string };  │                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## Enum Message (Exemplo Completo)

Um enum pode ter variantes de tipos completamente diferentes:

```rust
enum Message {
    Quit,                       // sem dados (como unit struct)
    Move { x: i32, y: i32 },    // campos nomeados (como struct)
    Write(String),              // uma String (como tuple struct)
    ChangeColor(i32, i32, i32), // três i32 (como tuple struct)
}

let m1 = Message::Quit;
let m2 = Message::Move { x: 10, y: 20 };
let m3 = Message::Write(String::from("Olá!"));
let m4 = Message::ChangeColor(255, 128, 0);
```

Se fossem structs separadas, cada uma seria um tipo diferente:

```rust
// Com structs precisaríamos de:
struct QuitMessage;
struct MoveMessage { x: i32, y: i32 }
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// E funções separadas para cada tipo 😫
fn process_quit(msg: QuitMessage) { }
fn process_move(msg: MoveMessage) { }
// etc...

// Com enum, UMA função serve para todas:
fn process(msg: Message) { }  // ✅ aceita qualquer variante
```

---

## Métodos em Enums

Assim como structs, enums podem ter métodos com `impl`:

```rust
impl Message {
    fn call(&self) {
        println!("Processando mensagem...");
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

---

## Resumo Visual: Tipos de Variantes

```text
┌────────────────────────────────────────────────────────────────┐
│  VARIANTES DE ENUM                                             │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  1. Sem dados (unit-like)                                      │
│     Quit                                                       │
│                                                                │
│  2. Tuple-style (dados posicionais)                            │
│     Write(String)                                              │
│     ChangeColor(i32, i32, i32)                                 │
│                                                                │
│  3. Struct-style (campos nomeados)                             │
│     Move { x: i32, y: i32 }                                    │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│  TODAS SÃO DO MESMO TIPO: Message                              │
└────────────────────────────────────────────────────────────────┘
```
