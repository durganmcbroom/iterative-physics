# Math Expression Parser & Lexer

This module implements a custom, hand-written recursive descent parser and lexer designed to process mathematical expressions into an Abstract Syntax Tree (AST). It supports standard arithmetic, variables, function calls, and implicit multiplication.

## 1. The Lexer (`Lexer<'a>`)

The lexer (tokenizer) transforms raw string input into a stream of atomic units called `Token`s. It utilizes a `Peekable<Chars>` iterator to traverse the input string character by character, grouping them into meaningful symbols.

### Tokenization Logic

The `exec` method identifies tokens based on the current character:

* **Whitespace:** Skipped automatically.
* **Numbers:** Consecutive digits (and dots) are captured and parsed into `f64`.
* **Identifiers:** Consecutive alphabetic characters (and underscores) are captured as text strings. These become either `Variable`s or `Function` names depending on context.
* **Operators & Symbols:** Single characters are mapped to static tokens:
    * Math: `+`, `-`, `*`, `/`, `^`
    * Structure: `(`, `)`, `,`, `=`

### Token Types

| **Token Variant** | **Description** | **Example** |
| :--- | :--- | :--- |
| `Op(Operation)` | Arithmetic operators | `+`, `*`, `^` |
| `Number(f64)` | Floating point values | `3.14`, `42.0` |
| `Text(String)` | Variables or Function names | `x`, `sin`, `width` |
| `OpenParen` / `CloseParen` | Grouping symbols | `(`, `)` |
| `Comma` | Function argument separator | `,` |
| `Equals` | Comparison operator | `=` |

## 2. The Parser

The parsing logic converts the stream of tokens into a hierarchical `Node` structure (the AST). It employs a **Recursive Descent** strategy, where each function corresponds to a specific level of operator precedence.

### Operator Precedence (Low to High)

The functions call each other in a specific order to ensure operations bind correctly.

1. **Expression (`expression`)**:
    * Handles the lowest precedence operations.
    * Specifically handles **Comparisons** (e.g., `x = y`).

2. **Addition (`addition`)**:
    * Handles **Addition** (`+`) and **Subtraction** (`-`).
    * Left-associative.

3. **Multiplication (`multiplication`)**:
    * Handles **Multiplication** (`*`) and **Division** (`/`).
    * **Implicit Multiplication:** The parser detects juxtaposed terms (e.g., `2x` or `3(y+2)`) and treats them as multiplication without requiring an explicit `*` token.

4. **Signed (`signed`)**:
    * Handles **Unary Negation** (e.g., `-5`).
    * If a minus sign is detected, it treats it as `0 - x`.

5. **Exponentiation (`exponentiation`)**:
    * Handles **Exponents** (`^`).

6. **Atom (`atom`)**:
    * The base unit of the expression.
    * Handles **Numbers**, **Parenthesized expressions**, **Variables**, and **Function Calls**.

### AST Structure (`Node`)

The result of the parsing process is a tree of `Node` enums:

```rust
pub enum Node {
    Arithmetic {
        operation: Operation,
        left: Box<Node>,
        right: Box<Node>,
    },
    Comparison {
        left: Box<Node>,
        right: Box<Node>,
    },
    Function {
        name: String,
        args: Vec<Box<Node>>,
    },
    Number(f64),
    Variable(String),
}