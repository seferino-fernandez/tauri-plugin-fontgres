default:
    @just --list

# Full gate: everything CI checks
verify:
    cargo fmt --all --check
    just lint
    just test
    just doc
    just js
    just example
    just package

# Lint the Rust code with Clippy, failing on any warning
lint:
    cargo clippy --all-targets -- -D warnings

# Run the Rust tests
test:
    cargo test --all-targets

# Build the API docs, failing on rustdoc warnings
doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

# Build the JavaScript bindings into dist-js and type-check them
js:
    pnpm install --frozen-lockfile
    pnpm build
    pnpm exec tsc --noEmit -p .

# Type-check and build the example app's frontend, then compile its Rust side
example: js
    cd examples/tauri-app && pnpm install --frozen-lockfile && pnpm build
    cd examples/tauri-app/src-tauri && cargo check

# Check what the crate and the npm package would publish
package:
    cargo publish --dry-run --allow-dirty
    npm pack --dry-run

# Format the Rust code and apply Clippy's auto-fixable suggestions
format:
    cargo fmt --all
    cargo clippy --all-targets --fix --allow-dirty --allow-staged

# Run the example app
dev: js
    cd examples/tauri-app && pnpm install --frozen-lockfile && pnpm tauri dev
