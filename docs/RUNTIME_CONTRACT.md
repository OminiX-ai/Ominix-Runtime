# Runtime Contract

This contract defines the boundary between `ominix-router` and hardware or
engine backends.

## Non-Goals

- No OpenAI-compatible JSON in the runtime layer.
- No tokenizer or chat-template ownership in this layer.
- No direct CUDA or CANN API types in backend-neutral structs.
- No backend-specific graph capture flags in router-visible request schemas.

## Request Contract

`GenerateRequest` must contain:

- stable request id
- canonical model id
- tokenized `input_ids`
- sampling limits required by the runtime
- streaming preference

Tokenizer and chat template work belongs above this layer or in a dedicated
token boundary adapter. Backends can expose tokenizer helpers, but runtime
scheduling should not depend on public prompt text.

## Capability Contract

Each backend worker must report:

- worker id
- backend kind: SGLang, OminiX-CUDA, OminiX-Ascend, Fake
- canonical model id
- maximum batch token budget
- maximum running requests
- paged KV support
- FP8 KV support
- graph capture support
- prefill/decode split support

Runtime admission must reject a request before enqueue if no worker can satisfy
the requested model and resource constraints.

## State Contract

Allowed state transitions:

```text
Waiting -> Prefill -> Decode -> Draining -> Finished
Waiting -> Aborted
Prefill -> Aborted
Decode -> Aborted
Any active state -> Failed
```

Backends may internally have more states. They must map them into this state
contract before emitting events.

## Batch Contract

`BatchPlan` is a runtime-level plan, not a backend kernel launch descriptor.

It must include:

- target worker
- phase: prefill, decode, or mixed
- request ids
- token budget

Backend-specific adapters may reject a batch with `CapabilityMismatch` when a
runtime plan violates backend rules. That rejection should be treated as a
runtime bug unless the capability changed between planning and execution.

## KV Lease Contract

`KVLease` reserves KV blocks for a request on a worker. The contract must support
future tiered memory and remote transfer without changing public API contracts.

Initial fields:

- request id
- worker id
- block ids
- tokens reserved

Future fields:

- tier: device, host, local disk, remote
- dtype
- page size
- prefix hash
- lease expiry
- pin priority

## Event Contract

Backends emit `WorkerEvent` records:

- `PrefillDone`
- `Token`
- `Usage`
- `Done`
- `Error`

The runtime must preserve ordering per request. Cross-request ordering is not
guaranteed.

## Conformance Gates

A backend is runtime-compatible only when it passes:

- submit accepted request
- reject unsupported model
- reject capacity overflow
- emit prefill before token
- emit done or error exactly once
- abort active request
- report capabilities
- survive fake KV exhaustion
- run deterministic fake backend tests without GPU/NPU

