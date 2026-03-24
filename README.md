# Rust Craft Optimizer GPL

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A **craft optimizer engine** written in Rust, designed to compute optimal production chains for games with crafting mechanics.

> **GPL-3.0 licensed** — this project is and will remain free software.

---

## Status

**Work in progress** — This is the initial setup phase. Core architecture is being defined, and the first building blocks are being put in place.

**What's coming:**
- [ ] Core data structures (`Resource`, `ResourceSet`, `Recipe`)
- [ ] Optimization algorithm
- [ ] Universal JSON format
- [ ] CLI interface
- [ ] REST API
- [ ] Desktop GUI
- [ ] Game converters (Minecraft, Factorio, ...)

---

## Vision

A **game-agnostic** engine that can:
- Read recipes from any game (via a universal JSON format or native converters)
- Calculate optimal crafting sequences based on chosen criteria (time, resources, steps)
- Be used as a library, a CLI tool, a REST API, or a desktop application

---

## Project Structure

- Rust-Craft-Optimizer-GPL/
  - crates/
    - rcogpl-core/ — Generic optimization engine (planned)
    - rcogpl-shared/ — Common types (planned)
    - rcogpl-universal/ — Universal JSON format plugin (planned)
    - rcogpl-api/ — REST API server (planned)
    - rcogpl-gui/ — Desktop GUI (planned)
  - Cargo.toml — Workspace configuration
  - LICENSE — GPL-3.0

---

## Getting Started

### Build

```bash
git clone https://github.com/ThMrre/Rust-Craft-Optimizer-GPL.git
cd Rust-Craft-Optimizer-GPL
cargo build
```

## License
GNU General Public License v3.0 — see LICENSE.

## About
This is a personal project built to learn Rust, explore optimization algorithms, and create something useful for the crafting game community. Contributions and suggestions are welcome once the foundation is stable.
