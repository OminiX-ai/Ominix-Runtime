# ORT-001 Runtime Core Domain Model

Implement and stabilize the backend-neutral Rust domain model for
`ominix-runtime`.

## Scope

- `crates/ominix-runtime-core/**`
- `docs/RUNTIME_CONTRACT.md`

## Requirements

- define request, state, capability, batch, decode step, KV lease, event, and
  error types
- keep the crate backend-independent
- document each public type
- add invariant tests

## Acceptance

- `cargo test` passes
- no HTTP, CUDA, CANN, SGLang, tokenizer, or serde assumptions are required for
  core tests
- docs describe how router and backends consume each type

