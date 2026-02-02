# 🌌 Iterative Physics Simulation

This project implements a custom physics engine built from scratch, featuring a recursive descent parser, a numerical equation solver, and a symplectic integrator for 2D rigid body dynamics.

Below is an overview of the implementation details and the architectural decisions behind them.

## 📚 Documentation Modules

| Topic | Description | Link |
| :--- | :--- | :--- |
| **🧮 Math Parsing** | Recursive descent lexer & parser documentation. Explains how strings are converted into Abstract Syntax Trees (ASTs). | [View Docs](/docs/parsing.md) |
| **⚙️ Math Processing** | Details on the `Environment` runtime, variable resolution, and the Newton-Raphson root solver for dynamic evaluation. | [View Docs](/docs/processing.md) |
| **💥 Collisions** | Deep dive into the **Direct Edge Intersection** algorithm, explaining why it was chosen over SAT to handle concave bodies. | [View Docs](/docs/collisions.md) |
| **⏱️ Ticking** | The core game loop. break down of the order of operations: Evaluation -> Integration -> Collision -> Resolution. | [View Docs](/docs/ticking.md) |
| **📈 Integration** | Specifics on the **Symplectic Euler (Leapfrog)** integrator used to maintain energy stability over time. | [Jump to Section](/docs/ticking.md#2-integration-strategy-leapfrog) |

---

## 🤖 Notice of AI Assistance

> **Transparency Note**
>
> **Google Gemini** was heavily utilized for the **Framing and styling** of React components in the frontend (`web/` directory) and for generating parts of this documentation.
>
> ⚠️ **Core Implementation:** All physics logic, mathematical solvers, and engine algorithms (specifically within `engine/`) were conceptualized and written by myself without AI generation.