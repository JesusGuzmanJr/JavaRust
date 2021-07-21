# Rust Implementation

## Setup

### Rust

Install the xcode command line tools.

    xcode-select --install


Install the Rust toolchain version manager.

    brew install rustup


Install the latest Rust toolchain. This will install the latest `rustc` (the compiler), `cargo` (the package manager), and  update `rustup` (the toolchain version manager).

    rustup-init


Install the source file watcher `cargo-watch`.

    cargo install cargo-watch


Install the outdated dependency checker `cargo-outdated`.

    cargo install cargo-outdated


Install the linter `clippy`.

    rustup component add clippy

- Install docker: https://docs.docker.com/docker-for-mac/install/

## Development
Automatically build and run on source changes.

    cargo watch

Make a debug build.

    cargo build

Run unit tests run.

    cargo test

Make a production build.

    cargo build --profile production

Clear build artifacts.

    cargo clean

Check for outdated dependencies.

    cargo outdated

Make a production build image.

    docker build -t rust-car-rentals:latest

Run an ephemeral docker container.

    docker run --rm -it rust-car-rentals:latest
