# AGENTS.md

Instructions for automated assistants working in this repository.

## Repo overview

- Python bindings for the MXP Protocol.
- `docs/`: positioning, comparisons, demos, community info.
- Build pipeline uses `maturin` plus Rust core.

## Preferred workflows

- Keep changes focused and small.
- For code changes, run the relevant tests and linters.
- For docs-only edits, tests are optional.

## Tests and checks

- `pytest tests/`
- `black python/ tests/`
- `ruff check python/ tests/`
- `cargo fmt`
- `cargo clippy -- -D warnings`

## Documentation entry points

- `README.md`
- `docs/positioning.md`
- `docs/comparison.md`
- `docs/demos.md`
- `docs/community.md`
