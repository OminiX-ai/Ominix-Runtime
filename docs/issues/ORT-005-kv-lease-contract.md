# ORT-005 KV Lease Contract

Define the first KV block and lease API for OminiX runtime.

## Requirements

- KV block id
- request to block lease mapping
- reservation and release semantics
- future tiered-memory fields documented
- compatibility notes for Mooncake, HiCache, and native OminiX KV

## Acceptance

- fake KV allocator simulates capacity exhaustion
- runtime behavior is deterministic when KV is unavailable

