# 🦀 rust-playground

A personal playground to learn, explore, and experiment with the Rust programming language.

This repo is organized as a series of small, self-contained projects — from simple syntax demos to CLI tools and web backends.

---

## 📚 Learning Plan

### 🧭 Why Rust?
As a Machine Learning Engineer, learning Rust offers:
- Systems-level control (great for performance-critical workloads)
- Memory safety without garbage collection
- Potential to build ML tooling or WebAssembly-backed inference

---

## 🚀 Phases

### 🟢 Phase 1: Core Language
✅ Goal: Understand Rust syntax, memory model, and toolchain

| Week | Topic                     | Project Idea                  |
|------|---------------------------|--------------------------------|
| 1    | Install + `hello_world`   | Print "Hello, Rustacean!"      |
| 1    | Variables, Functions      | Unit converter (e.g., km → mi) |
| 2    | Ownership & Borrowing     | Text reversal tool             |
| 2    | Structs, Enums            | Basic task manager (CRUD)      |
| 3    | Pattern Matching          | FizzBuzz + file I/O            |
| 3    | Collections               | Word frequency counter         |

### 🟡 Phase 2: Intermediate Rust
✅ Goal: Build real-world utilities and CLI tools

| Week | Topic                     | Project Idea                        |
|------|---------------------------|-------------------------------------|
| 4    | Error Handling            | CSV parser / summary tool           |
| 4    | Modules & Crates          | Random password generator           |
| 5    | Traits & Generics         | Pluggable calculator engine         |
| 5    | Testing & Macros          | Unit-tested CLI quiz                |

### 🔴 Phase 3: Real Apps
✅ Goal: Understand web, async, and systems programming

| Week | Topic                     | Project Idea                        |
|------|---------------------------|-------------------------------------|
| 6    | Async / `tokio`           | File downloader with progress bar   |
| 6    | Web APIs (`actix` / `axum`)| Joke API server                     |
| 7    | WebAssembly               | Web-based Rust calculator (via WASM)|
| 8    | FFI / C Interop           | Call Rust from Python (or vice versa) |

---

## 🛠️ Setup

### 1. Install Rust
```bash
brew install rustup
rustup-init
source $HOME/.cargo/env
rustc --version
```

### 2. Install CLI tools
```bash
cargo install cargo-edit      # Add/remove dependencies easily
cargo install cargo-outdated  # Show outdated crates
```

### 3. Recommended VSCode Extensions
Auto-suggested by `.vscode/extensions.json`:
- `rust-analyzer`
- `CodeLLDB`
- `Dependi`
- `Rust Test Lens`
- `Error Lens`
- `Better TOML`
- `Bracket Pair Colorizer 2`

## 🧠 Resources
- [The Rust Book (Official)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Crates.io (Libraries)](https://crates.io)
- [Cheats.rs – Handy Cheatsheet](https://cheats.rs)
- [Are We Learning Yet? (ML in Rust)](https://www.arewelearningyet.com/)

## 🙋‍♂️ About
Created by [Sagar](http://Sagar-py.github.io) — ML engineer exploring systems programming and low-level performance.
Join my Newsletter - http://dhokla-express.beehiiv.com