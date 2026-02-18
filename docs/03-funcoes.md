# Funções

## Sintaxe Básica

```rust
fn nome_da_funcao() {
    println!("Olá!");
}
```

**Convenção:** Rust usa `snake_case` para nomes de funções (não `camelCase` como JS).

---

## Parâmetros

Diferente do JS, você **precisa** declarar o tipo dos parâmetros:

```rust
fn imprimir_valor(x: i32) {
    println!("Valor: {x}");
}

fn imprimir_medida(valor: i32, unidade: char) {
    println!("Medida: {valor}{unidade}");
}
```

```
┌─────────────────────────────────────────────────┐
│  JS:   function foo(x, y) { ... }               │
│  Rust: fn foo(x: i32, y: char) { ... }          │
│                 ↑       ↑                       │
│           tipo obrigatório                      │
└─────────────────────────────────────────────────┘
```

---

## Retorno de Valores

### Sintaxe com `->`:

```rust
fn retorna_cinco() -> i32 {
    5  // sem ponto e vírgula = retorna esse valor
}

fn soma_um(x: i32) -> i32 {
    x + 1  // última expressão é o retorno
}
```

### A pegadinha do ponto e vírgula

```rust
fn cinco() -> i32 {
    5      // ✅ retorna 5
}

fn erro() -> i32 {
    5;     // ❌ ERRO! O ; transforma em statement, retorna ()
}
```

```
┌─────────────────────────────────────────────────┐
│  5      → expressão, retorna o valor 5          │
│  5;     → statement, retorna () (unit/vazio)    │
└─────────────────────────────────────────────────┘
```

---

## Statements vs Expressions

Isso é importante em Rust e não existe no JS dessa forma:

| Tipo | Retorna valor? | Exemplo |
|------|----------------|---------|
| **Statement** | Não | `let x = 5;` |
| **Expression** | Sim | `5`, `x + 1`, `{ ... }` |

### Blocos como expressões

```rust
let resultado = {
    let x = 2;
    x * 3  // sem ; = retorna 6
};

println!("{resultado}");  // 6
```

**Comparando com JS:**

```javascript
// JS: precisa de IIFE ou arrow
const resultado = (() => {
    const x = 2;
    return x * 3;
})();
```

```rust
// Rust: bloco diretamente
let resultado = {
    let x = 2;
    x * 3
};
```

---

## Função sem Retorno Explícito

Funções que não retornam nada retornam `()` (unit type):

```rust
fn diz_oi() {
    println!("Oi!");
}

// É o mesmo que:
fn diz_oi() -> () {
    println!("Oi!");
}
```

---

## Usando `return` Explícito

Você pode usar `return` para sair cedo da função:

```rust
fn maior_que_dez(x: i32) -> bool {
    if x > 10 {
        return true;  // sai aqui
    }
    false  // retorno implícito
}
```

Mas o estilo idiomático em Rust é usar expressões:

```rust
fn maior_que_dez(x: i32) -> bool {
    x > 10  // mais limpo
}
```

---

## Resumo

```
┌────────────────────────────────────────────────────────────┐
│  fn nome(param: Tipo) -> TipoRetorno {                     │
│      corpo                                                 │
│  }                                                         │
├────────────────────────────────────────────────────────────┤
│  • Parâmetros precisam de tipo                             │
│  • Retorno declarado com ->                                │
│  • Última expressão (sem ;) é o retorno                    │
│  • Blocos {} também são expressões                         │
└────────────────────────────────────────────────────────────┘
```
