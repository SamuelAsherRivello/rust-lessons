# AGENTS.md

## Project Overview

This repository is a small Rust lessons workspace. Lesson crates live under
[`Lessons`](./Lessons).

The active lesson crates are:

| Package | Path | Role |
|---|---|---|
| `lesson_01_hello_world` | `Lessons/Lesson_01_HelloWorld` | Minimal `main` function that prints to the terminal. |
| `lesson_02_primitive_types` | `Lessons/Lesson_02_PrimitiveTypes` | Logs common Rust value types from named functions. |
| `lesson_03_custom_types` | `Lessons/Lesson_03_CustomTypes` | Demonstrates structs and enums. |
| `lesson_04_syntax` | `Lessons/Lesson_04_Syntax` | Demonstrates flow-control syntax. |
| `lesson_05_unit_tests` | `Lessons/Lesson_05_UnitTests` | Demonstrates small functions with unit tests. |
| `lesson_06_scoping` | `Lessons/Lesson_06_Scoping` | Demonstrates ownership, borrowing, and scopes. |
| `lesson_07_generics` | `Lessons/Lesson_07_Generics` | Demonstrates generic functions and structs. |
| `lesson_08_traits` | `Lessons/Lesson_08_Traits` | Demonstrates shared behavior through traits. |
| `lesson_09_attributes_and_macros` | `Lessons/Lesson_09_AttributesAndMacros` | Demonstrates attributes and simple macros. |
| `lesson_10_patterns` | `Lessons/Lesson_10_Patterns` | Demonstrates observer, strategy, and builder patterns. |

## Developer Workflows

### 01. First-Time Setup

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./Scripts/Common/Install_Dependencies.ps1
```

### 02. Run Lessons

| # | Script | Direct command |
|---|---|---|
| 01 | `./Scripts/Common/Run_Lesson_01_HelloWorld.ps1` | `cargo run --manifest-path ./Lessons/Lesson_01_HelloWorld/Cargo.toml` |
| 02 | `./Scripts/Common/Run_Lesson_02_PrimitiveTypes.ps1` | `cargo run --manifest-path ./Lessons/Lesson_02_PrimitiveTypes/Cargo.toml` |
| 03 | `./Scripts/Common/Run_Lesson_03_CustomTypes.ps1` | `cargo run --manifest-path ./Lessons/Lesson_03_CustomTypes/Cargo.toml` |
| 04 | `./Scripts/Common/Run_Lesson_04_Syntax.ps1` | `cargo run --manifest-path ./Lessons/Lesson_04_Syntax/Cargo.toml` |
| 05 | `./Scripts/Common/Run_Lesson_05_UnitTests.ps1` | `cargo run --manifest-path ./Lessons/Lesson_05_UnitTests/Cargo.toml` |
| 06 | `./Scripts/Common/Run_Lesson_06_Scoping.ps1` | `cargo run --manifest-path ./Lessons/Lesson_06_Scoping/Cargo.toml` |
| 07 | `./Scripts/Common/Run_Lesson_07_Generics.ps1` | `cargo run --manifest-path ./Lessons/Lesson_07_Generics/Cargo.toml` |
| 08 | `./Scripts/Common/Run_Lesson_08_Traits.ps1` | `cargo run --manifest-path ./Lessons/Lesson_08_Traits/Cargo.toml` |
| 09 | `./Scripts/Common/Run_Lesson_09_AttributesAndMacros.ps1` | `cargo run --manifest-path ./Lessons/Lesson_09_AttributesAndMacros/Cargo.toml` |
| 10 | `./Scripts/Common/Run_Lesson_10_Patterns.ps1` | `cargo run --manifest-path ./Lessons/Lesson_10_Patterns/Cargo.toml` |

Script command shape:

```powershell
powershell.exe -ExecutionPolicy Bypass -File ./Scripts/Common/Run_Lesson_01_HelloWorld.ps1
```

### Build Only

```powershell
cargo check --workspace
```

### Test

```powershell
cargo test --workspace
```

## Conventions

- Keep each lesson self-contained with its own `Cargo.toml` and `src/main.rs`.
- Keep lesson `Cargo.toml` files basic: package metadata plus direct dependencies.
- Use TitleCase lesson folder names under `Lessons`.
- Use underscore-separated TitleCase script names under `Scripts/Common`, such as
  `Install_Dependencies.ps1` and `Run_Lesson_01_HelloWorld.ps1`.
- Lessons in this repo should log to the terminal only unless the user explicitly
  asks for UI, graphics, networking, or external services.
- Do not add shared crates, runtime plugin systems, or framework-specific
  scaffolding unless the user explicitly asks.
