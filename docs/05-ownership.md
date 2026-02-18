# Ownership (Posse)

Este é **o** conceito mais importante do Rust. É o que permite que Rust seja seguro sem garbage collector.

## Por que isso existe?

No JS, você não pensa sobre memória. O garbage collector cuida disso:

```javascript
let obj = { nome: "João" };
obj = null;  // GC limpa depois (quando quiser)
```

Em Rust, **não tem GC**. A memória é liberada de forma determinística quando o "dono" sai de escopo.

---

## Stack vs Heap (Conceito Fundamental)

Antes de entender ownership, você precisa entender onde os dados vivem:

```
┌──────────────────────────────────────────────────────────────┐
│                         MEMÓRIA                              │
├─────────────────────────┬────────────────────────────────────┤
│         STACK           │              HEAP                  │
├─────────────────────────┼────────────────────────────────────┤
│ • Tamanho fixo/conhecido│ • Tamanho dinâmico/desconhecido   │
│ • Rápida                │ • Mais lenta                       │
│ • Organizada (pilha)    │ • Desorganizada                    │
│ • Limpa automaticamente │ • Precisa de gerenciamento         │
├─────────────────────────┼────────────────────────────────────┤
│ i32, bool, char, [T; N] │ String, Vec<T>, Box<T>             │
│ Referências (&T)        │ Dados que crescem/encolhem         │
└─────────────────────────┴────────────────────────────────────┘
```

### Exemplo visual: String

```rust
let s = String::from("olá");
```

```
    STACK                          HEAP
┌───────────────┐             ┌─────────────┐
│ s             │             │             │
├───────────────┤      ┌─────►│  "olá"      │
│ ptr ──────────┼──────┘      │             │
│ len: 4        │             └─────────────┘
│ capacity: 4   │
└───────────────┘

s na stack contém:
- ponteiro para os dados
- tamanho atual
- capacidade total
```

---

## As 3 Regras do Ownership

```
┌─────────────────────────────────────────────────────────────────┐
│  1. Todo valor tem UM dono (owner)                              │
│  2. Só pode ter UM dono por vez                                 │
│  3. Quando o dono sai de escopo, o valor é dropado (liberado)   │
└─────────────────────────────────────────────────────────────────┘
```

---

## Move (Transferência de Posse)

Quando você atribui um valor de heap para outra variável, o ownership **move**:

```rust
let s1 = String::from("olá");
let s2 = s1;  // ownership MOVE para s2

println!("{}", s1);  // ❌ ERRO! s1 não é mais válido
println!("{}", s2);  // ✅ OK
```

**Por que isso acontece?**

Se ambos `s1` e `s2` apontassem para o mesmo dado na heap, quando um saísse de escopo, a memória seria liberada. Depois, quando o outro saísse, tentaria liberar de novo = **double free** (bug grave).

```
ANTES do move:           DEPOIS do move:
┌────┐                   ┌────┐
│ s1 │──────┐            │ s1 │ (inválido)
└────┘      │            └────┘
            ▼
         ┌──────┐        ┌────┐
         │"olá" │        │ s2 │──────┐
         └──────┘        └────┘      │
                                     ▼
                                  ┌──────┐
                                  │"olá" │
                                  └──────┘
```

---

## Copy (Cópia Automática)

Tipos simples (que vivem na stack) são **copiados**, não movidos:

```rust
let x = 5;
let y = x;  // COPIA (não move)

println!("{}", x);  // ✅ OK
println!("{}", y);  // ✅ OK
```

### Quem tem Copy?

| Tipo | Copy? |
|------|-------|
| i32, u32, f64, etc. | Sim |
| bool, char | Sim |
| [i32; 5] (array fixo) | Sim |
| &T (referências) | Sim |
| String, Vec | **Não** |
| Structs com String | **Não** |

---

## Clone (Cópia Explícita)

Se você **realmente** quer duplicar dados da heap:

```rust
let s1 = String::from("olá");
let s2 = s1.clone();  // cópia profunda

println!("{}", s1);  // ✅ OK
println!("{}", s2);  // ✅ OK
```

```
Após clone():
┌────┐            ┌──────┐
│ s1 │───────────►│"olá" │  (heap 1)
└────┘            └──────┘

┌────┐            ┌──────┐
│ s2 │───────────►│"olá" │  (heap 2, cópia separada)
└────┘            └──────┘
```

**Cuidado:** `clone()` pode ser caro para dados grandes.

---

## Ownership em Funções

Passar um valor para uma função **também move**:

```rust
fn main() {
    let nome = String::from("Rust");
    imprime(nome);  // ownership move para a função

    println!("{}", nome);  // ❌ ERRO! nome foi movido
}

fn imprime(s: String) {
    println!("{}", s);
}  // s é dropado aqui
```

### Retornando ownership

```rust
fn main() {
    let s1 = cria_string();  // recebe ownership
    let s2 = recebe_e_devolve(s1);  // s1 move, recebe de volta em s2
}

fn cria_string() -> String {
    String::from("olá")  // move para quem chamou
}

fn recebe_e_devolve(s: String) -> String {
    s  // move de volta
}
```

Isso é verboso. Por isso existem **referências** (próximo capítulo).

---

## Resumo Visual

```
┌─────────────────────────────────────────────────────────────┐
│  OWNERSHIP EM AÇÃO                                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  let s1 = String::from("hi");                               │
│      ▲                                                      │
│      │ s1 é o dono                                          │
│      │                                                      │
│  let s2 = s1;  ← MOVE (s1 morre)                            │
│      ▲                                                      │
│      │ s2 é o novo dono                                     │
│      │                                                      │
│  }  ← escopo termina, s2 é dropado, memória liberada        │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│  Tipos Copy (i32, bool...):  copia, não move                │
│  Tipos Heap (String, Vec):   move, ou use .clone()          │
└─────────────────────────────────────────────────────────────┘
```
