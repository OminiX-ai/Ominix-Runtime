# ORT-004 BatchPlan And DecodeStep Contract

Define backend-neutral batch planning without leaking CUDA or Ascend launch
details.

## Requirements

- prefill/decode/mixed phase model
- request id list
- token budget
- worker target
- typed rejection path for impossible plans

## Acceptance

- fake planner builds deterministic batches
- capacity tests prove token budget accounting

