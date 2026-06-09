# rust-oeis-playground

![Coverage](https://raw.githubusercontent.com/kitsuyui/octocov-central/main/badges/kitsuyui/rust-oeis-playground/coverage.svg)

This is a playground for implementing sequences from the [OEIS](https://oeis.org/) in Rust.

## Directory structure

- `src/{sequence_name}/mod.rs` - Contains the implementation of the sequence.

## Development

This repository uses [lefthook](https://lefthook.dev/) to run the same checks as CI
locally, so problems surface before they reach CI.

```sh
# Install the Git hooks (once; requires lefthook on your PATH)
lefthook install
```

Once installed, the hooks run automatically:

- **pre-commit**: `cargo fmt --all -- --check` and `cargo clippy -- -D warnings`
- **pre-push**: the above plus `cargo test`

CI still runs the full suite (see `.github/workflows/`); the hooks only bring that
feedback earlier on your machine.

## License

The OEIS sequences are licensed under the [OEIS End-User License Agreement](https://oeis.org/wiki/The_OEIS_End-User_License_Agreement). The implementations in this repository are licensed under the MIT license.

The other code parts in this repository are licensed under the MIT license.
