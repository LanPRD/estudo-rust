# if let e let...else

Controle de fluxo conciso para quando você só quer tratar **um** padrão de um enum.

## O Problema: Match Verboso

Quando você só quer agir em um caso específico:

```rust
let config_max = Some(3u8);

// Com match (verboso)
match config_max {
    Some(max) => println!("Máximo configurado: {max}"),
    _ => (),  // não faz nada, mas é obrigatório
}
```

Esse `_ => ()` é boilerplate irritante.

---

## if let

Mesmo comportamento, menos código:

```rust
let config_max = Some(3u8);

if let Some(max) = config_max {
    println!("Máximo configurado: {max}");
}
```

A sintaxe é: `if let PADRÃO = EXPRESSÃO { ... }`

```text
┌─────────────────────────────────────────────────────────────────┐
│  match vs if let                                                │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  match valor {              │  if let Padrão = valor {          │
│      Padrão => codigo,      │      codigo                       │
│      _ => (),               │  }                                │
│  }                          │                                   │
│                             │                                   │
│  Quando usar match:         │  Quando usar if let:              │
│  - Precisa tratar TODOS     │  - Só quer tratar UM caso         │
│    os casos                 │  - Outros casos: ignorar          │
│  - Quer checagem exaustiva  │  - Quer código mais conciso       │
│                             │                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Funciona com qualquer Enum

```rust
enum Cor {
    Rgb(u8, u8, u8),
    Hex(String),
}

let cor = Cor::Rgb(255, 128, 0);

if let Cor::Rgb(r, g, b) = cor {
    println!("RGB: ({r}, {g}, {b})");
}
```

---

## if let com else

Equivalente a `match` com dois braços:

```rust
let mut count = 0;

// Com match
match coin {
    Coin::Quarter(state) => println!("Quarter de {state:?}!"),
    _ => count += 1,
}

// Com if let + else (mesmo comportamento)
if let Coin::Quarter(state) = coin {
    println!("Quarter de {state:?}!");
} else {
    count += 1;
}
```

---

## Trade-off

⚠️ `if let` **não tem checagem exaustiva**!

Se você esquecer um caso importante, o compilador não avisa. Use `match` quando precisar garantir que tratou todos os casos.

---

## let...else: Caminho Feliz

Quando você quer extrair um valor **ou sair da função**:

### Problema com if let

```rust
fn describe_quarter(coin: Coin) -> Option<String> {
    // Código útil fica aninhado dentro do if
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} é bem antigo!"))
        } else {
            Some(format!("{state:?} é relativamente novo."))
        }
    } else {
        None
    }
}
```

### Solução com let...else

```rust
fn describe_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    // Caminho feliz: state está disponível no nível principal
    if state.existed_in(1900) {
        Some(format!("{state:?} é bem antigo!"))
    } else {
        Some(format!("{state:?} é relativamente novo."))
    }
}
```

```text
┌─────────────────────────────────────────────────────────────────┐
│  let...else                                                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  let PADRÃO = expressão else {                                  │
│      // DEVE sair do fluxo: return, break, continue ou panic    │
│  };                                                             │
│                                                                 │
│  // variáveis do padrão disponíveis aqui (caminho feliz)        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Múltiplas Validações

`let...else` brilha com várias checagens em sequência:

```rust
fn process_data(input: Option<String>) -> Result<i32, &'static str> {
    let Some(text) = input else {
        return Err("Input vazio");
    };

    let Ok(number) = text.parse::<i32>() else {
        return Err("Não é um número");
    };

    // Caminho feliz: text e number disponíveis
    Ok(number * 2)
}
```

Comparação com JS/TS (early return / guard clauses):

```javascript
function processData(input) {
    if (!input) return { error: "Input vazio" };

    const number = parseInt(input);
    if (isNaN(number)) return { error: "Não é um número" };

    return { value: number * 2 };
}
```

---

## Quando Usar Cada Um

| Situação                          | Use               |
| --------------------------------- | ----------------- |
| Tratar TODOS os casos             | `match`           |
| Tratar UM caso, ignorar resto     | `if let`          |
| Tratar UM caso COM else           | `if let ... else` |
| Extrair ou sair da função         | `let ... else`    |
| Múltiplas validações em sequência | `let ... else`    |

---

## Resumo Visual

```text
┌────────────────────────────────────────────────────────────────┐
│  FORMAS DE EXTRAIR VALORES DE ENUMS                            │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  1. match (exaustivo)                                          │
│     match valor {                                              │
│         Some(x) => usa_x,                                      │
│         None => trata_none,                                    │
│     }                                                          │
│                                                                │
│  2. if let (um caso)                                           │
│     if let Some(x) = valor {                                   │
│         usa_x                                                  │
│     }                                                          │
│                                                                │
│  3. if let + else (dois casos)                                 │
│     if let Some(x) = valor {                                   │
│         usa_x                                                  │
│     } else {                                                   │
│         trata_none                                             │
│     }                                                          │
│                                                                │
│  4. let...else (extrai ou sai)                                 │
│     let Some(x) = valor else {                                 │
│         return / break / continue                              │
│     };                                                         │
│     usa_x  // caminho feliz                                    │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```
