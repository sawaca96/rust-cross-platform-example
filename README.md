# Calculator Library

This is a Rust-based calculator library that can be used in both JavaScript (via WebAssembly) and Python. It provides basic arithmetic operations: addition, subtraction, multiplication, and division.

## Features

- **JavaScript**: Provides WebAssembly functions to be used in JavaScript.
- **Python**: Provides Python bindings using `pyo3`.

## Prerequisites

- Rust and Cargo installed
- Node.js and Yarn installed
- Python and Poetry installed

# Building for JavaScript

- cd js
- yarn install
- yarn build
- yarn serve

# Building for Python

- cd python
- poetry install
- poetry run maturin develop --manifest-path ../Cargo.toml --features python-extension
- poetry run pytest
