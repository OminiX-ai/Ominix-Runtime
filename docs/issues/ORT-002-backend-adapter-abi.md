# ORT-002 Backend Adapter ABI

Define the first backend adapter ABI for SGLang, OminiX-CUDA, OminiX-Ascend,
and fake backends.

## Scope

- `crates/ominix-runtime-core/src/lib.rs`
- `docs/RUNTIME_CONTRACT.md`

## Requirements

- add adapter lifecycle: capability, submit, plan, poll events, abort
- define capability mismatch behavior
- document synchronous and async implementation options
- keep the first ABI small enough for fake backend conformance

## Acceptance

- fake backend implements the ABI
- docs explain how a real backend maps its internal scheduler to the ABI

