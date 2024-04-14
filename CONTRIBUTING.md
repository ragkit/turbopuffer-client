# Contributing

# Overview

- Install toolchains on your computer:
  - [`just`](https://github.com/casey/just#installation) (command runner)
  - [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html) (rust)
  - [`pnpm`](https://pnpm.io/installation) (javascript)

## Setup

This sets up some commit hooks, lints, and formatting toolchains.

```
pnpm i
```

## Build

```
cargo build
```

## Test

_Note: You need to create a `.env` file to run tests locally. See `.env.example`._

```
cargo test
```

## Commit style

Commits should follow [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/). Quick summary:

- All lowercase
- Message format: `<type>: <description>`
  - Where type is like: `chore`, `feat`, `docs`, `ci`, `style`, etc.
  - And description is a description of what the commit does.

# Code of Conduct

Contributions, including communications over issues, must follow our code of conduct. See: [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md).
