# HyperReview Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-12-13

## Active Technologies
- TypeScript 5+ (React), Rust 1.75+ (Tauri v2) + Tauri v2, React 18, Vite, git2-rs, rusqlite, tree-sitter (002-frontend-backend-integration)
- SQLite (via rusqlite) for local metadata; file system for git repositories (002-frontend-backend-integration)

- Rust 1.75+ (Tauri v2 compatibility) + git2-rs (Git operations), tree-sitter (code analysis), rusqlite (local metadata), reqwest (HTTP client), rayon (concurrency), thiserror + anyhow (error handling) (001-backend-implementation)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 1.75+ (Tauri v2 compatibility): Follow standard conventions

## Recent Changes
- 002-frontend-backend-integration: Added TypeScript 5+ (React), Rust 1.75+ (Tauri v2) + Tauri v2, React 18, Vite, git2-rs, rusqlite, tree-sitter

- 001-backend-implementation: Added Rust 1.75+ (Tauri v2 compatibility) + git2-rs (Git operations), tree-sitter (code analysis), rusqlite (local metadata), reqwest (HTTP client), rayon (concurrency), thiserror + anyhow (error handling)

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
