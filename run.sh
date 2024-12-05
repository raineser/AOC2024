#!/bin/sh

set -e # Exit early if any commands fail

(
  cd "$(dirname "$0")" # Ensure compile steps are run within the repository directory
  cargo build --release --manifest-path Cargo.toml
)

exec ./target/release/aoc2024 "$@"
