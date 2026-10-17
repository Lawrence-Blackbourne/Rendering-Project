# Vulkan Rendering Library (`rendering-project`)
A work-in-progress, high-level graphics library written in **Rust** built on top of **Vulkan** and **GLFW**. The goal is creating an idiomatic rust API to allow for rendering, keeping the controlability and high performance of Vulkan but increasing simplicity.

## Architecture & Workspace Structure
This repository is organised as a Cargo workspace containing two crates:

### 1. `rendering-project` (Core Vulkan Abstraction, Waiting on further progress from xml-parser to proceed)
Provides a modular wrapper around the Vulkan setup and rendering pipeline

### 2. `xml-parser` (Custom Streaming XML Parser & Vulkan Format Codegen)
A zero-dependency streaming XML parser designed to parse the official Khronos Vulkan API registry
specification (`vk.xml`) and generate Rust data structures.

## Tech Stack
* **Language:** Rust (Edition 2024)
* **Graphics & Windowing:** Vulkan (`ash` 0.38), `glfw` 0.62, but work is being done to remove ash and use the Vulkan API directly.
* **Benchmarking:** `criterion` 0.8 (`benches/test.rs`)

## Building & Running

```bash
# Build the workspace
cargo build

# Run unit tests
cargo test

# Run Criterion benchmarks
cargo bench
```

Note that the code does not currently compile.
