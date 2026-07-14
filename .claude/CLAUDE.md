# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`simple-re-engine` is a from-scratch regular-expression engine written in Rust (edition 2024, no external dependencies). It is at an early stage: only the **parsing/tokenization** front-end exists. There is no matcher/executor yet, and `main` runs a hard-coded regex to print parse diagnostics rather than exposing a real CLI or library API.

## Commands

```sh
cargo build            # compile
cargo run              # run main.rs (parses the hard-coded email regex, prints debug output)
cargo test             # run tests (none exist yet)
cargo test <name>      # run a single test by name substring
cargo check            # fast type-check without producing a binary
cargo clippy           # lint
```

## Architecture

The parser turns a regex string into a flat `Vec<Token>`, driven by a single cursor.

- **`ParseContext`** (`src/parser.rs`) is the shared mutable state threaded through every parse step: `pos` (the cursor index into the regex string) and `tokens` (the accumulated output). Groups are parsed into their **own** child `ParseContext` whose `tokens` become the `value` of a single `Group` token in the parent — this is how nesting is represented.

- **`process_general`** (`src/parser.rs`) is the central dispatch: it looks at the character under `ctx.pos` and routes `(`, `[`, `|`, quantifiers (`* ? +`), and `{` to their handlers, defaulting to advancing the cursor for literals.

- **`Parser` trait + `Process::process`** (`src/parser.rs`): each construct (`Group`, `Bracket`, …) is a unit struct implementing `Parser::parse`. `Process::process` is a thin generic dispatcher over the trait. Each handler is responsible for advancing `ctx.pos` past the construct it consumes.

- **`Token` / `TokenType`** (`src/token.rs`): a token is a `TokenType` tag plus a type-erased `value: Box<dyn Any>`. The concrete payload type depends on the tag (e.g. a `Group`'s value is a `Vec<Token>`), so consumers must downcast via `Any`.

### Important notes on the current state

- **Cursor advancement is a shared responsibility and currently inconsistent.** `main::parse` increments `ctx.pos` in its loop, `process_general` also increments in most match arms, and individual handlers (`Group`, `Bracket`) increment too. When changing parsing logic, trace who owns the `pos` increment for a given path to avoid double-advancing or skipping characters.
- Several `TokenType` variants (`Or`, `Repeat`, `Literal`, `GroupUncaptured`) and quantifier/`{` handling are declared but not yet implemented — the arms are stubs that only advance the cursor.
- Parsing mixes byte indexing (`regex.as_bytes()[ctx.pos]`) with char indexing (`regex.chars().nth(ctx.pos)`); these agree only for ASCII. Keep this in mind before adding Unicode support.
