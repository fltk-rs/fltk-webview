# Repository Guidelines

## Project Structure & Module Organization
- Root crate: `Cargo.toml`, library entry at `src/lib.rs` (public `Webview` API).
- FFI layer: `fltk-webview-sys/` with `build.rs` and platform helpers (`cocoa_helper.m`, `gtk_helper.c`).
- Examples: `examples/*.rs` (e.g., `basic.rs`, `dispatch.rs`, `markdown.rs`).
- CI: `.github/workflows/rust.yml` builds on Windows, macOS, and Linux.
- Assets: `screenshots/` used in the README.

## Build, Test, and Development Commands
- Build: `cargo build` (add `--features fltk/fltk-bundled` to use bundled FLTK like CI).
- Run example: `cargo run --example basic` (replace `basic` with any file in `examples/`).
- Lint: `cargo clippy --all-targets -- -D warnings` (treat warnings as errors).
- Format: `cargo fmt --all` (required before PRs).
- Docs: `cargo doc --open` (browse crate docs, README is included).
- Test: `cargo test` (add tests as described below; currently minimal).

Linux note: install `libwebkit2gtk-4.1-dev` and common X11 deps (see workflow). Windows/macOS need no extra packages.

## Coding Style & Naming Conventions
- Rust style with `rustfmt` (4-space indent, max line width per default config).
- Naming: `snake_case` for functions/modules, `CamelCase` for types/traits, `SCREAMING_SNAKE_CASE` for consts.
- Keep platform-specific code behind `cfg` gates and isolate FFI in `fltk-webview-sys`.

## Testing Guidelines
- Prefer small unit tests co-located with modules (`#[cfg(test)] mod tests { ... }`) or `tests/` for integration.
- Name tests after behavior, e.g., `creates_webview_from_fltk_window`.
- Run with `cargo test`; avoid platform GUI assumptions—mock or guard with `#[cfg]`.

## Commit & Pull Request Guidelines
- Commits: imperative present, concise, scoped. Example: `fix: correct SetFocus usage on Windows` or `docs: expand usage section`.
- PRs must include: summary, rationale, affected platforms, screenshots if UI-visible, and any linked issues.
- CI must pass on all platforms; run `cargo fmt`, `cargo clippy`, and build examples locally where feasible.
- For Linux changes, note any new `apt`/`dnf` packages or `pkg-config` requirements.

