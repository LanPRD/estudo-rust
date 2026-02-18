# Controle de Fluxo

## if / else

Sintaxe similar ao JS, mas **sem parênteses** obrigatórios:

```rust
let number = 7;

if number < 5 {
    println!("menor que 5");
} else {
    println!("maior ou igual a 5");
}
```

### Condição DEVE ser bool

```rust
// JS: if (1) { ... }      ✅ funciona (truthy)
// Rust: if 1 { ... }      ❌ ERRO!

if number != 0 { ... }     // ✅ precisa ser bool explícito
```

### else if

```rust
if number % 4 == 0 {
    println!("Divisível por 4");
} else if number % 3 == 0 {
    println!("Divisível por 3");
} else {
    println!("Outro");
}
```

---

## if como Expressão

Diferente do JS, `if` em Rust **retorna valor**:

```rust
let resultado = if condition { 10 } else { 20 };
```

É tipo o ternário do JS, mas mais legível:

```
┌─────────────────────────────────────────────────┐
│  JS:   const x = condition ? 10 : 20;           │
│  Rust: let x = if condition { 10 } else { 20 }; │
└─────────────────────────────────────────────────┘
```

**Importante:** Os dois braços devem retornar o **mesmo tipo**:

```rust
let x = if true { 5 } else { "texto" };  // ❌ ERRO!
```

---

## loop (loop infinito)

```rust
loop {
    println!("Para sempre...");
    break;  // sai do loop
}
```

### loop que retorna valor

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // retorna 20
    }
};

println!("{result}");  // 20
```

Isso é útil para retry patterns ou busca com resultado.

---

## Labels (Rótulos) para Loops Aninhados

Quando você tem loops dentro de loops:

```rust
'outer: loop {
    println!("Loop externo");

    loop {
        println!("Loop interno");
        break;        // sai do loop interno
        break 'outer; // sai do loop EXTERNO
    }
}
```

```
┌─────────────────────────────────────────────────┐
│  'nome: loop {                                  │
│      loop {                                     │
│          break 'nome;  // sai do loop rotulado  │
│      }                                          │
│  }                                              │
└─────────────────────────────────────────────────┘
```

O label começa com `'` (apóstrofo).

---

## while

```rust
let mut number = 3;

while number != 0 {
    println!("{number}!");
    number -= 1;
}

println!("LIFTOFF!");
```

Saída:
```
3!
2!
1!
LIFTOFF!
```

---

## for (iteração)

### Iterando arrays

```rust
let array = [10, 20, 30, 40, 50];

for element in array {
    println!("{element}");
}
```

**Não use while para iterar arrays!** É mais lento e pode causar bugs:

```rust
// ❌ Evite isso
let mut i = 0;
while i < array.len() {
    println!("{}", array[i]);
    i += 1;
}

// ✅ Use isso
for element in array {
    println!("{element}");
}
```

### Ranges

```rust
// 1, 2, 3 (não inclui o 4)
for i in 1..4 {
    println!("{i}");
}

// 1, 2, 3, 4 (inclui o 4)
for i in 1..=4 {
    println!("{i}");
}
```

### Contagem regressiva

```rust
for number in (1..4).rev() {
    println!("{number}!");
}
println!("LIFTOFF!");
```

Saída:
```
3!
2!
1!
LIFTOFF!
```

---

## Comparação com JS

| JS | Rust |
|----|------|
| `for (let i = 0; i < 5; i++)` | `for i in 0..5` |
| `for (const x of arr)` | `for x in arr` |
| `while (cond)` | `while cond` |
| `do { } while()` | `loop { ... if cond { break; } }` |
| `condition ? a : b` | `if condition { a } else { b }` |

---

## Resumo Visual

```
┌─────────────────────────────────────────────────────────┐
│  LOOPS                                                  │
├─────────────────────────────────────────────────────────┤
│  loop { }           → infinito (use break para sair)    │
│  while cond { }     → enquanto condição for true        │
│  for x in iter { }  → para cada item do iterador        │
├─────────────────────────────────────────────────────────┤
│  RANGES                                                 │
├─────────────────────────────────────────────────────────┤
│  1..5   → 1, 2, 3, 4       (não inclui 5)               │
│  1..=5  → 1, 2, 3, 4, 5    (inclui 5)                   │
│  (1..5).rev() → 4, 3, 2, 1 (reverso)                    │
├─────────────────────────────────────────────────────────┤
│  CONTROLE                                               │
├─────────────────────────────────────────────────────────┤
│  break         → sai do loop atual                      │
│  break 'label  → sai do loop rotulado                   │
│  break valor   → sai e retorna valor                    │
│  continue      → pula para próxima iteração             │
└─────────────────────────────────────────────────────────┘
```
