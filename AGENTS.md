# AGENTS.md — AI Coding Agent Context & Rules

This repository is a full-stack web application featuring a **Rust (Axum)** backend and a **Vue.js (TypeScript/Bun)** frontend.
As an AI Agent, you **MUST** strictly adhere to the technical stack, directory boundaries, and architecture patterns outlined below.

---

## 1. System Architecture & Tech Stack

- **Backend**: `axum` (Web framework), `tokio` (Async runtime), `tower-http` (CORS/Layers), and `serde` (Serialization).
- **Frontend**: Located entirely inside the `ui/` directory. Vue 3 (Composition API), TypeScript, and `bun` as the package manager/runtime.

---

## 2. Core Directives & Guardrails

### 🦀 Rust & Axum Backend Rules
- **Extractor Ordering**: Axum extractors (`Path`, `Query`, `State`, `Json`) **MUST** be ordered correctly. `State` or `Json` extractors must be evaluated last according to Axum's compile-time rules.
- **Thread Safety**: All handlers and state types must be `Send + 'static`. Do **NOT** hold non-Send types (like `MutexGuard` or `Rc`) across an `.await` boundary. Use `Arc` and `tokio::sync::Mutex` if asynchronous synchronization is mandatory.
- **Code Style**: NEVER use `unsafe` unless explicitly authorized. Write exhaustive pattern matching; avoid uninformative catch-all `_` wildcards. Prefer functional iterator chains over manual loops.

### 🟢 VueJS & Bun Frontend Rules (`ui/` context)
- **Dependency Management**: **ONLY** use `bun` commands inside the `ui/` folder (e.g., `bun add <pkg>`, `bun run dev`). Do **NOT** generate `package-lock.json` or `yarn.lock`.
- **Vue Standards**: Write clean Vue 3 scripts using `<script setup lang="ts">`. Enforce strict TypeScript typing; avoid using `any`.

---

## 3. Operational & Verification Commands

Before declaring a task finished, execute validation steps using the following workflows:

### Backend Validation
```bash
# Formats, lints, and runs backend tests
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

### Frontend Validation
```bash
# Commands must be executed inside the ui/ directory
cd ui
bun run lint
bun run typecheck
bun run test:unit # if configured
```

---

## 5. Security & Maintenance Principles
- **No Hardcoded Secrets**: Never write API keys, database credentials, or secret keys into code or logs. Utilize `.env` files or system environment variables.
- **Error Handling**: Implement domain-specific error enums implementing `IntoResponse` for Axum rather than relying on generic string errors.
- **Formatting Enforcements**: Trust existing project configuration files (`rustfmt.toml`, `.eslintrc`, `.prettierrc`). Do not write custom formatters or bypass automated checks.
