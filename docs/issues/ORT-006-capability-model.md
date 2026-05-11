# ORT-006 Capability Model And Planner Inputs

Define capabilities that let router/runtime compare SGLang, CUDA, and Ascend
workers without backend-specific code.

## Requirements

- backend kind
- model id
- max batch tokens
- max running requests
- paged KV, FP8 KV, graph capture, prefill/decode split support
- future planner input fields

## Acceptance

- capability mismatch is a typed runtime error
- router can perform model/backend selection from capability records alone

