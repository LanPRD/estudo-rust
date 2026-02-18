# Tipos de Dados

No JS você não pensa muito em tipos. No Rust, **tudo tem um tipo** e o compilador precisa saber qual é.

## Tipos Numéricos Inteiros

Diferente do JS (que tem só `number`), Rust tem vários tipos de inteiros:

```
┌──────────────────────────────────────────────────────────────────┐
│  INTEIROS COM SINAL (podem ser negativos)                        │
├────────┬───────┬─────────────────────────────────────────────────┤
│  i8    │ 1 B   │  -128 a 127                                     │
│  i16   │ 2 B   │  -32.768 a 32.767                               │
│  i32   │ 4 B   │  -2 bilhões a 2 bilhões (padrão)                │
│  i64   │ 8 B   │  número gigante                                 │
│  i128  │ 16 B  │  número absurdo                                 │
├────────────────────────────────────────────────────────────────────┤
│  INTEIROS SEM SINAL (apenas positivos)                           │
├────────┬───────┬─────────────────────────────────────────────────┤
│  u8    │ 1 B   │  0 a 255                                        │
│  u16   │ 2 B   │  0 a 65.535                                     │
│  u32   │ 4 B   │  0 a 4 bilhões                                  │
│  u64   │ 8 B   │  0 a 18 quintilhões                             │
│  u128  │ 16 B  │  0 a número absurdo                             │
└──────────────────────────────────────────────────────────────────┘
```

### Qual usar?

- **i32** é o padrão. Na dúvida, use esse.
- **u8** para bytes (0-255)
- **usize** para índices de arrays (ajusta ao sistema 32/64 bits)

```rust
let x = 42;        // Rust infere i32
let y: u8 = 255;   // Tipo explícito
```

---

## Tipos de Ponto Flutuante

```rust
let x = 2.0;       // f64 (padrão, mais preciso)
let y: f32 = 3.0;  // f32 (menos preciso, mais rápido)
```

| Tipo | Precisão |
|------|----------|
| f32  | ~7 dígitos |
| f64  | ~15 dígitos |

---

## Booleanos

```rust
let t = true;
let f: bool = false;
```

**Importante:** Rust não converte números para bool automaticamente:

```rust
// JS: if (1) { ... }     ✅ funciona
// Rust: if 1 { ... }     ❌ ERRO!

if number != 0 { ... }    // ✅ precisa comparar explicitamente
```

---

## Caracteres (char)

```rust
let c = 'z';
let emoji = '😻';
```

- Usa **aspas simples** (diferente de strings)
- Cada `char` ocupa 4 bytes (Unicode completo)

---

## Tuplas

Agrupam valores de tipos diferentes com tamanho fixo:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Desestruturação (igual JS!)
let (x, y, z) = tup;

// Acesso por índice (com ponto, não colchetes)
let primeiro = tup.0;  // 500
let segundo = tup.1;   // 6.4
```

```
┌─────────────────────────────────────────────┐
│  Tupla: (500, 6.4, 1)                       │
│         ↓    ↓    ↓                         │
│  Índice: .0  .1   .2                        │
│  Tipo:  i32  f64  u8                        │
└─────────────────────────────────────────────┘
```

---

## Arrays

Tamanho fixo, todos elementos do mesmo tipo:

```rust
let a = [1, 2, 3, 4, 5];           // [i32; 5]
let meses = ["Jan", "Fev", "Mar"]; // [&str; 3]

// Tipo explícito: [tipo; tamanho]
let b: [i32; 5] = [1, 2, 3, 4, 5];

// Repetição: [valor; quantidade]
let zeros = [0; 5];  // [0, 0, 0, 0, 0]
```

### Acesso

```rust
let primeiro = a[0];  // 1
let segundo = a[1];   // 2
```

### Erro em tempo de execução

Diferente do JS (que retorna `undefined`), Rust **crasha** se você acessar índice inválido:

```rust
let a = [1, 2, 3];
let x = a[10];  // 💥 panic! (erro em tempo de execução)
```

```
┌───────────────────────────────────────────────┐
│  JS:   arr[100] → undefined (silencioso)      │
│  Rust: arr[100] → PANIC! (programa para)      │
└───────────────────────────────────────────────┘
```

Isso é **intencional**: Rust prefere crashar do que deixar você ler memória inválida.

---

## Resumo: JS vs Rust

| JS | Rust |
|----|------|
| `number` (tudo) | `i32`, `u32`, `f64`, etc. |
| `true/false` | `true/false` (só bool em condições) |
| `'a'` ou `"a"` | `'a'` (char) vs `"a"` (string) |
| `[1, "a"]` (array misto) | Não permitido |
| `arr[100]` → `undefined` | `arr[100]` → panic |
