# HyperReview Development Guidelines

Auto-generated from all feature plans. Last updated: 2025-12-30

## Active Technologies
- TypeScript 5.x + React 18+ (Frontend), Rust 1.75+ (Backend) + Tauri 1.8/2.x, Vite, Zustand, git2-rs, rusqlite, reqwest, serde, thiserror (001-merge-new-frontend)
- SQLite for backend metadata (hyper_review.db), localStorage for frontend user preferences (001-merge-new-frontend)

- Rust 1.75+ (Tauri v2), TypeScript 5+ (React 18) + git2-rs (Git operations), rusqlite (metadata), reqwest (HTTP client), serde (serialization) (005-gerrit-integration)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 1.75+ (Tauri v2), TypeScript 5+ (React 18): Follow standard conventions

## Recent Changes
- 001-merge-new-frontend: Added TypeScript 5.x + React 18+ (Frontend), Rust 1.75+ (Backend) + Tauri 1.8/2.x, Vite, Zustand, git2-rs, rusqlite, reqwest, serde, thiserror

- 005-gerrit-integration: Added Rust 1.75+ (Tauri v2), TypeScript 5+ (React 18) + git2-rs (Git operations), rusqlite (metadata), reqwest (HTTP client), serde (serialization)

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
