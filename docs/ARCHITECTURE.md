# OminiX Runtime Architecture

`ominix-runtime` is the shared runtime policy layer for OminiX inference. It is
not a public API server and not a CUDA or Ascend kernel repository.

## Layer Position

```text
Layer 0: OminiX-API
  OpenAI-compatible API, auth, response formatting, SSE

Layer 1: ominix-router
  queueing, admission, routing, health and load aggregation

Layer 2a: ominix-runtime
  request state machine, batch abstraction, KV lease contract, backend adapter

Layer 2b/3: OminiX-CUDA / OminiX-Ascend / SGLang adapter
  backend-aware scheduling, graph policy, rank mapping, kernels, device memory
```

## Design Principles

- Keep backend-neutral policy in Rust.
- Keep hardware-specific rules in the backend repos.
- Make all backend capabilities explicit before scheduling a request.
- Treat SGLang as a reference backend and compatibility target, not the final
  OminiX runtime owner.
- Make fake backends first-class so router/runtime conformance can be tested
  without a GPU or NPU.

## Core Objects

`GenerateRequest` is a tokenized request. Public OpenAI JSON should not cross
into this layer.

`RequestState` records the runtime lifecycle: waiting, prefill, decode,
draining, finished, aborted, or failed.

`BackendCapability` is the contract a worker advertises. It must include model,
backend kind, token capacity, graph support, KV support, and split prefill/decode
support.

`BatchPlan` is the backend-neutral decision for a step. It says which worker,
which phase, which requests, and which token budget.

`DecodeStep` binds a `BatchPlan` to KV leases. Backends may lower this into CUDA,
CANN, SGLang gRPC, or another engine.

`WorkerEvent` is the stream from runtime/backend back to router/API.

## Runtime Flow

1. `ominix-router` selects a runtime namespace and backend group.
2. `ominix-runtime` validates the request against backend capabilities.
3. Runtime transitions request state to waiting or rejects admission.
4. Runtime forms `BatchPlan` objects.
5. Runtime leases KV blocks through the KV contract.
6. Backend adapter executes prefill/decode and emits `WorkerEvent` records.
7. Router/API convert events into public stream responses.

## SGLang Migration Strategy

The working DeepSeek V4 Flash path still relies on SGLang for runtime policy.
Migration should be staged:

1. keep SGLang adapter as a backend
2. match SGLang request/event behavior in conformance tests
3. move request lifecycle ownership into `ominix-runtime`
4. add OminiX-CUDA and OminiX-Ascend native adapters
5. move KV lease management from implicit backend state to explicit runtime
   contracts
6. add prefill/decode disaggregation only after single-node runtime semantics
   are stable

## DeepSeek V4 Flash Validation Target

Runtime workstreams should validate CUDA behavior on the provisioned 8x5090
host. Credentials are intentionally not stored in this repository. Runtime
agents should treat that machine as a backend validation target, not as part of
the checked-in architecture.
