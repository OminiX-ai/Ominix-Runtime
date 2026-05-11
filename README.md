# OminiX Runtime

`ominix-runtime` is the backend-neutral runtime policy layer for OminiX
inference serving.

It is the layer between traffic routing and hardware execution:

```text
OminiX-API
  -> ominix-router
  -> ominix-runtime
  -> OminiX-CUDA / OminiX-Ascend / SGLang adapter
```

## Role

The runtime owns Layer 2a: backend-neutral inference workflow policy.

It defines:

- request state machine
- backend capability model
- batch and decode-step abstractions
- KV lease and KV block registry contract
- scheduler events and stream semantics
- backend adapter interface for CUDA, Ascend, and SGLang
- conformance tests for runtime-compatible backends

It does not own:

- public HTTP/OpenAI endpoints
- user auth or API response formatting
- CUDA kernels or CANN kernels
- backend-specific graph capture and stream execution
- full model implementation

## Why This Exists

The current DeepSeek V4 Flash proof uses:

```text
OminiX-API -> ominix-sglang -> SGLang runtime -> CUDA backend
```

That path works, but SGLang still owns the runtime policy engine: scheduling,
batching, KV cache policy, and model runtime orchestration.

`ominix-runtime` is the long-term OminiX-owned replacement for that middle
runtime policy layer. The first goal is not to beat SGLang immediately. The
first goal is to make the contracts explicit enough that OminiX can gradually
move runtime ownership from SGLang into OminiX without breaking CUDA or Ascend
backend work.

The active CUDA validation target for DeepSeek V4 Flash is a provisioned 8x5090
host. Access details must be handed to operators securely and must not be
committed to this repository.

## First Milestone

Milestone 1 is a contract milestone:

- compile the core Rust type crate
- define the backend adapter boundary
- define `GenerateRequest`, `RequestState`, `BatchPlan`, `DecodeStep`,
  `KVLease`, `BackendCapability`, and `WorkerEvent`
- add a fake backend conformance test
- document migration from `ominix-sglang`

Milestone 1 is complete only when both `OminiX-CUDA` and `OminiX-Ascend` can
state their required capabilities through this contract, even if they still run
behind existing prototype services.

## Documents

- [Architecture](docs/ARCHITECTURE.md)
- [Runtime Contract](docs/RUNTIME_CONTRACT.md)
- [Workstreams](docs/WORKSTREAMS.md)

## Repo Status

This repository starts as a design and contract repo. It intentionally contains
only a small Rust core crate until the interfaces stabilize.
