# Contributing to Rust Craft Optimizer GPL

First off, thank you for considering contributing to this project. This is a personal project built for learning and for the community. Any help is welcome — whether it's reporting a bug, suggesting a feature, writing code, or improving documentation.

## Current Status

The project is in **early setup phase**. The foundation is being built. If you're interested in contributing, please open an issue first to discuss what you'd like to work on.

## Branch Strategy

This project uses a simplified Git Flow workflow:

| Branch | Purpose |
|--------|---------|
| `main` | Production-ready code. Only updated via merges from `develop`. Never commit directly. |
| `develop` | Integration branch. All features and fixes are merged here. |
| `feat/*` | Feature branches. Start from `develop`. Example: `feat/add-minecraft-converter` |
| `fix/*` | Bugfix branches. Start from `develop`. Example: `fix/resource-set-bug` |

### Workflow Example

```bash
# 1. Start from develop
git checkout develop
git pull origin develop

# 2. Create a feature branch
git checkout -b feat/your-feature-name

# 3. Write code, commit, push
git push -u origin feat/your-feature-name

# 4. Open a Pull Request on GitHub targeting develop
```

Pull Requests are required for all changes. The main branch is protected — only merges from develop after review.

## How to Contribute

### 1. Open an Issue

Before writing code, open an issue to discuss:
- What you want to do
- Why it's useful
- How you plan to implement it

This avoids wasted work and ensures alignment with the project's direction.

### 2. Fork & Clone

Fork the repository and clone it locally:

```bash
git clone https://github.com/your-username/Rust-Craft-Optimizer-GPL.git
cd Rust-Craft-Optimizer-GPL
```

### 3. Create a Branch

Use a descriptive branch name:

```bash
git checkout -b feat/add-minecraft-converter
# or
git checkout -b fix/resource-set-bug
```

### 4. Write Code

Follow the conventions:
- Run cargo fmt before committing
- Run cargo clippy and fix warnings
- Write tests for new functionality
- Keep commits atomic (one logical change per commit)

### 5. Commit Messages

Use Conventional Commits:
- feat(core): add resource normalization
- fix(solver): prevent infinite loop on cycles
- docs(readme): update installation instructions
- test(universal): add validation tests

### 6. Push & Open a Pull Request

Push your branch and open a Pull Request against `develop`. Include:
- A clear title
- Description of what changed and why
- Link to related issue

## Project Structure

See README.md for the current structure.

## Rust Guidelines

- Use thiserror for library errors, anyhow for binaries
- Avoid unwrap() in library code — propagate errors with ?
- Prefer enum over stringly-typed values when possible
- Document public API with /// comments

## License

By contributing, you agree that your contributions will be licensed under the GNU General Public License v3.0.

## Questions?

Open an issue or reach out. All discussions are public to keep things transparent.

Thank you for helping make this project better.
