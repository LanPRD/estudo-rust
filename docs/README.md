# Rust - Anotações de Estudo

Documentação pessoal baseada no livro oficial do Rust, com foco em quem vem do JavaScript/TypeScript.

## Índice

| #   | Tema                                                         | Descrição                                            |
| --- | ------------------------------------------------------------ | ---------------------------------------------------- |
| 01  | [Variáveis e Mutabilidade](./01-variaveis-e-mutabilidade.md) | `let`, `mut`, `const`, shadowing                     |
| 02  | [Tipos de Dados](./02-tipos-de-dados.md)                     | Inteiros, floats, bool, char, tuplas, arrays         |
| 03  | [Funções](./03-funcoes.md)                                   | Parâmetros, retorno, statements vs expressions       |
| 04  | [Controle de Fluxo](./04-controle-de-fluxo.md)               | if/else, loop, while, for, ranges                    |
| 05  | [Ownership](./05-ownership.md)                               | Stack vs Heap, move, copy, clone                     |
| 06  | [Referências e Borrowing](./06-referencias-e-borrowing.md)   | `&T`, `&mut T`, regras do borrow checker             |
| 07  | [Slices](./07-slices.md)                                     | `&str` vs `String`, slices de arrays                 |
| 08  | [Structs](./08-structs.md)                                   | Definição, tuple structs, `#[derive(Debug)]`, `dbg!` |
| 09  | [Métodos](./09-metodos.md)                                   | `impl`, `&self`, associated functions, construtores  |
| 10  | [Enums](./10-enums.md)                                       | Variantes, dados associados                          |
| 11  | [Option enum](./11-enum-option.md)                           | `Option<T>`, sem null                                |
| 12  | [if let](./12-if-let.md)                                     | `if let`, `let...else`, controle conciso             |

## Mapa Mental: Conceitos Conectados

```text
                    ┌─────────────┐
                    │  Variáveis  │
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
         ┌────────┐   ┌────────┐   ┌─────────┐
         │  Tipos │   │ Funções│   │ Controle│
         └────┬───┘   └────────┘   └─────────┘
              │
              ▼
    ┌─────────────────┐
    │    Ownership    │ ◄── Conceito central do Rust
    └────────┬────────┘
             │
    ┌────────┴────────┐
    ▼                 ▼
┌───────────┐    ┌─────────┐
│Referências│    │ Slices  │
│& Borrowing│    │ &str    │
└───────────┘    └─────────┘
                     │
                     ▼
               ┌─────────┐
               │ Structs │
               └────┬────┘
                    │
                    ▼
               ┌─────────┐
               │ Métodos │
               └────┬────┘
                    │
                    ▼
               ┌─────────┐
               │  Enums  │ ◄── Option<T> substitui null!
               └────┬────┘
                    │
                    ▼
               ┌─────────┐
               │ if let  │ ◄── Controle de fluxo conciso
               └─────────┘
```

## Legenda dos Símbolos

- ✅ Correto / Funciona
- ❌ Erro / Não funciona
- ⚠️ Atenção / Cuidado

## Dica de Leitura

Se você vem do JS/TS, os capítulos 1-4 serão familiares. A partir do capítulo 5 (Ownership) é onde Rust se diferencia radicalmente. Dedique mais tempo a esses.
