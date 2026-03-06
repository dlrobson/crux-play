coverage-html-directory := ""
coverage-threshold := "0"
profile := "dev"
docker-image := file_name(justfile_directory())
docker-tag := "local"
[private]
_coverage-html-output-directory-argument := if coverage-html-directory != "" { "--output-dir=" + coverage-html-directory } else { "" }

default: audit build lint format-check test

audit:
    cargo deny --locked check

build: cargo-build web-build

lint: cargo-lint web-lint

format: cargo-fmt web-format

format-check: (cargo-fmt "--check") web-format-check

test: cargo-test

docker-build:
    docker build -t {{ docker-image }}:{{ docker-tag }} -f images/production/Dockerfile .

# ── Cargo ───────────────────────────────────────────────────────────────────

cargo-build:
    cargo build --locked --profile {{ profile }}

cargo-lint *ARGS:
    cargo clippy --locked --profile {{ profile }} --workspace --all-features --all-targets {{ ARGS }} -- -D warnings

cargo-lint-fix *ARGS: (cargo-lint "--fix" "--allow-dirty" ARGS)

cargo-coverage:
    cargo +nightly llvm-cov --all-features --workspace --locked --branch
    cargo +nightly llvm-cov report --html {{ _coverage-html-output-directory-argument }} --fail-under-lines={{ coverage-threshold }}

cargo-doc $RUSTDOCFLAGS="-D warnings":
    cargo doc --locked --profile {{ profile }} --lib --no-deps --all-features --document-private-items

cargo-fmt *ARGS:
    cargo +nightly fmt {{ ARGS }}

cargo-test *ARGS:
    cargo test --locked --profile {{ profile }} {{ ARGS }}

cargo-unit-test: (cargo-test "--lib")

# ── Web ─────────────────────────────────────────────────────────────────────

_web := justfile_directory() / "web"

# Build all Rust-generated web artifacts (typegen + wasm)
web-artifacts:
    cargo xtask

web-install:
    cd {{ _web }} && npm install

web-clean:
    rm -rf \
        {{ _web }}/generated \
        {{ _web }}/build \
        {{ _web }}/node_modules \
        {{ _web }}/.svelte-kit \
        {{ justfile_directory() }}/.pnpm-store

web-build: web-artifacts web-install
    cd {{ _web }} && npm run build

# Run npm build only — assumes web artifacts are already present on disk (used in CI)
web-build-only: web-install
    cd {{ _web }} && npm run build

web-lint: web-artifacts web-install
    cd {{ _web }} && npm run lint

# Run npm lint only — assumes web artifacts are already present on disk (used in CI)
web-lint-only: web-install
    cd {{ _web }} && npm run lint

web-lint-fix: web-install
    cd {{ _web }} && npm run lint:fix

web-format: web-install
    cd {{ _web }} && npm run format

web-format-check: web-install
    cd {{ _web }} && npm run format:check

web: web-build
    cd {{ _web }} && npm run preview -- --host

web-dev: web-artifacts web-install
    cd {{ _web }} && npm run dev -- --host
