# Variáveis e Mutabilidade

## O Básico: `let` vs `let mut`

Em JS/TS você usa `let` para variáveis que mudam e `const` para as que não mudam.

**No Rust é o contrário:**

```rust
let x = 5;       // imutável (como const no JS)
let mut y = 5;   // mutável (como let no JS)
```

```
┌─────────────────────────────────────────────────┐
│           JS/TS             │       Rust        │
├─────────────────────────────┼───────────────────┤
│  const x = 5  (não muda)    │  let x = 5        │
│  let y = 5    (pode mudar)  │  let mut y = 5    │
└─────────────────────────────────────────────────┘
```

### Por que o padrão é imutável?

Rust assume que você **não quer** mudar valores. Isso evita bugs onde você altera algo sem querer. Se precisa mudar, você diz explicitamente com `mut`.

```rust
let x = 5;
x = 6;  // ❌ ERRO! Não pode reatribuir

let mut y = 5;
y = 6;  // ✅ OK
```

---

## Constantes (`const`)

Constantes são **sempre** imutáveis e precisam de tipo explícito:

```rust
const TRES_HORAS_EM_SEGUNDOS: u32 = 60 * 60 * 3;
```

**Diferença de `let`:**
- `const` precisa do tipo (`: u32`)
- `const` só aceita valores calculáveis em tempo de compilação
- `const` usa SCREAMING_SNAKE_CASE por convenção

---

## Shadowing (Sombreamento)

Isso é algo que **não existe no JS/TS**: você pode redeclarar uma variável com `let` no mesmo escopo.

```rust
let x = 5;
let x = x + 1;   // x agora é 6 (nova variável!)
let x = x * 2;   // x agora é 12
```

### Por que usar shadowing?

**1. Mudar o tipo da variável:**

```rust
let spaces = "   ";        // tipo: &str (string)
let spaces = spaces.len(); // tipo: usize (número)
```

Com `mut` isso **não funciona**:

```rust
let mut spaces = "   ";
spaces = spaces.len();  // ❌ ERRO! Não pode mudar tipo
```

**2. Cada `let` cria uma nova variável na memória:**

```rust
let x = 5;
println!("Endereço: {:p}", &x);  // 0x7fff5a2b

let x = x + 1;
println!("Endereço: {:p}", &x);  // 0x7fff5a3c (diferente!)
```

### Shadowing em escopos

```rust
let x = 5;

{
    let x = x * 2;  // x = 10 (apenas aqui dentro)
    println!("{x}"); // 10
}

println!("{x}");  // 5 (o x original)
```

```
┌──────────────────────────────────────┐
│  Escopo externo                      │
│  x = 5                               │
│  ┌────────────────────────────────┐  │
│  │  Escopo interno                │  │
│  │  x = 10 (sombra o x externo)   │  │
│  │  ...morre aqui                 │  │
│  └────────────────────────────────┘  │
│  x = 5 (volta a valer)               │
└──────────────────────────────────────┘
```

---

## Resumo Rápido

| Conceito | Sintaxe | Pode mudar? | Pode mudar tipo? |
|----------|---------|-------------|------------------|
| Imutável | `let x = 5` | Não | - |
| Mutável | `let mut x = 5` | Sim | Não |
| Constante | `const X: u32 = 5` | Nunca | - |
| Shadowing | `let x = ...` de novo | Cria nova | Sim |
