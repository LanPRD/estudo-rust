# Referências e Borrowing

Mover ownership toda hora é inconveniente. **Referências** permitem usar um valor sem tomar posse dele.

## O que é uma Referência?

Uma referência é como um "empréstimo" do valor:

```rust
fn main() {
    let s = String::from("olá");

    let tamanho = calcula_tamanho(&s);  // empresta s

    println!("'{}' tem {} bytes", s, tamanho);  // s ainda é válido!
}

fn calcula_tamanho(s: &String) -> usize {  // recebe referência
    s.len()
}
```

```
Sem referência (move):        Com referência (borrow):
┌──────┐                      ┌──────┐
│ main │                      │ main │
│  s ──┼──► dados             │  s ──┼──────────► dados
└──────┘    (move)            └──────┘              ▲
     ↓                             ↓                │
┌─────────┐                   ┌─────────┐           │
│ função  │                   │ função  │           │
│  s ─────┼──► dados          │  &s ────┼───────────┘
└─────────┘    (recebeu)      └─────────┘  (só aponta)
```

---

## Referência Imutável (`&T`)

Por padrão, referências são **imutáveis** - você pode ler, mas não modificar:

```rust
fn main() {
    let s = String::from("olá");
    mostra(&s);
    println!("{}", s);  // ainda funciona
}

fn mostra(texto: &String) {
    println!("{}", texto);
    // texto.push_str("!");  // ❌ ERRO! Não pode modificar
}
```

---

## Referência Mutável (`&mut T`)

Para modificar, você precisa de uma referência mutável:

```rust
fn main() {
    let mut s = String::from("olá");  // s precisa ser mut

    adiciona_exclamacao(&mut s);

    println!("{}", s);  // "olá!"
}

fn adiciona_exclamacao(texto: &mut String) {
    texto.push_str("!");
}
```

---

## As Regras do Borrowing

```
┌─────────────────────────────────────────────────────────────────┐
│  1. Você pode ter MUITAS referências imutáveis (&T)             │
│     OU                                                          │
│  2. UMA referência mutável (&mut T)                             │
│                                                                 │
│  NUNCA ambas ao mesmo tempo!                                    │
└─────────────────────────────────────────────────────────────────┘
```

### Por que essa regra?

Evita **data races** em tempo de compilação:

```rust
let mut s = String::from("olá");

let r1 = &s;      // ✅ OK
let r2 = &s;      // ✅ OK (múltiplas imutáveis)
let r3 = &mut s;  // ❌ ERRO! Já tem refs imutáveis

println!("{}, {}", r1, r2);
```

### Múltiplas referências mutáveis

```rust
let mut s = String::from("olá");

let r1 = &mut s;
let r2 = &mut s;  // ❌ ERRO! Só pode ter uma mut por vez

println!("{}, {}", r1, r2);
```

---

## Escopo de Referências (NLL)

Referências "morrem" na última vez que são usadas, não no fim do bloco:

```rust
let mut s = String::from("olá");

let r1 = &s;
let r2 = &s;
println!("{}, {}", r1, r2);  // última vez que r1, r2 são usados
// r1 e r2 morrem aqui

let r3 = &mut s;  // ✅ OK agora!
r3.push_str("!");
println!("{}", r3);
```

Isso é chamado **Non-Lexical Lifetimes (NLL)**.

---

## Dangling References (Referência Pendurada)

Rust **não permite** retornar referência para variável local:

```rust
fn dangle() -> &String {
    let s = String::from("olá");
    &s  // ❌ ERRO! s será destruída, referência apontaria para nada
}
```

```
O que aconteceria sem proteção:
┌──────────┐
│ função   │
│   s ─────┼──► "olá" (heap)
│   &s     │      ▲
└──────────┘      │
      ↓           │
   s é dropado    │
   heap liberada  │
                  │
┌──────────┐      │
│ chamador │      │
│   ref ───┼──────┘  (aponta para memória liberada = BUG!)
└──────────┘
```

**Solução:** retorne o valor, não a referência:

```rust
fn no_dangle() -> String {
    let s = String::from("olá");
    s  // move ownership para quem chamou
}
```

---

## Comparação com JS

| Conceito | JS | Rust |
|----------|-----|------|
| Passar objeto para função | Referência sempre | Move (ou `&` para emprestar) |
| Modificar parâmetro | Sempre permitido | Precisa de `&mut` |
| Múltiplos acessos | Sem restrição | Controlado pelo borrow checker |
| Memória inválida | Possível (bugs) | Impossível (compile-time) |

---

## Resumo Visual

```
┌────────────────────────────────────────────────────────────────┐
│  REFERÊNCIAS                                                   │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  &T      → referência imutável (só leitura)                    │
│  &mut T  → referência mutável (leitura + escrita)              │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│  REGRAS                                                        │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  ┌─────────────────────────────────────────────────┐           │
│  │ Pode ter:                                       │           │
│  │   • N refs imutáveis (&T, &T, &T...)           │           │
│  │   OU                                            │           │
│  │   • 1 ref mutável (&mut T)                      │           │
│  │                                                 │           │
│  │ NUNCA misturar!                                 │           │
│  └─────────────────────────────────────────────────┘           │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│  SINTAXE                                                       │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  let s = String::from("hi");                                   │
│                                                                │
│  let r1 = &s;           // cria referência imutável            │
│  let r2 = &mut s;       // cria referência mutável (s mut)     │
│                                                                │
│  fn foo(x: &String)     // recebe ref imutável                 │
│  fn bar(x: &mut String) // recebe ref mutável                  │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```
