# OminiX Runtime Workstreams

These are executable contracts for agent workers. Each workstream should become
or map to a GitHub issue.

## ORT-001 Runtime Core Domain Model

Owner: runtime core worker

Scope:

- `crates/ominix-runtime-core/**`
- `docs/RUNTIME_CONTRACT.md`

Deliverables:

- finalized Rust structs and enums for `GenerateRequest`, `RequestState`,
  `BackendCapability`, `BatchPlan`, `DecodeStep`, `KVLease`, and `WorkerEvent`
- no backend-specific dependencies
- unit tests for state and event invariants

Acceptance:

- `cargo test` passes
- public structs are documented
- no CUDA, CANN, SGLang, or HTTP dependencies leak into core

## ORT-002 Backend Adapter ABI

Owner: backend ABI worker

Scope:

- `crates/ominix-runtime-core/src/lib.rs`
- `docs/RUNTIME_CONTRACT.md`

Deliverables:

- `BackendAdapter` trait
- capability negotiation semantics
- submit, plan, poll, abort lifecycle
- error taxonomy

Acceptance:

- fake backend implements the trait
- docs explain how SGLang, CUDA, and Ascend adapters implement the same ABI

## ORT-003 Request State Machine

Owner: scheduler state worker

Deliverables:

- explicit transition table
- invalid transition tests
- request timeout and abort semantics
- event finalization rules

Acceptance:

- every active request finishes with exactly one terminal event
- abort is idempotent

## ORT-004 BatchPlan And DecodeStep

Owner: batching contract worker

Deliverables:

- backend-neutral batch planner interface
- prefill/decode/mixed phase representation
- token budget accounting
- conformance tests with fake backend

Acceptance:

- planner can form deterministic batches from waiting requests
- backend can reject impossible plans with typed error

## ORT-005 KV Lease Contract

Owner: KV contract worker

Deliverables:

- KV block id and lease lifecycle
- reservation, release, eviction hooks
- future tiered memory fields documented
- compatibility notes for Mooncake, HiCache, and OminiX native KV

Acceptance:

- fake KV allocator can simulate capacity exhaustion
- runtime rejects or backpressures requests on KV exhaustion

## ORT-006 Capability Model And Planner Inputs

Owner: capability worker

Deliverables:

- capability schema for CUDA, Ascend, SGLang
- model id canonicalization hook
- graph capture, FP8 KV, paged KV, split prefill/decode flags
- planner input doc for future auto configuration

Acceptance:

- router can compare workers without backend-specific code
- capability mismatch errors are structured

## ORT-007 Fake Backend Conformance Suite

Owner: test harness worker

Deliverables:

- fake backend crate or module
- deterministic stream tests
- capacity and abort tests
- compatibility test checklist for real backends

Acceptance:

- all tests run without GPU/NPU
- failures identify the violated contract

## ORT-008 SGLang Bridge Retirement Plan

Owner: migration worker

Deliverables:

- map `ominix-sglang` functions to runtime/router ownership
- identify which pieces are discarded, migrated, or kept as fallback
- staged plan to use SGLang only as a backend adapter

Acceptance:

- no long-term architecture doc treats `ominix-sglang` as the center of the stack
- SGLang remains a fallback/reference backend

