# Métodos

Métodos são funções associadas a uma struct. Se você vem do JS/TS, pense neles como métodos de classe.

## Sintaxe Básica

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

let rect = Rectangle { width: 30, height: 50 };
println!("{}", rect.area());  // 1500
```

```text
┌─────────────────────────────────────────────────────────────┐
│  JS/TS                          │  Rust                     │
├─────────────────────────────────┼───────────────────────────┤
│  class Rectangle {              │  struct Rectangle { ... } │
│    area() {                     │                           │
│      return this.width *        │  impl Rectangle {         │
│             this.height;        │    fn area(&self) -> u32 {│
│    }                            │      self.width *         │
│  }                              │      self.height          │
│                                 │    }                      │
│                                 │  }                        │
├─────────────────────────────────┼───────────────────────────┤
│  rect.area()                    │  rect.area()              │
│  (this é implícito)             │  (self é explícito)       │
└─────────────────────────────────────────────────────────────┘
```

---

## Os 3 Tipos de `self`

O primeiro parâmetro de um método é sempre alguma forma de `self`:

| Sintaxe     | Significado         | Quando usar                    |
| ----------- | ------------------- | ------------------------------ |
| `&self`     | Referência imutável | Só lê dados (mais comum)       |
| `&mut self` | Referência mutável  | Modifica dados                 |
| `self`      | Toma ownership      | Transforma/consome a instância |

### `&self` - Só leitura

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height  // só lê
    }
}

let rect = Rectangle { width: 30, height: 50 };
rect.area();
println!("{:?}", rect);  // ✅ rect ainda existe
```

### `&mut self` - Modificação

```rust
impl Rectangle {
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

let mut rect = Rectangle { width: 30, height: 50 };
rect.double_size();
println!("{:?}", rect);  // Rectangle { width: 60, height: 100 }
```

### `self` - Consome a instância

```rust
impl Rectangle {
    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle { width: side, height: side }
    }
}

let rect = Rectangle { width: 30, height: 50 };
let square = rect.into_square();
// println!("{:?}", rect);  // ❌ ERRO! rect foi consumido
```

**Quando usar `self` (ownership)?** Raramente. Útil quando o método transforma a instância em outra coisa e você quer impedir uso do original.

---

## Automatic Referencing

Em C/C++, você precisa de operadores diferentes:

- `object.method()` - quando é valor
- `object->method()` - quando é ponteiro

**Rust não tem `->`.** O compilador adiciona `&`, `&mut` ou `*` automaticamente:

```rust
// Essas duas linhas são equivalentes:
rect.area();
(&rect).area();
```

Isso funciona porque o método declara explicitamente o tipo de `self` que espera.

---

## Métodos com Parâmetros Extras

Métodos podem ter mais parâmetros além de `self`:

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

let rect1 = Rectangle { width: 30, height: 50 };
let rect2 = Rectangle { width: 10, height: 40 };

rect1.can_hold(&rect2)  // true
```

```text
┌──────────────────────────────────────────────────────────┐
│  rect1.can_hold(&rect2)                                  │
│    ↑       ↑      ↑                                      │
│    │       │      └── segundo parâmetro (other)          │
│    │       └── nome do método                            │
│    └── primeiro parâmetro implícito (self)               │
└──────────────────────────────────────────────────────────┘
```

---

## Método com Mesmo Nome de Campo

Você pode criar um método com o mesmo nome de um campo:

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

let rect = Rectangle { width: 30, height: 50 };

rect.width   // campo: 30 (u32)
rect.width() // método: true (bool)
```

**Uso comum:** Criar getters que fazem validação ou formatação.

```text
┌─────────────────────────────────────┐
│  rect.width   → acessa CAMPO        │
│  rect.width() → chama MÉTODO        │
│          ↑↑                         │
│     parênteses fazem a diferença    │
└─────────────────────────────────────┘
```

---

## Associated Functions (Construtores)

Funções no bloco `impl` que **não** recebem `self` são chamadas de _associated functions_. São como métodos estáticos.

```rust
impl Rectangle {
    // Associated function - não tem self
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

// Chamadas com :: (não com .)
let rect = Rectangle::new(30, 50);
let square = Rectangle::square(25);
```

```text
┌──────────────────────────────────────────────────────────┐
│  MÉTODO vs ASSOCIATED FUNCTION                           │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Método (tem self):                                      │
│    rect.area()                                           │
│        ↑                                                 │
│      ponto                                               │
│                                                          │
│  Associated function (sem self):                         │
│    Rectangle::new(30, 50)                                │
│              ↑↑                                          │
│         dois pontos                                      │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### `Self` com S maiúsculo

Dentro do bloco `impl`, `Self` é um alias para o tipo:

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {  // Self = Rectangle
        Self { width, height }                  // Self = Rectangle
    }
}
```

**Você já conhece associated functions:**

```rust
String::from("texto")
Vec::new()
```

---

## Múltiplos Blocos `impl`

Você pode ter vários blocos `impl` para a mesma struct:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
```

**Quando usar?**

- Implementar traits diferentes (capítulo 10)
- Separar métodos com generics diferentes
- Organização de código

---

## Resumo Visual

```text
┌────────────────────────────────────────────────────────────────┐
│  ANATOMIA DE UM BLOCO impl                                     │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  impl Rectangle {                                              │
│      │                                                         │
│      └── tipo associado                                        │
│                                                                │
│      // Método (tem self)                                      │
│      fn area(&self) -> u32 { ... }                             │
│              ↑                                                 │
│              └── primeiro parâmetro sempre é self              │
│                                                                │
│      // Associated function (sem self)                         │
│      fn new(w: u32, h: u32) -> Self { ... }                    │
│                                  ↑                             │
│                                  └── Self = Rectangle          │
│  }                                                             │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│  CHAMADAS                                                      │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  rect.area()           // método: usa ponto                    │
│  Rectangle::new(1, 2)  // associated fn: usa ::                │
│                                                                │
├────────────────────────────────────────────────────────────────┤
│  TIPOS DE self                                                 │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  &self      │ empréstimo imutável │ só lê                      │
│  &mut self  │ empréstimo mutável  │ modifica                   │
│  self       │ toma ownership      │ consome                    │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

---

## Comparação Final: Função vs Método

```rust
// FUNÇÃO: area recebe Rectangle
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
area(&rect);  // chamada

// MÉTODO: area pertence a Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
rect.area();  // chamada
```

**Por que preferir métodos?**

1. Organização: tudo sobre Rectangle fica junto
2. Sintaxe mais limpa: `rect.area()` vs `area(&rect)`
3. Autocomplete: IDEs mostram métodos disponíveis
