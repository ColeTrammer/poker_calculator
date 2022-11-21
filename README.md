# Poker Calculator

## Running locally

### One Time Setup

```sh
cargo install cargo-make
cargo install wasm-pack --features curl/static-curl
```

### Backend

To start the backend, use the following commands. The watch command restarts the server when any backend file changes, which runs at http://localhost:8000.

```sh
cd backend
cargo make watch
```

### Frontend

To start the frontend, use the following commands. This starts a static file server at http://localhost:3000, and recompiles the frontend on change.

```sh
cd frontend
cargo make watch
```

### Testing and Linting

Tests and linting also use `cargo-make`.

```sh
cargo make test     # Test
cargo make clippy   # Lint
```
