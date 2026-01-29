# Equation Solver & Evaluator

This module acts as the "runtime" for the parsed AST. It manages the environment, handles variable scope, and implements a numerical solver to resolve unknown variables dynamically using Newton's Method.

## 1. The Environment

The `Environment` struct serves as the container for all mathematical context. It is constructed from a list of string expressions which are parsed and categorized.

```rust
pub struct Environment {
    equations: Vec<Equation>,            // Relations like "F = m * a"
    functions: HashMap<String, Function>, // Callables like "sin(x)" or "f(x) = x^2"
    constants: HashMap<String, f64>,      // Fixed values like "pi" or "e"
}
```

### Setup & Analysis (`build`)

When building an environment:

1. **Parsing:** All input strings are parsed into ASTs.
2. **Classification:**
    * **Function Definitions:** Expressions like `f(x) = x^2` are detected. The logic is extracted and stored in `functions` for later use.
    * **Equations:** Expressions like `a^2 + b^2 = c^2` are analyzed for dependencies (which variables appear in them) and stored in `equations`.
3. **Dependency Analysis:** Each equation is tagged with a `HashSet<String>` containing every variable it uses. This allows the solver to quickly find relevant equations when looking for a missing variable.

## 2. Evaluation Logic (`evaluate`)

The core function `evaluate` recursively traverses the AST to compute a final `f64` result. It passes around a `Frame` which manages scope, recursion depth, and memoization.

### Variable Resolution Strategy

When the evaluator encounters a `Node::Variable(name)`, it follows a strict hierarchy to resolve the value:

1. **Local Scope:** Checks function arguments (e.g., if inside `f(x)`, what is `x`?).
2. **Memoization:** Checks if this variable has already been solved and cached in the current session.
3. **Constants:** Checks for universal constants (e.g., `pi`).
4. **Equation Solver (The Magic):**
    * If the variable is still unknown, the engine searches the `equations` list for any equation that *contains* this variable.
    * If found, it triggers the **Root Finding** algorithm to solve that equation for the missing variable.

## 3. Root Finding (Newton's Method)

If a variable `x` is unknown, but exists in an equation like `y = x + 10` (and we know `y`), the engine solves for `x`.

### The Algorithm (`find_root`)

The solver uses **Newton's Method** to numerically estimate the value.

1. **Transformation:** It converts an equation `Left = Right` into a root problem $f(x) = 0$ by rearranging it to `Left - Right = 0`.
2. **Numerical Differentiation:** Since we don't have analytical derivatives, the slope (derivative) is approximated using the **Finite Difference Method**:

   $$
   f'(x) \approx \frac{f(x + \epsilon) - f(x)}{\epsilon}
   $$

3. **Iteration:** It updates the guess repeatedly until convergence:

   $$
   x_{n+1} = x_n - \frac{f(x_n)}{f'(x_n)}
   $$

### Cycle Detection (`Frame`)

To prevent infinite loops (e.g., Equation A needs B, B needs A), the `Frame` struct tracks the recursion stack.

* Each equation has a unique `id`.
* Before solving an equation, its ID is pushed to `frame.stack`.
* If the solver encounters an ID that is already in the stack, it aborts that path (`VariableResolution::Ignore`), preventing stack overflows.

## 4. Functions & Built-ins

The system supports two types of functions:

| Type | Description |
| :--- | :--- |
| **`Mathematical`** | User-defined functions created at runtime (e.g., `f(x) = x^2`). These are evaluated by traversing their stored AST node. |
| **`Baked`** | Native Rust closures for performance-critical standard library operations. |

### Built-in Library

The `builtin` module provides standard mathematical functions mapped to Rust's `f64` methods:

* **Trigonometry:** `sin`, `cos`, `tan`, `asin`, `acos`, `atan`
* **Logarithms:** `ln` (base $e$), `log` (base 10), `log2` (base 2)
* **Roots:** `sqrt`, `nrt` (nth root)

## 5. Usage Flow

1. **Initialize:** Create an `Environment` with a list of string equations.
2. **Context:** Provide "known" values (overrides) via a HashMap.
3. **Evaluate:** Ask the environment to solve for a specific target variable.