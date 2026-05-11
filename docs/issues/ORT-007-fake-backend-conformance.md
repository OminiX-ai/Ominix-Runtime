# ORT-007 Fake Backend Conformance Suite

Build a GPU-free conformance suite for runtime-compatible backends.

## Requirements

- deterministic fake backend
- streaming event fixture
- capacity overflow fixture
- abort fixture
- finalization fixture

## Acceptance

- `cargo test` runs locally without CUDA, CANN, or SGLang
- each failure points to a named runtime contract violation

