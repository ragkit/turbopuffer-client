# Read up on just here: https://github.com/casey/just

set fallback := true
set shell := ["bash", "-uc"]
set windows-shell := ["sh", "-uc"]

# `just --list` (or just `just`) will print all the recipes in the current
# Justfile. `just RECIPE` will run the macro/job.
_default:
  @just --list

# List all recipes.
list:
  @just --list

# Install required packages.
install:
  pnpm install

# Builds everything.
build:
  cargo build

# Test everything.
test:
  cargo test

# Typically doesn't need to be run.
# Format in editor/on commit should do this automatically.
format:
  cargo fmt
  pnpm format

#
publish *args:
  cargo publish --dry-run {{ args }}

# TODO: Move this to CI/CD.
#
publish-push *args:
  cargo publish {{ args }}
