# ORT-008 SGLang Bridge Retirement Plan

Map the current `ominix-sglang` bridge into the future router/runtime/backend
architecture.

## Requirements

- classify each `ominix-sglang` function as migrate, discard, or keep as
  fallback
- define the SGLang backend adapter role
- identify tests that should move into runtime conformance

## Acceptance

- no long-term architecture doc keeps `ominix-sglang` as the primary runtime
- SGLang remains available as a compatibility backend

